use std::sync::{Arc, RwLock};

use axum::{http::StatusCode, response::IntoResponse};

use crate::db::AppData;

pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "OK").into_response()
}
