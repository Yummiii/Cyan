use crate::commands::Command;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}
