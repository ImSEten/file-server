pub const IP: &str = "127.0.0.1";
pub const PORT: u16 = 10086;

#[derive(clap::Parser)]
#[command(name = "FileClient")]
#[command(about = "FileClient is my own file server's client", long_about = None)]
pub struct Flags {
    /// server listening ip addr
    #[arg(long, help = "server listening ip addr", default_value = IP)]
    pub ip: String,

    /// server listening ip port
    #[arg(short, long, default_value_t = PORT, help = "server listening ip port")]
    port: u16,

    /// 子命令
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// 子命令枚举
#[derive(clap::Subcommand)]
pub enum Commands {
    #[command(
        name = "file",
        about = "file subcommand, to operate the file in server"
    )]
    File {
        #[command(subcommand)]
        command: Option<FileCommand>,
    },
}

#[derive(clap::Subcommand)]
pub enum FileCommand {
    #[command(name = "list", about = "list files in server")]
    List {},
    #[command(name = "upload_file", about = "upload files to server")]
    UploadFile {},
    #[command(name = "download_file", about = "download files from server")]
    DownloadFile {},
    #[command(name = "delete_file", about = "delete files from server")]
    DeleteFile {},
}
