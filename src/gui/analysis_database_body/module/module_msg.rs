use std::{collections::HashMap, fs::File, io::Read, path::PathBuf};

use diesel::{
    ExpressionMethods, QueryDsl, Queryable, RunQueryDsl, SqliteConnection, backend::Backend,
    deserialize::FromSql, sql_types::Binary, sqlite::Sqlite,
};
use xml::EventReader;

use crate::{gui::analysis_database_body::schema::schema_msg, util::get_wechat_path};

use super::module_macro_msg::Contact;

#[derive(Queryable, Debug)]
pub struct Dbinfo {
    pub table_index: Option<i32>,
    pub table_version: Option<i32>,
    pub table_desc: Option<String>,
}

#[derive(Queryable, Debug, Clone)]
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
            3 => MsgData::Image(Self::load_image_data(
                wechat_path,
                self.bytes_extra
                    .as_ref()
                    .unwrap()
                    .map
                    .get(&4)
                    .map(|v| v[0].clone()),
                true,
            )),
            34 => MsgData::Audio(self.msg_svr_id.unwrap()),
            43 => MsgData::Video((
                Self::load_image_data(
                    wechat_path,
                    self.bytes_extra
                        .as_ref()
                        .unwrap()
                        .map
                        .get(&3)
                        .map(|v| v[0].clone()),
                    false,
                ),
                get_wechat_path(wechat_path).map(|mut wechat_path| {
                    self.bytes_extra.as_ref().unwrap().map.get(&4).map(|v| {
                        wechat_path.push(&v[0]);
                        wechat_path
                    })
                }),
            )),
            47 => MsgData::Emotion(Self::load_emotion_data(&self.str_content)),
            50 => MsgData::Voip(Self::load_voip_msg_data(&self.str_content)),
            _ => MsgData::Other(self.str_content.clone().unwrap_or("".to_string())),
        }
    }

    fn load_voip_msg_data(str_content: &Option<String>) -> String {
        let mut r_str = String::new();
        if let Some(str_content) = str_content {
            let mut read = EventReader::new(str_content.as_bytes());
            loop {
                let r = read.next();
                match r {
                    Ok(xml::reader::XmlEvent::StartElement {
                        name,
                        attributes: _,
                        namespace: _,
                    }) => {
                        if name.local_name == "msg" {
                            loop {
                                let r = read.next();
                                match r {
                                    Ok(xml::reader::XmlEvent::CData(data)) => {
                                        r_str.push_str(&data);
                                    }
                                    Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                        if name.local_name == "msg" {
                                            break;
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }

                        if name.local_name == "msg_type" {
                            loop {
                                let r = read.next();
                                match r {
                                    Ok(xml::reader::XmlEvent::Characters(data)) => {
                                        match data.as_str() {
                                            "100" => {
                                                r_str = format!(
                                                    "{} {}",
                                                    iced_aw::Bootstrap::Telephone,
                                                    r_str
                                                )
                                            }
                                            "101" => {
                                                r_str = format!(
                                                    "{} {}",
                                                    iced_aw::Bootstrap::Camera,
                                                    r_str
                                                )
                                            }
                                            _ => {}
                                        }
                                    }
                                    Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                        if name.local_name == "msg_type" {
                                            break;
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    Ok(xml::reader::XmlEvent::EndDocument) => break,
                    Err(_e) => {
                        return format!(
                            "{} 音视频通话，旧版数据无法解析",
                            iced_aw::Bootstrap::Telephone
                        );
                    }
                    _ => {}
                }
            }
        }
        r_str
    }

    fn load_emotion_data(str_content: &Option<String>) -> Option<String> {
        match str_content {
            Some(str_content) => {
                let read = EventReader::new(str_content.as_bytes());
                for r in read {
                    match r {
                        Ok(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace: _,
                        }) => {
                            if name.local_name == "emoji" {
                                for attr in attributes.iter() {
                                    if attr.name.local_name == "cdnurl" {
                                        return if attr.value.is_empty() {
                                            None
                                        } else {
                                            Some(attr.value.clone())
                                        };
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                None
            }
            None => None,
        }
    }

    fn load_image_data(
        wechat_path: &Option<String>,
        image_path: Option<String>,
        decrypt: bool,
    ) -> Result<Vec<u8>, anyhow::Error> {
        if let Some(image_path) = image_path {
            let mut path = get_wechat_path(wechat_path)?;
            path.push(image_path);
            let mut file = File::open(path)?;
            let mut buf = vec![];
            file.read_to_end(&mut buf)?;
            let r = if decrypt {
                let mut xor_key = 0;
                for header in IMAGE_HEADER {
                    let v = header
                        .1
                        .iter()
                        .zip(&buf[..header.1.len()])
                        .map(|(a, b)| a ^ b)
                        .collect::<Vec<_>>();
                    if v.iter().map(|a| *a == v[0]).fold(true, |a, b| a && b) {
                        xor_key = v[0];
                        break;
                    }
                }
                buf.iter().map(|u| *u ^ xor_key).collect()
            } else {
                buf
            };
            Ok(r)
        } else {
            Err(anyhow::anyhow!("未找到图片路径"))
        }
    }

    pub fn get_user_name(&self, contact: &HashMap<Option<String>, Contact>) -> Option<String> {
        if let Some(byte_ex) = &self.bytes_extra {
            if let Some(data) = byte_ex.map.get(&1) {
                if let Some(data) = data.get(0) {
                    if let Some(contact) = contact.get(&Some(data.to_owned())) {
                        let text = contact
                            .nick_name
                            .as_ref()
                            .cloned()
                            .unwrap_or("".to_string());
                        if let Some(remark) = &contact.remark {
                            if remark.len() > 0 {
                                return Some(format!("{}({})", text, remark));
                            }
                        }
                        return Some(text);
                    }
                }
            }
        }
        None
    }
}

const IMAGE_HEADER: &'static [(&str, &'static [u8])] = &[
    ("jpg", &[0xff, 0xd8]),
    ("png", &[0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a]),
    ("gif", &[0x47, 0x49, 0x46]),
    ("bmp", &[0x42, 0x4d]),
    ("tiff", &[0x49, 0x49]),
    ("tiff", &[0x4d, 0x4d]),
    ("ico", &[0x00, 0x00, 0x01, 0x00]),
    ("webp", &[0x52, 0x49, 0x46, 0x46]),
];

pub enum MsgData {
    Text(String),
    Image(Result<Vec<u8>, anyhow::Error>),
    Audio(i64),
    Video(
        (
            Result<Vec<u8>, anyhow::Error>,
            Result<std::option::Option<PathBuf>, anyhow::Error>,
        ),
    ),
    Emotion(Option<String>),
    Voip(String),
    Other(String),
}

#[derive(Debug, Clone)]
pub struct BytesExtra {
    _head: Vec<u8>,
    pub map: HashMap<u8, Vec<String>>,
}

impl BytesExtra {
    fn read_data(data: &[u8]) -> (usize, &[u8]) {
        if data[0] < 128 {
            (data[0] as usize + 1, &data[1..data[0] as usize + 1])
        } else {
            let p = (data[0] - 128) as usize + data[1] as usize * 128;
            (p + 2, &data[2..p + 2])
        }
    }
}

impl FromSql<Binary, Sqlite> for BytesExtra {
    fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        let bytes = Vec::<u8>::from_sql(bytes)?;
        let split = bytes.splitn(2, |b| b == &26).collect::<Vec<_>>();
        let head = split[0].to_vec();
        let mut body = split[1];
        let mut map = HashMap::new();
        loop {
            let (index, mut inner_body) = BytesExtra::read_data(body);
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
        Ok(BytesExtra { _head: head, map })
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

pub fn get_msg_by_user_name(
    user_name: &Option<String>,
    conns: &mut HashMap<usize, SqliteConnection>,
) -> Result<Vec<(usize, Msg)>, anyhow::Error> {
    let keys = conns.keys().map(|k| *k).collect::<Vec<_>>();
    let mut vec = vec![];
    for k in keys {
        let conn = conns.get_mut(&k).unwrap();
        let sql = schema_msg::MSG::table
            .select(schema_msg::MSG::all_columns)
            .filter(schema_msg::MSG::StrTalker.eq(user_name))
            .order_by(schema_msg::MSG::CreateTime);
        for msg in sql.load::<Msg>(conn)? {
            vec.push((k, msg));
        }
    }
    vec.sort_by(|(_, a), (_, b)| {
        a.create_time
            .unwrap_or_default()
            .cmp(&b.create_time.unwrap_or_default())
    });
    Ok(vec)
}

#[derive(Queryable, Debug, Clone)]
pub struct MsgReportData {
    pub is_sender: Option<i32>,
    pub create_time: Option<i32>,
    pub str_talker: Option<String>,
    pub bytes_extra: Option<BytesExtra>,
}

pub fn get_report_data(
    user_name: &Option<Option<String>>,
    conns: &mut HashMap<usize, SqliteConnection>,
) -> Result<Vec<MsgReportData>, anyhow::Error> {
    let mut vec = vec![];
    for conn in conns.values_mut() {
        let mut sql = schema_msg::MSG::table
            .select((
                schema_msg::MSG::IsSender,
                schema_msg::MSG::CreateTime,
                schema_msg::MSG::StrTalker,
                schema_msg::MSG::BytesExtra,
            ))
            .into_boxed();
        if let Some(user_name) = user_name {
            sql = sql.filter(schema_msg::MSG::StrTalker.eq(user_name));
        }
        let mut r = sql.load::<MsgReportData>(conn)?;
        vec.append(&mut r);
    }
    Ok(vec)
}
