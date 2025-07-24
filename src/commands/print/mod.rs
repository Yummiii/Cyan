use crate::database::Conn;
use clap::Parser;

mod list;
mod new;
mod save;

#[derive(Debug, Parser)]
pub enum PrintCommand {
    New,
    List(list::Args),
    Save(save::Args),
}

impl PrintCommand {
    pub async fn run(self, conn: Conn) -> anyhow::Result<()> {
        match self {
            PrintCommand::New => new::run(conn).await,
            PrintCommand::List(args) => list::run(args, conn).await,
            PrintCommand::Save(args) => save::run(args, conn).await,
        }
    }
}
