// use crate::core::database::DbPool;

// Application state (no database needed for compress endpoint)
#[derive(Debug, Clone)]
pub struct AppState {
    // pub db_pool: DbPool,
}

impl AppState {
    pub fn new(/* db_pool: DbPool */) -> Self {
        Self { /* db_pool */ }
    }
}
