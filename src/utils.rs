/// Creates a decorative line by repeating the given symbol to fit the terminal width.
///
/// # Arguments
///
/// * `symbol` - A string slice that holds the symbol to be repeated.
///
/// # Returns
///
/// * `String` - A string containing the repeated symbol, adjusted to the terminal width.
///
/// # Examples
///
/// ```
/// use extract_images::utils::decorator;
/// let line = decorator("=");
/// assert!(!line.is_empty());
/// ```
pub fn decorator(symbol: &str) -> String {
    let width = term_size::dimensions().map(|(w, _)| w).unwrap_or(60);
    symbol.repeat(width)
}

/// Formats a file size in bytes to a human-readable string.
///
/// # Arguments
///
/// * `bytes` - The number of bytes to format.
///
/// # Returns
///
/// * `String` - A formatted string representing the file size.
///
/// # Examples
///
/// ```
/// use extract_images::utils::format_file_size;
/// assert_eq!(format_file_size(1024), "1.0 KB");
/// assert_eq!(format_file_size(1048576), "1.0 MB");
/// ```
pub fn format_file_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.1} {}", size, UNITS[unit_index])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decorator_not_empty() {
        let result = decorator("-");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_format_file_size() {
        assert_eq!(format_file_size(0), "0.0 B");
        assert_eq!(format_file_size(512), "512.0 B");
        assert_eq!(format_file_size(1024), "1.0 KB");
        assert_eq!(format_file_size(1048576), "1.0 MB");
        assert_eq!(format_file_size(1073741824), "1.0 GB");
    }
}