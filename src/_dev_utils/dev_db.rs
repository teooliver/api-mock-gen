use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::time::Duration;

type Db = Pool<Postgres>;

const PG_DEV_POSTGRES_URL: &str = "postgres://db_user:12345@localhost:5432/mock-gen-db";

pub async fn init_dev_db() -> Result<(), Box<dyn std::error::Error>> {
    {
        let root_db = new_db_pool(PG_DEV_POSTGRES_URL).await?;
        sqlx::migrate!().run(&root_db).await?;
    }

    Ok(())
}

async fn new_db_pool(db_con_url: &str) -> Result<Db, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(Duration::from_millis(500))
        .connect(db_con_url)
        .await
}
