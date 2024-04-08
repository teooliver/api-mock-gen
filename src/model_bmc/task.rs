use crate::ctx::Ctx;
use crate::model_bmc::ModelManager;
use crate::model_bmc::Result;
use chrono::DateTime;
use chrono::Utc;
use rand::Rng;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use strum_macros::Display;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub assigned_to: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
    pub color: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TaskForCreate {
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub assigned_to: Option<Uuid>,
    pub color: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TaskForUpdate {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<TaskStatus>,
    pub assigned_to: Option<Uuid>,
    pub started_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
    pub color: Option<String>,
}

#[derive(Clone, Display, Debug, Deserialize, Serialize)]
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

impl TaskStatus {
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

pub struct TaskBmc;

impl TaskBmc {
    pub async fn create(_ctx: &Ctx, mm: &ModelManager, task_c: TaskForCreate) -> Result<Uuid> {
        let db = mm.db();
        let (id,) = sqlx::query_as::<_, (Uuid,)>(
            "INSERT INTO task (
            title,
            description,
            status,
            assigned_to,
            color,
            )
            values ($1, $2, $3, $4, $5) return id",
        )
        .bind(task_c.title)
        .bind(task_c.description)
        .bind(task_c.status.to_string())
        .bind(task_c.assigned_to)
        .bind(task_c.color)
        .fetch_one(db)
        .await?;

        Ok(id)
    }
}

#[cfg(test)]
mod tests {
    use crate::_dev_utils;

    use super::*;
    use anyhow::Result;

    #[tokio::test]
    async fn test_create_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();

        let task_title = "Some Title";

        let task_c = TaskForCreate {
            title: task_title.to_string(),
            description: Some("Some Description".to_string()),
            status: TaskStatus::Backlog,
            assigned_to: None,
            color: None,
        };

        let id = TaskBmc::create(&ctx, &mm, task_c).await?;

        let (title,): (String,) = sqlx::query_as("SELECT title from task where id = $1")
            .bind(id)
            .fetch_one(mm.db())
            .await?;

        assert_eq!(title, task_title);

        // -- Clean
        let count = sqlx::query("DELETE FROM task WHERE id = $1")
            .bind(id)
            .execute(mm.db())
            .await?
            .rows_affected();
        assert_eq!(count, 1, "Did not delete 1 row?");

        Ok(())
    }
}
