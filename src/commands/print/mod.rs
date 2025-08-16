use crate::database::Conn;
use clap::Parser;

mod list;
mod new;
mod save;
mod copy;

#[derive(Debug, Parser)]
pub enum PrintCommand {
    /// New screenshot
    New,
    /// List screenshots in the database
    List(list::Args),
    /// Save the screenshots in the database to a directory
    Save(save::Args),
    /// Copy a screenshot from the database to the clipboard
    Cp(copy::Args),
}

impl PrintCommand {
    pub async fn run(self, conn: Conn) -> anyhow::Result<()> {
        match self {
            PrintCommand::New => new::run(conn).await,
            PrintCommand::List(args) => list::run(args, conn).await,
            PrintCommand::Save(args) => save::run(args, conn).await,
            PrintCommand::Cp(args) => copy::run(args, conn).await,
        }
    }
}
