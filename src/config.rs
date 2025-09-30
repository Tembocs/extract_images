use clap::Parser;
use std::path::PathBuf;

/// Windows background image extractor
#[derive(Parser, Debug)]
#[command(name = "extract_images")]
#[command(about = "Extract Windows background images from Content Delivery Manager assets")]
#[command(version)]
pub struct Config {
    /// Output directory (default: ~/Desktop/processed_backgrounds)
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Minimum file size in KB to consider as wallpaper (default: 100)
    #[arg(short = 's', long, default_value = "100")]
    pub min_size_kb: u64,

    /// Custom prefix for renamed files (default: "image")
    #[arg(short, long, default_value = "image")]
    pub prefix: String,

    /// Skip files with these extensions (comma-separated)
    #[arg(short = 'e', long)]
    pub exclude_extensions: Option<String>,

    /// Verbose output
    #[arg(short, long)]
    pub verbose: bool,

    /// Dry run - don't actually copy files
    #[arg(short = 'n', long)]
    pub dry_run: bool,
}

impl Config {
    pub fn min_size_bytes(&self) -> u64 {
        self.min_size_kb * 1024
    }

    pub fn excluded_extensions(&self) -> Vec<String> {
        self.exclude_extensions
            .as_ref()
            .map(|s| s.split(',').map(|ext| ext.trim().to_lowercase()).collect())
            .unwrap_or_default()
    }
}