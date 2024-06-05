use arguments::{Cli, Commands};
use clap::Parser;
use commands::sync;
use configs::Configs;
use lazy_static::lazy_static;
use rusqlite::Connection;

mod arguments;
mod commands;
mod configs;
mod database;

#[cfg(target_os = "windows")]
mod windows;

lazy_static! {
    pub static ref CONFIGS: Configs = Configs::get();
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let conn = database::get()?;

    run(conn, args).await
}

#[cfg(target_os = "linux")]
pub async fn run(conn: Connection, args: Cli) -> anyhow::Result<()> {
    use commands::print;

    match args.command {
        Commands::Print => print::run(conn).await,
        Commands::Sync { delete, path } => sync::run(conn, delete, path).await,
    }
}

#[cfg(target_os = "windows")]
pub async fn run(conn: Connection, args: Cli) -> anyhow::Result<()> {
    use tokio::task;
    use windows::{screenshots, to_tray};

    if let Some(command) = args.command {
        match command {
            Commands::Sync => sync::run(conn).await,
        }
    } else {
        task::spawn(async {
            to_tray();
        });

        screenshots::start_watcher(conn).await
    }
}
