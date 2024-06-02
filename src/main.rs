use arguments::{Cli, Commands};
use clap::Parser;
use commands::print;

mod arguments;
mod commands;
mod database;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let conn = database::get()?;

    match args.command {
        Commands::Print => print::run(conn),
    }
    .await
}
