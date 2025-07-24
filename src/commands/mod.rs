use crate::{commands::print::PrintCommand, database::Conn};
use clap::Parser;

mod print;

#[derive(Debug, Parser)]
pub enum Command {
    Print {
        #[clap(subcommand)]
        command: PrintCommand,
    },
}

impl Command {
    pub async fn run(self, conn: Conn) -> anyhow::Result<()> {
        match self {
            Command::Print { command } => command.run(conn).await,
        }
    }
}
