use crate::{ctx::Ctx, helpers::PROJECT_COLORS, model_bmc::TaskBmc};
use fake::faker::lorem::en::*;
use fake::Fake;
use rand::Rng;
use tokio::sync::OnceCell;
use tracing::info;

use crate::model_bmc::{self, ModelManager, Task, TaskForCreate};

mod dev_db;

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

pub async fn seed_tasks(
    ctx: &Ctx,
    mm: &ModelManager,
    amount: Option<i8>,
) -> model_bmc::Result<Vec<Task>> {
    let amount_of_tasks = match amount {
        Some(amount) => amount,
        None => 20,
    };

    let mut tasks: Vec<Task> = vec![];
    for _n in 1..=amount_of_tasks {
        let random_task = new_random_task(None);
        let id = TaskBmc::create(ctx, mm, random_task).await?;
        let task = TaskBmc::get(ctx, mm, id).await?;

        tasks.push(task);
    }

    Ok(tasks)
}

pub fn new_random_task(title: Option<String>) -> TaskForCreate {
    let color = rand::thread_rng()
        .gen_range(0..(PROJECT_COLORS.len() - 1))
        .to_string();

    let title = match title {
        Some(title) => title,
        None => Words(3..5).fake::<Vec<String>>().join(" "),
    };

    TaskForCreate {
        title,
        description: Some(Words(3..10).fake::<Vec<String>>().join(" ")),
        status: None,
        color: Some(color),
    }
}

pub fn dangerously_drop_table() {
    todo!()
}
