use rust_compress_api::{AppConfig, core::database};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    // Load configuration
    let config = AppConfig::from_env()?;

    // Create database connection pool
    let db_pool = database::create_pool(&config.database.url).await?;

    match args[1].as_str() {
        "count" => {
            let count = rust_compress_api::services::admin::count_items(&db_pool).await?;
            println!("Total items in database: {}", count);
        }
        "clear" => {
            if args.len() < 3 || args[2] != "--confirm" {
                println!("WARNING: This will delete all items from the database.");
                println!("To confirm, run with: admin clear --confirm");
                return Ok(());
            }

            let deleted = rust_compress_api::services::admin::clear_all_items(&db_pool).await?;
            println!("Deleted {} items from database", deleted);
        }
        "stats" => {
            let stats = rust_compress_api::services::admin::get_database_stats(&db_pool).await?;
            println!("Database Statistics:");
            println!("  Total items: {}", stats.total_items);
            println!("  Total data size: {} bytes", stats.total_data_size);
        }
        _ => {
            print_usage();
        }
    }

    Ok(())
}

fn print_usage() {
    println!("Usage: admin <command>");
    println!("Commands:");
    println!("  count    - Count total items in database");
    println!("  clear    - Delete all items from database (requires --confirm)");
    println!("  stats    - Show database statistics");
}
