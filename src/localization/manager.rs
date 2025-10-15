use std::collections::HashMap;
use serde_json::Value;
use crate::localization::{Language, LocalizationError, TranslationLoader, SystemLocaleDetector, SettingsManager};

pub type TranslationMap = HashMap<String, Value>;

pub struct LocalizationManager {
    current_language: Language,
    translations: HashMap<Language, TranslationMap>,
}

impl LocalizationManager {
    /// Create a new LocalizationManager with automatic language detection and translation loading
    pub fn new() -> Result<Self, LocalizationError> {
        let mut manager = LocalizationManager {
            current_language: Language::English,
            translations: HashMap::new(),
        };
        
        // Load translations for all supported languages
        let loader = TranslationLoader::new();
        for language in [Language::English, Language::Russian, Language::PortugueseBrazilian] {
            match loader.load_translation(language) {
                Ok(translations) => {
                    manager.translations.insert(language, translations);
                }
                Err(e) => {
                    eprintln!("Warning: Failed to load translations for {:?}: {}", language, e);
                    // Insert empty map as fallback
                    manager.translations.insert(language, HashMap::new());
                }
            }
        }
        
        // Determine initial language from settings or system locale
        let settings_manager = SettingsManager::new().unwrap_or_default();
        let initial_language = settings_manager.load_language_preference()
            .or_else(SystemLocaleDetector::detect)
            .unwrap_or(Language::English);
            
        manager.current_language = initial_language;
        
        Ok(manager)
    }

    /// Get translated text for a given key
    pub fn get_text(&self, key: &str) -> String {
        // Try to get translation from current language
        if let Some(translation_map) = self.translations.get(&self.current_language)
            && let Some(value) = self.get_nested_value(translation_map, key)
            && let Some(text) = value.as_str() {
            return text.to_string();
        }

        // Fallback to English if current language doesn't have the key
        if self.current_language != Language::English
            && let Some(translation_map) = self.translations.get(&Language::English)
            && let Some(value) = self.get_nested_value(translation_map, key)
            && let Some(text) = value.as_str() {
            return text.to_string();
        }

        // Final fallback: return the key itself
        key.to_string()
    }

    /// Set the current language
    pub fn set_language(&mut self, language: Language) -> Result<(), LocalizationError> {
        self.current_language = language;
        Ok(())
    }

    /// Set the current language and persist to settings
    pub fn set_language_with_persistence(&mut self, language: Language) -> Result<(), LocalizationError> {
        self.current_language = language;
        
        // Persist the language preference to settings
        let settings_manager = SettingsManager::new().unwrap_or_default();
        if let Err(e) = settings_manager.save_language_preference(language) {
            eprintln!("Warning: Failed to save language preference: {}", e);
            // Don't fail the language change if we can't save settings
        }
        
        Ok(())
    }

    /// Get the current language
    pub fn get_current_language(&self) -> Language {
        self.current_language
    }

    /// Get list of available languages
    pub fn get_available_languages(&self) -> Vec<Language> {
        vec![Language::English, Language::Russian, Language::PortugueseBrazilian]
    }

    /// Load translations for a specific language
    pub fn load_translations(&mut self, language: Language, translations: TranslationMap) {
        self.translations.insert(language, translations);
    }

    /// Helper method to get nested values from translation map using dot notation
    /// e.g., "buttons.load" -> translations["buttons"]["load"]
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