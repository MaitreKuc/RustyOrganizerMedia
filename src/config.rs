use std::path::PathBuf;

pub struct Config {
    pub source: PathBuf,
    pub output: PathBuf,
    pub tmdb_api_key: String,
    pub tvdb_api_key: String,
    pub scan_interval: u64,
    pub cache_db: PathBuf,
    pub dry_run: bool,
}