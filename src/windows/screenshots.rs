use std::{sync::mpsc};

use known_folders::{get_known_folder_path, KnownFolder};
use notify::{event::{AccessKind, AccessMode}, Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};

pub async fn start_watcher() -> anyhow::Result<()> {
    let (tx, rx) = mpsc::channel();
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    watcher.watch(&get_known_folder_path(KnownFolder::Screenshots).unwrap(), RecursiveMode::Recursive)?;
    
    for res in rx {
        match res {
            Ok(event) => {
                if let EventKind::Access(kind) = event.kind {
                    if let AccessKind::Close(mode) = kind {
                        if mode == AccessMode::Write {
                            let source = event.paths[0].as_path();
                           println!("{:?}", source);
                        }
                    }
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}
