use std::sync::{Arc, RwLock};

use axum::{
    routing::{delete, get, post},
    Router,
};

use crate::{
    controllers::{
        create_user, get_all_tasks_from_user, get_user_by_id, get_users, remove_user_by_id,
    },
    db::AppData,
};

pub fn user_routes(shared_state: &Arc<RwLock<AppData>>) -> Router {
    Router::new()
        .route(
            "/users",
            get({
                let shared_state = Arc::clone(&shared_state);
                move || get_users(Arc::clone(&shared_state))
            }),
        )
        .route(
            "/users/:id",
            get({
                let shared_state = Arc::clone(&shared_state);
                move |path| get_user_by_id(path, Arc::clone(&shared_state))
            }),
        )
        .route(
            "/users",
            post({
                let shared_state = Arc::clone(&shared_state);
                move |body| create_user(body, Arc::clone(&shared_state))
            }),
        )
        .route(
            "/users/:id",
            delete({
                let shared_state = Arc::clone(&shared_state);
                move |path| remove_user_by_id(path, Arc::clone(&shared_state))
            }),
        )
        .route(
            "/users/:id/tasks",
            get({
                let shared_state = Arc::clone(&shared_state);
                move |path| get_all_tasks_from_user(path, Arc::clone(&shared_state))
            }),
        )
}
