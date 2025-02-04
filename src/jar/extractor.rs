use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use zip::ZipArchive;

/// Extracts a .jar file to the specified directory
pub fn extract_jar(jar_path: &str, output_dir: &str) -> std::io::Result<()> {
    let path = Path::new(jar_path);
    let file = File::open(&path)?;
    let mut archive = ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = Path::new(output_dir).join(file.name());

        if file.is_dir() {
            std::fs::create_dir_all(&outpath)?;
        } else {
            if let Some(parent) = outpath.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let mut outfile = File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }
    Ok(())
}
