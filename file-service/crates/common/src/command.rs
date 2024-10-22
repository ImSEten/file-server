use std::io::Result;
use tokio::io::{AsyncRead, AsyncReadExt};

pub struct Command {
    pub name: String,
    pub cmd: tokio::process::Command,
}

impl Command {
    pub fn new(name: String, args: Vec<String>) -> Self {
        let mut cmd = tokio::process::Command::new(name.clone());
        cmd.args(args);
        cmd.stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped());

        Command { name, cmd }
    }

    pub async fn run(name: String, args: Vec<String>) -> Result<(String, String)> {
        let mut cmd = Self::new(name, args);
        let child = cmd.cmd.spawn().expect("start cmd error");
        let _pid = child.id().expect("cannot get child pid");
        let (stdout, stderr, status) = tokio::join!(
            read_std(child.stdout),
            read_std(child.stderr),
            cmd.cmd.status()
        );
        status?;
        Ok((stdout, stderr))
    }
}

pub async fn read_std<T>(std: Option<T>) -> String
where
    T: AsyncRead + Unpin,
{
    let mut std = std;
    if let Some(mut std) = std.take() {
        let mut out = String::new();
        std.read_to_string(&mut out).await.unwrap_or_else(|e| {
            println!("read to string error {:?}", e);
            0
        });
        return out;
    }
    String::new()
}
