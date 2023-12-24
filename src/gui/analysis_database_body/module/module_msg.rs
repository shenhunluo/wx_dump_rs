use std::{collections::HashMap, fs::File, io::Read, path::PathBuf};

use diesel::{Queryable, SqliteConnection, QueryDsl, RunQueryDsl, ExpressionMethods, deserialize::FromSql, backend::Backend, sql_types::Binary, sqlite::Sqlite};

use crate::gui::analysis_database_body::schema::schema_msg;

#[derive(Queryable, Debug)]
pub struct Dbinfo {
    pub table_index: Option<i32>,
    pub table_version: Option<i32>,
    pub table_desc: Option<String>,
}

#[derive(Queryable, Debug)]
pub struct Msg {
    pub local_id: Option<i32>,
    pub talker_id: Option<i32>,
    pub msg_svr_id: Option<i64>,
    pub r#type: Option<i32>,
    pub sub_type: Option<i32>,
    pub is_sender: Option<i32>,
    pub create_time: Option<i32>,
    pub sequence: Option<i32>,
    pub status_ex: Option<i32>,
    pub flag_ex: Option<i32>,
    pub status: Option<i32>,
    pub msg_server_seq: Option<i32>,
    pub msg_sequence: Option<i32>,
    pub str_talker: Option<String>,
    pub str_content: Option<String>,
    pub display_content: Option<String>,
    pub reserved0: Option<i32>,
    pub reserved1: Option<i32>,
    pub reserved2: Option<i32>,
    pub reserved3: Option<i32>,
    pub reserved4: Option<String>,
    pub reserved5: Option<String>,
    pub reserved6: Option<String>,
    pub compress_content: Option<Vec<u8>>,
    pub bytes_extra: Option<BytesExtra>,
    pub bytes_trans: Option<Vec<u8>>,
}

impl Msg {
    pub fn load_msg_data(&self, wechat_path: &Option<String>) -> MsgData {
        match self.r#type.unwrap() {
            1 => MsgData::Text(self.str_content.clone().unwrap_or("".to_string())),
            3 => MsgData::Image(Self::load_image_data(wechat_path, self.bytes_extra.as_ref().unwrap().map.get(&4).map(|v| v[0].clone()))),
            34 => MsgData::Audio(self.msg_svr_id.unwrap()),
            _ => MsgData::Other(self.str_content.clone().unwrap_or("".to_string())),
        }
    }

    fn load_image_data(wechat_path: &Option<String>, image_path: Option<String>) -> Result<Vec<u8>, anyhow::Error> {
        if let Some(image_path) = image_path {
            let mut path = if let Some(wechat_path) = wechat_path {
                let mut wechat_path_buf = PathBuf::new();
                wechat_path_buf.push(wechat_path);
                wechat_path_buf
            } else {
                let mut wechat_path_buf = dirs::document_dir().ok_or(anyhow::anyhow!("fail to get document directory"))?;
                wechat_path_buf.push("WeChat Files");
                wechat_path_buf
            };
            path.push(image_path);
            let mut file = File::open(path)?;
            let mut buf = vec![];
            file.read_to_end(&mut buf)?;
            let mut xor_key = 0;
            for header in IMAGE_HEADER {
                let v = header.1.iter().zip(&buf[..header.1.len()]).map(|(a,b)| a^b).collect::<Vec<_>>();
                if v.iter().map(|a| *a==v[0]).fold(true, |a,b| a&&b) {
                    xor_key = v[0];
                    break;
                }
            }
            let r = buf.iter().map(|u| *u ^ xor_key).collect();
            Ok(r)
        } else {
            Err(anyhow::anyhow!("未找到图片路径"))
        }
    }
}

const IMAGE_HEADER: &'static [(&str, &'static [u8])] = &[
    ("jpg",&[0xff,0xd8]),
    ("png",&[0x89,0x50,0x4e,0x47,0x0d,0x0a,0x1a,0x0a]),
    ("gif",&[0x47,0x49,0x46]),
    ("bmp",&[0x42,0x4d]),
    ("tiff",&[0x49,0x49]),
    ("tiff",&[0x4d,0x4d]),
    ("ico",&[0x00,0x00,0x01,0x00]),
    ("webp",&[0x52,0x49,0x46,0x46]),
];

pub enum MsgData {
    Text(String),
    Image(Result<Vec<u8>,anyhow::Error>),
    Audio(i64),
    Other(String)
}

#[derive(Debug)]
pub struct BytesExtra {
    head: Vec<u8>,
    pub map: HashMap<u8, Vec<String>>
}

impl BytesExtra {
    fn read_data(data: &[u8]) -> (usize,&[u8]) {
        if data[0] < 128 {
            (data[0] as usize + 1, &data[1..data[0] as usize + 1])
        } else {
            let p = (data[0] - 128) as usize + data[1] as usize * 128;
            (p + 2, &data[2..p+2])
        }
    }
}

impl<> FromSql<Binary, Sqlite> for BytesExtra {
    fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        let bytes = Vec::<u8>::from_sql(bytes)?;
        let split = bytes.splitn(2, |b| b == &26).collect::<Vec<_>>();
        let head = split[0].to_vec();
        let mut body = split[1];
        let mut map = HashMap::new();
        loop {
            let (index,mut inner_body) = BytesExtra::read_data(body);
            let key = inner_body[1];
            let mut vec = vec![];
            inner_body = &inner_body[3..];
            loop {
                let (index, data) = BytesExtra::read_data(inner_body);
                vec.push(String::from_utf8(data.to_vec()).unwrap());
                inner_body = &inner_body[index..];
                if inner_body.len() == 0 {
                    break;
                }
                inner_body = &inner_body[1..];
            }
            map.insert(key, vec);
            body = &body[index..];
            if body.len() == 0 {
                break;
            }
            body = &body[1..];
        }
        Ok(BytesExtra {
            head,
            map,
        })
    }
}

#[derive(Queryable, Debug)]
pub struct MsgTran {
    pub msg_local_id: Option<i32>,
    pub talker_id: Option<i32>,
}

#[derive(Queryable, Debug)]
pub struct Name2Id {
    pub usr_name: Option<String>,
}

pub fn get_msg_by_user_name(user_name: &Option<String>, conns: &mut HashMap<usize,SqliteConnection>) -> Result<Vec<(usize,Msg)>, anyhow::Error> {
    let mut keys = conns.keys().map(|k| *k).collect::<Vec<_>>();
    keys.sort();
    let mut vec = vec![];
    for k in keys {
        let conn = conns.get_mut(&k).unwrap();
        let sql = schema_msg::MSG::table.select(schema_msg::MSG::all_columns)
            .filter(schema_msg::MSG::StrTalker.eq(user_name))
            .order_by(schema_msg::MSG::CreateTime)
        ;
        for msg in sql.load::<Msg>(conn)? {
            vec.push((k,msg));
        }
    }
    Ok(vec)
}