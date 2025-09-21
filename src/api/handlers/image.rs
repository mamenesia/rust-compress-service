use crate::core::models::{CompressImageRequest, CompressImageResponse};
use crate::services::ImageCompressionService;
use axum::{
    http::StatusCode,
    response::Json,
};
use serde_json::{Value, json};
use tracing::error;

/// Compress an image from URL with resize option
///
/// Downloads an image from the provided URL, resizes it according to the specified percentage,
/// and returns the compressed image data along with compression statistics.
///
/// Response codes:
/// - 200: Successfully compressed image
/// - 400: Bad request (invalid URL, resize percentage, etc.)
/// - 413: Image too large
/// - 500: Internal server error
#[utoipa::path(
    post,
    path = "/compress",
    request_body = CompressImageRequest,
    responses(
        (status = 200, description = "Successfully compressed image", body = CompressImageResponse),
        (status = 400, description = "Bad request", body = Value),
        (status = 413, description = "Image too large", body = Value),
        (status = 500, description = "Internal server error", body = Value)
    )
)]
pub async fn compress_image_handler(
    Json(payload): Json<CompressImageRequest>,
) -> Result<Json<CompressImageResponse>, (StatusCode, Json<Value>)> {
    let service = ImageCompressionService::new();
    
    match service.compress_image(payload).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => {
            error!("Image compression failed: {:?}", e);
            
            let (status_code, error_message) = match e {
                crate::services::ImageProcessingError::InvalidResizePercentage(percentage) => {
                    (StatusCode::BAD_REQUEST, format!("Invalid resize percentage: {}. Must be between 1 and 100", percentage))
                }
                crate::services::ImageProcessingError::ImageTooLarge(size, max_size) => {
                    (StatusCode::PAYLOAD_TOO_LARGE, format!("Image too large: {} bytes. Maximum allowed: {} bytes", size, max_size))
                }
                crate::services::ImageProcessingError::DownloadError(_) => {
                    (StatusCode::BAD_REQUEST, "Failed to download image from URL".to_string())
                }
                crate::services::ImageProcessingError::DecodeError(_) => {
                    (StatusCode::BAD_REQUEST, "Invalid or corrupted image format".to_string())
                }
                crate::services::ImageProcessingError::UnsupportedFormat => {
                    (StatusCode::BAD_REQUEST, "Unsupported image format".to_string())
                }
                crate::services::ImageProcessingError::InvalidInput(msg) => {
                    (StatusCode::BAD_REQUEST, msg)
                }
            };
            
            Err((
                status_code,
                Json(json!({"error": error_message})),
            ))
        }
    }
}
