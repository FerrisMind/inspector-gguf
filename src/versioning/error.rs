use thiserror::Error;

#[derive(Error, Debug)]
pub enum VersioningError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Cargo.toml not found at path: {0}")]
    CargoTomlNotFound(String),
    
    #[error("Version line not found in Cargo.toml")]
    VersionLineNotFound,
    
    #[error("Invalid version format: {0}")]
    InvalidVersionFormat(String),
    
    #[error("Failed to parse version: {0}")]
    VersionParseError(String),
}