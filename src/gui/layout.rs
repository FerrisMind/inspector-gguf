//! Layout utilities for adaptive GUI sizing and responsive design.
//!
//! This module provides essential functions for creating responsive user interfaces
//! that adapt gracefully to different screen sizes and resolutions. The utilities
//! implement a mobile-first, adaptive design approach that ensures optimal usability
//! across desktop, tablet, and mobile form factors.
//!
//! # Design Philosophy
//!
//! The layout system follows these principles:
//!
//! - **Adaptive Scaling**: Elements scale proportionally with screen size
//! - **Minimum Usability**: Maintains usable sizes even on small screens
//! - **Maximum Efficiency**: Prevents oversized elements on large displays
//! - **Touch Friendliness**: Ensures adequate touch targets on all devices
//!
//! # Screen Size Categories
//!
//! The system recognizes four main screen categories:
//!
//! - **Large (1920px+)**: 4K displays and ultra-wide monitors
//! - **Medium (1440px+)**: Standard desktop and laptop displays  
//! - **Standard (1024px+)**: Tablets and smaller laptops
//! - **Small (<1024px)**: Mobile devices and compact displays
//!
//! # Usage
//!
//! ## Responsive Sidebar
//!
//! ```rust
//! use inspector_gguf::gui::get_sidebar_width;
//! use eframe::egui;
//!
//! fn create_sidebar(ctx: &egui::Context, ui: &mut egui::Ui) {
//!     let width = get_sidebar_width(ctx);
//!     
//!     egui::SidePanel::left("sidebar")
//!         .exact_width(width)
//!         .show(ctx, |ui| {
//!             // Sidebar content
//!         });
//! }
//! ```
//!
//! ## Adaptive Typography
//!
//! ```rust
//! use inspector_gguf::gui::get_adaptive_font_size;
//! use eframe::egui;
//!
//! fn create_heading(ctx: &egui::Context) -> egui::RichText {
//!     let size = get_adaptive_font_size(18.0, ctx);
//!     egui::RichText::new("Heading").size(size)
//! }
//! ```

#![allow(dead_code)] // Allow dead code since this module is extracted but not yet integrated

use eframe::egui;

/// Calculates adaptive sidebar width based on screen size and optimal proportions.
///
/// This function determines the appropriate sidebar width by analyzing the screen
/// dimensions and applying responsive design principles. It ensures the sidebar
/// maintains usable proportions across different display sizes while never becoming
/// too narrow or excessively wide.
///
/// # Sizing Strategy
///
/// - **Large Screens (1920px+)**: 15% of screen width, clamped to 120-200px range
/// - **Medium Screens (1440px+)**: Fixed 160px width for optimal desktop experience
/// - **Standard Screens (1024px+)**: Fixed 140px width for tablet compatibility
/// - **Small Screens (<1024px)**: Minimum 120px width for mobile usability
///
/// # Parameters
///
/// * `ctx` - The egui context containing screen dimension information
///
/// # Returns
///
/// The optimal sidebar width in pixels as a floating-point value.
///
/// # Examples
///
/// ## Creating a Responsive Sidebar
///
/// ```rust
/// use inspector_gguf::gui::get_sidebar_width;
/// use eframe::egui;
///
/// fn render_sidebar(ctx: &egui::Context) {
///     let width = get_sidebar_width(ctx);
///     
///     egui::SidePanel::left("main_sidebar")
///         .resizable(false)
///         .exact_width(width)
///         .show(ctx, |ui| {
///             ui.label("Sidebar content");
///         });
/// }
/// ```
///
/// ## Width-Based Layout Decisions
///
/// ```rust
/// use inspector_gguf::gui::get_sidebar_width;
/// use eframe::egui;
///
/// fn layout_buttons(ctx: &egui::Context, ui: &mut egui::Ui) {
///     let sidebar_width = get_sidebar_width(ctx);
///     let button_width = sidebar_width - 20.0; // Account for margins
///     
///     ui.add_sized([button_width, 32.0], egui::Button::new("Action"));
/// }
/// ```
pub fn get_sidebar_width(ctx: &egui::Context) -> f32 {
    let screen_size = ctx.screen_rect().width();
    // Минимальная ширина - 120px, максимальная - 200px
    // Для экранов шире 1920px используем 15% ширины экрана
    if screen_size >= 1920.0 {
        (screen_size * 0.15).clamp(120.0, 200.0)
    } else if screen_size >= 1440.0 {
        160.0 // Средний размер для 1440p
    } else if screen_size >= 1024.0 {
        140.0 // Для планшетов/маленьких десктопов
    } else {
        120.0 // Минимальный размер
    }
}

/// Calculates adaptive font size based on screen dimensions and base size.
///
/// This function implements responsive typography by scaling font sizes according
/// to screen dimensions. It ensures text remains readable and appropriately sized
/// across different display densities and form factors, from mobile devices to
/// large desktop monitors.
///
/// # Scaling Factors
///
/// - **Large Screens (1920px+)**: 1.2x scale (20% larger) for 4K displays
/// - **Medium Screens (1440px+)**: 1.1x scale (10% larger) for high-DPI displays
/// - **Standard Screens (1024px+)**: 1.0x scale (base size) for standard displays
/// - **Small Screens (<1024px)**: 0.9x scale (10% smaller) for mobile devices
///
/// # Parameters
///
/// * `base_size` - The base font size in pixels before scaling
/// * `ctx` - The egui context containing screen dimension information
///
/// # Returns
///
/// The scaled font size in pixels as a floating-point value.
///
/// # Examples
///
/// ## Responsive Text Elements
///
/// ```rust
/// use inspector_gguf::gui::get_adaptive_font_size;
/// use eframe::egui;
///
/// fn create_responsive_text(ctx: &egui::Context) {
///     let heading_size = get_adaptive_font_size(18.0, ctx);
///     let body_size = get_adaptive_font_size(14.0, ctx);
///     let small_size = get_adaptive_font_size(12.0, ctx);
///     
///     let heading = egui::RichText::new("Title").size(heading_size);
///     let body = egui::RichText::new("Content").size(body_size);
///     let caption = egui::RichText::new("Caption").size(small_size);
/// }
/// ```
///
/// ## Button Sizing
///
/// ```rust
/// use inspector_gguf::gui::get_adaptive_font_size;
/// use eframe::egui;
///
/// fn create_adaptive_button(ctx: &egui::Context, ui: &mut egui::Ui) {
///     let font_size = get_adaptive_font_size(16.0, ctx);
///     let button_height = get_adaptive_font_size(34.0, ctx);
///     
///     ui.add_sized(
///         [200.0, button_height],
///         egui::Button::new(egui::RichText::new("Click Me").size(font_size))
///     );
/// }
/// ```
pub fn get_adaptive_font_size(base_size: f32, ctx: &egui::Context) -> f32 {
    let screen_size = ctx.screen_rect().width();
    let scale_factor = if screen_size >= 1920.0 {
        1.2 // Увеличиваем на 20% для 4K
    } else if screen_size >= 1440.0 {
        1.1 // Увеличиваем на 10% для 1440p
    } else if screen_size >= 1024.0 {
        1.0 // Стандартный размер
    } else {
        0.9 // Уменьшаем на 10% для маленьких экранов
    };
    base_size * scale_factor
}

/// Calculates adaptive button width based on text content and constraints.
///
/// This function estimates the optimal button width by analyzing the text content
/// and applying heuristic sizing rules. It provides a balance between content
/// accommodation and layout constraints, ensuring buttons are neither too cramped
/// nor excessively wide.
///
/// # Sizing Algorithm
///
/// The function uses a character-based estimation approach:
/// - Approximately 8 pixels per character for average text
/// - Additional 40 pixels for padding and margins
/// - Clamped to the specified maximum width to prevent layout overflow
///
/// # Parameters
///
/// * `_ui` - The egui UI context (currently unused but reserved for future font measurement)
/// * `text` - The button text content to measure
/// * `_font_size` - The font size (currently unused but reserved for precise measurement)
/// * `max_width` - The maximum allowed button width in pixels
///
/// # Returns
///
/// The estimated optimal button width in pixels, never exceeding `max_width`.
///
/// # Examples
///
/// ## Responsive Button Layout
///
/// ```rust
/// use inspector_gguf::gui::get_adaptive_button_width;
/// use eframe::egui;
///
/// fn create_fitted_button(ui: &mut egui::Ui, text: &str, max_width: f32) {
///     let width = get_adaptive_button_width(ui, text, 16.0, max_width);
///     
///     ui.add_sized(
///         [width, 32.0],
///         egui::Button::new(text)
///     );
/// }
/// ```
///
/// ## Dynamic Button Sizing
///
/// ```rust
/// use inspector_gguf::gui::get_adaptive_button_width;
/// use eframe::egui;
///
/// fn layout_action_buttons(ui: &mut egui::Ui, available_width: f32) {
///     let buttons = ["Load", "Save", "Export"];
///     
///     for button_text in &buttons {
///         let width = get_adaptive_button_width(ui, button_text, 14.0, available_width);
///         ui.add_sized([width, 28.0], egui::Button::new(*button_text));
///     }
/// }
/// ```
///
/// # Notes
///
/// This function currently uses a heuristic approach for performance reasons.
/// Future versions may implement precise text measurement using the egui font
/// system for more accurate sizing.
pub fn get_adaptive_button_width(_ui: &egui::Ui, text: &str, _font_size: f32, max_width: f32) -> f32 {
    // Simple heuristic: estimate width based on character count
    // This avoids potential deadlocks with font measurement
    let estimated_width = text.len() as f32 * 8.0 + 40.0; // ~8px per character + padding
    estimated_width.min(max_width)
}