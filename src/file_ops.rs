use std::fs;
use std::io::{self, Error};
use std::path::Path;
use crate::config::Config;
use crate::utils::format_file_size;

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
        println!("\t🗑️  Previous directory exists, cleaning up...");
        fs::remove_dir_all(output_dir).map_err(|e| {
            Error::other(
                format!("Could not remove existing directory '{}': {}", output_dir.display(), e),
            )
        })?;
    }

    fs::create_dir_all(output_dir).map_err(|e| {
        Error::other(
            format!("Could not create directory '{}': {}", output_dir.display(), e),
        )
    })
}


/// Copies files from the source directory to the processed directory.
/// 
/// This function reads all the files in the `source_dir`, filters for image files,
/// renames them sequentially as `image_1.ext`, `image_2.ext`, etc., and copies 
/// them to the `processed_dir`. Only copies files that appear to be images based
/// on file size (>= 100KB) to filter out small icon files.
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
pub fn copy_files(processed_dir: &Path, source_dir: &Path, config: &Config) -> Result<u16, Error> {
    let mut files_copied: u16 = 0;

    let source_dir_iter = source_dir.read_dir().map_err(|e| {
        Error::other(
            format!("Could not read source directory '{}': {}", source_dir.display(), e),
        )
    })?;

    for entry in source_dir_iter {
        let entry = entry.map_err(|e| {
            Error::other(
                format!("Could not process directory entry: {}", e),
            )
        })?;

        // Skip if not a file
        if !entry.file_type().map_err(|e| {
            Error::other(format!("Could not get file type: {}", e))
        })?.is_file() {
            continue;
        }

        // Get file metadata to check size (filter out small files that are likely not wallpapers)
        let metadata = entry.metadata().map_err(|e| {
            Error::other(format!("Could not read file metadata: {}", e))
        })?;
        
        // Skip files smaller than configured size (likely not wallpaper images)
        if metadata.len() < config.min_size_bytes() {
            if config.verbose {
                println!("\t\t⏩ Skipped: {} (too small: {})", entry.file_name().to_string_lossy(), format_file_size(metadata.len()));
            }
            continue;
        }

        let file_name = entry
            .file_name()
            .into_string()
            .map_err(|_| io::Error::other("Filename is not valid UTF-8"))?;
        
        // Try to preserve original extension, default to jpg
        let extension = Path::new(&file_name)
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("jpg");

        // Check if extension should be excluded
        let excluded_extensions = config.excluded_extensions();
        if !excluded_extensions.is_empty() && excluded_extensions.contains(&extension.to_lowercase()) {
            if config.verbose {
                println!("\t\t⏩ Skipped: {} (excluded extension)", entry.file_name().to_string_lossy());
            }
            continue;
        }

        let new_name = format!("{}_{}.{}", config.prefix, files_copied + 1, extension);
        let new_path = processed_dir.join(&new_name);
        let source_path = entry.path();

        if config.dry_run {
            println!("\t\t🔍 Would copy: {} -> {} ({})", 
                source_path.file_name().unwrap().to_string_lossy(), 
                new_name, 
                format_file_size(metadata.len()));
        } else {
            fs::copy(&source_path, &new_path).map_err(|e| {
                Error::other(
                    format!(
                        "Could not copy file from '{}' to '{}': {}",
                        source_path.display(), new_path.display(), e
                    ),
                )
            })?;
        }
        
        // Print file copy progress for large operations
        if config.verbose || files_copied < 10 || files_copied.is_multiple_of(10) {
            let action = if config.dry_run { "Would copy" } else { "Copied" };
            println!("\t\t📄 {}: {} ({})", action, new_name, format_file_size(metadata.len()));
        }
        
        files_copied += 1;
    }

    Ok(files_copied)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_prepare_dir_creates_new_directory() {
        let temp_dir = tempdir().unwrap();
        let test_path = temp_dir.path().join("test_output");
        
        // Test creating a new directory
        assert!(prepare_dir(&test_path).is_ok());
        assert!(test_path.exists());
        assert!(test_path.is_dir());
    }

    #[test]
    fn test_prepare_dir_removes_existing_directory() {
        let temp_dir = tempdir().unwrap();
        let test_path = temp_dir.path().join("test_output");
        
        // Create a directory first
        fs::create_dir(&test_path).unwrap();
        fs::write(test_path.join("test_file.txt"), "test content").unwrap();
        
        // Test that prepare_dir removes the existing directory and creates a new one
        assert!(prepare_dir(&test_path).is_ok());
        assert!(test_path.exists());
        assert!(test_path.is_dir());
        assert!(!test_path.join("test_file.txt").exists());
    }

    #[test]
    fn test_copy_files_with_empty_directory() {
        let temp_dir = tempdir().unwrap();
        let source_dir = temp_dir.path().join("source");
        let dest_dir = temp_dir.path().join("dest");
        
        fs::create_dir(&source_dir).unwrap();
        fs::create_dir(&dest_dir).unwrap();
        
        let config = Config {
            output: None,
            min_size_kb: 100,
            prefix: "image".to_string(),
            exclude_extensions: None,
            verbose: false,
            dry_run: false,
        };
        let result = copy_files(&dest_dir, &source_dir, &config).unwrap();
        assert_eq!(result, 0);
    }
}