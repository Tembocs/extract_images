// main.rs
use std::fs;
use std::path::{Path, PathBuf};
use std::env;

fn main() {
    let home_dir = env::home_dir().expect("Could not fine home directory");
    let processed_image_dir = Path::new(home_dir.as_path())
                                .join("Desktop/processed_backgrounds");

    let source_image_dir = Path::new(home_dir.as_path())
                                .join("AppData/Local/Packages/Microsoft.Windows.ContentDeliveryManager_cw5n1h2txyewy/LocalState/Assets");

    let decorator_string: String = String::from("--------------------------------------------");

    println!("\n{}", decorator_string);
    println!("... working ...");
    prepare_dir(&processed_image_dir);
    rename(&processed_image_dir , &source_image_dir);
    println!("{}\n", decorator_string);
}

// TODO, consider a return type Result for error handling
fn prepare_dir(output_dir: &Path) {
    if output_dir.exists() {
        println!("A previous directory exist, deleting ...");

        fs::remove_dir_all(output_dir)
                .expect("could not remove processed image directory");
    }

    std::fs::create_dir(output_dir)
            .expect("could not create processed image directory");
}

// TODO, consider a return type Result for error handling
fn rename(processed_dir: &Path, source_dir: &Path) {
    let mut files_copied: u32 = 0;

    // This will be added at the end of each file name to differentiate one from the other
    let mut number = 1;

    // Get contents of a directory, source image directory
    let source_dir_iter = source_dir.read_dir()
                            .expect("Error: Could not iterate over directory.");

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
}