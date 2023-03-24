mod comment;
mod controllers;
mod db;
mod helpers;
mod post;
mod task;
mod user;

use controllers::{remove_task_by_id, update_task};
use db::AppData;
use std::sync::{Arc, RwLock};

use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use std::net::SocketAddr;

use crate::controllers::{create_task, create_user, get_user_by_id, get_users, remove_user_by_id};
use crate::controllers::{get_all_tasks, get_all_tasks_from_user};
use crate::db::generate_app_data;
use crate::{controllers::get_task_by_id, helpers::generate_json_db};

// use axum_macros::debug_handler;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let db = generate_app_data(100, 5);

    // TODO: Check if mocked_db folder exists, if not create it
    // QUESTION: Should the "collections" be created all in on json file,
    // or should we keep them separate
    // TODO: Serve those json files in routes, just as an example on how to
    // serve files on axun. Could be also usefull as a way of grabing all info all at once
    // in the case we have one json file with all collections
    generate_json_db(&db.tasks, "mocked_db/tasks_json_db.json".to_string());
    generate_json_db(&db.users, "mocked_db/users_json_db.json".to_string());
    generate_json_db(&db.posts, "mocked_db/posts_json_db.json".to_string());
    generate_json_db(&db.comments, "mocked_db/comments_json_db.json".to_string());

    // type Db = Arc<RwLock<AppData>>; ?
    // Explain in my own words why we need Arc and RwLock here
    let shared_state: Arc<RwLock<AppData>> = Arc::new(RwLock::new(db.clone()));

    // TODO: use `merge` and `nest` so we can organize these routes
    // in separate files. Also use `layer` or write our own middleware to
    // pass the db into the routes (so we dont have to move the shared_state
    // into each route one by one)
    let app = Router::new()
        // .route(
        //     "/generate-data",
        //     get({
        //         let shared_state = Arc::clone(&shared_state);
        //         move || generate_data(Arc::clone(&shared_state))
        //     }),
        // )
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
        .route(
            "/tasks",
            get({
                let shared_state = Arc::clone(&shared_state);
                move || get_all_tasks(Arc::clone(&shared_state))
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
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
