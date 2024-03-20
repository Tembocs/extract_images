// Import necessary standard libraries and external crates
use dirs::home_dir; // For finding the home directory
use std::fs;
use std::io::{self, Error, ErrorKind};
use std::path::Path;
use term_size; // For determining terminal size

/// The main entry point of the application.
/// It calls the `run_app` function and handles any errors that occur.
fn main() {
    match run_app() {
        Ok(()) => println!("Application completed successfully."),
        Err(e) => eprintln!("Application error: {}", e),
    }
}

/// Encapsulates the main logic of the application.
/// It prepares the directory, copies the files, and handles errors.
fn run_app() -> Result<(), Error> {
    // Retrieve the home directory or return an error if not found
    let home_dir = home_dir()
        .ok_or_else(|| Error::new(ErrorKind::NotFound, "Could not find home directory"))?;

    // Define paths for the processed image directory and the source image directory
    let processed_image_dir = home_dir.join("Desktop/processed_backgrounds");
    let source_image_dir = home_dir.join("AppData/Local/Packages/Microsoft.Windows.ContentDeliveryManager_cw5n1h2txyewy/LocalState/Assets");

    // Create a decorative string based on terminal size
    let decorator_string = decorator("-");
    println!("\n{}", decorator_string);
    println!("\t... working ...");

    // Prepare the directory for processed images
    prepare_dir(&processed_image_dir)?;
    println!("\tCopying files ...");

    // Copy files from the source directory to the processed directory
    let copied_files = copy_files(&processed_image_dir, &source_image_dir)?;
    println!("\tDone, {} files copied.", copied_files);

    // Print the decorative string again
    println!("{}\n", decorator_string);
    Ok(())
}

/// Prepares a directory for storing processed images.
/// If the directory exists, it is deleted and recreated.
fn prepare_dir(output_dir: &Path) -> Result<(), Error> {
    // Check if the directory already exists
    if output_dir.exists() {
        println!("A previous directory exists, deleting ...");
        fs::remove_dir_all(output_dir)?;
    }

    // Create a new directory
    fs::create_dir(output_dir).map_err(|e| {
        Error::new(
            ErrorKind::Other,
            format!("Could not create directory: {}.", e),
        )
    })
}

/// Copies image files from the source directory to the processed directory.
/// Renames the files during the copying process.
fn copy_files(processed_dir: &Path, source_dir: &Path) -> Result<u16, Error> {
    let mut files_copied: u16 = 0;

    // Read the source directory and handle any errors
    let source_dir_iter = source_dir.read_dir().map_err(|e| {
        Error::new(
            ErrorKind::Other,
            format!("Could not read source directory: {}.", e),
        )
    })?;

    // Iterate over the entries in the source directory
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

        // Copy the file from the source path to the new path
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

/// Creates a decorative string based on the terminal size.
/// Uses the given symbol to create a line that spans the width of the terminal.
fn decorator(symbol: &str) -> String {
    // Try to get the terminal width, default to 60 if unable to determine
    let width = term_size::dimensions().map(|(w, _)| w).unwrap_or(60);
    symbol.repeat(width)
}

// Rest of the code (if any) goes here
