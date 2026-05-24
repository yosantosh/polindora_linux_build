use crate::task_store::{active_tasks, complete_task, create_task, NewTask, TaskRecord};
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use std::{path::Path, str::FromStr};

pub struct AppDatabase {
    pool: SqlitePool,
}

impl AppDatabase {
    pub async fn open(path: impl AsRef<Path>) -> Result<Self, sqlx::Error> {
        let url = format!("sqlite://{}", path.as_ref().display());
        let options = SqliteConnectOptions::from_str(&url)?.create_if_missing(true);
        let pool = SqlitePool::connect_with(options).await?;

        sqlx::raw_sql(include_str!("../migrations/0001_initial.sql"))
            .execute(&pool)
            .await?;

        Ok(Self { pool })
    }

    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }

    pub async fn create_task(&self, task: NewTask) -> Result<(), sqlx::Error> {
        create_task(&self.pool, task).await
    }

    pub async fn active_tasks(&self) -> Result<Vec<TaskRecord>, sqlx::Error> {
        active_tasks(&self.pool).await
    }

    pub async fn complete_task(&self, id: &str) -> Result<bool, sqlx::Error> {
        complete_task(&self.pool, id).await
    }
}
