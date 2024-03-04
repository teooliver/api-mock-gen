mod login_routes;
pub mod mw_auth;
mod task_routes;
mod user_routes;

pub use login_routes::*;
pub use task_routes::*;
pub use user_routes::*;

pub const AUTH_TOKEN: &str = "auth-token";
