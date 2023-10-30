use anyhow::anyhow;
use base64::Engine;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
pub fn string_to_u8_vec(data: &String, encode: &String) -> anyhow::Result<Vec<u8>> {
    let mut buffer = vec![];
    match encode.to_ascii_lowercase().as_str() {
        "hex" => {
            for i in (0..data.len())
                .step_by(2)
                .map(|i| u8::from_str_radix(&data[i..i + 2], 16))
            {
                let i = i?;
                buffer.push(i);
            }
            Ok(buffer)
        }
        "base64" => {
            buffer = base64::engine::general_purpose::STANDARD_NO_PAD.decode(&data)?;
            Ok(buffer)
        }
        "string" => {
            buffer = data.as_bytes().to_vec();
            Ok(buffer)
        }
        _ => Err(anyhow!("错误的编码格式")),
    }
}

pub fn u8_to_string(data: &[u8], encode: &String) -> anyhow::Result<String> {
    match encode.to_ascii_lowercase().as_str() {
        "hex" => Ok(to_hex(data)),
        "base64" => Ok(base64::engine::general_purpose::STANDARD_NO_PAD.encode(&data)),
        "string" => Ok(String::from_utf8(
            data.split(|e| *e == 0).next().unwrap().to_vec(),
        )?),
        "u64be" => {
            let mut cur = std::io::Cursor::new(data);
            Ok(cur.read_u64::<BigEndian>()?.to_string())
        }
        "u64le" => {
            let mut cur = std::io::Cursor::new(data);
            Ok(cur.read_u64::<LittleEndian>()?.to_string())
        }
        "u32be" => {
            let mut cur = std::io::Cursor::new(data);
            Ok(cur.read_u32::<BigEndian>()?.to_string())
        }
        "u32le" => {
            let mut cur = std::io::Cursor::new(data);
            Ok(cur.read_u32::<LittleEndian>()?.to_string())
        }
        "u16be" => {
            let mut cur = std::io::Cursor::new(data);
            Ok(cur.read_u16::<BigEndian>()?.to_string())
        }
        "u16le" => {
            let mut cur = std::io::Cursor::new(data);
            Ok(cur.read_u16::<LittleEndian>()?.to_string())
        }
        "i64be" => {
            let mut cur = std::io::Cursor::new(data);
            Ok(cur.read_i64::<BigEndian>()?.to_string())
        }
        "i64le" => {
            let mut cur = std::io::Cursor::new(data);
            Ok(cur.read_i64::<LittleEndian>()?.to_string())
        }
        "i32be" => {
            let mut cur = std::io::Cursor::new(data);
            Ok(cur.read_i32::<BigEndian>()?.to_string())
        }
        "i32le" => {
            let mut cur = std::io::Cursor::new(data);
            Ok(cur.read_i32::<LittleEndian>()?.to_string())
        }
        "i16be" => {
            let mut cur = std::io::Cursor::new(data);
            Ok(cur.read_i16::<BigEndian>()?.to_string())
        }
        "i16le" => {
            let mut cur = std::io::Cursor::new(data);
            Ok(cur.read_i16::<LittleEndian>()?.to_string())
        }
        _ => Err(anyhow!("错误的编码格式")),
    }
}

pub fn to_hex(data: impl AsRef<[u8]>) -> String {
    let mut result = String::new();
    for i in data.as_ref() {
        result.push(HEX_TABLE[(i >> 4) as usize]);
        result.push(HEX_TABLE[(i & 0x0F) as usize]);
    }
    result
}

pub static HEX_TABLE: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
];
