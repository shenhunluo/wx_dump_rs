use std::collections::HashMap;

use diesel::{Queryable, SqliteConnection, QueryDsl, RunQueryDsl, ExpressionMethods};

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
    pub msg_svr_id: Option<i32>,
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
    pub bytes_extra: Option<Vec<u8>>,
    pub bytes_trans: Option<Vec<u8>>,
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

pub fn get_msg_by_user_name(user_name: &Option<String>, conns: &mut HashMap<usize,SqliteConnection>) -> Result<Vec<Msg>, anyhow::Error> {
    let mut keys = conns.keys().map(|k| *k).collect::<Vec<_>>();
    keys.sort();
    let mut vec = vec![];
    for k in keys {
        let conn = conns.get_mut(&k).unwrap();
        let sql = schema_msg::MSG::table.select(schema_msg::MSG::all_columns)
            .filter(schema_msg::MSG::StrTalker.eq(user_name))
            .order_by(schema_msg::MSG::CreateTime)
        ;
        let mut r = sql.load::<Msg>(conn)?;
        vec.append(&mut r);
    }
    Ok(vec)
}