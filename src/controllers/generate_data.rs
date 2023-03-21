use std::sync::{Arc, RwLock};

use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use uuid::Uuid;

use crate::db::AppData;

pub async fn generate_mock_data(
    Path(user_id): Path<String>,
    state: Arc<RwLock<AppData>>,
) -> impl IntoResponse {
    todo!()
}
