use diesel::{Connection, SqliteConnection};

pub mod module_emotion;
pub mod module_macro_msg;
pub mod module_media_msg;
pub mod module_msg;

pub fn get_conn(file_path: impl ToString) -> Result<SqliteConnection, anyhow::Error> {
    let conn = diesel::sqlite::SqliteConnection::establish(&format!(
        "sqlite://{}",
        file_path.to_string()
    ))?;
    Ok(conn)
}
