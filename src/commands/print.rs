#[cfg(target_os = "linux")]
pub async fn run(conn: rusqlite::Connection) -> anyhow::Result<()> {
    use ashpd::{desktop::screenshot::ScreenshotRequest, WindowIdentifier};
    use chrono::Utc;
    use tokio::fs;

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
                    "INSERT INTO screenshots (created_at, original_path, synced, data) VALUES (?1, ?2, ?3, ?4)",
                    (Utc::now().timestamp(), path.to_str().unwrap(), false, data),
                )?;
            }
        }
    }

    Ok(())
}
