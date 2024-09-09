use std::fs;
use std::io::{self, Error, ErrorKind};
use std::path::Path;

/// Prepares the output directory by deleting any existing directory and creating a new one.
///
/// # Arguments
///
/// * `output_dir` - A reference to a `Path` that specifies the directory to prepare.
///
/// # Returns
///
/// * `Result<(), Error>` - Returns `Ok(())` if the directory is successfully prepared,
///   otherwise returns an `Error` indicating what went wrong.
///
/// # Errors
///
/// This function will return an error if:
/// * The existing directory cannot be deleted.
/// * The new directory cannot be created.
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


/// Copies files from the source directory to the processed directory.
/// 
/// This function reads all the files in the `source_dir`, renames them sequentially
/// as `image_1.jpg`, `image_2.jpg`, etc., and copies them to the `processed_dir`.
/// 
/// # Arguments
/// 
/// * `processed_dir` - A reference to the path where the files will be copied to.
/// * `source_dir` - A reference to the path where the files will be copied from.
/// 
/// # Returns
/// 
/// * `Result<u16, Error>` - Returns the number of files copied on success, or an `Error` on failure.
/// 
/// # Errors
/// 
/// This function will return an error if:
/// 
/// * The `source_dir` cannot be read.
/// * Any entry in the `source_dir` cannot be processed.
/// * A file name in the `source_dir` is not valid UTF-8.
/// * A file cannot be copied from the `source_dir` to the `processed_dir`.
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