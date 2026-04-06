// scan le répertoire source pour trouver les fichiers médias
// Utilise hunch pour sortir les métadonnées de chaque fichier
// use hunch::hunch;
//
//fn main() {
//    let result = hunch("The.Walking.Dead.S05E03.720p.BluRay.x264-DEMAND.mkv");
//    assert_eq!(result.title(), Some("The Walking Dead"));
//    assert_eq!(result.season(), Some(5));
//    assert_eq!(result.episode(), Some(3));
//}


use hunch::hunch;
use walkdir::WalkDir;
use regex::Regex;

pub struct ScanResult {
    pub title: Option<String>,
    pub season: Option<i32>,
    pub episode: Option<i32>,
    pub type_id: Option<String>,
    pub year: Option<i32>,
}

/// Video file extensions we care about
const VIDEO_EXTENSIONS: &[&str] = &[
    "mkv", "mp4", "avi", "mov", "wmv", "flv", "webm", "m4v", "mpg", "mpeg", "ts", "m2ts",
];

pub fn scan_directory(file_name: &str) -> ScanResult {
    let result = hunch(file_name);

    // Si ce n'est pas une vidéo → retour vide
    if !VIDEO_EXTENSIONS.iter().any(|&ext| file_name.ends_with(ext)) {
        return ScanResult {
            title: None,
            season: None,
            episode: None,
            type_id: None,
            year: None,
        };
    }

    let season = result.season();
    let episode = result.episode();
    let year = result.year();

    let type_id = if season.unwrap_or(0) > 0 || episode.unwrap_or(0) > 0 {
        Some("TV Show".to_string())
    } else {
        Some("Movie".to_string())
    };

    ScanResult {
        title: result.title().map(|t| clean_title(&t.to_lowercase())),
        season,
        episode,
        year,
        type_id,
    }
}

// Supprime les liens symboliques qui ne pointent plus vers des fichiers valides
pub fn remove_old_links(dir: &std::path::Path) -> std::io::Result<()> {
    for entry in WalkDir::new(dir)
        .follow_links(false) // IMPORTANT
        .into_iter()
        .filter_map(|e| e.ok())
    {        
        let path = entry.path();
        if path.is_symlink() {
            println!("Checking symbolic link: {:?}", path);
            if let Ok(target) = std::fs::read_link(path) {
                if !target.exists() {
                    println!("Removing broken symbolic link: {:?}", path);
                    std::fs::remove_file(path)?;
                }
            } else {
                println!("Failed to read symbolic link: {:?}", path);
            }
        }
    }
    Ok(())
}




fn clean_title(title: &str) -> String {
    let mut t = title.to_lowercase();

    // 1. enlever numéro au début (0894, 12, etc.)
    let re_prefix = Regex::new(r"^\s*\d+\s*").unwrap();
    t = re_prefix.replace(&t, "").to_string();

    // 2. enlever sxxe incomplet ou complet (s02e, s02e05, s02e05v2)
    let re_season = Regex::new(r"(?i)\bs\d{1,2}e\d{0,3}v?\d*\b").unwrap();
    t = re_season.replace_all(&t, "").to_string();

    // 3. enlever résidus "s02e" seuls (cas hunch cassé)
    let re_broken = Regex::new(r"(?i)\bs\d{1,2}e\b").unwrap();
    t = re_broken.replace_all(&t, "").to_string();

    // 4. enlever doublons espaces
    let re_spaces = Regex::new(r"\s+").unwrap();
    t = re_spaces.replace_all(&t, " ").to_string();

    t.trim().to_string()
}