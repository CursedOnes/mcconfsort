use std::path::PathBuf;

use clap::Parser;
use walkdir::WalkDir;

pub mod sort_toml;
pub mod sort_json;

fn main() {
    let args = Args::parse();

    for entry in WalkDir::new(&args.path).follow_links(false) {
        let Ok(entry) = entry.map_err(|e| eprintln!("{}",e) ) else {continue};

        if !entry.file_type().is_file() {continue}

        //let Ok(metadata) = entry.metadata().map_err(|e| eprintln!("{}",e) ) else {continue};

        let Ok(canon_path) = entry.path().canonicalize().map_err(|e| eprintln!("{}",e) ) else {continue};

        if canon_path.to_string_lossy().contains(".git/") {continue}

        if !args.no_toml && entry.file_name().to_string_lossy().ends_with(".toml") {
            sort_toml::sort_toml(&*entry.path(), args.sorter_args())
                .map_err(|e| eprintln!("Failed to sort toml: {}",e) )
                .ok();
        }

        if !args.no_json && entry.file_name().to_string_lossy().ends_with(".json") {
            sort_json::sort_json(&*entry.path(), args.sorter_args())
                .map_err(|e| eprintln!("Failed to sort json: {}",e) )
                .ok();
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Path of folder to recursively sort in
    #[arg()]
    path: PathBuf,
    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
    /// Don't sort JSON files
    #[arg(long)]
    no_json: bool,
    /// Don't sort TOML files
    #[arg(long)]
    no_toml: bool,
    /// Don't write sorted files
    #[arg(short, long)]
    noop: bool,
}

pub struct SorterArgs {
    pub noop: bool,
    pub verbose: bool,
}

impl Args {
    fn sorter_args(&self) -> SorterArgs {
        SorterArgs {
            noop: self.noop,
            verbose: self.verbose,
        }
    }
}
