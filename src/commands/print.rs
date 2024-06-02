use chrono::Utc;
use rusqlite::Connection;

#[cfg(target_os = "linux")]
pub async fn run(conn: Connection) -> anyhow::Result<()> {
    use ashpd::{desktop::screenshot::ScreenshotRequest, WindowIdentifier};
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
                    "INSERT INTO screenshots (created_at, original_path, data) VALUES (?1, ?2, ?3)",
                    (Utc::now().timestamp(), path.to_str().unwrap(), data),
                )?;
            }
        }
    }

    Ok(())
}

#[cfg(target_os = "windows")]
pub async fn run(conn: Connection) -> anyhow::Result<()> {
    //i cannot express how much i hate windows
    use clipboard_win::{formats, get, get_clipboard, Monitor};
    use image::io::Reader;
    use std::{io::Cursor, process::Command, time::Duration};
    use tokio::{task, time::timeout};

    let cmd = Command::new("powershell")
        .args(&[
            "-Command",
            "Start-Process",
            "-FilePath",
            "ms-screenclip://?source=Cyan",
            "-Wait",
        ])
        .output()
        .unwrap();

    //??????????????????????????????

    if let Ok(data) = get_clipboard(formats::Bitmap) {
        let img = Reader::new(Cursor::new(data))
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap();

        let mut bytes: Vec<u8> = Vec::new();
        img.write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Png)
            .unwrap();

        conn.execute(
            "INSERT INTO screenshots (created_at, data) VALUES (?1, ?2)",
            (Utc::now().timestamp(), bytes),
        )
        .unwrap();
    }

    Ok(())
}
