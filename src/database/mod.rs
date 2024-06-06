use crate::configs::get_configs_dir;
use sqlx::{migrate::MigrateDatabase, pool::PoolConnection, Pool, Sqlite, SqlitePool};

pub mod screenshot;

pub struct Db {
    pub pool: Pool<Sqlite>,
}

impl Db {
    pub async fn conn(&self) -> anyhow::Result<PoolConnection<Sqlite>> {
        Ok(self.pool.acquire().await?)
    }
}

pub async fn init() -> anyhow::Result<Db> {
    let url = format!("sqlite:{}cyan.db", get_configs_dir()?);
    Sqlite::create_database(&url).await?;

    let pool = SqlitePool::connect(&url).await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS screenshots (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        created_at INTEGER NOT NULL,
        original_path TEXT,
        synced INTEGER NOT NULL,
        hash TEXT NOT NULL,
        data BLOB NOT NULL
    )",
    )
    .execute(&mut *pool.acquire().await?)
    .await?;

    Ok(Db { pool })
}
