use super::CmdCtx;
use crate::{database::screenshot::Screenshot, CONFIGS};
use chrono::{TimeZone, Utc};
use clap::{arg, Parser};
use std::path::Path;
use tokio::fs;

#[derive(Debug, Parser)]
pub struct SyncArgs {
    #[arg(short, long)]
    #[clap(default_value_t = false)]
    pub delete: bool,
    pub path: String,
}

pub async fn run(ctx: CmdCtx<SyncArgs>) -> anyhow::Result<()> {
    let mut i = 0;
    for ss in Screenshot::get_all_unsynced(&ctx.db).await? {
        let timestamp = Utc.timestamp_opt(ss.created_at, 0).unwrap();

        let dir = format!("{}/{}", ctx.args.path, timestamp.format("%Y-%m"));
        let dir = Path::new(&dir);

        fs::create_dir_all(&dir).await?;
        fs::write(dir.join(format!("{}.png", ss.created_at)), ss.data).await?;

        if ctx.args.delete || CONFIGS.cyan.delete_after_sync {
            if let Some(original) = ss.original_path {
                fs::remove_file(original).await?;
                Screenshot::delete(&ctx.db, ss.id).await?;
            }
        } else {
            Screenshot::set_synced(&ctx.db, ss.id, true).await?;
        }

        println!("Synced: {} [{}]", ss.created_at, ss.hash);
        i += 1;
    }
    println!("Synced {} files", i);

    Ok(())
}
