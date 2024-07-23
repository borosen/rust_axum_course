#![allow(unused)]

pub use self::error::{Error, Result};

use axum::{extract::{Path, Query}, middleware, response::{Html, IntoResponse, Response}, routing::{get, Route}, Router};
use serde::Deserialize;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

mod error;
mod web;

#[tokio::main]
async fn main() {
    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    // region:    --- Start server
    let tcp_listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("--> LISTENING  on {:?}", tcp_listener.local_addr().unwrap());
    println!();
    axum::serve(tcp_listener, routes_all.into_make_service()).await.unwrap();
    // endregion: --- Start server 
}

fn routes_static() -> Router {
    Router::new().nest_service("/", ServeDir::new("./"))
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    println!();

    res
}

// region:    --- Routes Hello

fn routes_hello() -> Router {
    Router::new()
    .route("/hello", get(handler_hello))
    .route(
        "/hello/:name",
        get(handler_hello2)
        
    )
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>
}

// e.g., `/hello?name=Jane`
async fn handler_hello(Query(HelloParams{name}): Query<HelloParams>) -> impl IntoResponse {
    println!("--> {:<12} - handler_hello", "HANDLER");
    let name = name.as_deref().unwrap_or("world");
    Html(format!("Hello <strong>{name}</strong>"))
}

// e.g., `/hello/Mark`
async fn handler_hello2(Path(name): Path<String>)-> impl IntoResponse {
    println!("--> {:<12} - handler_hello2 -- {name}", "HANDLER");

    Html(format!("Hello {name}"))
}

// endregion: --- Routes Hello