mod config;
mod controllers;
mod ctx;
mod db;
mod error;
mod helpers;
mod model_bmc;
mod models;
mod routes;
pub use self::error::Error;
use sqlx::types::Uuid;

pub mod _dev_utils;

use axum::response::{IntoResponse, Response};
use axum::routing::get_service;
use axum::{middleware, Json};
use db::AppData;
use serde_json::json;
use std::sync::{Arc, RwLock};
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use tracing::{debug, info};
use tracing_subscriber::fmt::format::json;
use tracing_subscriber::EnvFilter;

use axum::{routing::get, Router};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

use crate::helpers::generate_json_file;
use crate::model_bmc::ModelManager;
use crate::routes::mw_auth::mw_ctx_resolver;
use crate::routes::user_routes;
use crate::routes::{login_routes, mw_auth};
use crate::{
    controllers::{generate_mock_data, health_check},
    routes::task_routes,
};

pub use config::Config;
// use axum_macros::debug_handler;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // -- FOR DEV ONLY
    _dev_utils::init_dev().await;

    let mm = ModelManager::new().await?;

    // type Db = Arc<RwLock<AppData>>; ?
    // Explain in my own words why we need Arc and RwLock here
    let in_memory_db = AppData::generate_app_data(100, 5);
    // generate_json_files(&in_memory_db);
    let shared_state: Arc<RwLock<AppData>> = Arc::new(RwLock::new(in_memory_db.clone()));

    let cors = CorsLayer::new().allow_origin(Any);

    // Routes behind login
    let protected_routes = Router::new()
        .merge(user_routes(&shared_state))
        .merge(task_routes(&shared_state))
        .route_layer(middleware::from_fn(mw_auth::mw_require_auth));

    let api_routes = Router::new()
        .route("/health_check", get(move || health_check()))
        .route(
            "/regenerate_db",
            get({
                let shared_state = Arc::clone(&shared_state);
                move || generate_mock_data(Arc::clone(&shared_state))
            }),
        )
        .merge(login_routes())
        .merge(protected_routes)
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn(mw_ctx_resolver))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static())
        .layer(cors);

    let app = Router::new().nest("/api/v1", api_routes);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    info!("listening on {}", addr);
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

async fn main_response_mapper(res: Response) -> Response {
    let uuid = Uuid::new_v4();

    let service_error = res.extensions().get::<error::Error>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());

    let error_reponse = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "error":{
                    "type": client_error.as_ref(),
                    "req_uuid": uuid.to_string(),
                }
            });
            println!("    ->> client_error_body: {client_error_body}");
            (*status_code, Json(client_error_body)).into_response()
        });
    debug!("server log line -  {uuid} - Error: {service_error:?}");
    error_reponse.unwrap_or(res)
}
