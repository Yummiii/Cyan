use chrono::Utc;
use known_folders::{get_known_folder_path, KnownFolder};
use notify::{event::ModifyKind, Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use rusqlite::Connection;
use std::path::Path;
use tokio::{fs, sync::mpsc};

pub async fn start_watcher(conn: Connection) -> anyhow::Result<()> {
    let (tx, mut rx) = mpsc::channel(1);
    let mut watcher = RecommendedWatcher::new(
        move |res| {
            tx.blocking_send(res).unwrap();
        },
        Config::default(),
    )?;

    let dir = get_known_folder_path(KnownFolder::Screenshots).unwrap();
    watcher.watch(&dir, RecursiveMode::Recursive)?;

    while let Some(res) = rx.recv().await {
        match res {
            Ok(event) => {
                if let EventKind::Modify(ModifyKind::Any) = event.kind {
                    add_to_db(&conn, event.paths[0].as_path()).await?;
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}

async fn add_to_db(conn: &Connection, path: &Path) -> anyhow::Result<()> {
    println!("{:?}", path);
    if let Ok(data) = fs::read(&path).await {
        conn.execute(
            "INSERT INTO screenshots (created_at, original_path, data) VALUES (?1, ?2, ?3)",
            (Utc::now().timestamp(), path.to_str().unwrap(), data),
        )?;
    }
    Ok(())
}
