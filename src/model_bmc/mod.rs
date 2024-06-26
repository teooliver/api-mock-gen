// `Bmc` is short for Bakend Model Controller
mod board;
mod error;
mod status;
mod store;
mod task;
mod user;

pub use board::*;
pub use status::*;
pub use task::*;
pub use user::*;

pub use self::error::{Error, Result};
use self::store::{new_db_pool, Db};

#[derive(Clone)]
pub struct ModelManager {
    db: Db,
}

impl ModelManager {
    pub async fn new() -> Result<Self> {
        let db = new_db_pool().await?;

        Ok(ModelManager { db })
    }

    /// Returns the sqlx db pool reference.
    /// (Only for the model layer)
    pub(in crate::model_bmc) fn db(&self) -> &Db {
        &self.db
    }
}
