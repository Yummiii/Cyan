use chrono::Utc;
use lina_rs::macros::repo;
use sqlx::{Sqlite, prelude::FromRow};
use tabled::Tabled;

#[derive(Debug, FromRow)]
pub struct Screenshot {
    pub id: i64,
    pub created_at: i64,
    pub data: Vec<u8>,
}

#[derive(Debug, FromRow)]
pub struct NewScreenshot<'a> {
    pub created_at: i64,
    pub data: &'a Vec<u8>,
    pub hash: String,
}

impl<'a> NewScreenshot<'a> {
    pub fn new(data: &'a Vec<u8>) -> Self {
        Self {
            created_at: Utc::now().timestamp(),
            hash: blake3::hash(data).to_string(),
            data,
        }
    }
}

#[derive(Debug, FromRow, Tabled)]
pub struct ScreenshotMeta {
    pub id: i64,
    pub created_at: i64,
    pub hash: String,
}

#[repo(db = Sqlite)]
impl ScreenshotRepo {
    pub async fn create(&mut self, screenshot: &NewScreenshot<'_>) -> anyhow::Result<()> {
        sqlx::query("INSERT INTO screenshots (created_at, data, hash) VALUES (?, ?, ?)")
            .bind(screenshot.created_at)
            .bind(screenshot.data)
            .bind(&screenshot.hash)
            .execute(&mut *self.conn)
            .await?;
        Ok(())
    }

    pub async fn paginate_meta(
        &mut self,
        page: i32,
        page_size: i32,
    ) -> anyhow::Result<Vec<ScreenshotMeta>> {
        let screenshots = sqlx::query_as::<_, ScreenshotMeta>(
            "SELECT id, created_at, hash FROM screenshots ORDER BY created_at DESC LIMIT ? OFFSET ?",
        )
        .bind(page_size)
        .bind((page - 1) * page_size)
        .fetch_all(&mut *self.conn)
        .await?;
        Ok(screenshots)
    }

    pub async fn paginate_meta_saved(
        &mut self,
        saved: bool,
        page: i32,
        page_size: i32,
    ) -> anyhow::Result<Vec<ScreenshotMeta>> {
        let screenshots = sqlx::query_as::<_, ScreenshotMeta>(
            "SELECT id, created_at, hash FROM screenshots WHERE saved = ? ORDER BY created_at DESC LIMIT ? OFFSET ?",
        )
        .bind(saved)
        .bind(page_size)
        .bind((page - 1) * page_size)
        .fetch_all(&mut *self.conn)
        .await?;
        Ok(screenshots)
    }

    pub async fn list_meta_saved(&mut self, saved: bool) -> anyhow::Result<Vec<ScreenshotMeta>> {
        let screenshots = sqlx::query_as::<_, ScreenshotMeta>(
            "SELECT id, created_at, hash FROM screenshots WHERE saved = ? ORDER BY created_at DESC",
        )
        .bind(saved)
        .fetch_all(&mut *self.conn)
        .await?;
        Ok(screenshots)
    }

    pub async fn get(&mut self, id: i64) -> anyhow::Result<Screenshot> {
        let screenshot = sqlx::query_as::<_, Screenshot>(
            "SELECT id, created_at, data FROM screenshots WHERE id = ?",
        )
        .bind(id)
        .fetch_one(&mut *self.conn)
        .await?;
        Ok(screenshot)
    }

    pub async fn set_saved(&mut self, id: i64, saved: bool) -> anyhow::Result<()> {
        sqlx::query("UPDATE screenshots SET saved = ? WHERE id = ?")
            .bind(saved)
            .bind(id)
            .execute(&mut *self.conn)
            .await?;
        Ok(())
    }

    pub async fn count_saved(&mut self, saved: bool) -> anyhow::Result<i32> {
        let count =
            sqlx::query_scalar::<_, i32>("SELECT COUNT(id) FROM screenshots WHERE saved = ?")
                .bind(saved)
                .fetch_one(&mut *self.conn)
                .await?;
        Ok(count)
    }

    pub async fn count(&mut self) -> anyhow::Result<i32> {
        let count = sqlx::query_scalar::<_, i32>("SELECT COUNT(id) FROM screenshots")
            .fetch_one(&mut *self.conn)
            .await?;
        Ok(count)
    }
}
