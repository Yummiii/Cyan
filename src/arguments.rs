use clap::Parser;
use nestify::nest;

use crate::commands::{add::AddArgs, print::PrintArgs, sync::SyncArgs};

#[cfg(target_os = "linux")]
nest! {
    #[derive(Debug, Parser)]*
    pub struct Cli {
        #[clap(subcommand)]
        pub command: pub enum Commands {
            Print(PrintArgs),
            Sync(SyncArgs),
            Add(AddArgs)
        },
    }
}

#[cfg(target_os = "windows")]
nest! {
    #[derive(Debug, Parser)]*
    pub struct Cli {
        #[clap(subcommand)]
        pub command: Option<pub enum Commands {
            Sync,
        }>
    }
}
