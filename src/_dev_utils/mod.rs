use crate::{
    ctx::Ctx,
    helpers::PROJECT_COLORS,
    model_bmc::{TaskBmc, User, UserBmc, UserForCreate},
};
use fake::faker::lorem::en::*;
use fake::Fake;
use rand::Rng;
use tokio::sync::OnceCell;
use tracing::{debug, info};
use uuid::Uuid;

use crate::model_bmc::{self, ModelManager, Task, TaskForCreate};

mod build_db_state;
mod dev_db;
mod seed;
pub use build_db_state::*;
pub use seed::*;

pub async fn init_dev() {
    static INIT: OnceCell<()> = OnceCell::const_new();

    INIT.get_or_init(|| async {
        info!("{:<12} - init_dev_all()", "FOR-DEV-ONLY");
        dev_db::init_dev_db().await.unwrap();
    })
    .await;
}

/// Initialize test environment.
pub async fn init_test() -> ModelManager {
    static INIT: OnceCell<ModelManager> = OnceCell::const_new();
    let mm = INIT
        .get_or_init(|| async {
            init_dev().await;
            ModelManager::new().await.unwrap()
        })
        .await;

    mm.clone()
}

pub fn dangerously_drop_tables() {
    todo!()
}

pub fn clean_slate_dev_db() {
    todo!()
}
