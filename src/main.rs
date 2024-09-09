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

fn run_app() -> Result<(), Error> {
    let home_dir = home_dir()
        .ok_or_else(|| Error::new(ErrorKind::NotFound, "Could not find home directory"))?;

    let processed_image_dir = home_dir.join("Desktop/processed_backgrounds");
    let source_image_dir = home_dir.join("AppData/Local/Packages/Microsoft.Windows.ContentDeliveryManager_cw5n1h2txyewy/LocalState/Assets");

    let decorator_string = decorator("-");
    println!("\n{}", decorator_string);
    println!("\t... working ...");

    prepare_dir(&processed_image_dir)?;
    println!("\tCopying files ...");

    let copied_files = copy_files(&processed_image_dir, &source_image_dir)?;
    println!("\tDone, {} files copied.", copied_files);

    println!("{}\n", decorator_string);
    Ok(())
}