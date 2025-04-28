use std::fs::{self, create_dir_all};
use std::io;
use std::path::{Path, PathBuf};
use std::process::exit;
use walkdir::WalkDir;

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

fn walk_and_find_relevance(source: &Path) -> bool {
    for entry in WalkDir::new(source) {
        match entry {
            Ok(entry) => {
                if entry.path().ends_with(".md") {
                    return true;
                }
            }
            Err(err) => {
                eprintln!("{err}");
                return false;
            }
        }
    }

    return false;
}

fn walk_directory(source: &PathBuf, dest: &PathBuf) -> io::Result<()> {
    create_dir_all(dest)?;

    let walker = WalkDir::new(source).into_iter();

    for entry in walker {
        match entry {
            Ok(entry) => {
                let path = entry.path();

                // Calculate the relative path correctly
                let relative_path = match path.strip_prefix(source) {
                    Ok(rel_path) => rel_path,
                    Err(e) => {
                        eprintln!("Failed to strip prefix: {}", e);
                        continue;
                    }
                };

                let target_path = dest.join(relative_path);

                if path.is_dir() {
                    // Only create directories that have .md files
                    // (check handled inside walk_and_find_relevance)
                    if walk_and_find_relevance(path) {
                        create_dir_all(&target_path)?;
                        println!("Created directory: {}", target_path.display());
                    }
                } else if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
                    let org_path = target_path.with_extension("org");
                    println!("Converting {} to {}", path.display(), org_path.display());

                    // Create the .org file (add your conversion logic here)
                    let _ = fs::File::create(org_path)?;
                }
            }
            Err(err) => {
                eprintln!("Error walking directory: {}", err);
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("WalkDir error: {}", err),
                ));
            }
        }
    }

    Ok(())
}

/// Start the sync, it should after the cli provides relevant details
pub fn start_sync(params: &SyncParams) {
    if !params.valid() {
        exit(1)
    }

    if let Err(e) = walk_directory(&params.source, &params.destination) {
        eprintln!("Failed to sync directories: {}", e);
        exit(1);
    }

    println!("Sync completed successfully");
}
