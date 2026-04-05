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

pub struct ScanResult {
    pub title: Option<String>,
    pub season: Option<i32>,
    pub episode: Option<i32>,
    pub type_id: Option<String>,
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
        };
    }

    let season = result.season();
    let episode = result.episode();

    let type_id = if season.unwrap_or(0) > 0 || episode.unwrap_or(0) > 0 {
        Some("TV Show".to_string())
    } else {
        Some("Movie".to_string())
    };

    ScanResult {
        title: result.title().map(|t| t.to_string()),
        season,
        episode,
        type_id,
    }
}
