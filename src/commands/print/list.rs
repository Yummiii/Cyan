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
    /// The number of items per page
    #[clap(long, short = 'l', default_value_t = 15)]
    pub page_size: i32,
    /// The page number to display
    #[clap(long, short, default_value_t = 1)]
    pub page: i32,
}

pub async fn run(args: Args, mut conn: Conn) -> anyhow::Result<()> {
    let (list, count) = if args.all {
        (
            conn.screenshot()
                .paginate_meta(args.page, args.page_size)
                .await?,
            conn.screenshot().count().await?,
        )
    } else {
        (
            conn.screenshot()
                .paginate_meta_saved(args.saved, args.page, args.page_size)
                .await?,
            conn.screenshot().count_saved(args.saved).await?,
        )
    };

    let mut table = Table::new(&list);
    table.with(Style::modern());

    println!("{table}");
    println!(
        "Total screenshots: {count}; Pages: {}",
        (count + args.page_size - 1) / args.page_size
    );

    Ok(())
}
