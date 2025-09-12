use crate::core::database::DbPool;
use sqlx;

#[derive(Debug)]
pub struct DatabaseStats {
    pub total_items: i64,
    pub total_data_size: i64,
}

pub async fn count_items(pool: &DbPool) -> Result<i64, sqlx::Error> {
    let result = sqlx::query!("SELECT COUNT(*) as count FROM compressed_items")
        .fetch_one(pool)
        .await?;
    
    Ok(result.count.unwrap_or(0))
}

pub async fn clear_all_items(pool: &DbPool) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!("DELETE FROM compressed_items")
        .execute(pool)
        .await?;
    
    Ok(result.rows_affected())
}

pub async fn get_database_stats(pool: &DbPool) -> Result<DatabaseStats, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        SELECT 
            COUNT(*) as total_items,
            COALESCE(SUM(LENGTH(data)), 0) as total_data_size
        FROM compressed_items
        "#
    )
    .fetch_one(pool)
    .await?;
    
    Ok(DatabaseStats {
        total_items: result.total_items.unwrap_or(0),
        total_data_size: result.total_data_size.unwrap_or(0),
    })
}
