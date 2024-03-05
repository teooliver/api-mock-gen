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
    println!("->> {:<12} - mw_require_auth", "MIDDLEWARE");

    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    // Parse Token
    let (user_id, exp, sign) = auth_token
        .ok_or(Error::AuthFailNoAuthTokenCookie)
        .and_then(parse_token)?;

    // TODO: Token componnts validation

    Ok(next.run(req).await)
}

/// Parse a token of format `user-[user_id].[expiration].[signature]`
/// Returns(user_id,expiration,signature)
fn parse_token(token: String) -> Result<(u64, String, String), Error> {
    let (_whole, user_id, exp, sign) = lazy_regex::regex_captures!(
        r#"ˆuser-(\d+)\.(.+)\.(.+)"#, // literal regex
        &token
    )
    .ok_or(Error::AuthFailTokenWrongFormat)?;

    let user_id: u64 = user_id
        .parse()
        .map_err(|_| Error::AuthFailTokenWrongFormat)?;

    Ok((user_id, exp.to_string(), sign.to_string()))
}
