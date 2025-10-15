//! Layout utilities for adaptive GUI sizing
//! This module provides functions for responsive design

#![allow(dead_code)] // Allow dead code since this module is extracted but not yet integrated

use eframe::egui;

/// Calculate adaptive sidebar width based on screen size
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

/// Calculate adaptive font size based on screen size
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

/// Calculate adaptive button width based on text content
pub fn get_adaptive_button_width(_ui: &egui::Ui, text: &str, _font_size: f32, max_width: f32) -> f32 {
    // Simple heuristic: estimate width based on character count
    // This avoids potential deadlocks with font measurement
    let estimated_width = text.len() as f32 * 8.0 + 40.0; // ~8px per character + padding
    estimated_width.min(max_width)
}