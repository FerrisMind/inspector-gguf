use std::path::{Path, PathBuf};
use std::fs;
use serde::{Deserialize, Serialize};
use crate::localization::{Language, SettingsError};

/// Application settings structure for persistent storage.
///
/// This structure represents the complete application settings that are
/// persisted to disk. It includes user preferences and application state
/// that should be restored between sessions.
///
/// # Fields
///
/// - `language` - User's preferred interface language
/// - `version` - Application version (for settings migration)
///
/// # Serialization
///
/// The structure is serialized to JSON format for human-readable storage
/// and easy debugging. Example JSON output:
///
/// ```json
/// {
///   "language": "Russian",
///   "version": "1.0"
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    /// User's preferred interface language.
    pub language: Language,
    /// Application version for settings migration tracking.
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

/// Manages persistent storage of application settings across sessions.
///
/// The `SettingsManager` handles reading, writing, and validating application
/// settings stored in platform-appropriate locations. It provides atomic writes,
/// error recovery, and automatic directory creation.
///
/// # Storage Locations
///
/// Settings are stored in platform-specific directories:
/// - **Windows**: `%APPDATA%\InspectorGGUF\settings.json`
/// - **macOS**: `~/Library/Application Support/InspectorGGUF/settings.json`
/// - **Linux**: `~/.config/inspector-gguf/settings.json`
///
/// # Features
///
/// - **Atomic Writes**: Uses temporary files to prevent corruption
/// - **Error Recovery**: Handles corrupted files with backup and reset
/// - **Directory Management**: Automatically creates required directories
/// - **Permission Validation**: Checks write permissions before operations
/// - **Backup System**: Creates backups of corrupted settings files
///
/// # Examples
///
/// ## Basic Usage
///
/// ```rust
/// use inspector_gguf::localization::{SettingsManager, Language};
///
/// // Create settings manager
/// let settings_manager = SettingsManager::new()?;
///
/// // Save language preference
/// settings_manager.save_language_preference(Language::Russian)?;
///
/// // Load language preference
/// if let Some(language) = settings_manager.load_language_preference() {
///     println!("Saved language: {:?}", language);
/// }
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// ## Complete Settings Management
///
/// ```rust
/// use inspector_gguf::localization::{SettingsManager, AppSettings, Language};
///
/// let settings_manager = SettingsManager::new()?;
///
/// // Load complete settings
/// let mut settings = settings_manager.load_settings()?;
/// settings.language = Language::PortugueseBrazilian;
/// settings.version = "2.0".to_string();
///
/// // Save complete settings
/// settings_manager.save_settings(&settings)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// ## Error Recovery
///
/// ```rust
/// use inspector_gguf::localization::SettingsManager;
///
/// let settings_manager = SettingsManager::new()?;
///
/// // Reset to defaults if settings are corrupted
/// if !settings_manager.is_settings_file_valid() {
///     println!("Settings file is corrupted, resetting to defaults");
///     settings_manager.reset_to_defaults()?;
/// }
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub struct SettingsManager {
    settings_path: PathBuf,
}

impl SettingsManager {
    /// Creates a new SettingsManager with platform-appropriate settings directory.
    ///
    /// This constructor determines the correct settings directory for the current
    /// platform, creates the directory if it doesn't exist, and validates write
    /// permissions to ensure settings can be saved.
    ///
    /// # Platform Directories
    ///
    /// - **Windows**: `%APPDATA%\InspectorGGUF\settings.json`
    /// - **macOS**: `~/Library/Application Support/InspectorGGUF/settings.json`
    /// - **Linux**: `~/.config/inspector-gguf/settings.json`
    ///
    /// # Returns
    ///
    /// Returns a configured `SettingsManager` ready for use, or a `SettingsError`
    /// if the settings directory cannot be created or accessed.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The settings directory cannot be created
    /// - Write permissions are not available
    /// - Platform-specific directory detection fails
    ///
    /// # Examples
    ///
    /// ```rust
    /// use inspector_gguf::localization::SettingsManager;
    ///
    /// let settings_manager = SettingsManager::new()?;
    /// println!("Settings path: {:?}", settings_manager.get_settings_path());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new() -> Result<Self, SettingsError> {
        let settings_path = Self::get_platform_settings_path()?;
        let manager = SettingsManager { settings_path };
        
        // Ensure the settings directory exists and is writable
        manager.ensure_settings_directory()?;
        
        Ok(manager)
    }

    /// Loads the user's language preference from the settings file with error recovery.
    ///
    /// This method attempts to load the saved language preference from the settings
    /// file. If the file is corrupted or missing, it attempts to reset to defaults
    /// and return English as a fallback.
    ///
    /// # Returns
    ///
    /// Returns `Some(Language)` if a preference is found or defaults are successfully
    /// created, or `None` if all recovery attempts fail.
    ///
    /// # Error Recovery
    ///
    /// If loading fails, the method attempts to:
    /// 1. Reset settings to defaults
    /// 2. Return English as the fallback language
    /// 3. Return `None` only if all recovery attempts fail
    ///
    /// # Examples
    ///
    /// ```rust
    /// use inspector_gguf::localization::{SettingsManager, Language};
    ///
    /// let settings_manager = SettingsManager::new()?;
    ///
    /// match settings_manager.load_language_preference() {
    ///     Some(language) => {
    ///         println!("User prefers: {:?}", language);
    ///     }
    ///     None => {
    ///         println!("No language preference available, using system default");
    ///     }
    /// }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
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

    /// Saves the user's language preference to the settings file.
    ///
    /// This method updates the language preference in the settings file while
    /// preserving other settings. It uses atomic writes to prevent corruption
    /// and creates the settings file if it doesn't exist.
    ///
    /// # Arguments
    ///
    /// * `language` - The language preference to save
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on successful save, or a `SettingsError` if the operation fails.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The settings directory is not writable
    /// - The settings file cannot be created or updated
    /// - JSON serialization fails
    /// - Atomic write operation fails
    ///
    /// # Examples
    ///
    /// ```rust
    /// use inspector_gguf::localization::{SettingsManager, Language};
    ///
    /// let settings_manager = SettingsManager::new()?;
    ///
    /// // Save user's language preference
    /// settings_manager.save_language_preference(Language::Russian)?;
    /// println!("Language preference saved successfully");
    ///
    /// // Verify it was saved
    /// let loaded = settings_manager.load_language_preference();
    /// assert_eq!(loaded, Some(Language::Russian));
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn save_language_preference(&self, language: Language) -> Result<(), SettingsError> {
        let mut settings = self.load_settings().unwrap_or_default();
        settings.language = language;
        self.save_settings(&settings)
    }

    /// Returns the path to the settings file.
    ///
    /// This method provides access to the full path where settings are stored,
    /// which can be useful for debugging, backup operations, or manual file management.
    ///
    /// # Returns
    ///
    /// A reference to the `Path` where the settings file is located.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use inspector_gguf::localization::SettingsManager;
    ///
    /// let settings_manager = SettingsManager::new()?;
    /// let path = settings_manager.get_settings_path();
    /// println!("Settings stored at: {}", path.display());
    ///
    /// // Check if settings file exists
    /// if path.exists() {
    ///     println!("Settings file found");
    /// } else {
    ///     println!("Settings file will be created on first save");
    /// }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
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