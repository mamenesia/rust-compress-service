use std::process::Command;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Test the compression endpoint with a sample image
    let client = reqwest::Client::new();
    
    let test_payload = json!({
        "image_url": "https://httpbin.org/image/jpeg",
        "resize": 50,
        "quality": 60
    });
    
    println!("Testing image compression endpoint...");
    
    let response = client
        .post("http://localhost:3000/compress-image")
        .json(&test_payload)
        .send()
        .await?;
    
    if response.status().is_success() {
        let result: serde_json::Value = response.json().await?;
        println!("✅ Compression successful!");
        println!("Original size: {} bytes", result["original_size"]);
        println!("Compressed size: {} bytes", result["compressed_size"]);
        println!("Compression ratio: {:.2}", result["compression_ratio"]);
        
        if result["compression_ratio"].as_f64().unwrap() < 1.0 {
            println!("✅ Compression working correctly - file size reduced!");
        } else {
            println!("⚠️ Compression ratio > 1 - file got larger");
        }
    } else {
        println!("❌ Request failed: {}", response.status());
        println!("Response: {}", response.text().await?);
    }
    
    Ok(())
}
