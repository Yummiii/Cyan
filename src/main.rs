use arguments::{Cli, Commands};
use clap::Parser;
use commands::sync;
use commands::{add, print, CmdCtx};
use configs::Configs;
use lazy_static::lazy_static;

mod arguments;
mod commands;
mod configs;
mod database;

lazy_static! {
    pub static ref CONFIGS: Configs = Configs::get();
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let pool = database::init().await?;

    match args.command {
        Commands::Print(args) => print::run(CmdCtx::new(pool, args)).await,
        Commands::Sync(args) => sync::run(CmdCtx::new(pool, args)).await,
        Commands::Add(args) => add::run(CmdCtx::new(pool, args)).await,
    }
}
