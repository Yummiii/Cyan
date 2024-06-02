use rusqlite::Connection;

pub fn get() -> anyhow::Result<Connection> {
    let conn = Connection::open("prints.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS screenshots (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            created_at INTEGER NOT NULL,
            original_path TEXT not null,
            data BLOB NOT NULL
        )",
        (),
    )?;

    Ok(conn)
}
