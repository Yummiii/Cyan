use crate::configs::get_configs_dir;
use rusqlite::Connection;

#[derive(Debug)]
pub struct Screenshot {
    pub id: u64,
    pub created_at: i64,
    pub original_path: Option<String>,
    pub synced: bool,
    pub data: Vec<u8>,
}

pub fn get() -> anyhow::Result<Connection> {
    let conn = Connection::open(format!("{}cyan.db", get_configs_dir()?))?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS screenshots (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            created_at INTEGER NOT NULL,
            original_path TEXT,
            synced INTEGER NOT NULL,
            data BLOB NOT NULL
        )",
        (),
    )?;

    Ok(conn)
}
