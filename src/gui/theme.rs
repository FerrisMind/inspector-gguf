//! Theme constants and functions for the Inspector Gadget color scheme
//! This module handles all visual theming and styling

#![allow(dead_code)] // Allow dead code since this module is extracted but not yet integrated

use eframe::egui;
use egui::{FontData, FontDefinitions, FontFamily};
use std::collections::BTreeMap;

// Theme colors (Inspector Gadget palette)
pub const INSPECTOR_BLUE: egui::Color32 = egui::Color32::from_rgb(30, 58, 138);
pub const GADGET_YELLOW: egui::Color32 = egui::Color32::from_rgb(251, 191, 36);
pub const TECH_GRAY: egui::Color32 = egui::Color32::from_rgb(148, 163, 184);
#[allow(dead_code)]
pub const DANGER_RED: egui::Color32 = egui::Color32::from_rgb(239, 68, 68);
#[allow(dead_code)]
pub const SUCCESS_GREEN: egui::Color32 = egui::Color32::from_rgb(16, 185, 129);

/// Load custom font for the application
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

/// Apply Inspector Gadget theme to the application
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