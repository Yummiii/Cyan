use crate::{database::screenshot::Screenshot, CONFIGS};
use filetime::FileTime;
use glob::glob;
use gxhash::gxhash64;
use std::path::PathBuf;
use tokio::fs;

pub async fn run(conn: rusqlite::Connection, path: String) -> anyhow::Result<()> {
    let files = if PathBuf::from(&path).is_file() {
        vec![path]
    } else {
        let mut files = vec![];
        for entry in glob(&format!("{}/*.png", path)).unwrap().flatten() {
            files.push(entry.display().to_string());
        }
        files
    };

    for file in files {
        if let Ok(data) = fs::read(&file).await {
            if Screenshot::from_hash(&conn, gxhash64(&data, CONFIGS.cyan.hash_seed).to_string())
                .await?
                .is_none()
            {
                let time = FileTime::from_last_modification_time(&fs::metadata(&file).await?);
                Screenshot::add(
                    &conn,
                    Screenshot::new(time.unix_seconds(), Some(file), false, data),
                )
                .await?;
            }
        }
    }

    Ok(())
}
