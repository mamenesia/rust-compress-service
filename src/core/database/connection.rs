use sqlx::{PgPool, Pool, Postgres};
use tracing::info;

pub type DbPool = Pool<Postgres>;

pub async fn create_pool(database_url: &str) -> Result<DbPool, sqlx::Error> {
    info!("Creating database connection pool");
    PgPool::connect(database_url).await
}

pub async fn init_database(pool: &DbPool) -> Result<(), sqlx::Error> {
    info!("Initializing database schema");
    
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS compressed_items (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            name TEXT NOT NULL,
            data TEXT NOT NULL,
            created_at TIMESTAMPTZ DEFAULT NOW(),
            updated_at TIMESTAMPTZ DEFAULT NOW()
        )
        "#,
    )
    .execute(pool)
    .await?;

    info!("Database schema initialized successfully");
    Ok(())
}
