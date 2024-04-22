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
pub struct Status {
    pub id: Uuid,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct StatusForCreate {
    pub name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct StatusForUpdate {
    pub name: Option<String>,
}

pub struct StatusBmc;

impl StatusBmc {
    pub async fn create(_ctx: &Ctx, mm: &ModelManager, sprint_c: StatusForCreate) -> Result<Uuid> {
        let db = mm.db();
        let (id,) = sqlx::query_as::<_, (Uuid,)>(
            "INSERT INTO status (
            name
            )
            values ($1) RETURNING id",
        )
        .bind(sprint_c.name)
        .fetch_one(db)
        .await?;

        Ok(id)
    }
    // 1.07:46
    pub async fn get(_ctx: &Ctx, mm: &ModelManager, id: Uuid) -> Result<Status> {
        let db = mm.db();

        let task: Status = sqlx::query_as("SELECT * FROM status WHERE id = $1")
            .bind(id)
            .fetch_optional(db)
            .await?
            .ok_or(Error::EntityNotFound {
                entity: "sprint",
                id,
            })?;

        Ok(task)
    }

    pub async fn list(_ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Status>> {
        let db = mm.db();

        let tasks: Vec<Status> = sqlx::query_as("SELECT * FROM status ORDER by title LIMIT 10")
            .fetch_all(db)
            .await?;

        Ok(tasks)
    }

    pub async fn update(
        _ctx: &Ctx,
        mm: &ModelManager,
        id: Uuid,
        status: StatusForUpdate,
    ) -> Result<()> {
        let db = mm.db();

        let rows_affected = sqlx::query!(
            r#"UPDATE status
            SET name = $1
            WHERE id = $2"#,
            status.name,
            id
        )
        .execute(db)
        .await?
        .rows_affected();

        if rows_affected == 0 {
            return Err(Error::EntityNotFound {
                entity: "status",
                id,
            });
        }

        Ok(())
    }

    pub async fn delete(_ctx: &Ctx, mm: &ModelManager, id: Uuid) -> Result<()> {
        let db = mm.db();

        let count = sqlx::query("DELETE FROM status WHERE id = $1")
            .bind(id)
            .execute(db)
            .await?
            .rows_affected();

        if count == 0 {
            return Err(Error::EntityNotFound {
                entity: "status",
                id,
            });
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
    #[tokio::test]
    async fn test_status_create_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();

        let status_name = "TODO_STATUS";

        let status_c = StatusForCreate {
            name: status_name.to_string(),
        };

        let id = StatusBmc::create(&ctx, &mm, status_c).await?;

        let (name,): (String,) = sqlx::query_as("SELECT name FROM status WHERE id = $1")
            .bind(id)
            .fetch_one(mm.db())
            .await?;

        assert_eq!(name, status_name);

        let status = StatusBmc::get(&ctx, &mm, id).await?;
        assert_eq!(status.name, status_name);

        // -- Clean
        let count = sqlx::query("DELETE FROM status WHERE id = $1")
            .bind(id)
            .execute(mm.db())
            .await?
            .rows_affected();

        assert_eq!(count, 1, "Did not delete 1 row?");

        Ok(())
    }
}
