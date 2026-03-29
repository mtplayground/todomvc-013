use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub completed: bool,
    pub sort_order: i64,
}

#[cfg(feature = "ssr")]
pub mod queries {
    use super::Todo;
    use sqlx::SqlitePool;

    pub async fn insert(pool: &SqlitePool, title: &str) -> Result<Todo, sqlx::Error> {
        let max_order: Option<i64> =
            sqlx::query_scalar("SELECT MAX(sort_order) FROM todos")
                .fetch_one(pool)
                .await?;
        let next_order = max_order.unwrap_or(0) + 1;

        let id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO todos (title, completed, sort_order) VALUES (?, 0, ?) RETURNING id",
        )
        .bind(title)
        .bind(next_order)
        .fetch_one(pool)
        .await?;

        Ok(Todo {
            id,
            title: title.to_string(),
            completed: false,
            sort_order: next_order,
        })
    }

    pub async fn get_all(pool: &SqlitePool) -> Result<Vec<Todo>, sqlx::Error> {
        sqlx::query_as::<_, Todo>("SELECT id, title, completed, sort_order FROM todos ORDER BY sort_order ASC")
            .fetch_all(pool)
            .await
    }

    pub async fn update(
        pool: &SqlitePool,
        id: i64,
        title: &str,
        completed: bool,
    ) -> Result<bool, sqlx::Error> {
        let rows = sqlx::query("UPDATE todos SET title = ?, completed = ? WHERE id = ?")
            .bind(title)
            .bind(completed)
            .bind(id)
            .execute(pool)
            .await?
            .rows_affected();
        Ok(rows > 0)
    }

    pub async fn toggle(pool: &SqlitePool, id: i64) -> Result<bool, sqlx::Error> {
        let rows =
            sqlx::query("UPDATE todos SET completed = NOT completed WHERE id = ?")
                .bind(id)
                .execute(pool)
                .await?
                .rows_affected();
        Ok(rows > 0)
    }

    pub async fn delete(pool: &SqlitePool, id: i64) -> Result<bool, sqlx::Error> {
        let rows = sqlx::query("DELETE FROM todos WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?
            .rows_affected();
        Ok(rows > 0)
    }

    pub async fn toggle_all(pool: &SqlitePool, completed: bool) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE todos SET completed = ?")
            .bind(completed)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn clear_completed(pool: &SqlitePool) -> Result<u64, sqlx::Error> {
        let rows = sqlx::query("DELETE FROM todos WHERE completed = 1")
            .execute(pool)
            .await?
            .rows_affected();
        Ok(rows)
    }
}
