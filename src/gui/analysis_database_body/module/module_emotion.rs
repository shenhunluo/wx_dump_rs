#![allow(unused)]
#![allow(clippy::all)]

use std::collections::HashMap;

use diesel::{query_dsl::methods::SelectDsl, Queryable, RunQueryDsl, SqliteConnection};

use crate::gui::analysis_database_body::schema::schema_emotion;

#[derive(Queryable, Debug, Clone)]
pub struct Name2ID {
    pub usr_name: Option<String>,
}

#[derive(Queryable, Debug, Clone)]
pub struct EmotionItem {
    pub local_id: Option<i32>,
    pub product_id: Option<i32>,
    pub md5: String,
    pub reserved0: Option<i32>,
    pub reserved1: Option<String>,
    pub r#type: Option<i32>,
    pub app_id: Option<String>,
    pub from_url: Option<String>,
    pub thumb: Option<Vec<u8>>,
    pub data: Option<Vec<u8>>,
    pub reserved2: Option<i32>,
    pub reserved3: Option<String>,
    pub reserved4: Option<i32>,
}

pub fn get_emotion(
    conn: &mut SqliteConnection,
) -> Result<HashMap<Option<String>, EmotionItem>, anyhow::Error> {
    let name2id = schema_emotion::Name2ID::table
        .select(schema_emotion::Name2ID::all_columns)
        .load::<Name2ID>(conn)?;
    let mut map = HashMap::new();
    for (id, name) in name2id.iter().enumerate() {
        map.insert(id + 1, &name.usr_name);
    }
    let emotion_items = schema_emotion::EmotionItem::table
        .select(schema_emotion::EmotionItem::all_columns)
        .load::<EmotionItem>(conn)?;
    let mut r_map = HashMap::new();
    for item in emotion_items {
        if let Some(local_id) = item.local_id {
            if let Some(name) = map.get(&(local_id as usize)) {
                r_map.insert((*name).clone(), item);
            }
        }
    }
    return Ok(r_map);
}
