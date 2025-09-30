use extract_images::config::Config;
use extract_images::file_ops::{copy_files, prepare_dir};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_full_workflow() {
    let temp_dir = tempdir().unwrap();
    let source_dir = temp_dir.path().join("source");
    let dest_dir = temp_dir.path().join("dest");

    // Setup
    fs::create_dir(&source_dir).unwrap();
    
    // Create test files with different sizes
    fs::write(source_dir.join("small.jpg"), vec![0u8; 50_000]).unwrap(); // Too small
    fs::write(source_dir.join("large.jpg"), vec![0u8; 200_000]).unwrap(); // Valid size
    fs::write(source_dir.join("another.png"), vec![0u8; 150_000]).unwrap(); // Valid size

    let config = Config {
        output: None,
        min_size_kb: 100,
        prefix: "wallpaper".to_string(),
        exclude_extensions: None,
        verbose: false,
        dry_run: false,
    };

    // Test prepare_dir
    assert!(prepare_dir(&dest_dir).is_ok());
    assert!(dest_dir.exists());

    // Test copy_files
    let result = copy_files(&dest_dir, &source_dir, &config).unwrap();
    assert_eq!(result, 2); // Should copy 2 files (large.jpg and another.png)

    // Verify files were copied (order may vary due to filesystem iteration)
    let copied_files: Vec<_> = fs::read_dir(&dest_dir)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().to_string())
        .collect();
    
    assert_eq!(copied_files.len(), 2);
    assert!(copied_files.iter().any(|name| name.starts_with("wallpaper_") && name.ends_with(".jpg")));
    assert!(copied_files.iter().any(|name| name.starts_with("wallpaper_") && name.ends_with(".png")));
}

#[test]
fn test_dry_run_mode() {
    let temp_dir = tempdir().unwrap();
    let source_dir = temp_dir.path().join("source");
    let dest_dir = temp_dir.path().join("dest");

    // Setup
    fs::create_dir(&source_dir).unwrap();
    fs::write(source_dir.join("test.jpg"), vec![0u8; 200_000]).unwrap();

    let config = Config {
        output: None,
        min_size_kb: 100,
        prefix: "image".to_string(),
        exclude_extensions: None,
        verbose: true,
        dry_run: true,
    };

    prepare_dir(&dest_dir).unwrap();
    let result = copy_files(&dest_dir, &source_dir, &config).unwrap();
    
    // Should report 1 file processed but not actually copy it
    assert_eq!(result, 1);
    assert!(!dest_dir.join("image_1.jpg").exists());
}

#[test]
fn test_extension_filtering() {
    let temp_dir = tempdir().unwrap();
    let source_dir = temp_dir.path().join("source");
    let dest_dir = temp_dir.path().join("dest");

    // Setup
    fs::create_dir(&source_dir).unwrap();
    fs::write(source_dir.join("good.jpg"), vec![0u8; 200_000]).unwrap();
    fs::write(source_dir.join("bad.ico"), vec![0u8; 200_000]).unwrap();

    let config = Config {
        output: None,
        min_size_kb: 100,
        prefix: "image".to_string(),
        exclude_extensions: Some("ico,bmp".to_string()),
        verbose: false,
        dry_run: false,
    };

    prepare_dir(&dest_dir).unwrap();
    let result = copy_files(&dest_dir, &source_dir, &config).unwrap();
    
    // Should only copy the jpg file, not the ico
    assert_eq!(result, 1);
    assert!(dest_dir.join("image_1.jpg").exists());
}