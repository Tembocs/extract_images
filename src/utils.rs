use term_size;

pub fn decorator(symbol: &str) -> String {
    let width = term_size::dimensions().map(|(w, _)| w).unwrap_or(60);
    symbol.repeat(width)
}