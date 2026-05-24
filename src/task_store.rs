use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskPriority {
    Low,
    Medium,
    High,
}

impl TaskPriority {
    fn as_db_value(self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    Active,
    Completed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct TaskRecord {
    pub id: String,
    pub title: String,
    pub category: String,
    pub priority: String,
    pub status: String,
    pub due_date: Option<String>,
    pub created_at: String,
    pub completed_at: Option<String>,
    pub pomodoros_spent: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewTask {
    pub id: String,
    pub title: String,
    pub category: String,
    pub priority: TaskPriority,
    pub due_date: Option<String>,
}

pub async fn create_task(pool: &SqlitePool, task: NewTask) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO tasks (
            id, title, category, priority, status, due_date, created_at, completed_at, pomodoros_spent
        )
        VALUES (?1, ?2, ?3, ?4, 'active', ?5, datetime('now'), NULL, 0)
        "#,
    )
    .bind(task.id)
    .bind(task.title)
    .bind(task.category)
    .bind(task.priority.as_db_value())
    .bind(task.due_date)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn active_tasks(pool: &SqlitePool) -> Result<Vec<TaskRecord>, sqlx::Error> {
    sqlx::query_as::<_, TaskRecord>(
        r#"
        SELECT id, title, category, priority, status, due_date, created_at, completed_at, pomodoros_spent
        FROM tasks
        WHERE status = 'active'
        ORDER BY
            CASE priority
                WHEN 'high' THEN 0
                WHEN 'medium' THEN 1
                ELSE 2
            END,
            created_at DESC
        "#,
    )
    .fetch_all(pool)
    .await
}

pub async fn complete_task(pool: &SqlitePool, id: &str) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        r#"
        UPDATE tasks
        SET status = 'completed', completed_at = datetime('now')
        WHERE id = ?1 AND status = 'active'
        "#,
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() == 1)
}
