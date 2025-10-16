use serde::{Deserialize, Serialize};

/// Enumeration of supported languages in Inspector GGUF.
///
/// This enum represents all languages that the application supports for localization.
/// Each variant corresponds to a specific locale with its own translation file and
/// cultural conventions.
///
/// # Supported Languages
///
/// - **English** - Default language, serves as fallback for missing translations
/// - **Russian** - Full Cyrillic script support with Russian localization
/// - **Portuguese (Brazilian)** - Brazilian Portuguese variant with local conventions
///
/// # Examples
///
/// ```rust
/// use inspector_gguf::localization::Language;
///
/// // Create language from locale string
/// let lang = Language::from_locale("pt-BR").unwrap();
/// assert_eq!(lang, Language::PortugueseBrazilian);
///
/// // Get language code for file naming
/// assert_eq!(lang.to_code(), "pt-BR");
///
/// // Get display name in the language itself
/// assert_eq!(lang.display_name(), "Português (Brasil)");
/// ```
///
/// # Serialization
///
/// The enum implements `Serialize` and `Deserialize` for configuration storage
/// and can be safely stored in settings files or transmitted over APIs.
///
/// See also [`crate::localization::LocalizationManager`] for language management,
/// [`crate::localization::SystemLocaleDetector`] for automatic detection, and
/// [`crate::localization::SettingsManager`] for persistent storage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum Language {
    /// English language (default).
    ///
    /// Serves as the primary language and fallback for missing translations.
    /// Translation file: `translations/en.json`
    #[default]
    English,
    
    /// Russian language with Cyrillic script support.
    ///
    /// Full Russian localization with proper Cyrillic character handling.
    /// Translation file: `translations/ru.json`
    Russian,
    
    /// Brazilian Portuguese language variant.
    ///
    /// Uses Brazilian Portuguese conventions and terminology.
    /// Translation file: `translations/pt-BR.json`
    PortugueseBrazilian,
}

impl Language {
    /// Creates a Language variant from a locale string.
    ///
    /// This method parses various locale string formats and returns the corresponding
    /// Language variant. It handles multiple formats including ISO language codes,
    /// locale identifiers, and full language names.
    ///
    /// # Supported Formats
    ///
    /// - **English**: "en", "en-US", "en-GB", "english"
    /// - **Russian**: "ru", "ru-RU", "russian"
    /// - **Portuguese (Brazilian)**: "pt-BR", "pt_BR", "portuguese-brazilian", "portuguese_brazilian"
    ///
    /// # Arguments
    ///
    /// * `locale` - A locale string in various supported formats (case-insensitive)
    ///
    /// # Returns
    ///
    /// Returns `Some(Language)` if the locale is recognized, or `None` if unsupported.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use inspector_gguf::localization::Language;
    ///
    /// // ISO language codes
    /// assert_eq!(Language::from_locale("en"), Some(Language::English));
    /// assert_eq!(Language::from_locale("ru"), Some(Language::Russian));
    /// assert_eq!(Language::from_locale("pt-BR"), Some(Language::PortugueseBrazilian));
    ///
    /// // Locale identifiers
    /// assert_eq!(Language::from_locale("en-US"), Some(Language::English));
    /// assert_eq!(Language::from_locale("ru-RU"), Some(Language::Russian));
    ///
    /// // Full names (case-insensitive)
    /// assert_eq!(Language::from_locale("English"), Some(Language::English));
    /// assert_eq!(Language::from_locale("RUSSIAN"), Some(Language::Russian));
    ///
    /// // Unsupported locale
    /// assert_eq!(Language::from_locale("fr"), None);
    /// ```
    pub fn from_locale(locale: &str) -> Option<Self> {
        match locale.to_lowercase().as_str() {
            "en" | "en-us" | "en-gb" | "english" => Some(Language::English),
            "ru" | "ru-ru" | "russian" => Some(Language::Russian),
            "pt-br" | "pt_br" | "portuguese-brazilian" | "portuguese_brazilian" => Some(Language::PortugueseBrazilian),
            _ => None,
        }
    }

    /// Returns the standard language code for file naming and identification.
    ///
    /// This method returns the canonical language code used for translation file names,
    /// API communication, and internal identification. The codes follow standard
    /// conventions where possible.
    ///
    /// # Returns
    ///
    /// A static string slice containing the language code:
    /// - English: "en"
    /// - Russian: "ru"
    /// - Portuguese (Brazilian): "pt-BR"
    ///
    /// # Examples
    ///
    /// ```rust
    /// use inspector_gguf::localization::Language;
    ///
    /// assert_eq!(Language::English.to_code(), "en");
    /// assert_eq!(Language::Russian.to_code(), "ru");
    /// assert_eq!(Language::PortugueseBrazilian.to_code(), "pt-BR");
    ///
    /// // Use for file naming
    /// let lang = Language::Russian;
    /// let filename = format!("translations/{}.json", lang.to_code());
    /// assert_eq!(filename, "translations/ru.json");
    /// ```
    pub fn to_code(&self) -> &'static str {
        match self {
            Language::English => "en",
            Language::Russian => "ru", 
            Language::PortugueseBrazilian => "pt-BR",
        }
    }

    /// Returns the display name of the language in its own script/language.
    ///
    /// This method provides the native name of each language, which is most
    /// appropriate for user interface language selection menus. The names
    /// are displayed in the language's own script and follow local conventions.
    ///
    /// # Returns
    ///
    /// A static string slice containing the native language name:
    /// - English: "English"
    /// - Russian: "Русский" (in Cyrillic script)
    /// - Portuguese (Brazilian): "Português (Brasil)"
    ///
    /// # Examples
    ///
    /// ```rust
    /// use inspector_gguf::localization::Language;
    ///
    /// assert_eq!(Language::English.display_name(), "English");
    /// assert_eq!(Language::Russian.display_name(), "Русский");
    /// assert_eq!(Language::PortugueseBrazilian.display_name(), "Português (Brasil)");
    ///
    /// // Use in language selection UI
    /// for lang in [Language::English, Language::Russian, Language::PortugueseBrazilian] {
    ///     println!("Language option: {}", lang.display_name());
    /// }
    /// ```
    pub fn display_name(&self) -> &'static str {
        match self {
            Language::English => "English",
            Language::Russian => "Русский",
            Language::PortugueseBrazilian => "Português (Brasil)",
        }
    }
}

