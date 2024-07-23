use axum::{extract::Query, response::{Html, IntoResponse}, routing::get, Router};
use serde::Deserialize;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route(
        "/hello",
        get(handler_hello)
    );

    // region:    --- Start server
    let tcp_listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("--> LISTENING  on {:?}", tcp_listener.local_addr().unwrap());
    axum::serve(tcp_listener, routes_hello.into_make_service()).await.unwrap();
    // endregion: --- Start server 
}


// region:    --- Hello handler

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>
}

async fn handler_hello(Query(HelloParams{name}): Query<HelloParams>) -> impl IntoResponse {
    println!("--> {:<12} - handler_hello", "HANDLER");
    let name = name.as_deref().unwrap_or("world");
    Html(format!("Hello <strong>{name}</strong>"))
}

// endregion: --- Hello handler