use std::path::Path;

/// Ensures a directory exists
pub fn ensure_dir_exists(path: &str) -> std::io::Result<()> {
    let dir = Path::new(path);
    if !dir.exists() {
        std::fs::create_dir_all(dir)?;
    }
    Ok(())
}
