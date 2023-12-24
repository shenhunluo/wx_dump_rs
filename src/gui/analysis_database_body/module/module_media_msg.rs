
use diesel::{Queryable, SqliteConnection, QueryDsl, RunQueryDsl, ExpressionMethods};

use super::super::schema::schema_media_msg;

#[derive(Queryable, Debug)]
pub struct Media {
    pub key: Option<String>,
    pub reserved0: Option<i64>,
    pub buf: Option<Vec<u8>>,
    pub reserved1: Option<i32>,
    pub reserved2: Option<String>,
}

pub fn get_audio_pcm(id: i64, conn: &mut SqliteConnection) -> Result<Vec<u8>, anyhow::Error> {

    let m:Vec<Media> = schema_media_msg::Media::table.select(schema_media_msg::Media::all_columns)
        .filter(schema_media_msg::Media::Reserved0.eq(id))
        .load::<Media>(conn)?;
    if m.len() < 1 {
        return Err(anyhow::anyhow!("未找到该记录"));
    }
    let data = m[0].buf.as_ref().unwrap();
    match data[0] {
        2 => {
            let r = silk_sys::decode_silk(&data[1..], 24000)?;
            Ok(r)
        }
        23 => {
            let r = silk_sys::decode_silk(&data[7..], 24000)?;
            Ok(r)
        }
        _ => {
            Err(anyhow::anyhow!("无法解析记录"))
        }
    }
} 