mod controllers;
mod db;
mod helpers;
mod models;
mod routes;

use axum::routing::get_service;
use db::AppData;
use std::sync::{Arc, RwLock};
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::EnvFilter;

use axum::{routing::get, Router};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

use crate::helpers::generate_json_file;
use crate::routes::user_routes;
use crate::{
    controllers::{generate_mock_data, health_check},
    routes::task_routes,
};

// use axum_macros::debug_handler;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let in_memory_db = AppData::generate_app_data(100, 5);

    // TODO: Check if mocked_db folder exists, if not create it
    // QUESTION: Should the "collections" be created all in on json file,
    // or should we keep them separate
    // TODO: Serve those json files in routes, just as an example on how to
    // serve files on axum. Could be also usefull as a way of grabing all info all at once
    // in the case we have one json file with all collections
    generate_json_file(
        &in_memory_db.tasks,
        "mocked_db/tasks_json_db.json".to_string(),
    );
    generate_json_file(
        &in_memory_db.users,
        "mocked_db/users_json_db.json".to_string(),
    );
    generate_json_file(
        &in_memory_db.posts,
        "mocked_db/posts_json_db.json".to_string(),
    );
    generate_json_file(
        &in_memory_db.comments,
        "mocked_db/comments_json_db.json".to_string(),
    );

    // type Db = Arc<RwLock<AppData>>; ?
    // Explain in my own words why we need Arc and RwLock here
    let shared_state: Arc<RwLock<AppData>> = Arc::new(RwLock::new(in_memory_db.clone()));

    let cors = CorsLayer::new().allow_origin(Any);

    // TODO: use `merge` and `nest` so we can organize these routes
    // in separate files. Also use `layer` or write our own middleware to
    // pass the db into the routes (so we dont have to move the shared_state
    // into each route one by one)
    let app = Router::new()
        .route("/health_check", get(move || health_check()))
        .route(
            "/regenerate_db",
            get({
                let shared_state = Arc::clone(&shared_state);
                move || generate_mock_data(Arc::clone(&shared_state))
            }),
        )
        .merge(user_routes(&shared_state))
        .merge(task_routes(&shared_state))
        .fallback_service(routes_static())
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    info!("listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}
