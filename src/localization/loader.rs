use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde_json::Value;
use crate::localization::{Language, LocalizationError};

/// Type alias for translation data structure containing nested key-value pairs.
///
/// Translation maps store hierarchical translation data where keys can be accessed
/// using dot notation (e.g., "buttons.load" maps to `translations["buttons"]["load"]`).
pub type TranslationMap = HashMap<String, Value>;

/// Handles loading, validation, and management of translation files.
///
/// The `TranslationLoader` is responsible for reading translation files from disk,
/// validating their structure and completeness, and providing utilities for
/// translation management and analysis.
///
/// # Features
///
/// - **File Loading**: Reads JSON translation files from the `translations/` directory
/// - **Structure Validation**: Ensures all required sections and keys are present
/// - **Completeness Analysis**: Compares translations across languages for missing keys
/// - **Error Recovery**: Handles missing or corrupted translation files gracefully
/// - **Batch Operations**: Can load all translations at once with validation
///
/// # Translation File Structure
///
/// Translation files must follow this JSON structure:
///
/// ```json
/// {
///   "app": {
///     "title": "Inspector GGUF",
///     "version": "Version"
///   },
///   "buttons": {
///     "load": "Load",
///     "clear": "Clear",
///     "export": "Export"
///   },
///   "menu": {
///     "file": "File",
///     "export": "Export"
///   },
///   "export": {
///     "csv": "CSV",
///     "yaml": "YAML"
///   },
///   "messages": {
///     "loading": "Loading...",
///     "no_metadata": "No metadata available"
///   },
///   "settings": {
///     "title": "Settings",
///     "language": "Language"
///   },
///   "about": {
///     "title": "About",
///     "description": "GGUF file inspector"
///   },
///   "languages": {
///     "english": "English",
///     "russian": "Russian"
///   }
/// }
/// ```
///
/// # Examples
///
/// ## Basic Loading
///
/// ```rust
/// use inspector_gguf::localization::{TranslationLoader, Language};
///
/// let loader = TranslationLoader::new();
///
/// // Load specific language
/// let english_translations = loader.load_translation(Language::English)?;
/// let title = TranslationLoader::get_translation_value(&english_translations, "app.title");
/// assert_eq!(title, Some("Inspector GGUF".to_string()));
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// ## Batch Loading and Validation
///
/// ```rust
/// use inspector_gguf::localization::TranslationLoader;
///
/// let loader = TranslationLoader::new();
///
/// // Load all available translations
/// let all_translations = loader.load_all_translations()?;
///
/// // Generate completeness report
/// let report = loader.generate_completeness_report(&all_translations)?;
/// println!("Translation Status:\n{}", report);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub struct TranslationLoader;

impl TranslationLoader {
    /// Creates a new TranslationLoader instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use inspector_gguf::localization::TranslationLoader;
    ///
    /// let loader = TranslationLoader::new();
    /// ```
    pub fn new() -> Self {
        Self
    }

    /// Loads and validates a translation file for the specified language.
    ///
    /// This method reads the JSON translation file from the `translations/` directory,
    /// parses it, and validates its structure to ensure all required sections and
    /// keys are present.
    ///
    /// # Arguments
    ///
    /// * `language` - The language to load translations for
    ///
    /// # Returns
    ///
    /// Returns a `TranslationMap` containing the parsed and validated translation data,
    /// or a `LocalizationError` if the file is missing, corrupted, or invalid.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The translation file doesn't exist
    /// - The file cannot be read (permissions, I/O error)
    /// - The JSON format is invalid
    /// - Required sections or keys are missing
    /// - The translation structure is malformed
    ///
    /// # Examples
    ///
    /// ```rust
    /// use inspector_gguf::localization::{TranslationLoader, Language};
    ///
    /// let loader = TranslationLoader::new();
    ///
    /// // Load English translations
    /// let translations = loader.load_translation(Language::English)?;
    /// assert!(translations.contains_key("app"));
    /// assert!(translations.contains_key("buttons"));
    ///
    /// // Load Russian translations
    /// let russian_translations = loader.load_translation(Language::Russian)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn load_translation(&self, language: Language) -> Result<TranslationMap, LocalizationError> {
        let filename = format!("translations/{}.json", language.to_code());
        let path = Path::new(&filename);
        
        if !path.exists() {
            return Err(LocalizationError::TranslationNotFound(language));
        }

        let content = fs::read_to_string(path)
            .map_err(LocalizationError::Io)?;
        
        let translation: TranslationMap = serde_json::from_str(&content)
            .map_err(|e| LocalizationError::InvalidFormat(format!("JSON parsing error: {}", e)))?;
        
        // Validate the translation structure
        self.validate_translation(&translation)?;
        
        Ok(translation)
    }

    /// Validates that a translation map has the required structure and keys.
    ///
    /// This method performs comprehensive validation of translation data to ensure
    /// it contains all required sections and keys needed by the application.
    /// It checks both the presence of sections and the structure of nested objects.
    ///
    /// # Required Sections
    ///
    /// The following top-level sections must be present:
    /// - `app` - Application metadata (title, version)
    /// - `buttons` - UI button labels
    /// - `menu` - Menu item labels
    /// - `export` - Export format names
    /// - `messages` - User messages and notifications
    /// - `settings` - Settings dialog content
    /// - `about` - About dialog content
    /// - `languages` - Language display names
    ///
    /// # Arguments
    ///
    /// * `translation` - The translation map to validate
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if validation passes, or a `LocalizationError` describing
    /// the validation failure.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Any required section is missing
    /// - A section is not a JSON object
    /// - Required keys within sections are missing
    ///
    /// # Examples
    ///
    /// ```rust
    /// use inspector_gguf::localization::TranslationLoader;
    /// use std::collections::HashMap;
    /// use serde_json::json;
    ///
    /// let loader = TranslationLoader::new();
    ///
    /// // Valid translation structure
    /// let mut valid_translation = HashMap::new();
    /// valid_translation.insert("app".to_string(), json!({"title": "Test", "version": "1.0"}));
    /// valid_translation.insert("buttons".to_string(), json!({"load": "Load"}));
    /// // ... other required sections
    ///
    /// // This would pass validation (if complete)
    /// // let result = loader.validate_translation(&valid_translation);
    /// ```
    pub fn validate_translation(&self, translation: &TranslationMap) -> Result<(), LocalizationError> {
        let required_sections = [
            "app",
            "buttons", 
            "menu",
            "export",
            "messages",
            "settings",
            "about",
            "languages"
        ];

        for section in &required_sections {
            if !translation.contains_key(*section) {
                return Err(LocalizationError::InvalidFormat(
                    format!("Missing required section: {}", section)
                ));
            }
            
            // Ensure the section is an object, not a primitive value
            if !translation[*section].is_object() {
                return Err(LocalizationError::InvalidFormat(
                    format!("Section '{}' must be an object", section)
                ));
            }
        }

        // Validate specific required keys within sections
        self.validate_section_keys(translation, "app", &["title", "version"])?;
        self.validate_section_keys(translation, "buttons", &[
            "load", "clear", "export", "settings", "about", "close", "copy", "view"
        ])?;
        self.validate_section_keys(translation, "menu", &[
            "file", "export", "settings", "help"
        ])?;
        self.validate_section_keys(translation, "export", &[
            "csv", "yaml", "markdown", "html", "pdf"
        ])?;
        self.validate_section_keys(translation, "messages", &[
            "loading", "no_metadata", "export_failed", "file_open_error", "parsing_error"
        ])?;
        self.validate_section_keys(translation, "settings", &[
            "title", "language", "language_description"
        ])?;
        self.validate_section_keys(translation, "about", &[
            "title", "description", "built_with", "license", "copyright", "check_updates", "github"
        ])?;
        self.validate_section_keys(translation, "languages", &[
            "english", "russian", "portuguese_brazilian"
        ])?;

        Ok(())
    }

    /// Validate that a section contains all required keys
    fn validate_section_keys(
        &self,
        translation: &TranslationMap,
        section: &str,
        required_keys: &[&str]
    ) -> Result<(), LocalizationError> {
        let section_obj = translation[section].as_object()
            .ok_or_else(|| LocalizationError::InvalidFormat(
                format!("Section '{}' is not an object", section)
            ))?;

        for key in required_keys {
            if !section_obj.contains_key(*key) {
                return Err(LocalizationError::InvalidFormat(
                    format!("Missing required key '{}' in section '{}'", key, section)
                ));
            }
        }

        Ok(())
    }

    /// Get a translation value by key path (e.g., "buttons.load")
    pub fn get_translation_value(translation: &TranslationMap, key_path: &str) -> Option<String> {
        let parts: Vec<&str> = key_path.split('.').collect();
        let mut current_value = translation.get(parts[0])?;
        
        for part in parts.iter().skip(1) {
            current_value = current_value.as_object()?.get(*part)?;
        }
        
        current_value.as_str().map(|s| s.to_string())
    }

    /// Load all available translations
    pub fn load_all_translations(&self) -> Result<HashMap<Language, TranslationMap>, LocalizationError> {
        let mut translations = HashMap::new();
        
        let languages = [Language::English, Language::Russian, Language::PortugueseBrazilian];
        
        for language in &languages {
            match self.load_translation(*language) {
                Ok(translation) => {
                    translations.insert(*language, translation);
                }
                Err(LocalizationError::TranslationNotFound(_)) => {
                    // Skip missing translation files, but log the issue
                    eprintln!("Warning: Translation file not found for {:?}", language);
                }
                Err(e) => {
                    // Propagate other errors
                    return Err(e);
                }
            }
        }
        
        // Ensure at least English is available as fallback
        if !translations.contains_key(&Language::English) {
            return Err(LocalizationError::TranslationNotFound(Language::English));
        }
        
        // Validate completeness across all loaded translations
        self.validate_translation_completeness(&translations)?;
        
        Ok(translations)
    }

    /// Validate that all translations have the same keys as the English reference
    pub fn validate_translation_completeness(
        &self,
        translations: &HashMap<Language, TranslationMap>
    ) -> Result<(), LocalizationError> {
        let english_translation = translations.get(&Language::English)
            .ok_or(LocalizationError::TranslationNotFound(Language::English))?;
        
        let english_keys = self.extract_all_keys(english_translation);
        
        for (language, translation) in translations {
            if *language == Language::English {
                continue; // Skip English as it's the reference
            }
            
            let translation_keys = self.extract_all_keys(translation);
            let missing_keys = self.find_missing_keys(&english_keys, &translation_keys);
            
            if !missing_keys.is_empty() {
                eprintln!(
                    "Warning: Translation for {:?} is missing {} keys: {:?}",
                    language,
                    missing_keys.len(),
                    missing_keys
                );
                
                // For now, we just warn but don't fail
                // In a production system, you might want to fail or provide more sophisticated handling
            }
            
            let extra_keys = self.find_missing_keys(&translation_keys, &english_keys);
            if !extra_keys.is_empty() {
                eprintln!(
                    "Warning: Translation for {:?} has {} extra keys: {:?}",
                    language,
                    extra_keys.len(),
                    extra_keys
                );
            }
        }
        
        Ok(())
    }

    /// Extract all translation keys from a translation map in dot notation
    fn extract_all_keys(&self, translation: &TranslationMap) -> Vec<String> {
        let mut keys = Vec::new();
        Self::extract_keys_recursive(translation, String::new(), &mut keys);
        keys.sort();
        keys
    }

    /// Recursively extract keys from nested objects
    fn extract_keys_recursive(obj: &TranslationMap, prefix: String, keys: &mut Vec<String>) {
        for (key, value) in obj {
            let full_key = if prefix.is_empty() {
                key.clone()
            } else {
                format!("{}.{}", prefix, key)
            };
            
            if let Some(nested_obj) = value.as_object() {
                // Convert serde_json::Map to HashMap for recursion
                let mut nested_map = HashMap::new();
                for (k, v) in nested_obj {
                    nested_map.insert(k.clone(), v.clone());
                }
                Self::extract_keys_recursive(&nested_map, full_key, keys);
            } else {
                keys.push(full_key);
            }
        }
    }

    /// Find keys that are in reference but missing in target
    fn find_missing_keys(&self, reference_keys: &[String], target_keys: &[String]) -> Vec<String> {
        reference_keys
            .iter()
            .filter(|key| !target_keys.contains(key))
            .cloned()
            .collect()
    }

    /// Check if a specific translation key exists in all loaded translations
    pub fn check_key_completeness(
        &self,
        translations: &HashMap<Language, TranslationMap>,
        key_path: &str
    ) -> HashMap<Language, bool> {
        let mut results = HashMap::new();
        
        for (language, translation) in translations {
            let exists = Self::get_translation_value(translation, key_path).is_some();
            results.insert(*language, exists);
        }
        
        results
    }

    /// Get a list of all available translation keys from the English reference
    pub fn get_available_keys(&self, english_translation: &TranslationMap) -> Vec<String> {
        self.extract_all_keys(english_translation)
    }

    /// Validate and report on translation file completeness
    pub fn generate_completeness_report(
        &self,
        translations: &HashMap<Language, TranslationMap>
    ) -> Result<String, LocalizationError> {
        let english_translation = translations.get(&Language::English)
            .ok_or(LocalizationError::TranslationNotFound(Language::English))?;
        
        let english_keys = self.extract_all_keys(english_translation);
        let mut report = String::new();
        
        report.push_str("Translation Completeness Report\n");
        report.push_str("================================\n\n");
        
        report.push_str(&format!("Total keys in English reference: {}\n\n", english_keys.len()));
        
        for (language, translation) in translations {
            if *language == Language::English {
                continue;
            }
            
            let translation_keys = self.extract_all_keys(translation);
            let missing_keys = self.find_missing_keys(&english_keys, &translation_keys);
            let extra_keys = self.find_missing_keys(&translation_keys, &english_keys);
            
            let completeness_percentage = if english_keys.is_empty() {
                100.0
            } else {
                ((english_keys.len() - missing_keys.len()) as f64 / english_keys.len() as f64) * 100.0
            };
            
            report.push_str(&format!("Language: {:?}\n", language));
            report.push_str(&format!("  Completeness: {:.1}%\n", completeness_percentage));
            report.push_str(&format!("  Total keys: {}\n", translation_keys.len()));
            report.push_str(&format!("  Missing keys: {}\n", missing_keys.len()));
            report.push_str(&format!("  Extra keys: {}\n", extra_keys.len()));
            
            if !missing_keys.is_empty() {
                report.push_str("  Missing:\n");
                for key in &missing_keys {
                    report.push_str(&format!("    - {}\n", key));
                }
            }
            
            if !extra_keys.is_empty() {
                report.push_str("  Extra:\n");
                for key in &extra_keys {
                    report.push_str(&format!("    + {}\n", key));
                }
            }
            
            report.push('\n');
        }
        
        Ok(report)
    }
}

impl Default for TranslationLoader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::localization::Language;
    use std::collections::HashMap;

    #[test]
    fn test_translation_loader_creation() {
        let _loader = TranslationLoader::new();
        assert!(true); // Just verify it can be created
    }

    #[test]
    fn test_load_english_translation() {
        let loader = TranslationLoader::new();
        let result = loader.load_translation(Language::English);
        
        match result {
            Ok(translation) => {
                // Verify required sections exist
                assert!(translation.contains_key("app"));
                assert!(translation.contains_key("buttons"));
                assert!(translation.contains_key("menu"));
                assert!(translation.contains_key("export"));
                assert!(translation.contains_key("messages"));
                assert!(translation.contains_key("settings"));
                assert!(translation.contains_key("about"));
                assert!(translation.contains_key("languages"));
            }
            Err(e) => {
                panic!("Failed to load English translation: {}", e);
            }
        }
    }

    #[test]
    fn test_get_translation_value() {
        let loader = TranslationLoader::new();
        if let Ok(translation) = loader.load_translation(Language::English) {
            // Test getting a simple value
            let title = TranslationLoader::get_translation_value(&translation, "app.title");
            assert_eq!(title, Some("Inspector GGUF".to_string()));
            
            // Test getting a nested value
            let load_button = TranslationLoader::get_translation_value(&translation, "buttons.load");
            assert_eq!(load_button, Some("Load".to_string()));
            
            // Test non-existent key
            let non_existent = TranslationLoader::get_translation_value(&translation, "non.existent");
            assert_eq!(non_existent, None);
        }
    }

    #[test]
    fn test_validation() {
        let loader = TranslationLoader::new();
        
        // Test with valid translation
        if let Ok(translation) = loader.load_translation(Language::English) {
            let validation_result = loader.validate_translation(&translation);
            assert!(validation_result.is_ok());
        }
        
        // Test with invalid translation (missing section)
        let mut invalid_translation = HashMap::new();
        invalid_translation.insert("app".to_string(), serde_json::json!({"title": "Test"}));
        // Missing other required sections
        
        let validation_result = loader.validate_translation(&invalid_translation);
        assert!(validation_result.is_err());
    }

    #[test]
    fn test_load_all_translations() {
        let loader = TranslationLoader::new();
        let result = loader.load_all_translations();
        
        match result {
            Ok(translations) => {
                // Should at least have English
                assert!(translations.contains_key(&Language::English));
                
                // Check that we have some translations loaded
                assert!(!translations.is_empty());
                
                println!("Loaded {} translations", translations.len());
                for language in translations.keys() {
                    println!("  - {:?}", language);
                }
            }
            Err(e) => {
                panic!("Failed to load all translations: {}", e);
            }
        }
    }

    #[test]
    fn test_extract_all_keys() {
        let loader = TranslationLoader::new();
        if let Ok(translation) = loader.load_translation(Language::English) {
            let keys = loader.extract_all_keys(&translation);
            
            // Should have extracted keys
            assert!(!keys.is_empty());
            
            // Should contain expected keys
            assert!(keys.contains(&"app.title".to_string()));
            assert!(keys.contains(&"buttons.load".to_string()));
            assert!(keys.contains(&"settings.language".to_string()));
            
            println!("Extracted {} keys", keys.len());
        }
    }

    #[test]
    fn test_completeness_report() {
        let loader = TranslationLoader::new();
        if let Ok(translations) = loader.load_all_translations() {
            let report = loader.generate_completeness_report(&translations);
            
            match report {
                Ok(report_text) => {
                    assert!(report_text.contains("Translation Completeness Report"));
                    println!("Completeness Report:\n{}", report_text);
                }
                Err(e) => {
                    panic!("Failed to generate completeness report: {}", e);
                }
            }
        }
    }
}