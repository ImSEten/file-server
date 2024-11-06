use clap::{CommandFactory, Parser};
use tonic::transport::Server;

mod flags;

use service_protos::proto_file_service;

pub struct FileServer {
    ip: String,
    port: u16,
}

impl FileServer {
    pub fn new(ip: String, port: u16) -> Self {
        FileServer { ip, port }
    }
}

fn main() {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(2)
        .max_blocking_threads(2)
        .build()
        .expect("build tokio runtime error!");
    runtime.block_on(async_main());
}

async fn async_main() {
    let parse_flags = flags::Flags::parse();
    match parse_flags.command {
        Some(flags::Commands::Start { ip, port }) => {
            let file_server = FileServer::new(ip, port);
            println!("service starting...");
            create_grpc_service(file_server).await;
            println!("service exited");
        }
        Some(flags::Commands::Stop) => {
            println!("not implement!")
        }
        None => {
            if let Err(e) = flags::Flags::command().print_help() {
                println!("print_help failed {:?}", e);
            };
        }
    }
}

pub async fn create_grpc_service(file_server: FileServer) {
    let addr = (file_server.ip.clone() + ":" + file_server.port.to_string().as_str())
        .parse()
        .expect("cannot parse addr");
    println!(
        "Server is listening to {}:{}",
        file_server.ip, file_server.port
    );
    let trace_layer =
        tower::ServiceBuilder::new().layer(tower_http::trace::TraceLayer::new_for_grpc());
    let server = grpc_service::file_server::FileServer::default();
    let svc = proto_file_service::grpc_file_server::GrpcFileServer::new(server);
    Server::builder()
        .layer(trace_layer)
        .add_service(svc)
        .serve(addr)
        .await
        .expect("serve server err!");
}
