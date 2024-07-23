use axum::{response::Html, routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route(
        "/hello",
        get(|| async { Html("Hello world")})
    );

    // region:    --- Start server
    let tcp_listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("--> LISTENING  on {:?}", tcp_listener.local_addr().unwrap());
    axum::serve(tcp_listener, routes_hello.into_make_service()).await.unwrap();
    
}
