use aes::cipher::{BlockDecryptMut, KeyIvInit, block_padding::NoPadding};
use anyhow::Result;
use anyhow::anyhow;
use byteorder::BigEndian;
use byteorder::LittleEndian;
use byteorder::ReadBytesExt;
use hmac::Hmac;
use hmac::Mac;
use sha1::Sha1;
use std::fs::File;
use std::fs::create_dir;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

pub fn decrypt(
    save_path_str: &String,
    decrypt_path_str: &String,
    key: &[u8],
    need_check_hmac: bool,
    print_info: impl Fn(String),
) -> Result<()> {
    let save_path = Path::new(save_path_str);
    if !save_path.exists() || !save_path.is_dir() {
        return Err(anyhow!(format!(
            "指定的保存目录不存在或者不是文件夹，请检查。{}",
            save_path_str
        )));
    }

    let decrypt_path = Path::new(decrypt_path_str);
    if decrypt_path.exists() && !decrypt_path.is_dir() {
        return Err(anyhow!(
            "解密路径已经存在并且不是一个文件夹,decrypt_path:{}",
            decrypt_path_str
        ));
    }

    if !decrypt_path.exists() {
        create_dir(decrypt_path)?;
    }
    for entry in std::fs::read_dir(save_path)? {
        let entry = entry?;
        if let Some(extension) = entry.path().extension() {
            if extension != "db" {
                continue;
            }
        } else {
            continue;
        }
        let mut saved_file = File::open(entry.path())?;
        let mut buffer = vec![];
        saved_file.read_to_end(&mut buffer)?;
        let salt = &buffer[0..16];
        let mut byte_key = [0; 32];
        pbkdf2::pbkdf2::<Hmac<Sha1>>(key, salt, 64000, &mut byte_key)?;
        let first = &buffer[16..4096];
        let mac_salt = salt.iter().map(|i| i ^ 58).collect::<Vec<_>>();
        if check_hmac(first, &byte_key, &mac_salt, 1, 32)? {
            print_info(format!("文件{:?}密码正确", entry.file_name()));
            let pages: Vec<&[u8]> = buffer[..].chunks(4096).collect();
            let mut path_buf = PathBuf::from(decrypt_path);
            path_buf.push("decrypted_".to_owned() + entry.file_name().to_str().unwrap());
            let mut out_file = File::create(&path_buf)?;
            for (index, page) in pages.iter().enumerate() {
                let mut decrypted_page = vec![];
                if need_check_hmac {
                    decrypt_data_check_hmac(
                        index as u32 + 1,
                        page,
                        &byte_key,
                        &mut decrypted_page,
                        48,
                        48,
                        &mac_salt,
                        32,
                    )?;
                } else {
                    decrypt_data_uncheck_hmac(
                        index as u32 + 1,
                        page,
                        &byte_key,
                        &mut decrypted_page,
                        48,
                        48,
                    )?;
                }
                out_file.write_all(&decrypted_page)?;
            }
            out_file.flush()?;
            print_info(format!("文件{:?}解密成功", entry.file_name()));

            let saved_wal_file_str = save_path_str.to_owned()
                + std::path::MAIN_SEPARATOR_STR
                + entry.file_name().to_str().unwrap()
                + "-wal";
            let saved_wal_file_path = Path::new(&saved_wal_file_str);
            if saved_wal_file_path.exists() {
                let mut save_wal_file = File::open(&saved_wal_file_path)?;
                let mut save_wal_buffer = vec![];
                save_wal_file.read_to_end(&mut save_wal_buffer)?;
                if save_wal_buffer.len() != 0 {
                    let order_byte = save_wal_buffer[3];
                    let (mut dis_decrypt_sum1, mut dis_decrypt_sum2) =
                        get_check_sum(0, 0, &save_wal_buffer[..24], &order_byte)?;
                    let (mut decrypted_sum1, mut decrypted_sum2) =
                        get_check_sum(0, 0, &save_wal_buffer[..24], &order_byte)?;
                    path_buf.pop();
                    path_buf.push(
                        "decrypted_".to_owned()
                            + saved_wal_file_path.file_name().unwrap().to_str().unwrap(),
                    );
                    let mut decrypted_wal_file = File::create(&path_buf)?;
                    decrypted_wal_file.write_all(&save_wal_buffer[0..32])?;
                    let wal_frames: Vec<&[u8]> = save_wal_buffer[32..].chunks(24 + 4096).collect();
                    for wal_frame in wal_frames {
                        let mut decrypt_buf = vec![];
                        let mut cur = std::io::Cursor::new(&wal_frame[0..24]);
                        let page_index = cur.read_u32::<BigEndian>()?;
                        cur.set_position(16);
                        let wal_frame_sum1 = cur.read_u32::<BigEndian>()?;
                        let wal_frame_sum2 = cur.read_u32::<BigEndian>()?;
                        (dis_decrypt_sum1, dis_decrypt_sum2) = get_check_sum(
                            dis_decrypt_sum1,
                            dis_decrypt_sum2,
                            &wal_frame[..8],
                            &order_byte,
                        )?;
                        (dis_decrypt_sum1, dis_decrypt_sum2) = get_check_sum(
                            dis_decrypt_sum1,
                            dis_decrypt_sum2,
                            &wal_frame[24..],
                            &order_byte,
                        )?;
                        if need_check_hmac {
                            decrypt_data_check_hmac(
                                page_index,
                                &wal_frame[24..],
                                &byte_key,
                                &mut decrypt_buf,
                                48,
                                48,
                                &mac_salt,
                                32,
                            )?;
                        } else {
                            decrypt_data_uncheck_hmac(
                                page_index,
                                &wal_frame[24..],
                                &byte_key,
                                &mut decrypt_buf,
                                48,
                                48,
                            )?;
                        }
                        (decrypted_sum1, decrypted_sum2) = get_check_sum(
                            decrypted_sum1,
                            decrypted_sum2,
                            &wal_frame[..8],
                            &order_byte,
                        )?;
                        (decrypted_sum1, decrypted_sum2) = get_check_sum(
                            decrypted_sum1,
                            decrypted_sum2,
                            &decrypt_buf,
                            &order_byte,
                        )?;

                        if wal_frame_sum1 == dis_decrypt_sum1 && wal_frame_sum2 == dis_decrypt_sum2
                        {
                            decrypted_wal_file.write_all(&wal_frame[0..16])?;
                            decrypted_wal_file.write_all(&decrypted_sum1.to_be_bytes())?;
                            decrypted_wal_file.write_all(&decrypted_sum2.to_be_bytes())?;
                            decrypted_wal_file.write_all(&decrypt_buf)?;
                        } else {
                            decrypted_wal_file.write_all(&wal_frame[0..24])?;
                            decrypted_wal_file.write_all(&decrypt_buf)?;
                        }
                    }
                    print_info(format!(
                        "文件{:?}解密成功",
                        saved_wal_file_path.file_name().unwrap()
                    ));
                }

                let saved_shm_file_str = save_path_str.to_owned()
                    + std::path::MAIN_SEPARATOR_STR
                    + entry.file_name().to_str().unwrap()
                    + "-shm";
                let saved_shm_file_path = Path::new(&saved_shm_file_str);

                if saved_shm_file_path.exists() {
                    let mut save_shm_file = File::open(&saved_shm_file_path)?;
                    path_buf.pop();
                    path_buf.push(
                        "decrypted_".to_owned()
                            + saved_shm_file_path.file_name().unwrap().to_str().unwrap(),
                    );
                    let mut decrypted_shm_file = File::create(&path_buf)?;
                    std::io::copy(&mut save_shm_file, &mut decrypted_shm_file)?;
                }
            }
        } else {
            print_info(format!("文件{:?}密码错误", entry.file_name()));
        }
    }
    Ok(())
}

pub fn dev() -> anyhow::Result<()> {
    Ok(())
}

fn get_check_sum(mut s1: u32, mut s2: u32, list: &[u8], order_byte: &u8) -> Result<(u32, u32)> {
    let get_i32 = if *order_byte == 0x82 {
        |cursor: &mut std::io::Cursor<&[u8]>| cursor.read_u32::<LittleEndian>().unwrap()
    } else if *order_byte == 0x83 {
        |cursor: &mut std::io::Cursor<&[u8]>| cursor.read_u32::<BigEndian>().unwrap()
    } else {
        return Err(anyhow!("bad order_byte"));
    };
    let mut list = list.chunks(4).map(|data| {
        let mut cursor = std::io::Cursor::new(&data[..]);
        get_i32(&mut cursor)
    });
    loop {
        if let Some(first) = list.next() {
            s1 = u32::wrapping_add(u32::wrapping_add(s1, first), s2);
            s2 = u32::wrapping_add(u32::wrapping_add(s2, list.next().unwrap()), s1);
        } else {
            break;
        }
    }
    Ok((s1, s2))
}

fn decrypt_data<F>(
    index: u32,
    data: &[u8],
    key: &[u8],
    decrypted_data: &mut Vec<u8>,
    iv_offset: usize,
    dis_decrypted_offset: usize,
    check_fn: F,
) -> Result<()>
where
    F: Fn(&[u8], &[u8], u32) -> Result<bool>,
{
    let page = if index == 1 {
        decrypted_data.append(&mut "SQLite format 3\x00".as_bytes().to_vec());
        &data[16..]
    } else {
        &data[..]
    };
    if !check_fn(page, key, index)? {
        return Err(anyhow!(format!("数据校验未通过,index: {}", index)));
    }
    let iv = &page[page.len() - iv_offset..page.len() - iv_offset + 16];
    let mut decrypt_buf = vec![0u8; page.len() - dis_decrypted_offset];
    let decryptor = cbc::Decryptor::<aes::Aes256>::new_from_slices(&key, iv)?;
    decryptor
        .decrypt_padded_b2b_mut::<NoPadding>(
            &page[..page.len() - dis_decrypted_offset],
            &mut decrypt_buf,
        )
        .map_err(|e| anyhow!(format!("{:?}", e)))?;
    decrypted_data.append(&mut decrypt_buf);
    decrypted_data.append(&mut data[data.len() - dis_decrypted_offset..].to_vec());
    Ok(())
}

fn decrypt_data_check_hmac(
    index: u32,
    data: &[u8],
    key: &[u8],
    decrypted_data: &mut Vec<u8>,
    iv_offset: usize,
    dis_decrypted_offset: usize,
    salt: &[u8],
    hmac_offset: usize,
) -> Result<()> {
    decrypt_data(
        index,
        data,
        key,
        decrypted_data,
        iv_offset,
        dis_decrypted_offset,
        |data, key, index| check_hmac(data, key, salt, index, hmac_offset),
    )
}

fn check_hmac(
    data: &[u8],
    key: &[u8],
    salt: &[u8],
    index: u32,
    hmac_offset: usize,
) -> Result<bool> {
    let mut mac_key = [0u8; 32];
    pbkdf2::pbkdf2::<Hmac<Sha1>>(&key, &salt, 2, &mut mac_key)?;
    let mut hash_mac = Hmac::<Sha1>::new_from_slice(&mac_key)?;
    hash_mac.update(&data[..data.len() - hmac_offset]);
    hash_mac.update(&index.to_le_bytes());
    let r = hash_mac.finalize().into_bytes();
    Ok(r[..] == data[data.len() - hmac_offset..data.len() - hmac_offset + 20])
}

fn decrypt_data_uncheck_hmac(
    index: u32,
    data: &[u8],
    key: &[u8],
    decrypted_data: &mut Vec<u8>,
    iv_offset: usize,
    dis_decrypted_offset: usize,
) -> Result<()> {
    decrypt_data(
        index,
        data,
        key,
        decrypted_data,
        iv_offset,
        dis_decrypted_offset,
        |_, _, _| Ok(true),
    )
}
