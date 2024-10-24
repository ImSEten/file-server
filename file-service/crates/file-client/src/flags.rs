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
    #[command(name = "upload-file", about = "upload files to server")]
    UploadFile {
        /// server listening ip port
        #[arg(name = "local-file", short, long, help = "the local file, can be abs or relative")]
        local_file: String,
        /// server listening ip port
        #[arg(
            name = "remote-dir",
            short,
            long,
            help = "remote_dir is the dir in server, the file_name is the same as local"
        )]
        remote_dir: String,
    },
    #[command(name = "download-file", about = "download files from server")]
    DownloadFile {
        /// server listening ip port
        #[arg(short, long, default_value_t = String::new(), help = "download the file from server")]
        file: String,
    },
    #[command(name = "delete-file", about = "delete files from server")]
    DeleteFile {
        /// server listening ip port
        #[arg(short, long, default_value_t = String::new(), help = "delete file from server")]
        file: String,
    },
}
