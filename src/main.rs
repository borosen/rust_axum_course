#![allow(unused)] // For early development.

// region:    --- Modules

mod config;
mod ctx;
mod error;
mod log;
mod model;
mod web;

pub use self::error::{Error, Result};
pub use config::config; // use crate::config

use crate::model::ModelManager;
use crate::web::mw_auth::mw_ctx_resolve;
use crate::web::mw_res_map::mw_reponse_map;
use crate::web::{routes_login, routes_static};
use axum::{middleware, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;

// endregion: --- Modules

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .without_time() // enable in production
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // Initialize ModelManager.
    let mm = ModelManager::new().await?;

    // -- Define Routes
    // let routes_rpc = rpc::routes(mm.clone())
    //   .route_layer(middleware::from_fn(mw_ctx_require));

    let routes_all = Router::new()
        .merge(routes_login::routes())
        // .nest("/api", routes_rpc)
        .fallback_service(routes_static::serve_dir())
        .layer(middleware::map_response(mw_reponse_map))
        .layer(middleware::from_fn_with_state(mm.clone(), mw_ctx_resolve))
        .layer(CookieManagerLayer::new());

    // region:    --- Start server
    let tcp_listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    info!("LISTENING  on {:?}", tcp_listener.local_addr().unwrap());
    axum::serve(tcp_listener, routes_all.into_make_service())
        .await
        .unwrap();
    // endregion: --- Start server

    Ok(())
}
