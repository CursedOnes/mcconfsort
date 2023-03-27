use walkdir::WalkDir;

pub mod sort_toml;
pub mod sort_json;

fn main() {
    for entry in WalkDir::new(".").follow_links(false) {
        let Ok(entry) = entry.map_err(|e| eprintln!("{}",e) ) else {continue};

        if !entry.file_type().is_file() {continue}

        //let Ok(metadata) = entry.metadata().map_err(|e| eprintln!("{}",e) ) else {continue};

        let Ok(canon_path) = entry.path().canonicalize().map_err(|e| eprintln!("{}",e) ) else {continue};

        if canon_path.to_string_lossy().contains(".git/") {continue}

        if entry.file_name().to_string_lossy().ends_with(".toml") {
            sort_toml::sort_toml(&*entry.path())
                .map_err(|e| eprintln!("Failed to sort toml: {}",e) )
                .ok();
        }

        if entry.file_name().to_string_lossy().ends_with(".json") {
            sort_json::sort_json(&*entry.path())
                .map_err(|e| eprintln!("Failed to sort json: {}",e) )
                .ok();
        }
    }
}
