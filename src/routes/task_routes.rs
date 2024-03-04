use std::sync::{Arc, RwLock};

use axum::{
    middleware,
    routing::{delete, get, patch, post},
    Router,
};

use crate::{
    controllers::{
        create_task, get_all_tasks, get_task_by_id, remove_task_by_id, search_tasks, update_task,
    },
    db::AppData,
};

use super::mw_auth;

pub fn task_routes(shared_state: &Arc<RwLock<AppData>>) -> Router {
    Router::new()
        .route(
            "/tasks",
            get({
                let shared_state = Arc::clone(&shared_state);
                move || get_all_tasks(Arc::clone(&shared_state))
            }),
        )
        .route(
            "/search",
            get({
                let shared_state = Arc::clone(&shared_state);
                move |query| search_tasks(query, Arc::clone(&shared_state))
            }),
        )
        .route(
            "/tasks/:id",
            get({
                let shared_state = Arc::clone(&shared_state);
                move |path| get_task_by_id(path, Arc::clone(&shared_state))
            }),
        )
        .route(
            "/tasks",
            post({
                let shared_state = Arc::clone(&shared_state);
                move |body| create_task(body, Arc::clone(&shared_state))
            }),
        )
        .route(
            "/tasks",
            patch({
                let shared_state = Arc::clone(&shared_state);
                move |path| update_task(path, Arc::clone(&shared_state))
            }),
        )
        .route(
            "/tasks/:id",
            delete({
                let shared_state = Arc::clone(&shared_state);
                move |path| remove_task_by_id(path, Arc::clone(&shared_state))
            }),
        )
}
