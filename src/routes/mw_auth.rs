use crate::routes::AUTH_TOKEN;
use crate::Error;
use axum::{body::Body, http::Request, middleware::Next, response::Response};
use tower_cookies::Cookies;

// TODO: Return Result<Response, Err>
pub async fn mw_require_auth(
    cookies: Cookies,
    req: Request<Body>,
    next: Next,
) -> Result<Response, Error> {
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    auth_token.ok_or(Error::AuthFailNoAuthTokenCookie)?;

    Ok(next.run(req).await)
}
