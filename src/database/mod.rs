use crate::configs::get_configs_dir;
use rusqlite::Connection;

pub mod screenshot;

pub fn get() -> anyhow::Result<Connection> {
    let conn = Connection::open(format!("{}cyan.db", get_configs_dir()?))?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS screenshots (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            created_at INTEGER NOT NULL,
            original_path TEXT,
            synced INTEGER NOT NULL,
            hash TEXT NOT NULL,
            data BLOB NOT NULL
        )",
        (),
    )?;

    Ok(conn)
}
