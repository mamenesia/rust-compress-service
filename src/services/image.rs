use crate::core::models::{CompressImageRequest, CompressImageResponse};
use image::{ImageFormat, DynamicImage};
use std::io::Cursor;
use thiserror::Error;
use tracing::{info, warn, error};
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum ImageProcessingError {
    #[error("Failed to download image: {0}")]
    DownloadError(#[from] reqwest::Error),
    
    #[error("Failed to decode image: {0}")]
    DecodeError(#[from] image::ImageError),
    
    #[error("Invalid resize percentage: {0}. Must be between 1 and 100")]
    InvalidResizePercentage(u8),
    
    #[error("Unsupported image format")]
    UnsupportedFormat,
    
    #[error("Image too large: {0} bytes. Maximum allowed: {1} bytes")]
    ImageTooLarge(u64, u64),
}

pub struct ImageCompressionService {
    client: reqwest::Client,
    max_image_size: u64,
}

impl ImageCompressionService {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            max_image_size: 10 * 1024 * 1024, // 10MB limit
        }
    }

    pub async fn compress_image(&self, request: CompressImageRequest) -> Result<CompressImageResponse, ImageProcessingError> {
        // Validate resize percentage
        if request.resize == 0 || request.resize > 100 {
            return Err(ImageProcessingError::InvalidResizePercentage(request.resize));
        }

        info!("Starting image compression for URL: {}", request.image_url);

        // Download the image
        let image_data = self.download_image(&request.image_url).await?;
        let original_size = image_data.len() as u64;

        // Check image size limit
        if original_size > self.max_image_size {
            return Err(ImageProcessingError::ImageTooLarge(original_size, self.max_image_size));
        }

        info!("Downloaded image: {} bytes", original_size);

        // Decode the image
        let img = image::load_from_memory(&image_data)?;
        let content_type = self.detect_content_type(&image_data);

        // Resize the image
        let resized_img = self.resize_image(img, request.resize);
        
        // Compress the image
        let compressed_data = self.compress_image_data(resized_img, &content_type)?;
        let compressed_size = compressed_data.len() as u64;

        // Calculate compression ratio
        let compression_ratio = compressed_size as f64 / original_size as f64;

        info!(
            "Image compression completed. Original: {} bytes, Compressed: {} bytes, Ratio: {:.2}",
            original_size, compressed_size, compression_ratio
        );

        // Encode to base64
        let base64_data = base64::encode(&compressed_data);

        Ok(CompressImageResponse {
            id: Uuid::new_v4().to_string(),
            original_url: request.image_url,
            original_size,
            compressed_size,
            compression_ratio,
            resize_percentage: request.resize,
            compressed_data: base64_data,
            content_type,
            processed_at: chrono::Utc::now(),
        })
    }

    async fn download_image(&self, url: &str) -> Result<Vec<u8>, ImageProcessingError> {
        let response = self.client.get(url).send().await?;
        
        if !response.status().is_success() {
            return Err(ImageProcessingError::DownloadError(
                reqwest::Error::from(response.error_for_status().unwrap_err())
            ));
        }

        let bytes = response.bytes().await?;
        Ok(bytes.to_vec())
    }

    fn resize_image(&self, img: DynamicImage, resize_percentage: u8) -> DynamicImage {
        let (width, height) = img.dimensions();
        let scale_factor = resize_percentage as f32 / 100.0;
        
        let new_width = (width as f32 * scale_factor) as u32;
        let new_height = (height as f32 * scale_factor) as u32;

        info!("Resizing image from {}x{} to {}x{}", width, height, new_width, new_height);

        img.resize(new_width, new_height, image::imageops::FilterType::Lanczos3)
    }

    fn compress_image_data(&self, img: DynamicImage, content_type: &str) -> Result<Vec<u8>, ImageProcessingError> {
        let mut buffer = Vec::new();
        let mut cursor = Cursor::new(&mut buffer);

        let format = match content_type {
            "image/jpeg" => ImageFormat::Jpeg,
            "image/png" => ImageFormat::Png,
            "image/webp" => ImageFormat::WebP,
            _ => {
                warn!("Unsupported format {}, defaulting to JPEG", content_type);
                ImageFormat::Jpeg
            }
        };

        img.write_to(&mut cursor, format)?;
        Ok(buffer)
    }

    fn detect_content_type(&self, data: &[u8]) -> String {
        // Simple magic number detection
        if data.len() >= 3 {
            match &data[0..3] {
                [0xFF, 0xD8, 0xFF] => return "image/jpeg".to_string(),
                [0x89, 0x50, 0x4E] => return "image/png".to_string(),
                _ => {}
            }
        }
        
        if data.len() >= 12 && &data[0..4] == b"RIFF" && &data[8..12] == b"WEBP" {
            return "image/webp".to_string();
        }

        // Default to JPEG
        "image/jpeg".to_string()
    }
}

impl Default for ImageCompressionService {
    fn default() -> Self {
        Self::new()
    }
}
