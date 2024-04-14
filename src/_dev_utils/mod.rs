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
        user_id: Uuid::try_parse("0199bccd-c585-41fc-875d-6af430c270eb").ok(),
    }
}

pub async fn seed_users(
    ctx: &Ctx,
    mm: &ModelManager,
    amount: Option<i8>,
) -> model_bmc::Result<Vec<User>> {
    let amount_of_users = match amount {
        Some(amount) => amount,
        None => 20,
    };

    let mut users: Vec<User> = vec![];
    for _n in 1..=amount_of_users {
        let random_user = new_random_user(None);
        let id = UserBmc::create(ctx, mm, random_user).await?;
        let user = UserBmc::get(ctx, mm, id).await?;

        users.push(user);
    }

    Ok(users)
}

pub fn new_random_user(email: Option<String>) -> UserForCreate {
    let first_name = fake::faker::name::en::FirstName().fake::<String>();
    let last_name = fake::faker::name::en::LastName().fake::<String>();

    let email_provider = fake::faker::internet::en::FreeEmailProvider().fake::<String>();
    let email = match email {
        Some(email) => email,
        None => format!("{}@{}", first_name.to_lowercase(), email_provider),
    };

    UserForCreate {
        email,
        first_name: Some(first_name),
        last_name: Some(last_name),
    }
}

pub fn dangerously_drop_tables() {
    todo!()
}

pub fn clean_slate_dev_db() {
    todo!()
}
