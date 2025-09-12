use crate::core::models::{CompressedItem, CreateCompressedItem, UpdateCompressedItem};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("Item not found")]
    NotFound,
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("UUID parse error: {0}")]
    UuidParse(#[from] uuid::Error),
}

pub async fn get_all_items(pool: &PgPool) -> Result<Vec<CompressedItem>, DbError> {
    let items = sqlx::query_as!(
        CompressedItem,
        "SELECT id, name, data, created_at, updated_at FROM compressed_items ORDER BY created_at DESC"
    )
    .fetch_all(pool)
    .await?;

    Ok(items)
}

pub async fn get_item_by_id(pool: &PgPool, id: &str) -> Result<CompressedItem, DbError> {
    let uuid = Uuid::parse_str(id)?;
    
    let item = sqlx::query_as!(
        CompressedItem,
        "SELECT id, name, data, created_at, updated_at FROM compressed_items WHERE id = $1",
        uuid
    )
    .fetch_optional(pool)
    .await?
    .ok_or(DbError::NotFound)?;

    Ok(item)
}

pub async fn create_item(pool: &PgPool, item: CreateCompressedItem) -> Result<CompressedItem, DbError> {
    let created_item = sqlx::query_as!(
        CompressedItem,
        r#"
        INSERT INTO compressed_items (name, data)
        VALUES ($1, $2)
        RETURNING id, name, data, created_at, updated_at
        "#,
        item.name,
        item.data
    )
    .fetch_one(pool)
    .await?;

    Ok(created_item)
}

pub async fn update_item(
    pool: &PgPool,
    id: &str,
    item: UpdateCompressedItem,
) -> Result<CompressedItem, DbError> {
    let uuid = Uuid::parse_str(id)?;
    
    let updated_item = sqlx::query_as!(
        CompressedItem,
        r#"
        UPDATE compressed_items 
        SET name = COALESCE($2, name),
            data = COALESCE($3, data),
            updated_at = NOW()
        WHERE id = $1
        RETURNING id, name, data, created_at, updated_at
        "#,
        uuid,
        item.name,
        item.data
    )
    .fetch_optional(pool)
    .await?
    .ok_or(DbError::NotFound)?;

    Ok(updated_item)
}

pub async fn delete_item(pool: &PgPool, id: &str) -> Result<(), DbError> {
    let uuid = Uuid::parse_str(id)?;
    
    let result = sqlx::query!("DELETE FROM compressed_items WHERE id = $1", uuid)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(DbError::NotFound);
    }

    Ok(())
}
