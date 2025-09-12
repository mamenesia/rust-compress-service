/// Welcome endpoint
///
/// Returns a welcome message for the API
#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Welcome message", body = String)
    )
)]
pub async fn root() -> &'static str {
    "Welcome to the Rust Compress API!"
}

/// Health check endpoint
///
/// Returns a simple OK message to indicate the service is running
#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Health check OK", body = String)
    )
)]
pub async fn health_check() -> &'static str {
    "OK"
}
