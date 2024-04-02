use crate::Error;
use crate::{ctx::Ctx, routes::AUTH_TOKEN};
use async_trait::async_trait;
use axum::http::request::Parts;
use axum::{
    body::Body, extract::FromRequestParts, http::Request, middleware::Next, response::Response,
};
use tower_cookies::{Cookie, Cookies};
use uuid::Uuid;

// TODO: Return Result<Response, Err>
pub async fn mw_require_auth(
    ctx: Result<Ctx, Error>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, Error> {
    println!("->> {:<12} - mw_require_auth - {ctx:?} ", "MIDDLEWARE");

    ctx?;

    Ok(next.run(req).await)
}

pub async fn mw_ctx_resolver(
    // _state: AppData,
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, Error> {
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    let result_ctx = match auth_token
        .ok_or(Error::AuthFailNoAuthTokenCookie)
        .and_then(parse_token)
    {
        Ok((user_id, _exp, _sign)) => {
            // TODO: Token components validations
            Ok(Ctx::new(user_id))
        }
        Err(e) => Err(e),
    };

    // Remove the cookie if something else went wrong
    if result_ctx.is_err() && !matches!(result_ctx, Err(Error::AuthFailNoAuthTokenCookie)) {
        cookies.remove(Cookie::from(AUTH_TOKEN))
    }

    // Store the ctx_result in the request extension.
    req.extensions_mut().insert(result_ctx);

    Ok(next.run(req).await)
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Error> {
        println!("->> {:<12} - Ctx", "EXTRACTOR");

        parts
            .extensions
            .get::<Result<Ctx, Error>>()
            .ok_or(Error::AuthFailCtxNotInRequestExt)?
            .clone()
    }
}

/// Parse a token of format `[user_id].[expiration].[signature]`
/// Returns(user_id,expiration,signature)
fn parse_token(token: String) -> Result<(Uuid, String, String), Error> {
    let (_whole, user_id, exp, sign) =
        lazy_regex::regex_captures!(r#"(.*?)\.(.+)\.(.+)"#, &token).ok_or(Error::GotHere)?;

    let user_id: Uuid =
        Uuid::parse_str(user_id).map_err(|_| Error::AuthFailTokenUserIdWrongFormat)?;

    Ok((user_id, exp.to_string(), sign.to_string()))
}
