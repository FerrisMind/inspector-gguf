//! # Inspector GGUF
//!
//! A powerful GGUF file inspection tool with both graphical and command-line interfaces.
//! Inspector GGUF provides comprehensive analysis of GGUF (GPT-Generated Unified Format) 
//! model files used in machine learning and AI development.
//!
//! ## Features
//!
//! - **Deep GGUF Analysis**: Comprehensive metadata extraction and display
//! - **Modern GUI**: Intuitive interface with drag-and-drop support built with egui
//! - **Export Capabilities**: Multiple formats (CSV, YAML, Markdown, HTML, PDF)
//! - **Tokenizer Support**: Chat templates, token analysis, binary data handling
//! - **Internationalization**: Multi-language support (English, Russian, Portuguese Brazilian)
//! - **Performance Profiling**: Built-in puffin profiler integration
//! - **Auto-updates**: Automatic update checking from GitHub releases
//!
//! ## Quick Start
//!
//! ### As a Library
//!
//! ```rust
//! use inspector_gguf::format::load_gguf_metadata_sync;
//! use std::path::Path;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Load GGUF metadata from a file
//! let path = Path::new("model.gguf");
//! # // Skip actual file loading in doctest
//! # let metadata: Vec<(String, String)> = Vec::new();
//! # /*
//! let metadata = load_gguf_metadata_sync(path)?;
//! # */
//!
//! // Access metadata information
//! println!("Loaded {} metadata entries", metadata.len());
//! # Ok(())
//! # }
//! ```
//!
//! ### GUI Application
//!
//! ```rust,no_run
//! use inspector_gguf::gui::app::GgufApp;
//! use eframe::NativeOptions;
//!
//! # fn main() -> Result<(), eframe::Error> {
//! let options = NativeOptions::default();
//! eframe::run_native(
//!     "Inspector GGUF",
//!     options,
//!     Box::new(|_cc| Ok(Box::new(GgufApp::default()))),
//! )
//! # }
//! ```
//!
//! ## Module Organization
//!
//! - [`mod@format`] - GGUF file parsing and metadata extraction using Candle
//!   - [`format::load_gguf_metadata_sync`] - Synchronous GGUF metadata loading
//!   - [`format::load_gguf_metadata_with_full_content_sync`] - Extended metadata loading with full tokenizer content
//!   - [`format::readable_value_for_key`] - Human-readable value formatting
//! - [`gui`] - Graphical user interface components built with egui
//!   - [`gui::GgufApp`] - Main application struct implementing [`eframe::App`]
//!   - [`gui::apply_inspector_theme`] - Inspector Gadget theme application
//!   - [`gui::export_csv`], [`gui::export_yaml`], [`gui::export_markdown`] - Multi-format export functions
//!   - [`gui::load_gguf_metadata_async`] - Asynchronous file loading with progress tracking
//! - [`localization`] - Internationalization system with multi-language support
//!   - [`localization::LocalizationManager`] - Central localization coordinator
//!   - [`localization::Language`] - Supported language enumeration
//!   - [`localization::SystemLocaleDetector`] - Automatic locale detection
//! - [`versioning`] - Version management and update checking functionality
//!   - [`versioning::CargoUpdater`] - Cargo.toml version management
//!   - [`versioning::VersionCli`] - Command-line version operations
//!
//! ## Architecture
//!
//! Inspector GGUF follows a modular architecture with clear separation of concerns:
//!
//! - **Core Parsing**: The [`mod@format`] module handles all GGUF file operations using the [`candle`] framework
//! - **User Interface**: The [`gui`] module provides immediate-mode GUI components with [`gui::GgufApp`] as the central coordinator
//! - **Internationalization**: The [`localization`] module manages translations through [`localization::LocalizationManager`] and locale detection via [`localization::SystemLocaleDetector`]
//! - **Version Management**: The [`versioning`] module handles updates through [`versioning::CargoUpdater`] and CLI operations via [`versioning::VersionCli`]
//!
//! ## Error Handling
//!
//! All public APIs use structured error types that implement [`std::error::Error`].
//! Error handling follows Rust best practices with detailed error messages and
//! proper error propagation.
//!
//! ## Performance
//!
//! Inspector GGUF is designed for efficient handling of large GGUF files:
//!
//! - Streaming file parsing to minimize memory usage
//! - Async operations for non-blocking file loading
//! - Built-in profiling support with puffin integration
//! - Optimized data structures for metadata storage
//!
//! ## Platform Support
//!
//! - **Windows**: Native support with proper resource embedding
//! - **macOS**: Full compatibility with native file dialogs
//! - **Linux**: Complete support with system integration
//!
//! For more information, see the [GitHub repository](https://github.com/FerrisMind/inspector-gguf).

#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]
#![warn(rustdoc::private_intra_doc_links)]
#![warn(rustdoc::missing_crate_level_docs)]
#![warn(rustdoc::invalid_codeblock_attributes)]

pub mod format;
pub mod gui;
pub mod localization;
pub mod versioning;




