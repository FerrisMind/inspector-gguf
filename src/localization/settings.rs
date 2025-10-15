use std::path::{Path, PathBuf};
use std::fs;
use serde::{Deserialize, Serialize};
use crate::localization::{Language, SettingsError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub language: Language,
    pub version: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            language: Language::English,
            version: "1.0".to_string(),
        }
    }
}

pub struct SettingsManager {
    settings_path: PathBuf,
}

impl SettingsManager {
    /// Create a new SettingsManager with platform-appropriate settings directory
    pub fn new() -> Result<Self, SettingsError> {
        let settings_path = Self::get_platform_settings_path()?;
        let manager = SettingsManager { settings_path };
        
        // Ensure the settings directory exists and is writable
        manager.ensure_settings_directory()?;
        
        Ok(manager)
    }

    /// Load language preference from settings file with error recovery
    pub fn load_language_preference(&self) -> Option<Language> {
        match self.load_settings() {
            Ok(settings) => Some(settings.language),
            Err(_) => {
                // If loading fails, try to reset to defaults
                if self.reset_to_defaults().is_ok() {
                    Some(Language::English)
                } else {
                    None
                }
            }
        }
    }

    /// Save language preference to settings file
    pub fn save_language_preference(&self, language: Language) -> Result<(), SettingsError> {
        let mut settings = self.load_settings().unwrap_or_default();
        settings.language = language;
        self.save_settings(&settings)
    }

    /// Get the path to the settings file
    pub fn get_settings_path(&self) -> &Path {
        &self.settings_path
    }

    /// Load complete settings from file with error recovery
    pub fn load_settings(&self) -> Result<AppSettings, SettingsError> {
        if !self.settings_path.exists() {
            // Create default settings file if it doesn't exist
            let default_settings = AppSettings::default();
            if self.save_settings(&default_settings).is_err() {
                // If we can't save, just return default settings
                return Ok(default_settings);
            }
            return Ok(default_settings);
        }

        match fs::read_to_string(&self.settings_path) {
            Ok(content) => {
                match serde_json::from_str::<AppSettings>(&content) {
                    Ok(settings) => Ok(settings),
                    Err(_) => {
                        // Settings file is corrupted, create backup and use defaults
                        self.backup_corrupted_settings()?;
                        let default_settings = AppSettings::default();
                        self.save_settings(&default_settings)?;
                        Ok(default_settings)
                    }
                }
            }
            Err(_) => {
                // Can't read file, return defaults
                Ok(AppSettings::default())
            }
        }
    }

    /// Save complete settings to file with atomic write
    pub fn save_settings(&self, settings: &AppSettings) -> Result<(), SettingsError> {
        // Ensure parent directory exists
        if let Some(parent) = self.settings_path.parent() {
            fs::create_dir_all(parent).map_err(|_| SettingsError::DirectoryCreation)?;
        }

        let content = serde_json::to_string_pretty(settings)
            .map_err(|_| SettingsError::InvalidFormat)?;
        
        // Use atomic write: write to temporary file first, then rename
        let temp_path = self.settings_path.with_extension("tmp");
        
        fs::write(&temp_path, &content)
            .map_err(|_| SettingsError::WriteError)?;
        
        fs::rename(&temp_path, &self.settings_path)
            .map_err(|_| SettingsError::WriteError)?;
        
        Ok(())
    }

    /// Backup corrupted settings file
    fn backup_corrupted_settings(&self) -> Result<(), SettingsError> {
        if self.settings_path.exists() {
            let backup_path = self.settings_path.with_extension("backup");
            fs::copy(&self.settings_path, &backup_path)
                .map_err(|_| SettingsError::WriteError)?;
        }
        Ok(())
    }

    /// Validate settings directory permissions and create if necessary
    pub fn ensure_settings_directory(&self) -> Result<(), SettingsError> {
        if let Some(parent) = self.settings_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).map_err(|_| SettingsError::DirectoryCreation)?;
            }
            
            // Test write permissions by creating a temporary file
            let test_file = parent.join(".write_test");
            match fs::write(&test_file, "test") {
                Ok(_) => {
                    let _ = fs::remove_file(&test_file); // Clean up test file
                    Ok(())
                }
                Err(_) => Err(SettingsError::DirectoryCreation),
            }
        } else {
            Err(SettingsError::DirectoryCreation)
        }
    }

    /// Reset settings to default values
    pub fn reset_to_defaults(&self) -> Result<(), SettingsError> {
        let default_settings = AppSettings::default();
        self.save_settings(&default_settings)
    }

    /// Check if settings file exists and is readable
    pub fn is_settings_file_valid(&self) -> bool {
        if !self.settings_path.exists() {
            return false;
        }
        
        match fs::read_to_string(&self.settings_path) {
            Ok(content) => serde_json::from_str::<AppSettings>(&content).is_ok(),
            Err(_) => false,
        }
    }

    /// Get platform-appropriate settings directory path
    fn get_platform_settings_path() -> Result<PathBuf, SettingsError> {
        let settings_dir = if cfg!(target_os = "windows") {
            // Windows: %APPDATA%\InspectorGGUF\settings.json
            std::env::var("APPDATA")
                .map(PathBuf::from)
                .map_err(|_| SettingsError::DirectoryCreation)?
                .join("InspectorGGUF")
        } else if cfg!(target_os = "macos") {
            // macOS: ~/Library/Application Support/InspectorGGUF/settings.json
            std::env::var("HOME")
                .map(PathBuf::from)
                .map_err(|_| SettingsError::DirectoryCreation)?
                .join("Library")
                .join("Application Support")
                .join("InspectorGGUF")
        } else {
            // Linux/Unix: ~/.config/inspector-gguf/settings.json
            std::env::var("HOME")
                .map(PathBuf::from)
                .map_err(|_| SettingsError::DirectoryCreation)?
                .join(".config")
                .join("inspector-gguf")
        };

        Ok(settings_dir.join("settings.json"))
    }
}

impl Default for SettingsManager {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| SettingsManager {
            settings_path: PathBuf::from("settings.json"),
        })
    }
}