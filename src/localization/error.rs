use thiserror::Error;
use crate::localization::Language;

/// Errors that can occur during localization operations
#[derive(Debug, Error)]
pub enum LocalizationError {
    /// Translation file not found for the specified language
    #[error("Translation file not found for language: {0:?}")]
    TranslationNotFound(Language),
    
    /// Invalid translation file format or structure
    #[error("Invalid translation file format: {0}")]
    InvalidFormat(String),
    
    /// Requested translation key not found in the translation data
    #[error("Translation key not found: {0}")]
    KeyNotFound(String),
    
    /// Error occurred in settings management
    #[error("Settings error: {0}")]
    Settings(#[from] SettingsError),
    
    /// Input/output error during file operations
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    /// JSON parsing or serialization error
    #[error("JSON parsing error: {0}")]
    JsonParsing(#[from] serde_json::Error),
}

/// Errors that can occur during settings management operations
#[derive(Debug, Error)]
pub enum SettingsError {
    /// Failed to create the settings directory
    #[error("Failed to create settings directory")]
    DirectoryCreation,
    
    /// Failed to read the settings file
    #[error("Failed to read settings file")]
    ReadError,
    
    /// Failed to write to the settings file
    #[error("Failed to write settings file")]
    WriteError,
    
    /// Settings file has invalid format or structure
    #[error("Invalid settings format")]
    InvalidFormat,
    
    /// Input/output error during file operations
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    /// JSON parsing or serialization error
    #[error("JSON parsing error: {0}")]
    JsonParsing(#[from] serde_json::Error),
}