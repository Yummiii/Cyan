use crate::{CYAN_PATH, database::screenshots::ScreenshotRepo};
use lina_rs::macros::impl_repos;
use sqlx::{Pool, Sqlite, SqlitePool, migrate, pool::PoolConnection};
use std::path::Path;
use tokio::fs::File;

pub mod screenshots;

pub type Conn = PoolConnection<Sqlite>;

impl_repos!(
    db = Sqlite,
    name = Repos,
    method(name = "screenshot", repo = ScreenshotRepo)
);

pub async fn init_db() -> Pool<Sqlite> {
    let file = (*CYAN_PATH).join("cyan.db");

    if !Path::new(&file).exists() {
        File::create(&file).await.unwrap();
    }

    let pool = SqlitePool::connect(&format!("sqlite:{}", file.display()))
        .await
        .unwrap();

    migrate!().run(&pool).await.unwrap();

    pool
}
