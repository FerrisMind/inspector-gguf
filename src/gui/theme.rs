//! Theme constants and functions for the Inspector Gadget color scheme.
//!
//! This module provides comprehensive theming support for the Inspector GGUF application,
//! implementing the Inspector Gadget visual identity with adaptive design principles.
//! It handles color schemes, typography, spacing, and visual styling to create a
//! cohesive and professional user experience.
//!
//! # Color Palette
//!
//! The Inspector Gadget theme uses a carefully selected color palette that provides
//! excellent contrast and visual hierarchy:
//!
//! - **Primary Blue** ([`INSPECTOR_BLUE`]): Main brand color for buttons and accents
//! - **Accent Yellow** ([`GADGET_YELLOW`]): Highlight color for important elements
//! - **Neutral Gray** ([`TECH_GRAY`]): Secondary text and subtle elements
//! - **Status Colors**: Success green and danger red for feedback
//!
//! # Adaptive Design
//!
//! The theme system automatically adapts to different screen sizes and resolutions:
//!
//! - **Typography**: Font sizes scale based on screen dimensions
//! - **Spacing**: Margins and padding adjust for optimal density
//! - **Interactive Elements**: Button sizes and touch targets scale appropriately
//!
//! # Usage
//!
//! ## Basic Theme Application
//!
//! ```rust
//! use inspector_gguf::gui::{apply_inspector_theme, load_custom_font};
//! use eframe::egui;
//!
//! fn setup_theme(ctx: &egui::Context) {
//!     load_custom_font(ctx);
//!     apply_inspector_theme(ctx);
//! }
//! ```
//!
//! ## Using Theme Colors
//!
//! ```rust
//! use inspector_gguf::gui::{INSPECTOR_BLUE, GADGET_YELLOW, TECH_GRAY};
//! use eframe::egui;
//!
//! fn create_styled_text() -> egui::RichText {
//!     egui::RichText::new("Inspector GGUF")
//!         .color(GADGET_YELLOW)
//!         .strong()
//! }
//! ```

#![allow(dead_code)] // Allow dead code since this module is extracted but not yet integrated

use eframe::egui;
use egui::{FontData, FontDefinitions, FontFamily};
use std::collections::BTreeMap;

/// Primary brand color - deep blue used for buttons and main UI elements.
///
/// This color represents the Inspector Gadget's signature blue and is used for:
/// - Button backgrounds in inactive state
/// - Primary accent elements
/// - Focus indicators
/// - Brand identity elements
///
/// RGB: (30, 58, 138) - A professional deep blue that provides excellent contrast.
pub const INSPECTOR_BLUE: egui::Color32 = egui::Color32::from_rgb(30, 58, 138);

/// Accent highlight color - bright yellow for important elements and hover states.
///
/// This vibrant yellow is used for:
/// - Text highlights and important labels
/// - Button text in inactive state
/// - Hover state backgrounds
/// - Call-to-action elements
///
/// RGB: (251, 191, 36) - A warm, attention-grabbing yellow that maintains readability.
pub const GADGET_YELLOW: egui::Color32 = egui::Color32::from_rgb(251, 191, 36);

/// Secondary text color - neutral gray for supporting content.
///
/// This gray provides excellent readability while maintaining visual hierarchy:
/// - Secondary text and descriptions
/// - Placeholder text
/// - Subtle UI elements
/// - Disabled state indicators
///
/// RGB: (148, 163, 184) - A balanced gray that works well on dark backgrounds.
pub const TECH_GRAY: egui::Color32 = egui::Color32::from_rgb(148, 163, 184);

/// Error and danger state color - red for warnings and error messages.
///
/// Used for:
/// - Error messages and alerts
/// - Destructive action buttons
/// - Validation failure indicators
/// - Critical status displays
///
/// RGB: (239, 68, 68) - A clear, accessible red that conveys urgency without being harsh.
#[allow(dead_code)]
pub const DANGER_RED: egui::Color32 = egui::Color32::from_rgb(239, 68, 68);

/// Success state color - green for positive feedback and completion states.
///
/// Used for:
/// - Success messages and confirmations
/// - Completed operations
/// - Positive status indicators
/// - Achievement highlights
///
/// RGB: (16, 185, 129) - A vibrant green that clearly indicates positive outcomes.
#[allow(dead_code)]
pub const SUCCESS_GREEN: egui::Color32 = egui::Color32::from_rgb(16, 185, 129);

/// Loads the custom Rubik Distressed font and configures font families.
///
/// This function sets up the application's typography by loading the custom Rubik Distressed
/// font and configuring it as the primary font for both proportional and monospace text.
/// It also integrates the Phosphor icon font for consistent iconography throughout the application.
///
/// # Font Configuration
///
/// - **Primary Font**: Rubik Distressed - A distinctive font that matches the Inspector Gadget theme
/// - **Icon Font**: Phosphor Regular - Provides consistent, high-quality icons
/// - **Fallback**: System fonts are automatically used if custom fonts fail to load
///
/// # Usage
///
/// This function should be called once during application initialization, typically
/// in the main update loop before applying the theme:
///
/// ```rust
/// use inspector_gguf::gui::{load_custom_font, apply_inspector_theme};
/// use eframe::egui;
///
/// fn setup_ui(ctx: &egui::Context) {
///     load_custom_font(ctx);
///     apply_inspector_theme(ctx);
/// }
/// ```
///
/// # Font Loading
///
/// The function embeds the font data directly in the binary using `include_bytes!`,
/// ensuring the font is always available regardless of system font installation.
/// If the custom font fails to load, egui will gracefully fall back to system fonts.
///
/// # Parameters
///
/// * `ctx` - The egui context where fonts will be registered
pub fn load_custom_font(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        "rubik_distressed".to_owned(),
        std::sync::Arc::new(FontData::from_static(include_bytes!(
            "../../assets/fonts/RubikDistressed-Regular.ttf"
        ))),
    );

    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "rubik_distressed".to_owned());

    fonts
        .families
        .entry(FontFamily::Monospace)
        .or_default()
        .insert(0, "rubik_distressed".to_owned());

    // Add Phosphor icons as fallback fonts
    egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);

    ctx.set_fonts(fonts);
}

/// Applies the complete Inspector Gadget theme to the egui context.
///
/// This function configures all visual aspects of the application including colors,
/// typography, spacing, and interactive element styling. It creates a cohesive
/// visual experience that adapts to different screen sizes and provides excellent
/// usability across various devices.
///
/// # Theme Features
///
/// ## Color Scheme
/// - **Inactive Elements**: Blue background with yellow text
/// - **Hover States**: Gray background with blue text  
/// - **Active States**: Yellow background with blue text
/// - **Backgrounds**: Dark theme with layered panel colors
///
/// ## Adaptive Typography
/// - Font sizes automatically scale based on screen dimensions
/// - Consistent text styles for headings, body, buttons, and monospace text
/// - Optimal readability across different display densities
///
/// ## Responsive Spacing
/// - Margins and padding scale with screen size
/// - Touch-friendly interactive elements on smaller screens
/// - Appropriate information density for different form factors
///
/// # Screen Size Adaptations
///
/// - **4K+ (1920px+)**: 20% larger fonts and spacing
/// - **1440p (1440px+)**: 10% larger fonts and spacing  
/// - **Standard (1024px+)**: Base sizing
/// - **Small (<1024px)**: 10% smaller fonts and spacing
///
/// # Usage
///
/// This function should be called after [`load_custom_font`] in the main update loop.
/// It integrates with [`crate::gui::layout`] functions for responsive sizing:
///
/// ```rust
/// use inspector_gguf::gui::{load_custom_font, apply_inspector_theme};
/// use eframe::egui;
///
/// fn update_ui(ctx: &egui::Context) {
///     load_custom_font(ctx);
///     apply_inspector_theme(ctx);
///     
///     // Your UI code here...
/// }
/// ```
///
/// # Parameters
///
/// * `ctx` - The egui context to apply the theme to
///
/// # Examples
///
/// ## Basic Theme Application
///
/// ```rust
/// use inspector_gguf::gui::apply_inspector_theme;
/// use eframe::egui;
///
/// fn setup_theme(ctx: &egui::Context) {
///     apply_inspector_theme(ctx);
///     
///     // Theme is now active for all subsequent UI elements
/// }
/// ```
pub fn apply_inspector_theme(ctx: &egui::Context) {
    // Import the adaptive font size function from layout module
    use super::layout::get_adaptive_font_size;
    
    let mut style = (*ctx.style()).clone();
    let mut visuals = egui::Visuals::dark();

    // Единая цветовая схема Inspector Gadget для состояний кнопок:
    // Неактивные: синий фон (INSPECTOR_BLUE) с жёлтым текстом (GADGET_YELLOW)
    visuals.widgets.inactive.bg_fill = INSPECTOR_BLUE;
    visuals.widgets.inactive.weak_bg_fill = INSPECTOR_BLUE;
    visuals.widgets.inactive.corner_radius = egui::CornerRadius::same(8);
    visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, GADGET_YELLOW);

    // При наведении: серый фон (TECH_GRAY) с синим текстом (INSPECTOR_BLUE)
    visuals.widgets.hovered.bg_fill = TECH_GRAY;
    visuals.widgets.hovered.weak_bg_fill = TECH_GRAY;
    visuals.widgets.hovered.corner_radius = egui::CornerRadius::same(8);
    visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, INSPECTOR_BLUE);

    // При нажатии: жёлтый фон (GADGET_YELLOW) с синим текстом (INSPECTOR_BLUE)
    visuals.widgets.active.bg_fill = GADGET_YELLOW;
    visuals.widgets.active.weak_bg_fill = GADGET_YELLOW;
    visuals.widgets.active.corner_radius = egui::CornerRadius::same(8);
    visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0, INSPECTOR_BLUE);

    // Accent цвета
    visuals.selection.bg_fill = egui::Color32::from_rgb(53, 24, 162); // Цвет выделенного текста #3518a2
    visuals.hyperlink_color = GADGET_YELLOW;
    visuals.override_text_color = None;

    // Фоны панелей
    visuals.window_fill = egui::Color32::from_rgb(15, 23, 42);
    visuals.panel_fill = egui::Color32::from_rgb(30, 41, 59);
    visuals.faint_bg_color = egui::Color32::from_rgb(51, 65, 85);

    // Дополнительные элементы
    visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(30, 41, 59);
    visuals.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, egui::Color32::WHITE);
    visuals.widgets.open.bg_fill = egui::Color32::from_rgb(51, 65, 85);
    visuals.widgets.open.fg_stroke = egui::Stroke::new(1.0, egui::Color32::WHITE);

    // Адаптивная типографика
    let mut text_styles = BTreeMap::new();
    let heading_size = get_adaptive_font_size(16.0, ctx);
    let body_size = get_adaptive_font_size(14.0, ctx);
    let button_size = get_adaptive_font_size(14.0, ctx);
    let small_size = get_adaptive_font_size(12.0, ctx);
    let monospace_size = get_adaptive_font_size(14.0, ctx);

    text_styles.insert(
        egui::TextStyle::Heading,
        egui::FontId::new(heading_size, egui::FontFamily::Proportional),
    );
    text_styles.insert(
        egui::TextStyle::Body,
        egui::FontId::new(body_size, egui::FontFamily::Proportional),
    );
    text_styles.insert(
        egui::TextStyle::Button,
        egui::FontId::new(button_size, egui::FontFamily::Proportional),
    );
    text_styles.insert(
        egui::TextStyle::Small,
        egui::FontId::new(small_size, egui::FontFamily::Proportional),
    );
    text_styles.insert(
        egui::TextStyle::Monospace,
        egui::FontId::new(monospace_size, egui::FontFamily::Monospace),
    );
    style.text_styles = text_styles;

    // Адаптивные отступы и размеры
    let spacing_scale = if ctx.screen_rect().width() >= 1920.0 {
        1.2
    } else if ctx.screen_rect().width() >= 1440.0 {
        1.1
    } else if ctx.screen_rect().width() >= 1024.0 {
        1.0
    } else {
        0.9
    };

    style.spacing.item_spacing = egui::vec2(12.0 * spacing_scale, 12.0 * spacing_scale);
    style.spacing.button_padding = egui::vec2(12.0 * spacing_scale, 8.0 * spacing_scale);
    style.spacing.indent = 20.0 * spacing_scale;
    style.spacing.slider_width = 160.0 * spacing_scale;
    style.spacing.interact_size = egui::vec2(80.0 * spacing_scale, 32.0 * spacing_scale);

    // Применяем визуальные настройки через Style
    style.visuals = visuals;
    ctx.set_style(style);
}