use std::sync::{Arc, RwLock};

use axum::{http::StatusCode, response::IntoResponse};

use crate::db::AppData;

pub async fn generate_mock_data(state: Arc<RwLock<AppData>>) -> impl IntoResponse {
    state.write().unwrap().change_app_state();
    (StatusCode::OK, "New DB state generated").into_response()
}
