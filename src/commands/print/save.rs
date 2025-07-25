use crate::database::{Conn, Repos};
use chrono::{TimeZone, Utc};
use clap::Parser;
use sqlx::Acquire;
use std::path::PathBuf;
use tokio::fs;

#[derive(Debug, Parser)]
pub struct Args {
    /// Path where screenshots will be saved to
    destination: PathBuf,
}

pub async fn run(args: Args, mut conn: Conn) -> anyhow::Result<()> {
    let list = conn.screenshot().list_meta_saved(false).await?;
    let mut i = 0;

    let mut tx = conn.begin().await?;

    for screenshot in list {
        let screenshot = tx.screenshot().get(screenshot.id).await?;
        let timestamp = Utc.timestamp_opt(screenshot.created_at, 0).unwrap();

        let file = args.destination.join(timestamp.format("%Y-%m").to_string());
        fs::create_dir_all(&file).await?;

        let file = file.join(format!("{}.png", screenshot.created_at));
        fs::write(&file, screenshot.data).await?;

        tx.screenshot().set_saved(screenshot.id, true).await?;
        println!("Saved screenshot [{}] to {}", screenshot.id, file.display());

        i += 1;
    }

    tx.commit().await?;

    println!("Saved {i} screenshots");

    Ok(())
}
