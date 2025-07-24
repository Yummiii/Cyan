use crate::{arguments::Cli, database::init_db};
use clap::Parser;
use dirs::config_dir;
use std::{path::PathBuf, sync::LazyLock};
use tokio::fs::create_dir_all;

mod arguments;
mod commands;
mod database;

static CYAN_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    if cfg!(debug_assertions) {
        PathBuf::from("./")
    } else {
        config_dir().unwrap()
    }
    .join("cyan")
});

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    create_dir_all(&*CYAN_PATH).await?;

    let pool = init_db().await;
    let conn = pool.acquire().await?;

    args.command.run(conn).await
}
