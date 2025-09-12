use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow, ToSchema)]
#[schema(example = json!({
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "name": "Example Item",
    "data": "SGVsbG8gV29ybGQ=",
    "created_at": "2023-01-01T00:00:00Z",
    "updated_at": "2023-01-01T00:00:00Z"
}))]
pub struct CompressedItem {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: Uuid,
    #[schema(example = "My Data Item")]
    pub name: String,
    #[schema(example = "SGVsbG8gV29ybGQ=")]
    pub data: String, // Base64 encoded compressed data
    #[schema(example = "2023-01-01T00:00:00Z")]
    pub created_at: Option<DateTime<Utc>>,
    #[schema(example = "2023-01-01T00:00:00Z")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(example = json!({
    "name": "My New Item",
    "data": "SGVsbG8gV29ybGQ="
}))]
pub struct CreateCompressedItem {
    #[schema(example = "My New Item")]
    pub name: String,
    #[schema(example = "SGVsbG8gV29ybGQ=")]
    pub data: String, // Base64 encoded data to compress
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(example = json!({
    "name": "Updated Item Name"
}))]
pub struct UpdateCompressedItem {
    #[schema(example = "Updated Item Name")]
    pub name: Option<String>,
    #[schema(example = "SGVsbG8gV29ybGQ=")]
    pub data: Option<String>, // Base64 encoded data to compress
}
