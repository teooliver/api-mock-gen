use crate::ctx::Ctx;
use crate::model_bmc::ModelManager;
use crate::model_bmc::{Error, Result};
use chrono::DateTime;
use chrono::Utc;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serial_test::serial;
use sqlx::FromRow;
use strum_macros::Display;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, sqlx::FromRow)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    #[deprecated(note = "use status_id instead")]
    pub status: Option<String>,
    pub status_id: Option<Uuid>,
    pub color: Option<String>,
    pub user_id: Option<Uuid>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TaskForCreate {
    pub title: String,
    pub description: Option<String>,
    pub status: Option<String>,
    pub color: Option<String>,
    pub user_id: Option<Uuid>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TaskForUpdate {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub color: Option<String>,
    pub user_id: Option<Uuid>,
}

pub struct TaskBmc;

impl TaskBmc {
    pub async fn create(_ctx: &Ctx, mm: &ModelManager, task_c: TaskForCreate) -> Result<Uuid> {
        let db = mm.db();
        let (id,) = sqlx::query_as::<_, (Uuid,)>(
            "INSERT INTO task (
            title,
            description,
            color,
            user_id
            )
            values ($1, $2,$3, $4) RETURNING id",
        )
        .bind(task_c.title)
        .bind(task_c.description)
        .bind(task_c.color)
        .bind(task_c.user_id)
        .fetch_one(db)
        .await?;

        Ok(id)
    }
    // 1.07:46
    pub async fn get(_ctx: &Ctx, mm: &ModelManager, id: Uuid) -> Result<Task> {
        let db = mm.db();

        let task: Task = sqlx::query_as("SELECT * FROM task WHERE id = $1")
            .bind(id)
            .fetch_optional(db)
            .await?
            .ok_or(Error::EntityNotFound { entity: "task", id })?;

        Ok(task)
    }

    pub async fn list(_ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Task>> {
        let db = mm.db();

        let tasks: Vec<Task> = sqlx::query_as("SELECT * FROM task ORDER by title LIMIT 30")
            .fetch_all(db)
            .await?;

        Ok(tasks)
    }

    pub async fn update(
        _ctx: &Ctx,
        mm: &ModelManager,
        id: Uuid,
        task: TaskForUpdate,
    ) -> Result<()> {
        let db = mm.db();

        let rows_affected = sqlx::query!(
            r#"UPDATE task
            SET title = $1, description = $2
            WHERE id = $3"#,
            task.title,
            task.description,
            id
        )
        .execute(db)
        .await?
        .rows_affected();

        if rows_affected == 0 {
            return Err(Error::EntityNotFound { entity: "task", id });
        }

        Ok(())
    }

    pub async fn delete(_ctx: &Ctx, mm: &ModelManager, id: Uuid) -> Result<()> {
        let db = mm.db();

        let count = sqlx::query("DELETE FROM task WHERE id = $1")
            .bind(id)
            .execute(db)
            .await?
            .rows_affected();

        if count == 0 {
            return Err(Error::EntityNotFound { entity: "task", id });
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::_dev_utils::{self, new_random_task};

    use super::*;
    use anyhow::Result;

    // TODO: Check why Serial is breaking the tests
    // #[serial]
    #[tokio::test]
    async fn test_task_create_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();

        let task_title = "Some Title 2";

        let task_c = TaskForCreate {
            title: task_title.to_string(),
            description: Some("Some Description 2".to_string()),
            status: None,
            color: None,
            user_id: None,
        };

        let id = TaskBmc::create(&ctx, &mm, task_c).await?;

        let (title,): (String,) = sqlx::query_as("SELECT title FROM task WHERE id = $1")
            .bind(id)
            .fetch_one(mm.db())
            .await?;

        assert_eq!(title, task_title);

        let task = TaskBmc::get(&ctx, &mm, id).await?;
        assert_eq!(task.title, task_title);

        // -- Clean
        let count = sqlx::query("DELETE FROM task WHERE id = $1")
            .bind(id)
            .execute(mm.db())
            .await?
            .rows_affected();

        assert_eq!(count, 1, "Did not delete 1 row?");

        Ok(())
    }

    // #[serial]
    #[tokio::test]
    async fn test_task_get_err_not_found() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let id: Uuid = Uuid::try_parse("2be8791f-f9b9-48bc-85e3-818183c6deac").unwrap();
        let res = TaskBmc::get(&ctx, &mm, id).await;

        assert!(
            matches!(res, Err(Error::EntityNotFound { entity: "task", id })),
            "EntityNotFound not matching"
        );

        Ok(())
    }

    // #[serial]
    #[tokio::test]
    async fn test_task_list_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        // _dev_utils::seed_tasks(&ctx, &mm, Some(20)).await?;

        let tasks = TaskBmc::list(&ctx, &mm).await?;

        assert!(tasks.len() >= 1);

        Ok(())
    }

    // #[serial]
    #[tokio::test]
    async fn test__task_update_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();

        let fx_title = "test_update_ok - task 01".to_string();
        let fx_title_new = "test_update_ok - task 01 - new".to_string();

        let random_task = new_random_task(Some(fx_title));
        let task_updated = TaskForUpdate {
            title: Some(fx_title_new.clone()),
            description: random_task.description.clone(),
            color: None,
            status: None,
            user_id: None,
        };

        let id = TaskBmc::create(&ctx, &mm, random_task).await?;

        TaskBmc::update(&ctx, &mm, id, task_updated).await?;

        let (title,): (String,) = sqlx::query_as("SELECT title FROM task WHERE id = $1")
            .bind(id)
            .fetch_one(mm.db())
            .await?;

        assert_eq!(title, fx_title_new);

        Ok(())
    }

    // #[serial]
    #[tokio::test]
    async fn test_task_delete_err_not_found() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let id: Uuid = Uuid::try_parse("26af6714-7734-4ebf-9474-23af4f481688").unwrap();

        let res = TaskBmc::delete(&ctx, &mm, id).await;

        assert!(
            matches!(res, Err(Error::EntityNotFound { entity: "task", id })),
            "EntityNotFound not matching"
        );

        Ok(())
    }
}
