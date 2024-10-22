use clap::{CommandFactory, Parser};
use std::sync::Arc;
use tokio::sync::Mutex;

use file_service::file_client::Client;

mod flags;

fn main() {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(2)
        .max_blocking_threads(2)
        .build()
        .expect("build tokio runtime error!");
    runtime.block_on(async_main());
}

async fn create_client(server_ip: String, port: String) -> Arc<Mutex<Client>> {
    Arc::new(Mutex::new(
        Client::new(server_ip.to_string(), port.to_string()).await,
    ))
}

async fn async_main() {
    let parse_flags = flags::Flags::parse();
    let client = create_client(flags::IP.to_string(), flags::PORT.to_string()).await;
    match parse_flags.command {
        Some(flags::Commands::File { command }) => match command {
            Some(flags::FileCommand::List {}) => {
                if let Err(e) = client.lock().await.list().await {
                    println!("list returns error: {:?}", e);
                }
            }
            Some(flags::FileCommand::Upload {}) => {
                if let Err(e) = client.lock().await.upload().await {
                    println!("upload returns error: {:?}", e);
                }
            }
            Some(flags::FileCommand::Download {}) => {
                if let Err(e) = client.lock().await.download().await {
                    println!("download returns error: {:?}", e);
                }
            }
            Some(flags::FileCommand::Delete {}) => {
                if let Err(e) = client.lock().await.delete().await {
                    println!("delete returns error: {:?}", e);
                }
            }
            None => {
                let mut cmd = flags::Flags::command();
                for s in cmd.get_subcommands_mut() {
                    if s.get_name() == "file" {
                        s.print_help().expect("print subcommand file help failed");
                    }
                }
            }
        },
        None => {
            if let Err(e) = flags::Flags::command().print_help() {
                println!("print_help failed {:?}", e);
            };
        }
    }
}
