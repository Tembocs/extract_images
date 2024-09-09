use std::fs;
use std::io::{self, Error, ErrorKind};
use std::path::Path;

pub fn prepare_dir(output_dir: &Path) -> Result<(), Error> {
    if output_dir.exists() {
        println!("A previous directory exists, deleting ...");
        fs::remove_dir_all(output_dir)?;
    }

    fs::create_dir(output_dir).map_err(|e| {
        Error::new(
            ErrorKind::Other,
            format!("Could not create directory: {}.", e),
        )
    })
}

pub fn copy_files(processed_dir: &Path, source_dir: &Path) -> Result<u16, Error> {
    let mut files_copied: u16 = 0;

    let source_dir_iter = source_dir.read_dir().map_err(|e| {
        Error::new(
            ErrorKind::Other,
            format!("Could not read source directory: {}.", e),
        )
    })?;

    for entry in source_dir_iter {
        let entry = entry.map_err(|e| {
            Error::new(
                ErrorKind::Other,
                format!("Could not process directory entry: {}.", e),
            )
        })?;
        let file_name = entry
            .file_name()
            .into_string()
            .map_err(|_| io::Error::new(ErrorKind::InvalidData, "Filename is not valid UTF-8"))?;
        let extension = Path::new(&file_name)
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("jpg");

        let new_name = format!("image_{}.{}", files_copied + 1, extension);
        let new_path = processed_dir.join(new_name);
        let source_path = entry.path();

        fs::copy(&source_path, &new_path).map_err(|_| {
            Error::new(
                ErrorKind::Other,
                format!(
                    "Could not copy file from {:?} to {:?}",
                    source_path, new_path
                ),
            )
        })?;
        files_copied += 1;
    }

    Ok(files_copied)
}