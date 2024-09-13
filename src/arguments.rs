use clap::Parser;
use nestify::nest;
use crate::commands::{add::AddArgs, print::PrintArgs, sync::SyncArgs};

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
