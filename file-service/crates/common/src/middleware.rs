// todo: logger middleware

// tower::trace::TraceLayer 是 Tower 库提供的一个中间件层，用于在请求和响应的生命周期中添加详细的日志记录。这对于调试和监控网络服务非常有用，可以帮助你了解请求的处理过程和性能瓶颈。

// 主要功能
// 请求和响应的详细日志记录：TraceLayer 可以在请求到达和响应返回时记录详细的日志信息，包括请求的时间戳、请求和响应的数据、处理时间等。
// 自定义日志格式：你可以通过配置 TraceLayer 来自定义日志的格式和内容，以便更好地满足你的需求。
// 性能监控：通过记录请求的处理时间，TraceLayer 可以帮助你识别性能瓶颈和优化点。
// 使用示例
// 下面是一个使用 TraceLayer 的示例，结合 Hyper 构建一个简单的 HTTP 服务器。

// Cargo.toml
// 确保你的 Cargo.toml 文件包含以下依赖项：

// Toml
// 深色版本
// [dependencies]
// hyper = { version = "0.14", features = ["full"] }
// tower = "0.4"
// tower-http = "0.3"
// tokio = { version = "1", features = ["full"] }
// src/main.rs
// Rust
// 深色版本
// use hyper::{Body, Request, Response, Server};
// use hyper::service::{make_service_fn, service_fn};
// use tower::ServiceBuilder;
// use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
// use std::net::SocketAddr;

// async fn handle_request(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
//     Ok(Response::new(Body::from("Hello, World!")))
// }

// #[tokio::main]
// async fn main() {
//     let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

//     let make_svc = make_service_fn(|_conn| {
//         let service = ServiceBuilder::new()
//             .layer(
//                 TraceLayer::new_for_http()
//                     .make_span_with(DefaultMakeSpan::new().level(log::Level::Info))
//                     .on_request(DefaultOnRequest::new().level(log::Level::Debug))
//                     .on_response(DefaultOnResponse::new().level(log::Level::Debug)),
//             )
//             .service_fn(handle_request);

//         async { Ok::<_, hyper::Error>(service) }
//     });

//     let server = Server::bind(&addr).serve(make_svc);

//     if let Err(e) = server.await {
//         eprintln!("server error: {}", e);
//     }
// }
// 解释代码
// 导入必要的库：导入 hyper、tower、tower_http 和 tokio。
// 定义处理函数：handle_request 函数处理每个 HTTP 请求，并返回一个简单的响应。
// 创建服务构建器：使用 ServiceBuilder 创建一个服务构建器，并添加 TraceLayer。
// 配置 TraceLayer：
// make_span_with：配置如何创建跟踪跨度（span），这里设置日志级别为 Info。
// on_request：配置在请求到达时的日志记录，这里设置日志级别为 Debug。
// on_response：配置在响应返回时的日志记录，这里设置日志级别为 Debug。
// 启动服务器：使用 Server::bind 绑定到指定地址，并启动服务器。
// 日志输出
// 当你运行这个示例并发送 HTTP 请求时，你将会看到类似以下的日志输出：

// 深色版本
// INFO  hyper::server > 127.0.0.1:58392 - 127.0.0.1:3000 "GET / HTTP/1.1" 200
// DEBUG hyper::proto::h1::io > 127.0.0.1:58392 - 127.0.0.1:3000 GET / HTTP/1.1
// DEBUG hyper::proto::h1::io < 127.0.0.1:58392 - 127.0.0.1:3000 HTTP/1.1 200 OK
// 这些日志记录了请求的详细信息，包括请求方法、路径、状态码等。

// 自定义日志格式
// 你可以通过实现 MakeSpan、OnRequest 和 OnResponse 来自定义日志的格式和内容。例如：

// Rust
// 深色版本
// use tower_http::trace::{MakeSpan, OnRequest, OnResponse};
// use tracing::{info_span, Level};

// struct CustomMakeSpan;

// impl MakeSpan for CustomMakeSpan {
//     fn make_span(&self, request: &Request<Body>) -> tracing::Span {
//         info_span!(
//             "request",
//             method = %request.method(),
//             path = %request.uri().path(),
//             headers = ?request.headers(),
//         )
//     }
// }

// struct CustomOnRequest;

// impl OnRequest for CustomOnRequest {
//     fn on_request(&self, request: &Request<Body>, span: &tracing::Span) {
//         span.in_scope(|| {
//             tracing::info!("Handling request");
//         });
//     }
// }

// struct CustomOnResponse;

// impl OnResponse for CustomOnResponse {
//     fn on_response(self, response: &Response<Body>, latency: std::time::Duration, _span: &tracing::Span) {
//         tracing::info!("Request took {}ms", latency.as_millis());
//     }
// }

// #[tokio::main]
// async fn main() {
//     let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

//     let make_svc = make_service_fn(|_conn| {
//         let service = ServiceBuilder::new()
//             .layer(
//                 TraceLayer::new_for_http()
//                     .make_span_with(CustomMakeSpan)
//                     .on_request(CustomOnRequest)
//                     .on_response(CustomOnResponse),
//             )
//             .service_fn(handle_request);

//         async { Ok::<_, hyper::Error>(service) }
//     });

//     let server = Server::bind(&addr).serve(make_svc);

//     if let Err(e) = server.await {
//         eprintln!("server error: {}", e);
//     }
// }
// 通过这种方式，你可以完全控制日志的格式和内容，使其更符合你的需求。

// 希望这些解释和示例能帮助你更好地理解 tower::trace::TraceLayer 的用途和用法。如果有任何问题，欢迎继续提问！