use clap::Parser;

#[derive(Debug, Parser)]
pub struct PrintArgs;

#[cfg(target_os = "linux")]
pub async fn run(ctx: super::CmdCtx<PrintArgs>) -> anyhow::Result<()> {
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
        .modal(true)
        .send()
        .await?
        .response();

    if let Ok(screenshot) = screenshot {
        if let Ok(path) = screenshot.uri().to_file_path() {
            if let Ok(data) = fs::read(&path).await {
                Screenshot::add(
                    &ctx.db,
                    Screenshot::new(Utc::now().timestamp(), Some(path.display()), false, data),
                )
                .await?;
            }
        }
    } else {
        let notify = Notification::new("Erro ao salvar o print")
            .default_action(None)
            // .icon(Icon::Bytes(fs::read("/home/yummi/Downloads/GKBzQv5WkAA7i6F.png").await?))
            // .icon(Icon::with_names(&["dialog-question-symbolic"]))
            .body("yay")
            .priority(Priority::Normal);
        let proxy = NotificationProxy::new().await?;
        proxy.add_notification("com.zuraaa.Cyan", notify).await?;
    }

    Ok(())
}
