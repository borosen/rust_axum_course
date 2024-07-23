#![allow(unused)]

use axum::{routing::post, Json, Router};
use serde::{Deserialize};
use serde_json::{json, Value};

use crate::{Error, Result};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

async fn api_login(Json(LoginPayload{username, password}): Json<LoginPayload>) -> Result<Json<Value>> {

    println!("->> {:<12} - api_login", "HANDLER");

    if username != "demo1" || password != "welcome" {
        return Err(Error::LoginFail);
    }

    let body = Json(json!( {
        "result": {
            "success": true
        }
    }));
    Ok(body)
}