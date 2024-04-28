use crate::Error;
use crate::{ctx::Ctx, model_bmc::ModelManager};

use super::{seed_boards, seed_status, seed_tasks, seed_users};

pub async fn build_dev_db_state() -> Result<(), Error> {
    let mm = ModelManager::new().await?;
    let ctx = Ctx::root_ctx();

    // seed_users(&ctx, &mm, Some(10)).await?;
    // seed_status(&ctx, &mm).await?;
    // seed_boards(&ctx, &mm, Some(4)).await?;
    // seed_tasks(&ctx, &mm, Some(10)).await?;

    Ok(())
}
