// main.rs
use std::fs;
use std::path::{Path, PathBuf};
use std::env;
use std::io::{Error, ErrorKind};

fn main() -> Result<(), Error> {
    let home_dir = env::home_dir().expect("Could not fine home directory");
    let processed_image_dir = Path::new(home_dir.as_path())
                                .join("Desktop/processed_backgrounds");

    let source_image_dir = Path::new(home_dir.as_path())
                                .join("AppData/Local/Packages/Microsoft.Windows.ContentDeliveryManager_cw5n1h2txyewy/LocalState/Assets");

    let decorator_string: String = String::from("--------------------------------------------");

    println!("\n{}", decorator_string);
    println!("... working ...");

    prepare_dir(&processed_image_dir)?;
    rename(&processed_image_dir, &source_image_dir)?;

    println!("{}\n", decorator_string);
    Ok(())
}

fn prepare_dir(output_dir: &Path) -> Result<(), Error> {
    if output_dir.exists() {
        println!("A previous directory exist, deleting ...");

        match fs::remove_dir(output_dir) {
            Ok(()) => Ok(()),
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
                            format!("Could not remove directory: {}.", some_error));
                Err(input_error)
            }
        }
    }
}

// TODO, adding more Result return in some parts of this function.
fn rename(processed_dir: &Path, source_dir: &Path) -> Result<(), Error> {
    let mut files_copied: u32 = 0;

    // This will be added at the end of each file name to differentiate one from the other
    let mut number = 1;

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

    for entry in source_dir_iter {
        let new_entry = entry.expect("Error: error in DirEntry analysis.");
        let name = new_entry.file_name()
                    .into_string().expect("failed conversion");

        let new_name = format!("image_{}.jpg", number);
        let new_path = processed_dir.join(new_name);
        let source_path = source_dir.join(name);
        number += 1;

        match fs::copy(&source_path, &new_path) {
            Ok(_) => {
                files_copied += 1;
            },
            Err(_) => println!("could not copy '{:?}'", source_path)
        }
    }

    println!("done, {} files copied.", files_copied);
    Ok(())
}