
#![allow(unused)]
#![allow(clippy::all)]

use std::collections::HashMap;

use diesel::{Queryable, QueryDsl, RunQueryDsl, Identifiable, Connection, SqliteConnection, ExpressionMethods};
use crate::gui::analysis_database_body::schema::schema_macro_msg;

#[derive(Queryable, Debug)]
pub struct Contact {
    pub user_name: Option<String>,
    pub alias: Option<String>,
    pub encrypt_user_name: Option<String>,
    pub del_flag: Option<i32>,
    pub r#type: Option<i32>,
    pub verify_flag: Option<i32>,
    pub reserved1: Option<i32>,
    pub reserved2: Option<i32>,
    pub reserved3: Option<String>,
    pub reserved4: Option<String>,
    pub remark: Option<String>,
    pub nick_name: Option<String>,
    pub label_id_list: Option<String>,
    pub domain_list: Option<String>,
    pub chat_room_type: Option<i32>,
    pub py_initial: Option<String>,
    pub quan_pin: Option<String>,
    pub remark_py_initial: Option<String>,
    pub remark_quan_pin: Option<String>,
    pub big_head_img_url: Option<String>,
    pub small_head_img_url: Option<String>,
    pub head_img_md5: Option<String>,
    pub chat_room_notify: Option<i32>,
    pub reserved5: Option<i32>,
    pub reserved6: Option<String>,
    pub reserved7: Option<String>,
    pub extra_buf: Option<Vec<u8>>,
    pub reserved8: Option<i32>,
    pub reserved9: Option<i32>,
    pub reserved10: Option<String>,
    pub reserved11: Option<String>,
}


#[derive(Queryable, Debug)]
pub struct Session {
    pub str_usr_name: Option<String>,
    pub n_order: Option<i32>,
    pub n_un_read_count: Option<i32>,
    pub parent_ref: Option<String>,
    pub reserved0: Option<i32>,
    pub reserved1: Option<String>,
    pub str_nick_name: Option<String>,
    pub n_status: Option<i32>,
    pub n_is_send: Option<i32>,
    pub str_content: Option<String>,
    pub n_msg_type: Option<i32>,
    pub n_msg_local_id: Option<i32>,
    pub n_msg_status: Option<i32>,
    pub n_time: Option<i32>,
    pub edit_content: Option<String>,
    pub others_at_me: Option<i32>,
    pub reserved2: Option<i32>,
    pub reserved3: Option<String>,
    pub reserved4: Option<i32>,
    pub reserved5: Option<String>,
    pub bytes_xml: Option<Vec<u8>>,
}

pub fn get_sessions(conn: &mut SqliteConnection) -> Result<Vec<Session>, anyhow::Error> {
    let sql = schema_macro_msg::Session::table
        .select(schema_macro_msg::Session::all_columns)
        .order_by(schema_macro_msg::Session::nOrder.desc());
    let r = sql.load::<Session>(conn)?;
    Ok(r)
}

pub fn get_contact(conn: &mut SqliteConnection) -> Result<HashMap<Option<String>,Contact>, anyhow::Error> {
    let mut map = HashMap::new();
    let sql = schema_macro_msg::Contact::table
        .select(schema_macro_msg::Contact::all_columns);
    let r = sql.load::<Contact>(conn)?;
    for contact in r {
        map.insert(contact.user_name.clone(), contact);
    }
    Ok(map)
}