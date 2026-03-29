#[cfg(feature = "ssr")]
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

#[cfg(feature = "ssr")]
pub async fn init_pool() -> Result<SqlitePool, sqlx::Error> {
    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:todos.db?mode=rwc".to_string());

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    sqlx::migrate!().run(&pool).await?;

    Ok(pool)
}
