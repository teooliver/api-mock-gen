use std::sync::{Arc, RwLock};

use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use chrono::Utc;
use uuid::Uuid;

use crate::{
    db::AppData,
    models::{NewTask, Task, TaskStatus},
};

pub async fn get_all_tasks(state: Arc<RwLock<AppData>>) -> impl IntoResponse {
    (StatusCode::OK, Json(state.read().unwrap().get_tasks())).into_response()
}

pub async fn get_task_by_id(
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

    let task = state.read().unwrap().get_tasks_by_id(&id).cloned();

    match task {
        Some(task) => (StatusCode::FOUND, Json(task)).into_response(),
        None => (StatusCode::NOT_FOUND, "user not found").into_response(),
    }
}

pub async fn remove_task_by_id(
    Path(task_id): Path<String>,
    state: Arc<RwLock<AppData>>,
) -> impl IntoResponse {
    let id = Uuid::parse_str(&task_id);

    let id = match id {
        Ok(id) => id,
        Err(_error) => {
            return (StatusCode::BAD_REQUEST, "id should be of type UUID").into_response();
        }
    };

    let task = state.write().unwrap().remove_task_by_id(&id);

    match task {
        Some(task) => (StatusCode::FOUND, Json(task)).into_response(),
        None => (StatusCode::NOT_FOUND, "user not found").into_response(),
    }
}

pub async fn get_all_tasks_from_user(
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

    let tasks = state.read().unwrap().get_all_user_tasks(&id);

    match tasks {
        Some(tasks) => (StatusCode::FOUND, Json(tasks)).into_response(),
        None => (StatusCode::NOT_FOUND, "user not found").into_response(),
    }
}

pub async fn create_task(
    Json(payload): Json<NewTask>,
    state: Arc<RwLock<AppData>>,
) -> impl IntoResponse {
    let status: TaskStatus = match payload.status {
        Some(status) => TaskStatus::from(status.as_str()),
        None => TaskStatus::Backlog,
    };

    let color = payload.color.unwrap_or_default();

    let new_task = Task {
        id: Uuid::new_v4(),
        title: payload.title,
        description: payload.description,
        user_ref: payload.user_ref,
        color: Some(color),
        status,
        started_at: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        finished_at: None,
    };

    state.write().unwrap().create_task(new_task);
    (StatusCode::CREATED, "New task created").into_response()
}

pub async fn update_task(
    Json(payload): Json<Task>,
    state: Arc<RwLock<AppData>>,
) -> impl IntoResponse {
    state.write().unwrap().update_task(payload);
    (StatusCode::CREATED, "Task updated").into_response()
}
