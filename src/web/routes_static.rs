use axum::Router;
use tower_http::services::ServeDir;

pub fn serve_dir() -> Router {
    Router::new().nest_service("/", ServeDir::new("./"))
}
