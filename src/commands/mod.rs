use crate::{commands::print::PrintCommand, database::Conn};
use clap::Parser;

mod print;

#[derive(Debug, Parser)]
pub enum Command {
    /// Commands related to screenshots
    Prints {
        #[clap(subcommand)]
        command: PrintCommand,
    },
}

impl Command {
    pub async fn run(self, conn: Conn) -> anyhow::Result<()> {
        match self {
            Command::Prints { command } => command.run(conn).await,
        }
    }
}
