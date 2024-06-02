use ashpd::{desktop::screenshot::ScreenshotRequest, WindowIdentifier};
use chrono::Utc;
use rusqlite::Connection;
use tokio::fs;

pub async fn run(conn: Connection) -> anyhow::Result<()> {
    let screenshot = ScreenshotRequest::default()
        .identifier(WindowIdentifier::default())
        .interactive(true)
        .modal(false)
        .send()
        .await?
        .response();

    if let Ok(screenshot) = screenshot {
        if let Ok(path) = screenshot.uri().to_file_path() {
            if let Ok(data) = fs::read(&path).await {
                conn.execute(
                    "INSERT INTO screenshots (created_at, original_path, data) VALUES (?1, ?2, ?3)",
                    (Utc::now().timestamp(), path.to_str().unwrap(), data),
                )?;
            }
        }
    }

    Ok(())
}