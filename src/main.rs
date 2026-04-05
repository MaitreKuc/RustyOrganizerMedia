mod error;
mod scan;
mod config;
// ...les autres mod

use clap::Parser;
use config::Config;
use std::path::PathBuf;
use tracing::{info, error};
use walkdir::WalkDir;
use std::fs;

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

    /// Remove old symbolic links that no longer point to valid files
    #[arg(long, env = "REMOVE_OLD_LINKS", default_value_t = false)]
    remove_old_links: bool,
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
        remove_old_links: cli.remove_old_links,
    };

    

    if !config.source.exists() {
        error!("Source path does not exist: {:?}. Is rclone mounted?", config.source);
        std::process::exit(1);
    }else if !config.output.exists() {
        error!("Output path does not exist: {:?}. Please create it before running.", config.output);
        std::process::exit(1);
    }else if !config.remove_old_links {
        error!("Remove old links flag is required. Please set it via --remove-old-links or REMOVE_OLD_LINKS environment variable.");
        std::process::exit(1);
    }
    /*else if !config.tmdb_api_key.is_empty() {
        error!("TMDb API key is required. Please set it via --tmdb-api-key or TMDB_API_KEY environment variable.");
        std::process::exit(1);
    }else if !config.tvdb_api_key.is_empty() {
        error!("TVDB API key is required. Please set it via --tvdb-api-key or TVDB_API_KEY environment variable.");
        std::process::exit(1);
    }*/

    //Creation des dossiers
    let movies_dir = config.output.join("movies");
    let series_dir = config.output.join("series");

    if !movies_dir.exists() || !series_dir.exists() {
        fs::create_dir_all(&movies_dir)?;
        fs::create_dir_all(&series_dir)?;
    }

    println!("RustTMC starting");
    println!("Source: {:?}", config.source);
    println!("Output: {:?}", config.output);
    println!("Scan interval: {}s (0 = once)", config.scan_interval);


    if config.remove_old_links {
        println!("Removing old symbolic links...");
        for entry in WalkDir::new(&config.output).into_iter().filter_map(|e| e.ok()) {
            scan::remove_old_links(&movies_dir)?;
            scan::remove_old_links(&series_dir)?;
        }
        
        
    }


    
    for entry in WalkDir::new(&config.source).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let path = entry.path();
            let file_name = path.file_name().unwrap().to_string_lossy();    
            let result = scan::scan_directory(&file_name);

            if result.type_id != None {
                let title = result.title.clone().unwrap_or("Unknown".to_string());
                let season = result.season.unwrap_or(0);
                let episode = result.episode.unwrap_or(0);
                let type_id = result.type_id.clone().unwrap_or("Unknown".to_string());
                let year = result.year.unwrap_or(0);

                

                if type_id == "Movie" {
                    //Créer dossier fime et faire le lien symbolique
                    let movie_dir = movies_dir.join(&title);
                    if !movie_dir.exists() {
                        fs::create_dir_all(&movie_dir)?;
                    }

                    let original_path = path;
                    let link_path = movie_dir.join(&*file_name);
                    if !link_path.exists() {
                        println!("File: {}", file_name);
                        println!("Title: {:?}", &title);
                        println!("Season: {:?}", &season);
                        println!("Episode: {:?}", &episode);
                        println!("Type ID: {:?}", &type_id);
                        println!("Year: {:?}", &year);

                        std::os::unix::fs::symlink(&original_path, &link_path)?;
                    }
                    
                
                } else if type_id == "TV Show" {
                    //Créer dossier série, saison et faire le lien symbolique
                    let show_dir = series_dir.join(&title);
                    let season_dir = show_dir.join(format!("Season {}", season));
                    if !season_dir.exists() {
                        fs::create_dir_all(&season_dir)?;   
                    }

                    let original_path = path;
                    let link_path = season_dir.join(&*file_name);
                    if !link_path.exists() {
                        println!("File: {}", file_name);
                        println!("Title: {:?}", &title);
                        println!("Season: {:?}", &season);
                        println!("Episode: {:?}", &episode);
                        println!("Type ID: {:?}", &type_id);
                        println!("Year: {:?}", &year);
                        std::os::unix::fs::symlink(&original_path, &link_path)?;
                    }

                }

            }

            

        }
    }

    
    
    // ... ta logique ici


    Ok(())
}