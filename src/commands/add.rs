use super::CmdCtx;
use crate::{
    database::{screenshot::Screenshot, Db},
    CONFIGS,
};
use chrono::Utc;
use clap::Parser;
use filetime::FileTime;
use glob::glob;
use gxhash::gxhash64;
use nestify::nest;
use std::{
    io::{stdin, Read},
    path::PathBuf,
};
use tokio::fs;

nest! {
    #[derive(Debug, Parser)]*
    pub struct AddArgs {
        #[clap(subcommand)]
        pub command: pub enum AddCommands {
            Path(AddPathArgs),
            Stdin,
        }
    }
}

#[derive(Debug, Parser)]
pub struct AddPathArgs {
    pub path: String,
}

pub async fn run(ctx: CmdCtx<AddArgs>) -> anyhow::Result<()> {
    match ctx.args.command {
        AddCommands::Path(args) => add_path(ctx.db, args.path).await,
        AddCommands::Stdin => add_stdin(ctx.db).await,
    }
}

async fn add_stdin(db: Db) -> anyhow::Result<()> {
    let mut buffer = vec![];
    stdin().read_to_end(&mut buffer)?;

    if let Ok(hash) = add(&db, buffer, None, Utc::now().timestamp()).await {
        println!("Added: [{}]", hash);
    }

    Ok(())
}

async fn add_path(db: Db, path: String) -> anyhow::Result<()> {
    let files = if PathBuf::from(&path).is_file() {
        vec![path]
    } else {
        let mut files = vec![];
        for entry in glob(&format!("{}/*.png", path)).unwrap().flatten() {
            files.push(entry.display().to_string());
        }
        files
    };

    for file in files {
        let file = PathBuf::from(&file);
        if let Ok(data) = fs::read(&file).await {
            let time = FileTime::from_last_modification_time(&fs::metadata(&file).await?);
            let path = Some(file.canonicalize()?.display().to_string());

            if let Ok(hash) = add(&db, data, path, time.unix_seconds()).await {
                println!("Added: {} [{}]", file.display(), hash);
            }
        }
    }

    Ok(())
}

async fn add(db: &Db, data: Vec<u8>, path: Option<String>, time: i64) -> anyhow::Result<String> {
    let hash = gxhash64(&data, CONFIGS.cyan.hash_seed).to_string();
    if Screenshot::from_hash(db, &hash).await?.is_none() {
        Screenshot::add(db, Screenshot::new(time, path, false, data, hash.clone())).await?;

        Ok(hash)
    } else {
        Err(anyhow::anyhow!("Screenshot already exists"))
    }
}
