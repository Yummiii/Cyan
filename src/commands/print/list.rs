use crate::database::{Conn, Repos};
use clap::Parser;
use tabled::{Table, settings::Style};

#[derive(Debug, Parser)]
pub struct Args {
    /// Whether to list the saved screenshots
    #[clap(long, short, default_value_t = false)]
    pub saved: bool,
    /// Returns all screenshots
    #[clap(long, short, default_value_t = false)]
    pub all: bool,
}

pub async fn run(args: Args, mut conn: Conn) -> anyhow::Result<()> {
    let list = if args.all {
        conn.screenshot().list_meta().await?
    } else {
        conn.screenshot().list_meta_saved(args.saved).await?
    };

    let mut table = Table::new(&list);
    table.with(Style::modern());

    println!("{table}");
    println!("Total screenshots: {}", list.len());

    Ok(())
}
