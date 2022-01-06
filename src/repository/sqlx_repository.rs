use crate::error::Result;
use crate::models::Task;

use sqlx::PgPool;
use uuid::Uuid;

pub struct SqlxTaskRepository<'a> {
    pub pool: &'a PgPool,
}

impl SqlxTaskRepository<'_> {
    pub async fn create_task(&self, title: String, description: String) -> Result<Task> {
        let mut tx = self.pool.begin().await?;
        let task_id = Uuid::new_v4().to_string();

        sqlx::query!(
            r#"
            INSERT INTO tasks (id, title, descp)
            VALUES ($1, $2, $3) 
            "#,
            task_id,
            title,
            description
        )
        .execute(&mut tx)
        .await?;

        let record = sqlx::query!(
            r#"
            SELECT id, title, descp, completed
            FROM tasks
            WHERE id = $1
            "#,
            task_id,
        )
        .fetch_one(&mut tx)
        .await?;

        tx.commit().await?;

        Ok(Task{
            id: record.id,
            title: record.title,
            descp: record.descp,
            completed: record.completed,
        })
    }

    pub async fn complete_task(&self, id: String) -> Result<Task> {
        let mut tx = self.pool.begin().await?;

        sqlx::query!(
            r#"
            UPDATE tasks
            SET completed = TRUE
            WHERE id = $1
            "#,
            id,
        )
        .execute(&mut tx)
        .await?;

        let record = sqlx::query!(
            r#"
            SELECT id, title, descp, completed
            FROM tasks
            WHERE id = $1
            "#,
            id,
        )
        .fetch_one(&mut tx)
        .await?;

        tx.commit().await?;

        Ok(Task{
            id: record.id,
            title: record.title,
            descp: record.descp,
            completed: record.completed,
        })
    }
}
