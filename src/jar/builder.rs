use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use zip::write::FileOptions;
use zip::ZipWriter;

/// Creates a .jar file with the given contents
pub fn create_jar(output_path: &str, files: &[(&str, &[u8])]) -> std::io::Result<()> {
    let path = Path::new(output_path);
    let file = File::create(&path)?;
    let mut zip = ZipWriter::new(BufWriter::new(file));

    for (name, content) in files {
        zip.start_file(name, FileOptions::default())?;
        zip.write_all(content)?;
    }

    zip.finish()?;
    Ok(())
}
