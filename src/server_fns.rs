use crate::model::Todo;
use leptos::prelude::*;

#[server]
pub async fn add_todo(title: String) -> Result<Todo, ServerFnError> {
    use crate::model::queries;

    let pool = leptos_axum::extract::<axum::Extension<sqlx::SqlitePool>>()
        .await
        .map(|ext| ext.0)
        .map_err(|e| ServerFnError::new(format!("Failed to get database pool: {e}")))?;

    let todo = queries::insert(&pool, &title)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to add todo: {e}")))?;

    Ok(todo)
}

#[server]
pub async fn get_todos() -> Result<Vec<Todo>, ServerFnError> {
    use crate::model::queries;

    let pool = leptos_axum::extract::<axum::Extension<sqlx::SqlitePool>>()
        .await
        .map(|ext| ext.0)
        .map_err(|e| ServerFnError::new(format!("Failed to get database pool: {e}")))?;

    let todos = queries::get_all(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to get todos: {e}")))?;

    Ok(todos)
}

#[server]
pub async fn toggle_todo(id: i64) -> Result<(), ServerFnError> {
    use crate::model::queries;

    let pool = leptos_axum::extract::<axum::Extension<sqlx::SqlitePool>>()
        .await
        .map(|ext| ext.0)
        .map_err(|e| ServerFnError::new(format!("Failed to get database pool: {e}")))?;

    queries::toggle(&pool, id)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to toggle todo: {e}")))?;

    Ok(())
}

#[server]
pub async fn delete_todo(id: i64) -> Result<(), ServerFnError> {
    use crate::model::queries;

    let pool = leptos_axum::extract::<axum::Extension<sqlx::SqlitePool>>()
        .await
        .map(|ext| ext.0)
        .map_err(|e| ServerFnError::new(format!("Failed to get database pool: {e}")))?;

    queries::delete(&pool, id)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to delete todo: {e}")))?;

    Ok(())
}

#[server]
pub async fn toggle_all_todos(completed: bool) -> Result<(), ServerFnError> {
    use crate::model::queries;

    let pool = leptos_axum::extract::<axum::Extension<sqlx::SqlitePool>>()
        .await
        .map(|ext| ext.0)
        .map_err(|e| ServerFnError::new(format!("Failed to get database pool: {e}")))?;

    queries::toggle_all(&pool, completed)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to toggle all todos: {e}")))?;

    Ok(())
}

#[server]
pub async fn clear_completed() -> Result<(), ServerFnError> {
    use crate::model::queries;

    let pool = leptos_axum::extract::<axum::Extension<sqlx::SqlitePool>>()
        .await
        .map(|ext| ext.0)
        .map_err(|e| ServerFnError::new(format!("Failed to get database pool: {e}")))?;

    queries::clear_completed(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to clear completed: {e}")))?;

    Ok(())
}
