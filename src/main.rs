use rusqlite::Connection;

mod arguments;
mod commands;
mod database;

#[cfg(target_os = "windows")]
mod windows;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let conn = database::get()?;
    run(conn).await
}

#[cfg(target_os = "linux")]
pub async fn run(conn: Connection) -> anyhow::Result<()> {
    use arguments::{Cli, Commands};
    use clap::Parser;
    use commands::print;

    let args = Cli::parse();

    match args.command {
        Commands::Print => print::run(conn),
    }
    .await
}

#[cfg(target_os = "windows")]
pub async fn run(_conn: Connection) -> anyhow::Result<()> {
    use tokio::task;
    use windows::{screenshots, to_tray};

    task::spawn(async move {
        screenshots::start_watcher().await.unwrap();
    });

    to_tray();

    Ok(())
}
