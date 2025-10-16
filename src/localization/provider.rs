//! Language provider trait for accessing localized text.
//!
//! This module defines the [`LanguageProvider`] trait which provides a unified interface
//! for accessing translated text throughout the application. Any type that needs to
//! display localized text should implement this trait.
//!
//! # Design Pattern
//!
//! The trait uses a simple key-based lookup system where translation keys use dot notation
//! to access nested translation structures (e.g., "buttons.load" accesses the "load" key
//! within the "buttons" section).
//!
//! # Examples
//!
//! ```rust
//! use inspector_gguf::localization::{LanguageProvider, LocalizationManager};
//!
//! // LocalizationManager implements LanguageProvider
//! let manager = LocalizationManager::new()?;
//! let title = manager.t("app.title");
//! assert!(!title.is_empty());
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

/// Trait for types that provide access to localized text.
///
/// This trait defines the interface for accessing translated strings throughout
/// the application. It provides methods for simple text lookup and parameterized
/// text formatting with placeholder substitution.
///
/// # Implementation
///
/// The primary implementation is [`LocalizationManager`], which loads translations
/// from JSON files and provides fallback mechanisms for missing keys.
///
/// # Examples
///
/// ## Basic Text Retrieval
///
/// ```rust
/// use inspector_gguf::localization::{LanguageProvider, LocalizationManager, Language};
///
/// let mut manager = LocalizationManager::new()?;
/// manager.set_language(Language::English)?;
///
/// // Get simple translated text
/// let load_button = manager.t("buttons.load");
/// assert_eq!(load_button, "Load");
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// ## Parameterized Text
///
/// ```rust
/// use inspector_gguf::localization::{LanguageProvider, LocalizationManager};
///
/// let manager = LocalizationManager::new()?;
///
/// // Get text with parameter substitution
/// let error_msg = manager.t_with_args("messages.export_failed", &["file not found"]);
/// assert!(error_msg.contains("file not found"));
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// See also [`LocalizationManager`] for the main implementation and
/// [`crate::localization::Language`] for supported languages.
///
/// [`LocalizationManager`]: crate::localization::LocalizationManager
pub trait LanguageProvider {
    /// Retrieves translated text for the specified key.
    ///
    /// This method looks up the translation for the given key in the current language,
    /// with automatic fallback to English if the key is not found. If no translation
    /// exists in any language, the key itself is returned.
    ///
    /// # Arguments
    ///
    /// * `key` - Translation key in dot notation (e.g., "app.title", "buttons.load")
    ///
    /// # Returns
    ///
    /// The translated string, or the key itself if no translation is available.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use inspector_gguf::localization::{LanguageProvider, LocalizationManager, Language};
    ///
    /// let mut manager = LocalizationManager::new()?;
    ///
    /// // Get English translation
    /// manager.set_language(Language::English)?;
    /// assert_eq!(manager.t("app.title"), "Inspector GGUF");
    ///
    /// // Get Russian translation
    /// manager.set_language(Language::Russian)?;
    /// let russian_title = manager.t("app.title");
    /// assert!(!russian_title.is_empty());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    fn t(&self, key: &str) -> String;

    /// Retrieves translated text with parameter substitution.
    ///
    /// This method retrieves a translation and performs placeholder substitution,
    /// replacing `{0}`, `{1}`, etc. with the provided arguments in order.
    ///
    /// # Arguments
    ///
    /// * `key` - Translation key in dot notation
    /// * `args` - Array of string arguments to substitute into placeholders
    ///
    /// # Returns
    ///
    /// The translated string with placeholders replaced by the provided arguments.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use inspector_gguf::localization::{LanguageProvider, LocalizationManager};
    ///
    /// let manager = LocalizationManager::new()?;
    ///
    /// // Translation: "Export failed: {0}"
    /// let error = manager.t_with_args("messages.export_failed", &["disk full"]);
    /// assert!(error.contains("disk full"));
    ///
    /// // Translation: "New version available: {0}"
    /// let update = manager.t_with_args("messages.update_available", &["v2.0.0"]);
    /// assert!(update.contains("v2.0.0"));
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    fn t_with_args(&self, key: &str, args: &[&str]) -> String {
        let mut text = self.t(key);
        for (i, arg) in args.iter().enumerate() {
            text = text.replace(&format!("{{{}}}", i), arg);
        }
        text
    }
}

/// Implementation of LanguageProvider for LocalizationManager.
///
/// This implementation delegates to the manager's internal translation lookup
/// system, providing access to all loaded translations with automatic fallback.
impl LanguageProvider for crate::localization::LocalizationManager {
    fn t(&self, key: &str) -> String {
        self.get_text(key)
    }
}
