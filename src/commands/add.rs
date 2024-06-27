use super::CmdCtx;
use crate::{database::screenshot::Screenshot, CONFIGS};
use clap::Parser;
use filetime::FileTime;
use glob::glob;
use gxhash::gxhash64;
use std::path::PathBuf;
use tokio::fs;

#[derive(Debug, Parser)]
pub struct AddArgs {
    #[arg(short, long)]
    pub path: String,
}

pub async fn run(ctx: CmdCtx<AddArgs>) -> anyhow::Result<()> {
    let files = if PathBuf::from(&ctx.args.path).is_file() {
        vec![ctx.args.path]
    } else {
        let mut files = vec![];
        for entry in glob(&format!("{}/*.png", ctx.args.path)).unwrap().flatten() {
            files.push(entry.display().to_string());
        }
        files
    };

    for file in files {
        let file = PathBuf::from(&file);
        if let Ok(data) = fs::read(&file).await {
            let hash = gxhash64(&data, CONFIGS.cyan.hash_seed).to_string();
            if Screenshot::from_hash(&ctx.db, &hash).await?.is_none() {
                let time = FileTime::from_last_modification_time(&fs::metadata(&file).await?);
                Screenshot::add(
                    &ctx.db,
                    Screenshot::new(
                        time.unix_seconds(),
                        Some(file.canonicalize()?.display()),
                        false,
                        data,
                    ),
                )
                .await?;

                println!("Added: {} [{}]", file.display(), hash);
            }
        }
    }

    Ok(())
}
