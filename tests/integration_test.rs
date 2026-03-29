#![cfg(feature = "ssr")]

use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use todomvc_leptos::model::queries;

async fn setup_pool() -> SqlitePool {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .expect("Failed to create in-memory pool");
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");
    pool
}

#[tokio::test]
async fn test_insert_returns_todo() {
    let pool = setup_pool().await;
    let todo = queries::insert(&pool, "Buy milk").await.expect("insert failed");
    assert_eq!(todo.title, "Buy milk");
    assert!(!todo.completed);
    assert_eq!(todo.sort_order, 1);
}

#[tokio::test]
async fn test_insert_increments_sort_order() {
    let pool = setup_pool().await;
    let t1 = queries::insert(&pool, "First").await.expect("insert failed");
    let t2 = queries::insert(&pool, "Second").await.expect("insert failed");
    assert_eq!(t1.sort_order, 1);
    assert_eq!(t2.sort_order, 2);
}

#[tokio::test]
async fn test_insert_empty_title() {
    let pool = setup_pool().await;
    let todo = queries::insert(&pool, "").await.expect("insert failed");
    assert_eq!(todo.title, "");
}

#[tokio::test]
async fn test_get_all_empty() {
    let pool = setup_pool().await;
    let todos = queries::get_all(&pool).await.expect("get_all failed");
    assert!(todos.is_empty());
}

#[tokio::test]
async fn test_get_all_returns_sorted() {
    let pool = setup_pool().await;
    queries::insert(&pool, "First").await.expect("insert failed");
    queries::insert(&pool, "Second").await.expect("insert failed");
    queries::insert(&pool, "Third").await.expect("insert failed");

    let todos = queries::get_all(&pool).await.expect("get_all failed");
    assert_eq!(todos.len(), 3);
    assert_eq!(todos[0].title, "First");
    assert_eq!(todos[1].title, "Second");
    assert_eq!(todos[2].title, "Third");
}

#[tokio::test]
async fn test_toggle_flips_completed() {
    let pool = setup_pool().await;
    let todo = queries::insert(&pool, "Toggle me").await.expect("insert failed");
    assert!(!todo.completed);

    let toggled = queries::toggle(&pool, todo.id).await.expect("toggle failed");
    assert!(toggled);

    let todos = queries::get_all(&pool).await.expect("get_all failed");
    assert!(todos[0].completed);

    queries::toggle(&pool, todo.id).await.expect("toggle failed");
    let todos = queries::get_all(&pool).await.expect("get_all failed");
    assert!(!todos[0].completed);
}

#[tokio::test]
async fn test_toggle_nonexistent_id() {
    let pool = setup_pool().await;
    let result = queries::toggle(&pool, 9999).await.expect("toggle failed");
    assert!(!result);
}

#[tokio::test]
async fn test_update_title_and_completed() {
    let pool = setup_pool().await;
    let todo = queries::insert(&pool, "Original").await.expect("insert failed");

    let updated = queries::update(&pool, todo.id, "Updated", true)
        .await
        .expect("update failed");
    assert!(updated);

    let todos = queries::get_all(&pool).await.expect("get_all failed");
    assert_eq!(todos[0].title, "Updated");
    assert!(todos[0].completed);
}

#[tokio::test]
async fn test_update_nonexistent_id() {
    let pool = setup_pool().await;
    let result = queries::update(&pool, 9999, "Nope", false)
        .await
        .expect("update failed");
    assert!(!result);
}

#[tokio::test]
async fn test_update_title_only() {
    let pool = setup_pool().await;
    let todo = queries::insert(&pool, "Original").await.expect("insert failed");

    let updated = queries::update_title(&pool, todo.id, "Renamed")
        .await
        .expect("update_title failed");
    assert!(updated);

    let todos = queries::get_all(&pool).await.expect("get_all failed");
    assert_eq!(todos[0].title, "Renamed");
    assert!(!todos[0].completed);
}

#[tokio::test]
async fn test_update_title_nonexistent_id() {
    let pool = setup_pool().await;
    let result = queries::update_title(&pool, 9999, "Nope")
        .await
        .expect("update_title failed");
    assert!(!result);
}

#[tokio::test]
async fn test_delete() {
    let pool = setup_pool().await;
    let todo = queries::insert(&pool, "Delete me").await.expect("insert failed");

    let deleted = queries::delete(&pool, todo.id).await.expect("delete failed");
    assert!(deleted);

    let todos = queries::get_all(&pool).await.expect("get_all failed");
    assert!(todos.is_empty());
}

#[tokio::test]
async fn test_delete_nonexistent_id() {
    let pool = setup_pool().await;
    let result = queries::delete(&pool, 9999).await.expect("delete failed");
    assert!(!result);
}

#[tokio::test]
async fn test_toggle_all() {
    let pool = setup_pool().await;
    queries::insert(&pool, "A").await.expect("insert failed");
    queries::insert(&pool, "B").await.expect("insert failed");
    queries::insert(&pool, "C").await.expect("insert failed");

    queries::toggle_all(&pool, true).await.expect("toggle_all failed");
    let todos = queries::get_all(&pool).await.expect("get_all failed");
    assert!(todos.iter().all(|t| t.completed));

    queries::toggle_all(&pool, false).await.expect("toggle_all failed");
    let todos = queries::get_all(&pool).await.expect("get_all failed");
    assert!(todos.iter().all(|t| !t.completed));
}

#[tokio::test]
async fn test_toggle_all_empty_table() {
    let pool = setup_pool().await;
    queries::toggle_all(&pool, true).await.expect("toggle_all on empty should succeed");
}

#[tokio::test]
async fn test_clear_completed() {
    let pool = setup_pool().await;
    let t1 = queries::insert(&pool, "Keep").await.expect("insert failed");
    queries::insert(&pool, "Remove").await.expect("insert failed");
    queries::insert(&pool, "Also remove").await.expect("insert failed");

    queries::toggle(&pool, t1.id).await.expect("toggle failed");
    queries::toggle(&pool, t1.id).await.expect("toggle failed");
    // t1 is active, mark t2 and t3 completed
    queries::toggle_all(&pool, true).await.expect("toggle_all failed");
    queries::update(&pool, t1.id, "Keep", false).await.expect("update failed");

    let cleared = queries::clear_completed(&pool).await.expect("clear_completed failed");
    assert_eq!(cleared, 2);

    let todos = queries::get_all(&pool).await.expect("get_all failed");
    assert_eq!(todos.len(), 1);
    assert_eq!(todos[0].title, "Keep");
}

#[tokio::test]
async fn test_clear_completed_none_completed() {
    let pool = setup_pool().await;
    queries::insert(&pool, "Active").await.expect("insert failed");
    let cleared = queries::clear_completed(&pool).await.expect("clear_completed failed");
    assert_eq!(cleared, 0);

    let todos = queries::get_all(&pool).await.expect("get_all failed");
    assert_eq!(todos.len(), 1);
}
