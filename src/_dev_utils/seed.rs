use crate::{
    ctx::Ctx,
    helpers::PROJECT_COLORS,
    model_bmc::{
        self, ModelManager, Status, StatusBmc, StatusForCreate, Task, TaskBmc, TaskForCreate, User,
        UserBmc, UserForCreate,
    },
};
use fake::faker::lorem::en::*;
use fake::Fake;
use rand::Rng;
use strum_macros::Display;
use tokio::sync::OnceCell;
use tracing::{debug, info};
use uuid::Uuid;

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

pub async fn seed_status(ctx: &Ctx, mm: &ModelManager) -> model_bmc::Result<Vec<Status>> {
    let mut status_list: Vec<Status> = vec![];
    for value in TaskStatus::VALUES {
        let status_c = StatusForCreate { name: value.into() };

        let id = StatusBmc::create(ctx, mm, status_c).await?;
        let status = StatusBmc::get(ctx, mm, id).await?;

        status_list.push(status);
    }

    Ok(status_list)
}

#[derive(Clone, Debug, Display)]
pub enum TaskStatus {
    Done,
    InProgress,
    NotNedeed,
    ReadyToStart,
    Backlog,
}

impl From<&str> for TaskStatus {
    fn from(s: &str) -> TaskStatus {
        match s {
            "Done" => TaskStatus::Done,
            "InProgress" => TaskStatus::InProgress,
            "NotNeeded" => TaskStatus::NotNedeed,
            "ReadyToStart" => TaskStatus::ReadyToStart,
            _ => TaskStatus::Backlog,
        }
    }
}

impl Into<String> for TaskStatus {
    fn into(self) -> String {
        match self {
            TaskStatus::Done => "Done".to_string(),
            TaskStatus::InProgress => "InProgress".to_string(),
            TaskStatus::NotNedeed => "NotNeeded".to_string(),
            TaskStatus::ReadyToStart => "ReadyToStart".to_string(),
            TaskStatus::Backlog => "Backlog".to_string(),
        }
    }
}

impl TaskStatus {
    const VALUES: [Self; 5] = [
        TaskStatus::Done,
        TaskStatus::InProgress,
        TaskStatus::NotNedeed,
        TaskStatus::ReadyToStart,
        TaskStatus::Backlog,
    ];

    fn get_random_task_status() -> TaskStatus {
        // TODO: upddate to `variant_count` when stable so we dont have to
        // hard code the enum length in the `gen_range`, that way we can
        // avoid breaking functionality with the enum changes.
        //
        // https://github.com/rust-lang/rust/issues/73662
        // let enum_length = mem::variant_count::<TaskStatus>();

        match rand::thread_rng().gen_range(0..=3) {
            0 => TaskStatus::Done,
            1 => TaskStatus::InProgress,
            2 => TaskStatus::NotNedeed,
            3 => TaskStatus::ReadyToStart,
            _ => TaskStatus::Backlog,
        }
    }
}
