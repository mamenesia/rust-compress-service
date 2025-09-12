use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Request payload for image compression
#[derive(Debug, Deserialize, ToSchema)]
pub struct CompressImageRequest {
    /// URL of the image to compress
    #[schema(example = "https://example.com/image.jpg")]
    pub image_url: String,
    
    /// Resize percentage (1-100)
    #[schema(example = 50, minimum = 1, maximum = 100)]
    pub resize: u8,
    
    /// JPEG quality (1-100, optional, defaults to 75)
    #[schema(example = 75, minimum = 1, maximum = 100)]
    pub quality: Option<u8>,
}

/// Response for successful image compression
#[derive(Debug, Serialize, ToSchema)]
pub struct CompressImageResponse {
    /// Unique identifier for the compressed image
    pub id: String,
    
    /// Original image URL
    pub original_url: String,
    
    /// Original file size in bytes
    pub original_size: u64,
    
    /// Compressed file size in bytes
    pub compressed_size: u64,
    
    /// Compression ratio (compressed_size / original_size)
    pub compression_ratio: f64,
    
    /// Resize percentage applied
    pub resize_percentage: u8,
    
    /// Base64 encoded compressed image data
    pub compressed_data: String,
    
    /// MIME type of the image
    pub content_type: String,
    
    /// Processing timestamp
    pub processed_at: chrono::DateTime<chrono::Utc>,
}

/// Image compression statistics
#[derive(Debug, Serialize, ToSchema)]
pub struct ImageCompressionStats {
    /// Total images processed
    pub total_processed: u64,
    
    /// Total bytes saved through compression
    pub total_bytes_saved: u64,
    
    /// Average compression ratio
    pub average_compression_ratio: f64,
}
