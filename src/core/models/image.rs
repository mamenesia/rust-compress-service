use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Request payload for image compression
#[derive(Debug, Deserialize, ToSchema)]
pub struct CompressImageRequest {
    /// Base64 encoded image data (for direct upload)
    #[schema(example = "data:image/jpeg;base64,/9j/4AAQSkZJRgABAQAAAQ...")]
    pub image_data: Option<String>,
    
    /// URL of the image to compress (alternative to image_data)
    #[schema(example = "https://example.com/image.jpg")]
    pub image_url: Option<String>,
    
    /// Original filename
    #[schema(example = "photo.jpg")]
    pub filename: String,
    
    /// Content type
    #[schema(example = "image/jpeg")]
    pub content_type: String,
    
    /// Generate thumbnail (default: true)
    #[schema(example = true)]
    pub generate_thumbnail: Option<bool>,
    
    /// Thumbnail size in pixels (default: 150)
    #[schema(example = 150, minimum = 50, maximum = 300)]
    pub thumbnail_size: Option<u32>,
    
    /// JPEG quality (1-100, optional, defaults to 75)
    #[schema(example = 75, minimum = 1, maximum = 100)]
    pub quality: Option<u8>,
    
    /// Maximum width for resizing (optional)
    #[schema(example = 1920)]
    pub max_width: Option<u32>,
    
    /// Maximum height for resizing (optional)
    #[schema(example = 1080)]
    pub max_height: Option<u32>,
}

/// Response for successful image compression
#[derive(Debug, Serialize, ToSchema)]
pub struct CompressImageResponse {
    /// Unique identifier for the compressed image
    pub file_id: String,
    
    /// Original filename
    pub filename: String,
    
    /// Original file size in bytes
    pub original_size: u64,
    
    /// Compressed file size in bytes
    pub compressed_size: u64,
    
    /// Compression ratio (compressed_size / original_size)
    pub compression_ratio: f64,
    
    /// Base64 encoded compressed image data
    pub compressed_data: String,
    
    /// Base64 encoded thumbnail data (if generated)
    pub thumbnail_data: Option<String>,
    
    /// Thumbnail file size in bytes (if generated)
    pub thumbnail_size: Option<u64>,
    
    /// MIME type of the compressed image
    pub content_type: String,
    
    /// Processing timestamp
    pub processed_at: chrono::DateTime<chrono::Utc>,
    
    /// Processing duration in milliseconds
    pub processing_duration_ms: u64,
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
