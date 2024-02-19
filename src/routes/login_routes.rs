use crate::{Error, Result};
use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};

pub fn login_routes() -> Router {
    Router::new().route("/login", post(api_login))
}

async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:12} - api_login", "HANDLER");

    // TODO: Implement real auth logic
    if payload.username != "test_user_1" || payload.password != "password" {
        return Err(Error::LoginFail);
    }

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
