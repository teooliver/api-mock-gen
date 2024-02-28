use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
pub type DbResult<T> = core::result::Result<T, Error>;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub enum Error {
    TaksNotFound { id: Uuid },
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");

        // Create a placeholder Axum reponse.
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // Insert the Error into the reponse.
        response.extensions_mut().insert(self);

        response
    }
}
