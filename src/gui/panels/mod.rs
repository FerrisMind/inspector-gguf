//! Panel management system for organized UI components.
//!
//! This module provides a structured approach to UI organization by dividing the
//! interface into specialized panels, each responsible for specific functionality.
//! The panel system promotes code organization, maintainability, and consistent
//! user experience across different parts of the application.
//!
//! # Panel Architecture
//!
//! The panel system is organized into three main categories:
//!
//! ## Layout Panels ([`sidebar`], [`content`])
//! - **Sidebar Panel**: Action buttons, export controls, and navigation
//! - **Content Panel**: Main metadata display, filtering, and interaction area
//!
//! ## Modal Panels ([`dialogs`])
//! - **Settings Dialog**: Language preferences and configuration options
//! - **About Dialog**: Application information and update checking
//! - **Right-Side Panels**: Specialized content viewers for large data
//!
//! # Design Principles
//!
//! ## Separation of Concerns
//! Each panel handles a specific aspect of the user interface:
//! - Clear functional boundaries between panels
//! - Minimal coupling between panel implementations
//! - Consistent parameter patterns across panel functions
//!
//! ## Responsive Design
//! All panels adapt to different screen sizes:
//! - Adaptive sizing based on screen dimensions
//! - Consistent spacing and typography scaling
//! - Touch-friendly interactions on mobile devices
//!
//! ## State Management
//! Panels receive state through function parameters:
//! - No internal state storage in panel functions
//! - Clear data flow from application to panels
//! - Predictable behavior and easy testing
//!
//! # Usage Patterns
//!
//! ## Basic Panel Rendering
//!
//! ```rust
//! use inspector_gguf::gui::panels::{render_sidebar, render_content_panel};
//! use inspector_gguf::localization::LanguageProvider;
//! use eframe::egui;
//! use std::sync::{Arc, Mutex};
//!
//! fn render_main_ui<T: LanguageProvider>(
//!     ctx: &egui::Context,
//!     app: &mut T,
//!     // ... other parameters
//! ) {
//!     // Sidebar panel
//!     egui::SidePanel::left("sidebar")
//!         .show(ctx, |ui| {
//!             // render_sidebar(ctx, ui, app, /* ... other params */);
//!         });
//!
//!     // Main content panel
//!     egui::CentralPanel::default()
//!         .show(ctx, |ui| {
//!             // render_content_panel(ctx, ui, app, /* ... other params */);
//!         });
//! }
//! ```
//!
//! ## Dialog Management
//!
//! ```rust
//! use inspector_gguf::gui::panels::{render_settings_dialog, render_about_dialog};
//! use inspector_gguf::localization::{LanguageProvider, LocalizationManager};
//! use eframe::egui;
//!
//! fn render_dialogs<T: LanguageProvider>(
//!     ctx: &egui::Context,
//!     app: &mut T,
//!     show_settings: &mut bool,
//!     show_about: &mut bool,
//!     localization_manager: &mut LocalizationManager,
//!     update_status: &mut Option<String>,
//! ) {
//!     if *show_settings {
//!         // render_settings_dialog(ctx, ui, app, show_settings, localization_manager);
//!     }
//!
//!     if *show_about {
//!         // render_about_dialog(ctx, ui, app, show_about, update_status);
//!     }
//! }
//! ```
//!
//! # Panel Functions
//!
//! All panel functions follow consistent patterns:
//!
//! - **Context Parameter**: egui::Context for window-level operations
//! - **UI Parameter**: egui::Ui for panel-specific rendering (where applicable)
//! - **App Parameter**: Application instance implementing LanguageProvider
//! - **State Parameters**: Mutable references to relevant state
//!
//! This consistency makes the panel system predictable and easy to use across
//! different parts of the application.

pub mod sidebar;
pub mod content;
pub mod dialogs;

// Re-export panel functionality for clean API access

/// Renders the left sidebar panel with action buttons and export controls.
///
/// See [`sidebar::render_sidebar`] for detailed documentation.
pub use sidebar::render_sidebar;

/// Renders the main content panel with metadata display and filtering.
///
/// See [`content::render_content_panel`] for detailed documentation.
pub use content::render_content_panel;

/// Renders the settings dialog window for application configuration.
///
/// See [`dialogs::render_settings_dialog`] for detailed documentation.
pub use dialogs::render_settings_dialog;

/// Renders the about dialog window with application information.
///
/// See [`dialogs::render_about_dialog`] for detailed documentation.
pub use dialogs::render_about_dialog;

/// Renders specialized right-side panels for viewing large content.
///
/// See [`dialogs::render_right_side_panels`] for detailed documentation.
pub use dialogs::render_right_side_panels;