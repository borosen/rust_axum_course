#![allow(unused)]

use axum::{http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize, Deserialize)]
pub enum Error {
    LoginFail,

    // Auth error
    AuthFailNoAuthTokenCookie,
    
    // Model errors
    TicketDeleteFailIdNotFound {id: u64},
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");

        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}

// region:    --- Error boilerplate

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

// endregion: --- Error boilerplate