use std::env;
use std::fs;
use std::path::Path;

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
        copy_recursively(resource_dir, &dest_dir).unwrap();
    } else {
        eprintln!("Warning: 'resources' directory not found.");
    }
}

fn copy_recursively(src: &Path, dest: &Path) -> std::io::Result<()> {
    // Iterate over the source directory
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dest_path = dest.join(entry.file_name());

        if src_path.is_dir() {
            // If it's a directory, create the destination directory and recurse into it
            fs::create_dir_all(&dest_path)?;
            copy_recursively(&src_path, &dest_path)?;
        } else {
            // If it's a file, copy it
            fs::copy(&src_path, &dest_path)?;
        }
    }

    Ok(())
}
