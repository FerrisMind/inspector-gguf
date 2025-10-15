use thiserror::Error;
use crate::localization::Language;

#[derive(Debug, Error)]
pub enum LocalizationError {
    #[error("Translation file not found for language: {0:?}")]
    TranslationNotFound(Language),
    
    #[error("Invalid translation file format: {0}")]
    InvalidFormat(String),
    
    #[error("Translation key not found: {0}")]
    KeyNotFound(String),
    
    #[error("Settings error: {0}")]
    Settings(#[from] SettingsError),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON parsing error: {0}")]
    JsonParsing(#[from] serde_json::Error),
}

#[derive(Debug, Error)]
pub enum SettingsError {
    #[error("Failed to create settings directory")]
    DirectoryCreation,
    
    #[error("Failed to read settings file")]
    ReadError,
    
    #[error("Failed to write settings file")]
    WriteError,
    
    #[error("Invalid settings format")]
    InvalidFormat,
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON parsing error: {0}")]
    JsonParsing(#[from] serde_json::Error),
}