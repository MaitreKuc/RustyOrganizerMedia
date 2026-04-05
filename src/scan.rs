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

use walkdir::WalkDir;
use hunch::hunch;

pub fn scan_directory(path: &str) {
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let file_name = entry.file_name().to_string_lossy();
            let result = hunch(&file_name);

            let season = result.season(); 
            let episode = result.episode(); 
            let title = result.title(); 
            let type_id;

            if season.unwrap_or(0) > 0 && episode.unwrap_or(0) > 0 {
                type_id = Some("TV Show");
            } else if season.unwrap_or(0) == 0 && episode.unwrap_or(0) == 0 {
                type_id = Some("Movie");
            } else {
                type_id = None;
            }
            
            println!("File: {}", file_name);
            println!("Title: {:?}", title.unwrap_or("Unknown"));
            println!("Season: {:?}", season.unwrap_or(0));
            println!("Episode: {:?}", episode.unwrap_or(0));
            println!("Type ID: {:?}", type_id.unwrap_or("Unknown"));
            println!("-----------------------------");
        }
    }
}