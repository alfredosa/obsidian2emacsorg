// sync.rs
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;
use std::process::exit;

pub struct SyncParams {
    pub source: PathBuf,
    pub destination: PathBuf,
}

impl SyncParams {
    fn valid(&self) -> bool {
        let mut ret = true;

        if !self.destination.exists() {
            match fs::create_dir(self.destination.clone()) {
                Ok(_) => println!(
                    "destination folder: {} doesn't exist, creating...",
                    self.destination.to_str().unwrap_or("<unable to read dir>")
                ),
                Err(e) => println!("Failed to create directory: {}", e),
            }
        }

        if !self.source.exists() {
            eprintln!(
                "destination folder: {}",
                self.destination.to_str().unwrap_or("<unable to read dir>")
            );
            ret = false;
        }

        return ret;
    }
}

fn walk_directory(path: &PathBuf) {
    for entry in WalkDir::new(path) {
        match entry {
            Ok(entry) => {
                println!("{}", entry.path().display());
            }
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        }
    }
}

/// Start the sync, it should after the cli provides relevant details
pub fn start_sync(params: &SyncParams) {
    if !params.valid() {exit(1)}
    walk_directory(&params.source);

    // println!("starting sync at @Mon Apr  7 12:38:01 2025");
}
