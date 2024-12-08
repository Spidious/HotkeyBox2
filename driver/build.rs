use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    // Get the target directory for the build (target/release or target/debug)
    let target_dir = env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR not set");

    // Define the source and destination paths for resources
    let resource_dir = Path::new("resources");
    let dest_dir = Path::new(&target_dir).join("target").join("release").join("resources");

    // Ensure the destination directory exists
    fs::create_dir_all(&dest_dir).unwrap();

    // Copy all files from the resources directory to the destination
    if resource_dir.exists() {
        for entry in fs::read_dir(resource_dir).unwrap() {
            let entry = entry.unwrap();
            let src_path = entry.path();
            let dest_path = dest_dir.join(entry.file_name());
            if src_path.is_dir() {
                fs::create_dir_all(dest_path).unwrap();
            } else {
                fs::copy(src_path, dest_path).unwrap();
            }
        }
    } else {
        eprintln!("Warning: 'resources' directory not found.");
    }
}
