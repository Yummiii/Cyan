use clap::Parser;
use nestify::nest;

#[cfg(target_os = "linux")]
nest! {
    #[derive(Debug, Parser)]*
    pub struct Cli {
        #[clap(subcommand)]
        pub command: pub enum Commands {
            Print,
            Sync {
                #[arg(short, long)]
                #[clap(default_value_t = false)]
                delete: bool,
                #[arg(short, long)]
                path: String
            }
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
