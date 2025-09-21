//! Rust Compress API
//! 
//! A REST API for compressing and managing data, built with Rust and Axum,
//! following the 12-factor principles.

pub mod api;
pub mod core;
pub mod services;
pub mod utils;
pub mod docs;

// Re-export commonly used types for convenience
pub use core::config::AppConfig;
pub use core::models::AppState;
// pub use core::database::DbPool;
// pub use services::admin;
