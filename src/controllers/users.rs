use std::sync::{Arc, RwLock};

use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use uuid::Uuid;

use crate::{
    db::AppData,
    models::{NewUser, User},
};

pub async fn get_users(state: Arc<RwLock<AppData>>) -> impl IntoResponse {
    (StatusCode::OK, Json(state.read().unwrap().get_users())).into_response()
}

pub async fn get_user_by_id(
    Path(user_id): Path<String>,
    state: Arc<RwLock<AppData>>,
) -> impl IntoResponse {
    let id = Uuid::parse_str(&user_id);

    let id = match id {
        Ok(id) => id,
        Err(_error) => {
            return (StatusCode::BAD_REQUEST, "id not should be of type UUID").into_response();
        }
    };

    let user = state.read().unwrap().get_user_by_id(&id).cloned();

    match user {
        Some(user) => (StatusCode::FOUND, Json(user)).into_response(),
        None => (StatusCode::NOT_FOUND, "user not found").into_response(),
    }
}

pub async fn remove_user_by_id(
    Path(user_id): Path<String>,
    state: Arc<RwLock<AppData>>,
) -> impl IntoResponse {
    let id = Uuid::parse_str(&user_id);

    let id = match id {
        Ok(id) => id,
        Err(_error) => {
            return (StatusCode::BAD_REQUEST, "id should be of type UUID").into_response();
        }
    };

    let user = state.write().unwrap().remove_user_by_id(&id);

    match user {
        Some(user) => (StatusCode::FOUND, Json(user)).into_response(),
        None => (StatusCode::NOT_FOUND, "user not found").into_response(),
    }
}

pub async fn create_user(
    Json(payload): Json<NewUser>,
    state: Arc<RwLock<AppData>>,
) -> impl IntoResponse {
    let new_user = User {
        id: Uuid::new_v4(),
        name: payload.name,
        email: payload.email,
    };

    state.write().unwrap().create_user(new_user);
    (StatusCode::CREATED, "New user created").into_response()
}

// ======================================================================
// The query parameters for todos index
// #[derive(Debug, Deserialize, Default)]
// pub struct Pagination {
//     pub offset: Option<usize>,
//     pub limit: Option<usize>,
// }

// async fn todos_index(
//     pagination: Option<Query<Pagination>>,
//     State(db): State<Db>,
// ) -> impl IntoResponse {
//     let todos = db.read().unwrap();

//     let Query(pagination) = pagination.unwrap_or_default();

//     let todos = todos
//         .values()
//         .skip(pagination.offset.unwrap_or(0))
//         .take(pagination.limit.unwrap_or(usize::MAX))
//         .cloned()
//         .collect::<Vec<_>>();

//     Json(todos)
// }

// ======================================================================
