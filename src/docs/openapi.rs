use utoipa::OpenApi;
use utoipa_scalar::Scalar;

use crate::core::models::{CompressedItem, CreateCompressedItem, UpdateCompressedItem, CompressImageRequest, CompressImageResponse};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::handlers::root,
        crate::api::handlers::health_check,
        crate::api::handlers::get_items,
        crate::api::handlers::get_item,
        crate::api::handlers::create_item_handler,
        crate::api::handlers::update_item_handler,
        crate::api::handlers::delete_item_handler,
        crate::api::handlers::compress_image_handler,
    ),
    components(
        schemas(CompressedItem, CreateCompressedItem, UpdateCompressedItem, CompressImageRequest, CompressImageResponse)
    ),
    tags(
        (name = "rust-compress-api", description = "API for compressing and managing data")
    )
)]
pub struct ApiDoc;

/// Scalar API documentation handler
pub async fn scalar_handler() -> axum::response::Html<String> {
    axum::response::Html(Scalar::new(ApiDoc::openapi()).to_html())
}
