pub use self::error::{Error, Result};

use axum::{
    extract::{Path, Query},
    http::{Method, Uri},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::get,
    Json, Router,
};
use ctx::Ctx;
use log::log_request;
use model::ModelController;
use serde::Deserialize;
use serde_json::json;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use uuid::Uuid;

mod ctx;
mod error;
mod log;
mod model;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    let mc = ModelController::new().await?;

    let routes_apis = web::routes_tickets::routes(mc.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", routes_apis)
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn_with_state(
            mc.clone(),
            web::mw_auth::mw_ctx_resolver,
        ))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    // region:    --- Start server
    let tcp_listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("--> LISTENING  on {:?}", tcp_listener.local_addr().unwrap());
    println!();
    axum::serve(tcp_listener, routes_all.into_make_service())
        .await
        .unwrap();
    // endregion: --- Start server

    Ok(())
}

fn routes_static() -> Router {
    Router::new().nest_service("/", ServeDir::new("./"))
}

async fn main_response_mapper(
    ctx: Option<Ctx>,
    uri: Uri,
    req_method: Method,
    res: Response,
) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

    let uuid = Uuid::new_v4();
    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());

    // If client error, build new response
    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "error": {
                    "type": client_error.as_ref(),
                    "req_uuid": uuid.to_string()
                }
            });
            println!("    ->> client_body_error {client_error_body}");
            // Build the new response from  the client error
            (*status_code, Json(client_error_body)).into_response()
        });

    // Build server log line
    let client_error = client_status_error.unzip().1;
    let _log = log_request(uuid, req_method, uri, ctx, service_error, client_error).await;
    println!();
    error_response.unwrap_or(res)
}

// region:    --- Routes Hello

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello/:name", get(handler_hello2))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// e.g., `/hello?name=Jane`
async fn handler_hello(Query(HelloParams { name }): Query<HelloParams>) -> impl IntoResponse {
    println!("--> {:<12} - handler_hello", "HANDLER");
    let name = name.as_deref().unwrap_or("world");
    Html(format!("Hello <strong>{name}</strong>"))
}

// e.g., `/hello/Mark`
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("--> {:<12} - handler_hello2 -- {name}", "HANDLER");

    Html(format!("Hello {name}"))
}

// endregion: --- Routes Hello
