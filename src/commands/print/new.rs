use crate::database::{Conn, Repos, screenshots::NewScreenshot};
use std::{
    io::Write,
    process::{Command, Stdio},
    thread,
};

pub async fn run(mut conn: Conn) -> anyhow::Result<()> {
    let ss = Command::new("grimblast")
        .args(["--freeze", "save", "area", "-"])
        .output()
        .unwrap();

    if !ss.status.success() {
        return Err(anyhow::anyhow!("{}", String::from_utf8_lossy(&ss.stderr)));
    }

    conn.screenshot()
        .create(&NewScreenshot::new(&ss.stdout))
        .await?;

    let mut clip = Command::new("wl-copy")
        .args(["--type", "image/png"])
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();

    thread::scope(|s| {
        s.spawn(|| {
            let mut stdin = clip.stdin.take().unwrap();
            stdin.write_all(&ss.stdout).unwrap();
        });
    });

    clip.wait().unwrap();

    Ok(())
}
