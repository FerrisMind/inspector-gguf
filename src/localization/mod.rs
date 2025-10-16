//! Internationalization and localization system for Inspector GGUF.
//!
//! This module provides comprehensive internationalization (i18n) support for the Inspector GGUF
//! application, enabling multi-language user interfaces and automatic locale detection. The system
//! is designed to be extensible, performant, and user-friendly.
//!
//! # Architecture Overview
//!
//! The localization system is built around several key components:
//!
//! - **[`LocalizationManager`]** - Central coordinator for all localization operations with [`LocalizationManager::get_text`] and [`LocalizationManager::set_language`]
//! - **[`TranslationLoader`]** - Handles loading and validation of translation files with [`TranslationLoader::load_translation`] and [`TranslationLoader::load_all_translations`]
//! - **[`SystemLocaleDetector`]** - Automatic detection of system locale preferences via [`SystemLocaleDetector::detect`]
//! - **[`SettingsManager`]** - Persistent storage of user language preferences using [`SettingsManager::save_language_preference`] and [`SettingsManager::load_language_preference`]
//! - **[`Language`]** - Enumeration of supported languages with [`Language::from_locale`], [`Language::to_code`], and [`Language::display_name`]
//!
//! # Supported Languages
//!
//! The system currently supports three languages:
//! - **English** (`en`) - Default fallback language
//! - **Russian** (`ru`) - Full Cyrillic support
//! - **Portuguese (Brazilian)** (`pt-BR`) - Brazilian Portuguese variant
//!
//! # Translation File Format
//!
//! Translation files are stored as JSON in the `translations/` directory with the following structure:
//!
//! ```json
//! {
//!   "app": {
//!     "title": "Inspector GGUF",
//!     "version": "Version"
//!   },
//!   "buttons": {
//!     "load": "Load",
//!     "export": "Export"
//!   },
//!   "messages": {
//!     "loading": "Loading file..."
//!   }
//! }
//! ```
//!
//! # Usage Examples
//!
//! ## Basic Setup and Usage
//!
//! ```rust
//! use inspector_gguf::localization::{LocalizationManager, Language};
//!
//! // Initialize the localization manager with automatic language detection
//! let mut manager = LocalizationManager::new()?;
//!
//! // Get translated text using dot notation for nested keys
//! let title = manager.get_text("app.title");
//! let load_button = manager.get_text("buttons.load");
//!
//! // Change language programmatically
//! manager.set_language(Language::Russian)?;
//! let russian_title = manager.get_text("app.title");
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Language Detection and Settings
//!
//! ```rust
//! use inspector_gguf::localization::{SystemLocaleDetector, SettingsManager, Language};
//!
//! // Detect system locale
//! if let Some(detected_language) = SystemLocaleDetector::detect() {
//!     println!("Detected language: {:?}", detected_language);
//! }
//!
//! // Manage persistent language settings
//! let settings_manager = SettingsManager::new()?;
//! settings_manager.save_language_preference(Language::PortugueseBrazilian)?;
//! 
//! if let Some(saved_language) = settings_manager.load_language_preference() {
//!     println!("Saved language preference: {:?}", saved_language);
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Translation Loading and Validation
//!
//! ```rust
//! use inspector_gguf::localization::{TranslationLoader, Language};
//!
//! let loader = TranslationLoader::new();
//!
//! // Load specific language translation
//! let english_translations = loader.load_translation(Language::English)?;
//!
//! // Load all available translations
//! let all_translations = loader.load_all_translations()?;
//!
//! // Generate completeness report
//! let report = loader.generate_completeness_report(&all_translations)?;
//! println!("{}", report);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! # Key Features
//!
//! ## Automatic Fallback System
//! 
//! The localization system implements a robust fallback mechanism:
//! 1. Try to get text from the current language
//! 2. Fall back to English if the key is missing
//! 3. Return the key itself as a last resort
//!
//! ## Cross-Platform Locale Detection
//!
//! The system automatically detects user locale preferences on:
//! - **Windows**: Using Windows API (`GetUserDefaultLocaleName`)
//! - **macOS**: Using system defaults and environment variables
//! - **Linux/Unix**: Using standard environment variables (`LC_ALL`, `LANG`, etc.)
//!
//! ## Translation Validation
//!
//! All translation files are validated for:
//! - Required section presence (app, buttons, menu, etc.)
//! - Key completeness across languages
//! - JSON format correctness
//! - Nested structure integrity
//!
//! ## Persistent Settings
//!
//! User language preferences are stored in platform-appropriate locations:
//! - **Windows**: `%APPDATA%\InspectorGGUF\settings.json`
//! - **macOS**: `~/Library/Application Support/InspectorGGUF/settings.json`
//! - **Linux**: `~/.config/inspector-gguf/settings.json`
//!
//! # Error Handling
//!
//! The system uses structured error types for comprehensive error handling:
//! - [`LocalizationError`] - Translation loading and processing errors
//! - [`SettingsError`] - Settings file operations and validation errors
//!
//! All operations are designed to be resilient, with appropriate fallbacks and recovery mechanisms.
//!
//! # Thread Safety
//!
//! The localization system is designed to be thread-safe when used appropriately:
//! - [`LocalizationManager`] should be wrapped in `Arc<Mutex<>>` for shared access
//! - Translation data is immutable once loaded
//! - Settings operations use atomic file writes
//!
//! # Performance Considerations
//!
//! - Translation files are loaded once at startup and cached in memory
//! - Key lookups use efficient HashMap operations
//! - Nested key resolution is optimized for common access patterns
//! - Settings are persisted only when changed, not on every access

/// Language definitions and utilities for internationalization
pub mod language;
/// Error types for localization operations
pub mod error;
/// Central localization management system
pub mod manager;
/// Translation file loading and validation
pub mod loader;
/// System locale detection utilities
pub mod detector;
/// Persistent language preference settings
pub mod settings;
/// Translation provider interface and implementations
pub mod provider;

pub use language::Language;
pub use error::{LocalizationError, SettingsError};
pub use manager::LocalizationManager;
pub use loader::{TranslationLoader, TranslationMap};
pub use detector::SystemLocaleDetector;
pub use settings::{SettingsManager, AppSettings};
pub use provider::LanguageProvider;