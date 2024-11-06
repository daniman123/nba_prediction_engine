use dotenv::dotenv;
use std::env;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use std::path::Path;

fn init_tracing() -> WorkerGuard {
    dotenv().ok(); // Load .env file

    let filter = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());

    // Use a relative path for portability or create a path using an environment variable
    let log_path = env::var("LOG_DIR").unwrap_or_else(|_| "logs".to_string());
    let log_dir = Path::new(&log_path);

    // Create a file appender with hourly rotation.
    let file_appender = RollingFileAppender::new(Rotation::NEVER, log_dir, "nba_log.log");

    // Ensure non-blocking logging to avoid application slowdown
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    // Configure the subscriber
    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_env_filter(filter)
        .with_line_number(true)
        .with_file(true)
        .init();

    // Return the guard to ensure logs are flushed when the program exits
    guard
}


#[tokio::main]
async fn main() {
    let _guard = init_tracing(); // Initialize tracing and hold the guard

    tracing::info!("Starting the NBA prediction engine");

    // Call your data fetching functions
}
