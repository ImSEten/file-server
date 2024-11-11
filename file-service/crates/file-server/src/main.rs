use std::process::exit;

use actix_web::{App, HttpServer};
use clap::{CommandFactory, Parser};
use common::server::ServerInterface;
use common::Result;

use env_logger::Env;
use http_service::file_server::{index, list_file};
use log::{error, info};
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

struct GrpcRequest {}
struct GrpcResponse {}

struct HttpRequest {}
struct HttpResponse {}

#[tonic::async_trait]
impl ServerInterface<HttpRequest, HttpResponse> for FileServer {
    async fn start(&self) -> Result<()> {
        let addr = format!("{}:{}", self.ip, self.port);
        info!("Server is listening to {}:{}", self.ip, self.port);
        let http_task = tokio::spawn(async move {
            HttpServer::new(move || App::new().service(index).service(list_file))
                .bind(&addr)
                .expect("bind address fail")
                .workers(4)
                .run()
                .await
                .expect("start http failed!");
        });
        let _ = http_task.await;
        Ok(())
    }
    async fn stop(&self) -> Result<()> {
        todo!()
    }

    async fn stats(&self) -> Result<()> {
        todo!()
    }

    async fn request(&self, _request: HttpRequest) -> Result<HttpResponse> {
        todo!()
    }
}

#[tonic::async_trait]
impl ServerInterface<GrpcRequest, GrpcResponse> for FileServer {
    async fn start(&self) -> Result<()> {
        let addr = (self.ip.clone() + ":" + self.port.to_string().as_str())
            .parse()
            .expect("cannot parse addr");
        info!("Server is listening to {}:{}", self.ip, self.port);
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
        Ok(())
    }

    async fn stop(&self) -> Result<()> {
        todo!()
    }

    async fn stats(&self) -> Result<()> {
        todo!()
    }

    async fn request(&self, _request: GrpcRequest) -> Result<GrpcResponse> {
        todo!()
    }
}

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
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
        Some(flags::Commands::Start { ip, port, protocol }) => {
            let file_server = FileServer::new(ip, port);
            info!("service starting...");
            //todo ! start server also include http
            //may use --type =http to use http service
            match protocol.as_str() {
                "grpc" => {
                    let grpc_server = Box::new(file_server)
                        as Box<dyn ServerInterface<GrpcRequest, GrpcResponse>>;
                    grpc_server
                        .start()
                        .await
                        .expect("start grpc server failed!");
                }
                "http" => {
                    let http_server = Box::new(file_server)
                        as Box<dyn ServerInterface<HttpRequest, HttpResponse>>;
                    http_server
                        .start()
                        .await
                        .expect("start http server failed!");
                }
                _ => {
                    error!("unknown  protocol  {}", protocol);
                    exit(-1)
                }
            }

            info!("service exited");
        }
        Some(flags::Commands::Stop) => {
            info!("not implement!")
        }
        None => {
            if let Err(e) = flags::Flags::command().print_help() {
                info!("print_help failed {:?}", e);
            };
        }
    }
}
