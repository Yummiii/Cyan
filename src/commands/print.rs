use crate::{database::screenshot::Screenshot, CONFIGS};
use ashpd::{
    desktop::{
        notification::{Notification, NotificationProxy, Priority},
        screenshot::ScreenshotRequest,
    },
    WindowIdentifier,
};
use chrono::Utc;
use clap::Parser;
use gxhash::gxhash64;
use tokio::fs;

#[derive(Debug, Parser)]
pub struct PrintArgs;

pub async fn run(ctx: super::CmdCtx<PrintArgs>) -> anyhow::Result<()> {
    let screenshot = ScreenshotRequest::default()
        .identifier(WindowIdentifier::default())
        .interactive(true)
        .modal(true)
        .send()
        .await?;

    println!("{:?}", screenshot);
    let screenshot = screenshot.response();
    println!("{:?}", screenshot);

    if let Ok(screenshot) = screenshot {
        if let Ok(path) = screenshot.uri().to_file_path() {
            if let Ok(data) = fs::read(&path).await {
                let hash = gxhash64(&data, CONFIGS.cyan.hash_seed).to_string();
                Screenshot::add(
                    &ctx.db,
                    Screenshot::new(
                        Utc::now().timestamp(),
                        Some(path.display()),
                        false,
                        data,
                        hash,
                    ),
                )
                .await?;
            }
        }
    } else {
        let a = screenshot.unwrap_err();
        println!("{:?}", a);

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
