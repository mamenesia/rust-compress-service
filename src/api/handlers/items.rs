use crate::core::database::{
    DbError, create_item, delete_item, get_all_items, get_item_by_id, update_item,
};
use crate::core::models::{AppState, CompressedItem, CreateCompressedItem, UpdateCompressedItem};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde_json::{Value, json};
use uuid::Uuid;

/// Get all compressed items
///
/// Returns a list of all compressed items in the database
///
/// Response codes:
/// - 200: Successfully retrieved items
/// - 500: Internal server error
#[utoipa::path(
    get,
    path = "/items",
    responses(
        (status = 200, description = "List of compressed items", body = [CompressedItem]),
        (status = 500, description = "Internal server error", body = Value)
    )
)]
pub async fn get_items(
    State(state): State<AppState>,
) -> Result<Json<Vec<CompressedItem>>, (StatusCode, Json<Value>)> {
    match get_all_items(&state.db_pool).await {
        Ok(items) => Ok(Json(items)),
        Err(e) => {
            eprintln!("Error getting items: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to get items"})),
            ))
        }
    }
}

/// Get a specific compressed item by ID
///
/// Returns a single compressed item based on its ID
///
/// Response codes:
/// - 200: Successfully retrieved item
/// - 404: Item not found
/// - 500: Internal server error
#[utoipa::path(
    get,
    path = "/items/{id}",
    params(
        ("id" = Uuid, Path, description = "Item ID")
    ),
    responses(
        (status = 200, description = "Compressed item", body = CompressedItem),
        (status = 404, description = "Item not found", body = Value),
        (status = 500, description = "Internal server error", body = Value)
    )
)]
pub async fn get_item(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<CompressedItem>, (StatusCode, Json<Value>)> {
    match get_item_by_id(&state.db_pool, &id.to_string()).await {
        Ok(item) => Ok(Json(item)),
        Err(DbError::NotFound) => Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Item not found"})),
        )),
        Err(e) => {
            eprintln!("Error getting item: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to get item"})),
            ))
        }
    }
}

/// Create a new compressed item
///
/// Creates a new compressed item with the provided data
///
/// Response codes:
/// - 201: Successfully created item
/// - 400: Bad request
/// - 500: Internal server error
#[utoipa::path(
    post,
    path = "/items",
    request_body = CreateCompressedItem,
    responses(
        (status = 201, description = "Created compressed item", body = CompressedItem),
        (status = 400, description = "Bad request", body = Value),
        (status = 500, description = "Internal server error", body = Value)
    )
)]
pub async fn create_item_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateCompressedItem>,
) -> Result<(StatusCode, Json<CompressedItem>), (StatusCode, Json<Value>)> {
    match create_item(&state.db_pool, payload).await {
        Ok(item) => Ok((StatusCode::CREATED, Json(item))),
        Err(e) => {
            eprintln!("Error creating item: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to create item"})),
            ))
        }
    }
}

/// Update an existing compressed item
///
/// Updates an existing compressed item with the provided data
///
/// Response codes:
/// - 200: Successfully updated item
/// - 404: Item not found
/// - 500: Internal server error
#[utoipa::path(
    put,
    path = "/items/{id}",
    params(
        ("id" = Uuid, Path, description = "Item ID")
    ),
    request_body = UpdateCompressedItem,
    responses(
        (status = 200, description = "Updated compressed item", body = CompressedItem),
        (status = 404, description = "Item not found", body = Value),
        (status = 500, description = "Internal server error", body = Value)
    )
)]
pub async fn update_item_handler(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateCompressedItem>,
) -> Result<Json<CompressedItem>, (StatusCode, Json<Value>)> {
    match update_item(&state.db_pool, &id.to_string(), payload).await {
        Ok(item) => Ok(Json(item)),
        Err(DbError::NotFound) => Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Item not found"})),
        )),
        Err(e) => {
            eprintln!("Error updating item: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to update item"})),
            ))
        }
    }
}

/// Delete a compressed item
///
/// Deletes a compressed item based on its ID
///
/// Response codes:
/// - 200: Successfully deleted item
/// - 404: Item not found
/// - 500: Internal server error
#[utoipa::path(
    delete,
    path = "/items/{id}",
    params(
        ("id" = Uuid, Path, description = "Item ID")
    ),
    responses(
        (status = 200, description = "Item deleted successfully", body = Value),
        (status = 404, description = "Item not found", body = Value),
        (status = 500, description = "Internal server error", body = Value)
    )
)]
pub async fn delete_item_handler(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    match delete_item(&state.db_pool, &id.to_string()).await {
        Ok(_) => Ok(Json(json!({"message": "Item deleted successfully"}))),
        Err(DbError::NotFound) => Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Item not found"})),
        )),
        Err(e) => {
            eprintln!("Error deleting item: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to delete item"})),
            ))
        }
    }
}
