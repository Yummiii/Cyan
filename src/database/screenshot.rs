use gxhash::gxhash64;
use sqlx::prelude::FromRow;
use std::fmt::Display;

use crate::CONFIGS;

use super::Db;

#[derive(Debug, FromRow)]
pub struct Screenshot {
    pub id: i64,
    pub created_at: i64,
    pub original_path: Option<String>,
    pub synced: bool,
    pub hash: String,
    pub data: Vec<u8>,
}

//someday i will make this better, this day isn't today
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

    pub async fn add(db: &Db, screenshot: Screenshot) -> anyhow::Result<()> {
        sqlx::query("INSERT INTO screenshots (created_at, original_path, synced, hash, data) VALUES (?1, ?2, ?3, ?4, ?5)")
            .bind(screenshot.created_at)
            .bind(&screenshot.original_path)
            .bind(screenshot.synced)
            .bind(&screenshot.hash)
            .bind(&screenshot.data)
            .execute(&mut *db.conn().await?).await?;
        Ok(())
    }

    pub async fn get_all_unsynced(db: &Db) -> anyhow::Result<Vec<Screenshot>> {
        Ok(sqlx::query_as("SELECT * FROM screenshots WHERE synced = 0")
            .fetch_all(&mut *db.conn().await?)
            .await?)
    }

    pub async fn delete(db: &Db, id: i64) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM screenshots WHERE id = ?1")
            .bind(id)
            .execute(&mut *db.conn().await?)
            .await?;
        Ok(())
    }

    pub async fn set_synced(db: &Db, id: i64, synced: bool) -> anyhow::Result<()> {
        sqlx::query("UPDATE screenshots SET synced = ?1 WHERE id = ?2")
            .bind(synced)
            .bind(id)
            .execute(&mut *db.conn().await?)
            .await?;
        Ok(())
    }

    pub async fn from_hash(db: &Db, hash: &str) -> anyhow::Result<Option<Screenshot>> {
        Ok(sqlx::query_as("SELECT * FROM screenshots WHERE hash = ?1")
            .bind(hash)
            .fetch_optional(&mut *db.conn().await?)
            .await?)
    }
}
