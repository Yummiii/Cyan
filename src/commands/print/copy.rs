use crate::database::{Conn, Repos};
use clap::Parser;
use std::{
    io::Write,
    process::{Command, Stdio},
    thread,
};

#[derive(Debug, Parser)]
pub struct Args {
    /// ID of the screenshot to copy. If not provided, the latest screenshot will be used
    pub id: Option<i64>,
}

pub async fn run(args: Args, mut conn: Conn) -> anyhow::Result<()> {
    let ss = if let Some(id) = args.id {
        conn.screenshot().get(id).await?
    } else {
        conn.screenshot().get_latest().await?
    };

    let mut clip = Command::new("wl-copy")
        .args(["--type", "image/png"])
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();

    thread::scope(|s| {
        s.spawn(|| {
            let mut stdin = clip.stdin.take().unwrap();
            stdin.write_all(&ss.data).unwrap();
        });
    });

    clip.wait().unwrap();

    Ok(())
}
