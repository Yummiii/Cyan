use crate::{database::Screenshot, CONFIGS};
use chrono::{TimeZone, Utc};
use rusqlite::Connection;
use std::path::Path;
use tokio::fs;

pub async fn run(conn: Connection, delete: bool, sync_path: String) -> anyhow::Result<()> {
    let mut stmt = conn.prepare("SELECT * FROM screenshots WHERE synced = 0")?;
    let screenshots = stmt.query_map([], |r| {
        Ok(Screenshot {
            id: r.get(0)?,
            created_at: r.get(1)?,
            original_path: r.get(2)?,
            synced: r.get(3)?,
            data: r.get(4)?,
        })
    })?;

    for ss in screenshots.flatten() {
        let timestamp = Utc.timestamp_opt(ss.created_at, 0).unwrap();

        let dir = format!("{}/{}", sync_path, timestamp.format("%Y-%m"));
        let dir = Path::new(&dir);

        fs::create_dir_all(&dir).await?;
        fs::write(dir.join(format!("{}.png", ss.created_at)), ss.data).await?;

        if delete || CONFIGS.cyan.delete_after_sync {
            if let Some(path) = ss.original_path {
                fs::remove_file(path).await?;
                conn.execute("DELETE FROM screenshots WHERE id = ?1", [ss.id])?;
            }
        } else {
            conn.execute("UPDATE screenshots SET synced = 1 WHERE id = ?1", [ss.id])?;
        }
    }

    Ok(())
}
