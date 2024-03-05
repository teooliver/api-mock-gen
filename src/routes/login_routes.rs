use crate::{routes::AUTH_TOKEN, Error, Result};
use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

pub fn login_routes() -> Router {
    Router::new().route("/login", post(api_login))
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:12} - api_login", "HANDLER");

    // TODO: Implement real auth logic
    if payload.username != "test_user" || payload.password != "password" {
        return Err(Error::LoginFail);
    }

    // TODO: Implement real auth-token generation/signature
    let mut cookie = Cookie::new(AUTH_TOKEN, "user-1.exp.sign");
    cookie.set_http_only(true);
    cookie.set_path("/");
    cookies.add(cookie);

    // TODO: Set cookies

    let body = Json(json!({
        "result":{
            "success": true
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}
