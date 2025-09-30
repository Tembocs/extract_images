# extract_images

A Rust utility that extracts background images from Windows 10/11 Content Delivery Manager assets and saves them to your Desktop with organized naming.

## Description

Windows 10/11 stores wallpaper and background images in a hidden directory. This tool finds those images, copies them to your Desktop in a new folder called `processed_backgrounds`, and renames them sequentially as `image_1.jpg`, `image_2.jpg`, etc.

## Features

- 🖼️ Extracts Windows background images from system assets
- 📁 Creates organized folder structure on Desktop
- 🏷️ Sequential naming for easy browsing
- 🔄 Overwrites previous extractions (cleans up old files)
- 📊 Shows progress and file count

## Installation

### Prerequisites
- Windows 10 or 11
- Rust toolchain (install from [rustup.rs](https://rustup.rs/))

### Build from source
```bash
git clone https://github.com/Tembocs/extract_images.git
cd extract_images
cargo build --release
```

## Usage

### Basic Usage
Simply run the executable:
```bash
cargo run --release
```

Or if you've built the binary:
```bash
./target/release/extract_images
```

### Command Line Options
```bash
# Custom output directory
extract_images --output "C:\MyWallpapers"

# Set minimum file size (in KB)
extract_images --min-size-kb 200

# Use custom prefix for files
extract_images --prefix "wallpaper"

# Exclude certain file extensions
extract_images --exclude-extensions "ico,bmp"

# Verbose output
extract_images --verbose

# Dry run (see what would be copied without actually copying)
extract_images --dry-run

# Combine options
extract_images --output "D:\Images" --prefix "bg" --min-size-kb 150 --verbose
```

### What it does:
1. Check for existing output folder (default: `Desktop/processed_backgrounds`)
2. Delete it if it exists (to ensure fresh extraction)
3. Create a new output folder
4. Copy and rename all qualifying images from Windows assets directory
5. Display the number of files copied

## Output Location

Images are saved to: `%USERPROFILE%\Desktop\processed_backgrounds\`

## Technical Details

The program extracts images from:
`%USERPROFILE%\AppData\Local\Packages\Microsoft.Windows.ContentDeliveryManager_cw5n1h2txyewy\LocalState\Assets`

## Dependencies

- `dirs` - For cross-platform path handling
- `term_size` - For terminal width detection (decorative output)

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## Troubleshooting

**No images found?**
- Ensure you're running Windows 10/11
- Check if Content Delivery Manager is enabled in Windows settings
- Try running as administrator if permission issues occur

**Permission errors?**
- Run the command prompt/terminal as administrator
- Check that the Desktop is accessible and writable
