mod utils;
mod file_ops;

use dirs::home_dir;
use std::io::{Error, ErrorKind};
use utils::decorator;
use file_ops::{prepare_dir, copy_files};

fn main() {
    match run_app() {
        Ok(()) => println!("Application completed successfully."),
        Err(e) => eprintln!("Application error: {}", e),
    }
}

/// Runs the main application logic.
///
/// This function performs the following steps:
/// 1. Retrieves the home directory of the current user.
/// 2. Constructs paths for the processed image directory and the source image directory.
/// 3. Validates that the source directory exists.
/// 4. Prints a decorator string and a working message.
/// 5. Prepares the processed image directory by creating it if it doesn't exist.
/// 6. Copies files from the source image directory to the processed image directory.
/// 7. Prints the number of files copied and a completion message.
///
/// # Returns
/// 
/// * `Ok(())` if the function executes successfully.
/// * `Err(Error)` if an error occurs during execution.
///
/// # Errors
///
/// This function will return an error if:
/// - The home directory cannot be found.
/// - The source directory doesn't exist.
/// - There is an issue creating the processed image directory.
/// - There is an issue copying the files.
///
/// # Examples
///
/// ```
/// if let Err(e) = run_app() {
///     eprintln!("Application error: {}", e);
/// }
/// ```
fn run_app() -> Result<(), Error> {
    let home_dir = home_dir()
        .ok_or_else(|| Error::new(ErrorKind::NotFound, "Could not find home directory"))?;

    let processed_image_dir = home_dir.join("Desktop/processed_backgrounds");
    let source_image_dir = home_dir.join("AppData/Local/Packages/Microsoft.Windows.ContentDeliveryManager_cw5n1h2txyewy/LocalState/Assets");

    // Check if source directory exists
    if !source_image_dir.exists() {
        return Err(Error::new(
            ErrorKind::NotFound, 
            format!("Source directory not found: '{}'. This may indicate that Windows Content Delivery Manager is not enabled or you're not on Windows 10/11.", source_image_dir.display())
        ));
    }

    let decorator_string = decorator("-");
    println!("\n{}", decorator_string);
    println!("\t🖼️  Extracting Windows background images...");

    prepare_dir(&processed_image_dir)?;
    println!("\t📁 Created output directory: {}", processed_image_dir.display());
    println!("\t🔄 Copying files...");

    let copied_files = copy_files(&processed_image_dir, &source_image_dir)?;
    
    if copied_files == 0 {
        println!("\t⚠️  No suitable image files found in source directory.");
    } else {
        println!("\t✅ Done! {} image(s) copied to Desktop/processed_backgrounds", copied_files);
    }

    println!("{}\n", decorator_string);
    Ok(())
}