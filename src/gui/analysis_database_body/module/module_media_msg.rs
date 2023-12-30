
use diesel::{Queryable, SqliteConnection, QueryDsl, RunQueryDsl, ExpressionMethods};

use super::super::schema::schema_media_msg;

const SAMPLE_RATE: u32 = 20000;
#[derive(Queryable, Debug)]
pub struct Media {
    pub key: Option<String>,
    pub reserved0: Option<i64>,
    pub buf: Option<Vec<u8>>,
    pub reserved1: Option<i32>,
    pub reserved2: Option<String>,
}

pub fn get_audio_pcm(id: i64, sample_rate: u32, conn: &mut SqliteConnection) -> Result<Vec<i16>, anyhow::Error> {

    let m:Vec<Media> = schema_media_msg::Media::table.select(schema_media_msg::Media::all_columns)
        .filter(schema_media_msg::Media::Reserved0.eq(id))
        .load::<Media>(conn)?;
    if m.len() < 1 {
        return Err(anyhow::anyhow!("未找到该记录"));
    }
    let data = m[0].buf.as_ref().unwrap();
    match data[0] {
        2 => {
            let r = silk_sys::decode_silk(&data[1..], sample_rate)?;
            Ok(r)
        }
        35 => {
            let r = silk_sys::decode_silk(&data[7..], sample_rate)?;
            Ok(r)
        }
        _ => {
            Err(anyhow::anyhow!("无法解析记录"))
        }
    }
}

pub fn test_for_silk(conn: &mut SqliteConnection) -> Result<(), anyhow::Error> {
    let vec = schema_media_msg::Media::table.select(schema_media_msg::Media::all_columns)
        .load::<Media>(conn)?;
    let mut stream = None;
    for m in vec {
        let data = m.buf.as_ref().unwrap();
        // println!("load key: {}",m.key.as_ref().unwrap());
        match data[0] {
            2 => {
                // 1099511686545
                if m.key.as_ref().unwrap() == "1099511686541" {
                    let t = crate::gui::gui_util::play_audio(|sample_rate| Ok(silk_sys::decode_silk(&data[1..], sample_rate)?))?;
                    stream = Some(t);
                    println!("play audio. key: {}",m.key.unwrap());
                } else {
                    silk_sys::decode_silk(&data[1..], SAMPLE_RATE)?;
                }
            }
            35 => {
                if stream.is_none() {
                    let t = crate::gui::gui_util::play_audio(|sample_rate| Ok(silk_sys::decode_silk(&data[7..], sample_rate)?))?;
                    stream = Some(t);
                    println!("play audio. key: {}",m.key.unwrap());
                } else {
                    silk_sys::decode_silk(&data[7..], SAMPLE_RATE)?;
                }
            }
            _ => {
                println!("key: {}",m.key.unwrap());
                println!("data: {:?}",&data[..10]);
                Err(anyhow::anyhow!("无法解析记录"))?;
            }
        }
    }
    Ok(())
}