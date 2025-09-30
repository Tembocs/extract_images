use std::fmt;
use std::io;

/// Custom error types for the application
#[derive(Debug)]
pub enum ExtractError {
    /// I/O related errors
    Io(io::Error),
    /// Configuration errors
    Config(String),
    /// Source directory not found
    SourceNotFound(String),
    /// No images found in source
    NoImagesFound,
}

impl fmt::Display for ExtractError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExtractError::Io(err) => write!(f, "I/O error: {}", err),
            ExtractError::Config(msg) => write!(f, "Configuration error: {}", msg),
            ExtractError::SourceNotFound(path) => {
                write!(f, "Source directory not found: '{}'. This may indicate that Windows Content Delivery Manager is not enabled or you're not on Windows 10/11.", path)
            }
            ExtractError::NoImagesFound => write!(f, "No suitable image files found in source directory"),
        }
    }
}

impl std::error::Error for ExtractError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ExtractError::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for ExtractError {
    fn from(err: io::Error) -> Self {
        ExtractError::Io(err)
    }
}

pub type Result<T> = std::result::Result<T, ExtractError>;