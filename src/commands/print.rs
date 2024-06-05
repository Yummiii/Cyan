#[cfg(target_os = "linux")]
pub async fn run(conn: rusqlite::Connection) -> anyhow::Result<()> {
    use ashpd::{
        desktop::{
            notification::{Notification, NotificationProxy, Priority},
            screenshot::ScreenshotRequest,
        },
        WindowIdentifier,
    };
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
    } else {
        let notify = Notification::new("Erro ao salvar o print")
            .default_action(None)
            .body("yay")
            // .icon(Icon::Bytes(include_bytes!("/home/yummi/.xdg/pictures/Screenshots/Screenshot from 2024-06-05 01-13-13.png").to_vec()))
            .priority(Priority::Normal);
        let proxy = NotificationProxy::new().await?;
        proxy.add_notification("cyan", notify).await?;
    }

    Ok(())
}
