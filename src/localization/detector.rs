use crate::localization::Language;
use std::env;

/// Cross-platform system locale detector for automatic language detection.
///
/// The `SystemLocaleDetector` provides automatic detection of the user's preferred
/// language based on system locale settings. It supports multiple platforms and
/// uses platform-specific APIs and environment variables to determine the most
/// appropriate language setting.
///
/// # Platform Support
///
/// - **Windows**: Uses Windows API (`GetUserDefaultLocaleName`, `GetUserDefaultLCID`)
/// - **macOS**: Uses system defaults and environment variables
/// - **Linux/Unix**: Uses standard environment variables (`LC_ALL`, `LC_MESSAGES`, `LANG`)
///
/// # Detection Priority
///
/// The detector follows a priority-based approach:
/// 1. Platform-specific APIs (Windows API, macOS defaults)
/// 2. Environment variables in order: `LC_ALL` → `LC_MESSAGES` → `LANG` → `LANGUAGE`
/// 3. Returns `None` if no supported locale is detected
///
/// # Examples
///
/// ## Basic Detection
///
/// ```rust
/// use inspector_gguf::localization::{SystemLocaleDetector, Language};
///
/// // Detect system language
/// if let Some(detected_language) = SystemLocaleDetector::detect() {
///     println!("Detected language: {:?}", detected_language);
///     match detected_language {
///         Language::English => println!("System is set to English"),
///         Language::Russian => println!("Система настроена на русский язык"),
///         Language::PortugueseBrazilian => println!("Sistema configurado para português brasileiro"),
///     }
/// } else {
///     println!("Could not detect system language, using default");
/// }
/// ```
///
/// ## Manual Locale String Processing
///
/// ```rust
/// use inspector_gguf::localization::SystemLocaleDetector;
///
/// // Get raw locale string
/// if let Some(locale_string) = SystemLocaleDetector::get_system_locale_string() {
///     println!("Raw system locale: {}", locale_string);
/// }
/// ```
///
/// # Supported Locale Formats
///
/// The detector can parse various locale string formats:
/// - ISO codes: "en", "ru", "pt-BR"
/// - Full locales: "en_US.UTF-8", "ru_RU.UTF-8", "pt_BR.UTF-8"
/// - Windows format: "en-US", "ru-RU", "pt-BR"
/// - Named locales: "English", "Russian", "Portuguese_Brazil"
pub struct SystemLocaleDetector;

impl SystemLocaleDetector {
    /// Detects the system locale and returns the corresponding Language.
    ///
    /// This method attempts to determine the user's preferred language by checking
    /// platform-specific locale sources in order of preference. It returns the first
    /// supported language found, or `None` if no supported locale is detected.
    ///
    /// # Detection Process
    ///
    /// 1. **Platform-specific detection**: Uses native APIs when available
    /// 2. **Environment variables**: Checks standard locale environment variables
    /// 3. **Parsing and mapping**: Converts locale strings to supported Language variants
    ///
    /// # Returns
    ///
    /// Returns `Some(Language)` if a supported locale is detected, or `None` if:
    /// - No locale information is available
    /// - The detected locale is not supported by the application
    /// - The locale format cannot be parsed
    ///
    /// # Examples
    ///
    /// ```rust
    /// use inspector_gguf::localization::{SystemLocaleDetector, Language};
    ///
    /// match SystemLocaleDetector::detect() {
    ///     Some(Language::English) => {
    ///         println!("English locale detected");
    ///     }
    ///     Some(Language::Russian) => {
    ///         println!("Russian locale detected");
    ///     }
    ///     Some(Language::PortugueseBrazilian) => {
    ///         println!("Brazilian Portuguese locale detected");
    ///     }
    ///     None => {
    ///         println!("No supported locale detected, using default");
    ///     }
    /// }
    /// ```
    ///
    /// # Platform Behavior
    ///
    /// - **Windows**: Prioritizes `GetUserDefaultLocaleName()` API
    /// - **macOS**: Uses `defaults read -g AppleLocale` when available
    /// - **Linux**: Checks environment variables in standard order
    pub fn detect() -> Option<Language> {
        if let Some(locale_string) = Self::get_system_locale_string() {
            Language::from_locale(&locale_string)
        } else {
            None
        }
    }

    /// Retrieves the raw system locale string from various platform sources.
    ///
    /// This method attempts to get the unprocessed locale string from the system
    /// using platform-appropriate methods. The returned string can then be parsed
    /// to determine the appropriate Language variant.
    ///
    /// # Returns
    ///
    /// Returns `Some(String)` containing the raw locale identifier, or `None` if
    /// no locale information is available from any source.
    ///
    /// # Platform Sources
    ///
    /// - **Windows**: Windows API locale functions
    /// - **macOS**: System defaults and environment variables
    /// - **Linux/Unix**: Standard locale environment variables
    ///
    /// # Examples
    ///
    /// ```rust
    /// use inspector_gguf::localization::SystemLocaleDetector;
    ///
    /// if let Some(locale) = SystemLocaleDetector::get_system_locale_string() {
    ///     println!("System locale string: {}", locale);
    ///     // Example outputs:
    ///     // "en_US.UTF-8" (Linux)
    ///     // "en-US" (Windows)
    ///     // "ru_RU" (macOS)
    /// } else {
    ///     println!("No locale information available");
    /// }
    /// ```
    pub fn get_system_locale_string() -> Option<String> {
        // Try Windows-specific detection first
        #[cfg(target_os = "windows")]
        {
            if let Some(locale) = Self::get_windows_locale() {
                return Some(locale);
            }
        }

        // Try Unix/Linux environment variables
        Self::get_unix_locale()
    }

    /// Get locale from Unix/Linux environment variables
    fn get_unix_locale() -> Option<String> {
        // Priority order: LC_ALL > LC_MESSAGES > LANG > LANGUAGE
        let env_vars = ["LC_ALL", "LC_MESSAGES", "LANG", "LANGUAGE"];
        
        for var_name in &env_vars {
            if let Ok(locale_value) = env::var(var_name)
                && Self::is_valid_locale(&locale_value) {
                return Some(Self::parse_locale_string(&locale_value));
            }
        }

        // Additional fallback for macOS
        #[cfg(target_os = "macos")]
        {
            if let Some(macos_locale) = Self::get_macos_locale() {
                return Some(macos_locale);
            }
        }

        None
    }

    /// Check if a locale string is valid (not C, POSIX, or empty)
    fn is_valid_locale(locale: &str) -> bool {
        !locale.is_empty() && locale != "C" && locale != "POSIX"
    }

    /// macOS-specific locale detection
    #[cfg(target_os = "macos")]
    fn get_macos_locale() -> Option<String> {
        use std::process::Command;
        
        // Try to get locale from macOS defaults command
        if let Ok(output) = Command::new("defaults")
            .args(&["read", "-g", "AppleLocale"])
            .output()
        {
            if output.status.success() {
                let locale_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if Self::is_valid_locale(&locale_str) {
                    return Some(Self::parse_locale_string(&locale_str));
                }
            }
        }
        
        None
    }

    /// Parse locale string to extract language code
    /// Examples: "en_US.UTF-8" -> "en", "ru_RU.UTF-8" -> "ru", "pt_BR.UTF-8" -> "pt-BR"
    fn parse_locale_string(locale: &str) -> String {
        // Remove encoding and modifiers (e.g., UTF-8, @euro)
        let locale_part = locale
            .split('.')
            .next()
            .unwrap_or(locale)
            .split('@')
            .next()
            .unwrap_or(locale);
        
        // Handle special cases for Brazilian Portuguese
        if locale_part.eq_ignore_ascii_case("pt_BR") || 
           locale_part.eq_ignore_ascii_case("pt-BR") ||
           locale_part.eq_ignore_ascii_case("Portuguese_Brazil") {
            return "pt-BR".to_string();
        }
        
        // Handle other Portuguese variants (fallback to Brazilian Portuguese for now)
        if locale_part.starts_with("pt_") || locale_part.starts_with("pt-") {
            return "pt-BR".to_string();
        }
        
        // Handle Russian variants
        if locale_part.eq_ignore_ascii_case("ru_RU") || 
           locale_part.eq_ignore_ascii_case("ru-RU") ||
           locale_part.eq_ignore_ascii_case("Russian_Russia") {
            return "ru".to_string();
        }
        
        // Handle English variants
        if locale_part.starts_with("en_") || locale_part.starts_with("en-") ||
           locale_part.eq_ignore_ascii_case("English") {
            return "en".to_string();
        }
        
        // Extract base language code (before '_' or '-')
        let lang_code = if let Some(code) = locale_part.split('_').next() {
            code
        } else if let Some(code) = locale_part.split('-').next() {
            code
        } else {
            locale_part
        };
        
        lang_code.to_lowercase()
    }

    /// Windows-specific locale detection using Windows API
    #[cfg(target_os = "windows")]
    fn get_windows_locale() -> Option<String> {
        use winapi::um::winnls::{GetUserDefaultLocaleName, GetUserDefaultLCID, GetLocaleInfoW};
        use winapi::shared::ntdef::WCHAR;
        
        // Try GetUserDefaultLocaleName first (Vista+)
        let mut locale_name: [WCHAR; 85] = [0; 85]; // LOCALE_NAME_MAX_LENGTH is 85
        
        unsafe {
            let result = GetUserDefaultLocaleName(locale_name.as_mut_ptr(), locale_name.len() as i32);
            if result > 0 {
                // Convert wide string to String
                let len = locale_name.iter().position(|&c| c == 0).unwrap_or(locale_name.len());
                let wide_str = &locale_name[..len];
                
                if let Ok(locale_string) = String::from_utf16(wide_str) {
                    return Some(Self::parse_windows_locale(&locale_string));
                }
            }
        }
        
        // Fallback to GetUserDefaultLCID and GetLocaleInfoW for older Windows versions
        unsafe {
            let lcid = GetUserDefaultLCID();
            let mut lang_buffer: [WCHAR; 10] = [0; 10];
            
            let result = GetLocaleInfoW(
                lcid as u32,
                0x0059, // LOCALE_SISO639LANGNAME constant value
                lang_buffer.as_mut_ptr(),
                lang_buffer.len() as i32,
            );
            
            if result > 0 {
                let len = lang_buffer.iter().position(|&c| c == 0).unwrap_or(lang_buffer.len());
                let wide_str = &lang_buffer[..len];
                
                if let Ok(lang_string) = String::from_utf16(wide_str) {
                    return Some(lang_string.to_lowercase());
                }
            }
        }
        
        None
    }

    /// Parse Windows locale string to standard format
    /// Examples: "en-US" -> "en", "ru-RU" -> "ru", "pt-BR" -> "pt-BR"
    #[cfg(target_os = "windows")]
    fn parse_windows_locale(locale: &str) -> String {
        // Handle special case for Brazilian Portuguese
        if locale.starts_with("pt-BR") {
            return "pt-BR".to_string();
        }
        
        // Extract language code (before '-')
        if let Some(lang_code) = locale.split('-').next() {
            lang_code.to_lowercase()
        } else {
            locale.to_lowercase()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_locale_string() {
        // Basic locale parsing
        assert_eq!(SystemLocaleDetector::parse_locale_string("en_US.UTF-8"), "en");
        assert_eq!(SystemLocaleDetector::parse_locale_string("ru_RU.UTF-8"), "ru");
        assert_eq!(SystemLocaleDetector::parse_locale_string("pt_BR.UTF-8"), "pt-BR");
        assert_eq!(SystemLocaleDetector::parse_locale_string("en"), "en");
        
        // Handle modifiers and encodings
        assert_eq!(SystemLocaleDetector::parse_locale_string("en_US.UTF-8@euro"), "en");
        assert_eq!(SystemLocaleDetector::parse_locale_string("ru_RU@cyrillic"), "ru");
        
        // Portuguese variants
        assert_eq!(SystemLocaleDetector::parse_locale_string("pt-BR"), "pt-BR");
        assert_eq!(SystemLocaleDetector::parse_locale_string("Portuguese_Brazil"), "pt-BR");
        assert_eq!(SystemLocaleDetector::parse_locale_string("pt_PT"), "pt-BR"); // Fallback to Brazilian
        
        // Russian variants
        assert_eq!(SystemLocaleDetector::parse_locale_string("Russian_Russia"), "ru");
        assert_eq!(SystemLocaleDetector::parse_locale_string("ru-RU"), "ru");
        
        // English variants
        assert_eq!(SystemLocaleDetector::parse_locale_string("en_GB"), "en");
        assert_eq!(SystemLocaleDetector::parse_locale_string("English"), "en");
        
        // Edge cases
        assert_eq!(SystemLocaleDetector::parse_locale_string("C"), "c");
        assert_eq!(SystemLocaleDetector::parse_locale_string("POSIX"), "posix");
    }

    #[test]
    fn test_is_valid_locale() {
        assert!(SystemLocaleDetector::is_valid_locale("en_US"));
        assert!(SystemLocaleDetector::is_valid_locale("ru_RU.UTF-8"));
        assert!(!SystemLocaleDetector::is_valid_locale("C"));
        assert!(!SystemLocaleDetector::is_valid_locale("POSIX"));
        assert!(!SystemLocaleDetector::is_valid_locale(""));
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn test_parse_windows_locale() {
        assert_eq!(SystemLocaleDetector::parse_windows_locale("en-US"), "en");
        assert_eq!(SystemLocaleDetector::parse_windows_locale("ru-RU"), "ru");
        assert_eq!(SystemLocaleDetector::parse_windows_locale("pt-BR"), "pt-BR");
        assert_eq!(SystemLocaleDetector::parse_windows_locale("en"), "en");
    }

    #[test]
    fn test_detect_with_supported_locale() {
        // This test depends on the system locale, so we'll test the parsing logic
        let test_cases = vec![
            ("en", Some(Language::English)),
            ("ru", Some(Language::Russian)),
            ("pt-BR", Some(Language::PortugueseBrazilian)),
            ("fr", None), // Unsupported language
        ];

        for (locale_str, expected) in test_cases {
            let result = Language::from_locale(locale_str);
            assert_eq!(result, expected, "Failed for locale: {}", locale_str);
        }
    }
}