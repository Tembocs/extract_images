// main.rs
use std::fs;
use std::path::Path;
use std::io::{Error, ErrorKind};
use dirs::home_dir;


/// Starting point.
fn main() -> Result<(), Error> {
    let home_dir = home_dir().expect("Could not find home directory");
    let processed_image_dir = Path::new(home_dir.as_path())
                                .join("Desktop/processed_backgrounds");

    let source_image_dir = Path::new(home_dir.as_path())
                                .join("AppData/Local/Packages/Microsoft.Windows.ContentDeliveryManager_cw5n1h2txyewy/LocalState/Assets");

    let decorator_string = decorator("-", 60);

    println!("\n{}", decorator_string);
    println!("\t... working ...");

    prepare_dir(&processed_image_dir)?;
    println!("\t copying files ...");
    
    let copied_files = copy_files(&processed_image_dir, &source_image_dir)?;

    match copied_files {
        files_count => println!("\tdone, {} files copied.", files_count),

        // TODO does not work as expected!
        // Err(error_sms) => println!("Could not copy files: {}", error_sms)
    }

    println!("{}\n", decorator_string);
    Ok(())
}


/// Check if a previous directory with the same name exists, then delete it.
/// Otherwise create a new one.
fn prepare_dir(output_dir: &Path) -> Result<(), Error> {
    if output_dir.exists() {
        println!("A previous directory exist, deleting ...");

        match fs::remove_dir_all(output_dir) {
            Ok(()) => {
                // Successful deleted the previous directory, create a new one.
                match fs::create_dir(output_dir) {
                    Ok(()) => Ok(()),
                    Err(some_error) => {
                        let input_error = Error::new(ErrorKind::Other,
                                    format!("Could not create directory: {}.", some_error));
                        Err(input_error)
                    }
                }
            },

            // Failed to delete the existing directory
            Err(some_error) => {
                let input_error = Error::new(ErrorKind::Other,
                            format!("Could not remove directory: {}.", some_error));
                Err(input_error)
            }
        }
    } else {
        match fs::create_dir(output_dir) {
            Ok(()) => Ok(()),
            Err(some_error) => {
                let input_error = Error::new(ErrorKind::Other,
                            format!("Could not create directory: {}.", some_error));
                Err(input_error)
            }
        }
    }
}


/// Copy images files and rename them.
fn copy_files(processed_dir: &Path, source_dir: &Path) -> Result<u16, Error> {
    let mut files_copied: u16 = 0;

    // Get contents of a directory, source image directory
    let source_dir_iter = match source_dir.read_dir() {
        Ok(dir_iterator) => dir_iterator,
        Err(some_error) => {
            let read_error = Error::new(ErrorKind::Other,
                        format!("Could not create directory iterator. {}.", some_error));

            // Use return statement when the two match results have incompatible types.
            return Err(read_error);
        }
    };

    // This is added at the end of each file name to differentiate one from another.
    let mut number: u8 = 1;

    for entry in source_dir_iter {
        let new_entry = match entry {
            Ok(an_entry) => an_entry,
            Err(entry_error) => {
                let new_entry_error = Error::new(ErrorKind::Other,
                            format!("Could not get directory entry. {}.", entry_error));
                return Err(new_entry_error);
            }
        };

        // Set a new file name and its path.
        let name = new_entry.file_name()
                    .into_string().expect("failed conversion");

        let new_name = format!("image_{}.jpg", number);
        let new_path = processed_dir.join(new_name);
        let source_path = source_dir.join(name);
        number += 1;

        // Do the copying using the new file name and path.
        match fs::copy(&source_path, &new_path) {
            Ok(_) => {
                files_copied += 1;
            },
            Err(_) => println!("could not copy '{:?}'", source_path)
        }
    }

    Ok(files_copied)
}

/// Create a string for decoration.
fn decorator(symbol: &str, times: u8) -> String {
    let mut deco = String::from("");

    for _ in 0..times {
        deco += &symbol;
    }

    return deco;
}