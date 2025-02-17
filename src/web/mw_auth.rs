use axum::async_trait;
use axum::body::Body;
use axum::extract::{FromRequestParts, Request};
use axum::http::request::Parts;
use axum::middleware::Next;
use axum::response::Response;
use lazy_regex::regex_captures;
use tower_cookies::{Cookie, Cookies};

use crate::ctx::Ctx;
use crate::web::AUTH_TOKEN;
use crate::{Error, Result};

pub async fn mw_require_auth(ctx: Result<Ctx>, req: Request<Body>, next: Next) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth", "MIDDLEWARE");

    ctx?;

    Ok(next.run(req).await)
}

pub async fn mw_ctx_resolver(
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response> {
    println!("->> {:<12} - mw_ctx_resolver", "MIDDLEWARE");

    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    let result_ctx = match auth_token
        .ok_or(Error::AuthFailNoAuthTokenCookie)
        .and_then(parse_token)
    {
        Ok((userid, _exp, _sign)) => Ok(Ctx::new(userid)),
        Err(e) => Err(e),
    };

    if result_ctx.is_err() && !matches!(result_ctx, Err(Error::AuthFailNoAuthTokenCookie)) {
        cookies.remove(Cookie::build(AUTH_TOKEN).build())
    }

    req.extensions_mut().insert(result_ctx);

    Ok(next.run(req).await)
}

#[async_trait]
// region:    --- Ctx Extractor
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        println!("->> {:<12} - Ctx", "EXTRACTOR");

        parts
            .extensions
            .get::<Result<Ctx>>()
            .ok_or(Error::AuthFailCtxNotInRequestExt)?
            .clone()
    }
}
// endregion: --- Ctx Extractor

// Parse token of format `user-[userid].[expiration].[signature]`
// Returns (user_id, expiration, signature)
fn parse_token(token: String) -> Result<(u64, String, String)> {
    println!("->> {:<12} - parse_token - {token}", "MIDDLEWARE");

    let (_whole, user_id, expiration, signature) =
        regex_captures!(r#"^user-(\d+)\.(.+)\.(.+)$"#, &token)
            .ok_or(Error::AuthFailTokenWrongFormat)?;
    let user_id: u64 = user_id
        .parse()
        .map_err(|_| Error::AuthFailTokenWrongFormat)?;
    Ok((user_id, expiration.to_owned(), signature.to_owned()))
}
