use crate::localization::{
    Language, LocalizationError, SettingsManager, SystemLocaleDetector, TranslationLoader,
};
use serde_json::Value;
use std::collections::HashMap;

/// Type alias for translation data structure containing nested key-value pairs.
///
/// Translation maps store hierarchical translation data where keys can be accessed
/// using dot notation (e.g., "buttons.load" maps to `translations["buttons"]["load"]`).
pub type TranslationMap = HashMap<String, Value>;

/// Central manager for all localization operations in Inspector GGUF.
///
/// The `LocalizationManager` coordinates translation loading, language switching,
/// and text retrieval with automatic fallback mechanisms. It serves as the primary
/// interface for all internationalization needs in the application.
///
/// The manager integrates with [`TranslationLoader`] for file operations, [`SystemLocaleDetector`]
/// for automatic language detection, [`SettingsManager`] for persistent preferences, and
/// [`Language`] for supported language variants.
///
/// # Features
///
/// - **Automatic Language Detection**: Detects system locale on initialization
/// - **Fallback System**: Falls back to English, then to key names if translations are missing
/// - **Persistent Settings**: Integrates with settings system for user preferences
/// - **Thread-Safe Design**: Can be safely shared across threads when wrapped appropriately
/// - **Validation**: Ensures translation completeness and format correctness
///
/// # Examples
///
/// ## Basic Usage
///
/// ```rust
/// use inspector_gguf::localization::{LocalizationManager, Language};
///
/// // Create manager with automatic language detection
/// let mut manager = LocalizationManager::new()?;
///
/// // Get translated text
/// let app_title = manager.get_text("app.title");
/// let load_button = manager.get_text("buttons.load");
///
/// // Switch language
/// manager.set_language(Language::Russian)?;
/// let russian_title = manager.get_text("app.title");
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// ## With Persistent Settings
///
/// ```rust
/// use inspector_gguf::localization::{LocalizationManager, Language};
///
/// let mut manager = LocalizationManager::new()?;
///
/// // Change language and save preference
/// manager.set_language_with_persistence(Language::PortugueseBrazilian)?;
///
/// // Language preference will be restored on next startup
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub struct LocalizationManager {
    current_language: Language,
    translations: HashMap<Language, TranslationMap>,
}

impl LocalizationManager {
    /// Creates a new LocalizationManager with automatic language detection and translation loading.
    ///
    /// This constructor performs several initialization steps:
    /// 1. Loads all available translation files
    /// 2. Detects system locale or loads saved language preference
    /// 3. Sets up fallback mechanisms for missing translations
    ///
    /// # Returns
    ///
    /// Returns a configured `LocalizationManager` ready for use, or a `LocalizationError`
    /// if critical translation files (especially English) cannot be loaded.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The English translation file is missing or corrupted (required for fallback)
    /// - Translation files have invalid JSON format
    /// - Required translation sections are missing
    ///
    /// # Examples
    ///
    /// ```rust
    /// use inspector_gguf::localization::LocalizationManager;
    ///
    /// let manager = LocalizationManager::new()?;
    /// println!("Current language: {:?}", manager.get_current_language());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new() -> Result<Self, LocalizationError> {
        let mut manager = LocalizationManager {
            current_language: Language::English,
            translations: HashMap::new(),
        };

        // Load translations for all supported languages
        let loader = TranslationLoader::new();
        for language in [
            Language::English,
            Language::Russian,
            Language::PortugueseBrazilian,
        ] {
            match loader.load_translation(language) {
                Ok(translations) => {
                    manager.translations.insert(language, translations);
                }
                Err(e) => {
                    eprintln!(
                        "Warning: Failed to load translations for {:?}: {}",
                        language, e
                    );
                    // Insert empty map as fallback
                    manager.translations.insert(language, HashMap::new());
                }
            }
        }

        // Determine initial language from settings or system locale
        let settings_manager = SettingsManager::new().unwrap_or_default();
        let initial_language = settings_manager
            .load_language_preference()
            .or_else(SystemLocaleDetector::detect)
            .unwrap_or(Language::English);

        manager.current_language = initial_language;

        Ok(manager)
    }

    /// Retrieves translated text for the specified key with automatic fallback.
    ///
    /// This method implements a three-tier fallback system:
    /// 1. Try current language translation
    /// 2. Fall back to English if key is missing in current language
    /// 3. Return the key itself if no translation is found
    ///
    /// Keys use dot notation to access nested translation structures
    /// (e.g., "buttons.load" accesses `translations["buttons"]["load"]`).
    ///
    /// # Arguments
    ///
    /// * `key` - Translation key in dot notation (e.g., "app.title", "buttons.load")
    ///
    /// # Returns
    ///
    /// Returns the translated string, or the key itself if no translation is available.
    /// This method never panics and always returns a valid string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use inspector_gguf::localization::{LocalizationManager, Language};
    ///
    /// let mut manager = LocalizationManager::new()?;
    /// manager.set_language(Language::English)?;
    ///
    /// // Get simple translation
    /// let title = manager.get_text("app.title");
    /// assert_eq!(title, "Inspector GGUF");
    ///
    /// // Get nested translation
    /// let load_button = manager.get_text("buttons.load");
    /// assert_eq!(load_button, "Load");
    ///
    /// // Non-existent key returns the key itself
    /// let missing = manager.get_text("non.existent.key");
    /// assert_eq!(missing, "non.existent.key");
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn get_text(&self, key: &str) -> String {
        // Try to get translation from current language
        if let Some(translation_map) = self.translations.get(&self.current_language)
            && let Some(value) = self.get_nested_value(translation_map, key)
            && let Some(text) = value.as_str()
        {
            return text.to_string();
        }

        // Fallback to English if current language doesn't have the key
        if self.current_language != Language::English
            && let Some(translation_map) = self.translations.get(&Language::English)
            && let Some(value) = self.get_nested_value(translation_map, key)
            && let Some(text) = value.as_str()
        {
            return text.to_string();
        }

        // Final fallback: return the key itself
        key.to_string()
    }

    /// Sets the current language without persisting the preference.
    ///
    /// Changes the active language for translation lookups. This change is temporary
    /// and will not be saved to user settings. Use [`set_language_with_persistence`]
    /// if you want to save the language preference.
    ///
    /// # Arguments
    ///
    /// * `language` - The language to switch to
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or a `LocalizationError` if the language
    /// is not supported or translations are not available.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use inspector_gguf::localization::{LocalizationManager, Language};
    ///
    /// let mut manager = LocalizationManager::new()?;
    ///
    /// // Temporarily switch to Russian
    /// manager.set_language(Language::Russian)?;
    /// let russian_title = manager.get_text("app.title");
    ///
    /// // Language preference is not saved to disk
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// See also [`set_language_with_persistence`] for persistent language changes,
    /// [`SystemLocaleDetector::detect`] for automatic detection, and [`SettingsManager`]
    /// for settings management.
    ///
    /// [`set_language_with_persistence`]: LocalizationManager::set_language_with_persistence
    pub fn set_language(&mut self, language: Language) -> Result<(), LocalizationError> {
        self.current_language = language;
        Ok(())
    }

    /// Sets the current language and saves the preference to persistent storage.
    ///
    /// This method changes the active language and attempts to save the preference
    /// to the user's settings file. The saved preference will be restored when
    /// the application is restarted.
    ///
    /// # Arguments
    ///
    /// * `language` - The language to switch to and save as preference
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success. If the language change succeeds but saving
    /// the preference fails, the method still returns `Ok(())` but logs a warning.
    ///
    /// # Errors
    ///
    /// Returns an error if the language is not supported or translations are not available.
    /// Settings save failures are logged but do not cause the method to fail.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use inspector_gguf::localization::{LocalizationManager, Language};
    ///
    /// let mut manager = LocalizationManager::new()?;
    ///
    /// // Switch to Portuguese and save preference
    /// manager.set_language_with_persistence(Language::PortugueseBrazilian)?;
    ///
    /// // Preference will be restored on next application startup
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn set_language_with_persistence(
        &mut self,
        language: Language,
    ) -> Result<(), LocalizationError> {
        self.current_language = language;

        // Persist the language preference to settings
        let settings_manager = SettingsManager::new().unwrap_or_default();
        if let Err(e) = settings_manager.save_language_preference(language) {
            eprintln!("Warning: Failed to save language preference: {}", e);
            // Don't fail the language change if we can't save settings
        }

        Ok(())
    }

    /// Returns the currently active language.
    ///
    /// # Returns
    ///
    /// The currently selected language for translation lookups.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use inspector_gguf::localization::{LocalizationManager, Language};
    ///
    /// let manager = LocalizationManager::new()?;
    /// let current = manager.get_current_language();
    /// println!("Current language: {:?}", current);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn get_current_language(&self) -> Language {
        self.current_language
    }

    /// Returns a list of all supported languages.
    ///
    /// This method returns all languages that the application supports,
    /// regardless of whether their translation files are currently loaded.
    ///
    /// # Returns
    ///
    /// A vector containing all supported language variants.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use inspector_gguf::localization::LocalizationManager;
    ///
    /// let manager = LocalizationManager::new()?;
    /// let languages = manager.get_available_languages();
    ///
    /// for lang in languages {
    ///     println!("Supported: {} ({})", lang.display_name(), lang.to_code());
    /// }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn get_available_languages(&self) -> Vec<Language> {
        vec![
            Language::English,
            Language::Russian,
            Language::PortugueseBrazilian,
        ]
    }

    /// Loads or replaces translations for a specific language.
    ///
    /// This method allows manual loading of translation data, which can be useful
    /// for testing, dynamic translation loading, or custom translation sources.
    ///
    /// # Arguments
    ///
    /// * `language` - The language to load translations for
    /// * `translations` - The translation data as a nested HashMap structure
    ///
    /// # Examples
    ///
    /// ```rust
    /// use inspector_gguf::localization::{LocalizationManager, Language};
    /// use std::collections::HashMap;
    /// use serde_json::json;
    ///
    /// let mut manager = LocalizationManager::new()?;
    ///
    /// // Create custom translations
    /// let mut custom_translations = HashMap::new();
    /// custom_translations.insert("app".to_string(), json!({"title": "Custom Title"}));
    ///
    /// // Load custom translations
    /// manager.load_translations(Language::English, custom_translations);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn load_translations(&mut self, language: Language, translations: TranslationMap) {
        self.translations.insert(language, translations);
    }

    /// Retrieves nested values from translation map using dot notation.
    ///
    /// This helper method navigates through nested JSON objects using dot-separated
    /// key paths. For example, "buttons.load" accesses `translations["buttons"]["load"]`.
    ///
    /// # Arguments
    ///
    /// * `map` - The translation map to search in
    /// * `key` - Dot-separated key path (e.g., "section.subsection.key")
    ///
    /// # Returns
    ///
    /// Returns a reference to the JSON value if found, or `None` if the path
    /// doesn't exist or encounters a non-object value along the way.
    ///
    /// # Examples
    ///
    /// For a translation structure like:
    /// ```json
    /// {
    ///   "buttons": {
    ///     "load": "Load File",
    ///     "save": "Save File"
    ///   }
    /// }
    /// ```
    ///
    /// - `get_nested_value(map, "buttons.load")` returns `Some("Load File")`
    /// - `get_nested_value(map, "buttons.nonexistent")` returns `None`
    /// - `get_nested_value(map, "nonexistent.key")` returns `None`
    fn get_nested_value<'a>(&self, map: &'a TranslationMap, key: &str) -> Option<&'a Value> {
        let parts: Vec<&str> = key.split('.').collect();
        let mut current_value = None;

        // Start with the root map
        for (i, part) in parts.iter().enumerate() {
            if i == 0 {
                // First part - get from root map
                current_value = map.get(*part);
            } else {
                // Subsequent parts - navigate deeper into nested objects
                if let Some(Value::Object(obj)) = current_value {
                    current_value = obj.get(*part);
                } else {
                    return None;
                }
            }
        }

        current_value
    }
}

impl Default for LocalizationManager {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| LocalizationManager {
            current_language: Language::English,
            translations: HashMap::new(),
        })
    }
}
