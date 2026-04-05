mod error;
mod scan;
mod config;
// ...les autres mod

use clap::Parser;
use config::Config;
use std::path::PathBuf;
use tracing::{info, error};

#[derive(Parser)]
#[command(name = "rustmediacenter", about = "Rust Media Center - Organized symlinks from rclone mount")]
struct Cli {
    /// Path to the rclone mount (source)
    #[arg(long, env = "RCLONE_MOUNT_PATH", default_value = "/mnt/remote/torbox")]
    source: PathBuf,

    /// Path to the organized output directory
    #[arg(long, env = "OUTPUT_PATH", default_value = "/mnt/rustmediacenter")]
    output: PathBuf,

    /// TMDb API key
    #[arg(long, env = "TMDB_API_KEY")]
    tmdb_api_key: String,

    /// TVDB API key
    #[arg(long, env = "TVDB_API_KEY")]
    tvdb_api_key: String,

    /// Scan interval in seconds (0 = run once)
    #[arg(long, env = "SCAN_INTERVAL", default_value = "300")]
    scan_interval: u64,

    /// SQLite cache database path
    #[arg(long, env = "CACHE_DB_PATH", default_value = "/mnt/rustmediacenter/.rustmediacenter-cache.db")]
    cache_db: PathBuf,

    /// Log level (trace, debug, info, warn, error)
    #[arg(long, env = "LOG_LEVEL", default_value = "info")]
    log_level: String,

    /// Dry run mode - show what would be done without creating symlinks
    #[arg(long, default_value = "false")]
    dry_run: bool,
}

#[tokio::main]
async fn main() -> Result<(), error::AppError> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();
    
    let config = Config {
        source: cli.source,
        output: cli.output,
        tmdb_api_key: cli.tmdb_api_key,
        tvdb_api_key: cli.tvdb_api_key,
        scan_interval: cli.scan_interval,
        cache_db: cli.cache_db,
        dry_run: cli.dry_run,
    };

    scan::scan_directory(&config.source.to_string_lossy());
    // ... ta logique ici


    Ok(())
}