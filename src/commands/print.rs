#[cfg(target_os = "linux")]
pub async fn run(conn: rusqlite::Connection) -> anyhow::Result<()> {
    use ashpd::{desktop::screenshot::ScreenshotRequest, WindowIdentifier};
    use chrono::Utc;
    use tokio::fs;

    use crate::database::screenshot::Screenshot;

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
                Screenshot::add(
                    &conn,
                    Screenshot::new(Utc::now().timestamp(), Some(path.display()), false, data),
                )
                .await?;
            }
        }
    }

    Ok(())
}
