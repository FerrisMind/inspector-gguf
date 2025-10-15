use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum Language {
    #[default]
    English,
    Russian,
    PortugueseBrazilian,
}

impl Language {
    /// Create Language from locale string (e.g., "en", "ru", "pt-BR")
    pub fn from_locale(locale: &str) -> Option<Self> {
        match locale.to_lowercase().as_str() {
            "en" | "en-us" | "en-gb" | "english" => Some(Language::English),
            "ru" | "ru-ru" | "russian" => Some(Language::Russian),
            "pt-br" | "pt_br" | "portuguese-brazilian" | "portuguese_brazilian" => Some(Language::PortugueseBrazilian),
            _ => None,
        }
    }

    /// Get language code for file naming and identification
    pub fn to_code(&self) -> &'static str {
        match self {
            Language::English => "en",
            Language::Russian => "ru", 
            Language::PortugueseBrazilian => "pt-BR",
        }
    }

    /// Get display name in the language itself
    pub fn display_name(&self) -> &'static str {
        match self {
            Language::English => "English",
            Language::Russian => "Русский",
            Language::PortugueseBrazilian => "Português (Brasil)",
        }
    }
}

