
use axum::body::Body;
use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use lazy_regex::regex_captures;
use tower_cookies::Cookies;

use crate::web::AUTH_TOKEN;
use crate::{Error, Result};

pub async fn mw_require_auth(cookies: Cookies, req: Request<Body>, next: Next) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth", "MIDDLEWARE");

    let auth_cookie = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());


    // parse toekn
    let parsed_token = auth_cookie.ok_or(Error::AuthFailNoAuthTokenCookie)
        .and_then(parse_token)?;

    println!("--> {:<12} - mw_require_auth - {parsed_token:?}", "MIDDLEWARE");

    Ok(next.run(req).await)
}

// Parse token of format `user-[userid].[expiration].[signature]`
// Returns (user_id, expiration, signature)
fn parse_token(token:String) -> Result<(u64, String, String)> {
    println!("->> {:<12} - parse_token - {token}", "MIDDLEWARE");

    let (_whole, user_id, expiration, signature) = regex_captures!(
        r#"^user-(\d+)\.(.+)\.(.+)$"#,
        &token
    ).ok_or(Error::AuthFailTokenWrongFormat)?;
    let user_id:u64 = user_id.parse().map_err(|_| Error::AuthFailTokenWrongFormat)?;
    Ok((user_id, expiration.to_owned(), signature.to_owned()))
}