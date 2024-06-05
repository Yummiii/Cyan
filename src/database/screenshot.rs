use gxhash::gxhash64;
use rusqlite::Connection;
use serde::Deserialize;
use serde_rusqlite::from_rows;
use std::fmt::Display;

use crate::CONFIGS;

#[derive(Debug, Deserialize)]
pub struct Screenshot {
    pub id: u64,
    pub created_at: i64,
    pub original_path: Option<String>,
    pub synced: bool,
    pub hash: String,
    pub data: Vec<u8>,
}

impl Screenshot {
    pub fn new(
        created_at: i64,
        original_path: Option<impl Display>,
        synced: bool,
        data: Vec<u8>,
    ) -> Self {
        Screenshot {
            id: 0,
            created_at,
            original_path: original_path.map(|p| p.to_string()),
            synced,
            hash: gxhash64(&data, CONFIGS.cyan.hash_seed).to_string(),
            data,
        }
    }

    pub async fn add(conn: &Connection, screenshot: Screenshot) -> anyhow::Result<()> {
        conn.execute(
        "INSERT INTO screenshots (created_at, original_path, synced, hash, data) VALUES (?1, ?2, ?3, ?4, ?5)",
        (
            screenshot.created_at,
            screenshot.original_path,
            screenshot.synced,
            screenshot.hash,
            screenshot.data,
        ),
    )?;
        Ok(())
    }

    pub async fn from_hash(conn: &Connection, hash: String) -> anyhow::Result<Option<Screenshot>> {
        let mut stmt = conn.prepare("SELECT * FROM screenshots WHERE hash = ?1")?;
        let mut screenshots = from_rows::<Screenshot>(stmt.query([hash])?);

        if let Some(ss) = screenshots.next() {
            Ok(Some(ss.unwrap()))
        } else {
            Ok(None)
        }
    }
}
