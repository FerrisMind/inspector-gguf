//! GUI module system for Inspector GGUF application.
//!
//! This module provides a comprehensive graphical user interface built with egui/eframe
//! for analyzing and exploring GGUF (GPT-Generated Unified Format) files. The GUI system
//! is organized into specialized components that handle different aspects of the user experience.
//!
//! # Architecture
//!
//! The GUI system follows a modular architecture with clear separation of concerns:
//!
//! - **Application Core** ([`app`]): Main application state and eframe integration
//! - **Visual System** ([`theme`], [`layout`]): Theming, styling, and responsive design
//! - **Data Operations** ([`export`], [`loader`]): File I/O and format conversion
//! - **User Interface** ([`panels`]): Organized UI components and interactions
//! - **External Services** ([`updater`]): Version checking and update management
//!
//! # Component Organization
//!
//! ## Core Application ([`app`])
//! The [`GgufApp`] struct serves as the central orchestrator, implementing the [`eframe::App`]
//! trait and managing application state, user interactions, and GUI rendering. It integrates
//! with [`crate::localization::LocalizationManager`] for multi-language support and uses
//! [`crate::format`] functions for GGUF file processing.
//!
//! ## Visual System
//! - [`theme`]: Inspector Gadget color scheme and font management with [`apply_inspector_theme`] and [`load_custom_font`]
//! - [`layout`]: Responsive sizing utilities for adaptive UI elements including [`get_sidebar_width`] and [`get_adaptive_font_size`]
//!
//! ## Data Processing
//! - [`export`]: Multi-format export with functions like [`export_csv`], [`export_yaml`], [`export_markdown`], [`export_html`], and [`export_pdf_from_markdown`]
//! - [`loader`]: Asynchronous GGUF file loading with [`load_gguf_metadata_async`] and progress tracking via [`LoadingResult`]
//!
//! ## User Interface ([`panels`])
//! Organized panel system for clean UI structure with functions like [`render_sidebar`], 
//! [`render_content_panel`], [`render_settings_dialog`], and [`render_right_side_panels`]:
//! - Sidebar: Action buttons and export controls using [`export`] functions
//! - Content: Metadata display and filtering with [`crate::format`] integration
//! - Dialogs: Settings with [`crate::localization`] integration and about windows with [`updater`] integration
//! - Right panels: Special content viewers for chat templates, tokens using [`crate::format::get_full_tokenizer_content`]
//!
//! # Usage Patterns
//!
//! ## Basic Application Setup
//!
//! ```rust
//! use inspector_gguf::gui::GgufApp;
//! use eframe::egui;
//!
//! // Create and run the application
//! let app = GgufApp::default();
//! let native_options = eframe::NativeOptions::default();
//! # // Note: This example doesn't actually run the app to avoid blocking tests
//! # let _ = (app, native_options);
//! // eframe::run_native("Inspector GGUF", native_options, Box::new(|_cc| Box::new(app)));
//! ```
//!
//! ## Theme Application
//!
//! ```rust
//! use inspector_gguf::gui::{apply_inspector_theme, load_custom_font};
//! use eframe::egui;
//!
//! fn setup_ui(ctx: &egui::Context) {
//!     load_custom_font(ctx);
//!     apply_inspector_theme(ctx);
//! }
//! ```
//!
//! ## Export Operations
//!
//! ```rust
//! use inspector_gguf::gui::export_csv;
//! use std::path::Path;
//!
//! let metadata = vec![
//!     ("model.name".to_string(), "example-model".to_string()),
//!     ("model.version".to_string(), "1.0".to_string()),
//! ];
//! let metadata_refs: Vec<(&String, &String)> = metadata.iter().map(|(k, v)| (k, v)).collect();
//! let path = Path::new("metadata.csv");
//! 
//! # std::fs::create_dir_all(path.parent().unwrap_or(Path::new("."))).ok();
//! export_csv(&metadata_refs, path).expect("Export should succeed");
//! # std::fs::remove_file(path).ok();
//! ```
//!
//! # Re-export Structure
//!
//! This module re-exports key functionality for convenient access:
//!
//! - **Application**: [`GgufApp`] - Main application struct
//! - **Theme System**: Color constants and theming functions
//! - **Layout Utilities**: Responsive sizing functions
//! - **Export Functions**: All export format functions
//! - **Loading System**: Async file loading functionality
//! - **Panel System**: UI panel rendering functions
//!
//! The re-export structure allows users to access functionality through clean,
//! organized imports while maintaining internal module boundaries.

pub mod app;
pub mod theme;
pub mod export;
pub mod loader;
pub mod updater;
pub mod layout;
pub mod panels;

// Re-export main application struct and key functionality
pub use app::GgufApp;

// Theme system re-exports
pub use theme::{
    apply_inspector_theme, 
    load_custom_font, 
    INSPECTOR_BLUE, 
    GADGET_YELLOW, 
    TECH_GRAY, 
    DANGER_RED, 
    SUCCESS_GREEN
};

// Layout utilities re-exports
pub use layout::{
    get_sidebar_width, 
    get_adaptive_font_size, 
    get_adaptive_button_width
};

// Export system re-exports (all public functions)
pub use export::{
    ensure_extension,
    sanitize_for_markdown,
    escape_markdown_text,
    show_base64_dialog,
    export_csv,
    export_yaml,
    export_markdown,
    export_markdown_to_file,
    export_html,
    export_html_to_file,
    export_pdf_from_markdown
};

// File loader re-exports
pub use loader::{
    load_gguf_metadata_async, 
    LoadingResult, 
    MetadataEntry
};

// Update checker re-exports
pub use updater::check_for_updates;

// Panel system re-exports
pub use panels::{
    render_sidebar,
    render_content_panel,
    render_settings_dialog,
    render_about_dialog,
    render_right_side_panels
};