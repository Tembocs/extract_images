use term_size;

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
/// # Panics
///
/// This function will panic if the terminal width cannot be determined and the default width is used.
pub fn decorator(symbol: &str) -> String {
    let width = term_size::dimensions().map(|(w, _)| w).unwrap_or(60);
    symbol.repeat(width)
}