use arguments::{Cli, Commands};
use clap::Parser;
use rusqlite::Connection;

mod arguments;
mod commands;
mod database;

#[cfg(target_os = "windows")]
mod windows;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let conn = database::get()?;
    let args = Cli::parse();

    run(conn, args).await
}

#[cfg(target_os = "linux")]
pub async fn run(conn: Connection, args: Cli) -> anyhow::Result<()> {
    use commands::print;

    match args.command {
        Commands::Print => print::run(conn),
    }
    .await
}

#[cfg(target_os = "windows")]
pub async fn run(conn: Connection, args: Cli) -> anyhow::Result<()> {
    use tokio::task;
    use windows::{screenshots, to_tray};

    if let Some(command) = args.command {
        match command {
            Commands::Sync => todo!(),
        }
    } else {
        task::spawn(async {
            to_tray();
        });

        screenshots::start_watcher(conn).await.unwrap();
    }

    Ok(())
}
