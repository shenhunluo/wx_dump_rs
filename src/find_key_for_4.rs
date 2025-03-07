use std::{fs::File, io::Read};

use anyhow::Result;
use hmac::{Hmac, Mac};
use sha2::Sha512;

mod action;
mod util;
mod wx_util;

fn main() -> Result<()> {
    let mut saved_file = File::open(
        "D:\\wechat\\xwechat_files\\zhangdexin783818_4b34\\db_storage\\message\\message_0.db",
    )
    .unwrap();
    let mut buffer = vec![];
    saved_file.read_to_end(&mut buffer)?;
    let salt = &buffer[0..16];
    let mut byte_key = [0; 32];

    let key = "".as_bytes();

    pbkdf2::pbkdf2::<Hmac<Sha512>>(key, salt, 64000, &mut byte_key)?;
    let first = &buffer[16..4096];
    let mac_salt = salt.iter().map(|i| i ^ 58).collect::<Vec<_>>();
    if check_hmac(first, &byte_key, &mac_salt, 1, 32)? {}

    Ok(())
}

fn check_hmac(
    data: &[u8],
    key: &[u8],
    salt: &[u8],
    index: u32,
    hmac_offset: usize,
) -> Result<bool> {
    let mut mac_key = [0u8; 32];
    pbkdf2::pbkdf2::<Hmac<Sha512>>(&key, &salt, 2, &mut mac_key)?;
    let mut hash_mac = Hmac::<Sha512>::new_from_slice(&mac_key)?;
    hash_mac.update(&data[..data.len() - hmac_offset]);
    hash_mac.update(&index.to_le_bytes());
    let r = hash_mac.finalize().into_bytes();
    Ok(r[..] == data[data.len() - hmac_offset..data.len() - hmac_offset + 20])
}
