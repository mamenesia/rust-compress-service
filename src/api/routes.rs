use axum::{
    Router,
    routing::{get, post, put, delete},
};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing::Level;

use crate::core::models::AppState;
use crate::api::handlers::{
    root, health_check, get_items, get_item, create_item_handler, 
    update_item_handler, delete_item_handler, compress_image_handler
};
use crate::docs::scalar_handler;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .route("/items", get(get_items).post(create_item_handler))
        .route("/items/{id}", get(get_item).put(update_item_handler).delete(delete_item_handler))
        .route("/compress-image", post(compress_image_handler))
        .route("/scalar", get(scalar_handler))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO)),
        )
}
