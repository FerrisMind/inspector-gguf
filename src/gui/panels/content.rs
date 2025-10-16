//! Central content panel functionality.
//!
//! This module implements the main content area of the Inspector GGUF application,
//! responsible for displaying GGUF metadata, providing filtering capabilities,
//! handling drag-and-drop file operations, and showing loading progress. The content
//! panel serves as the primary information display and interaction area.
//!
//! # Panel Features
//!
//! ## Metadata Display
//! - **Structured Layout**: Organized display of key-value metadata pairs
//! - **Special Handling**: Custom viewers for large content (chat templates, tokens)
//! - **Binary Data**: Base64 encoding and external viewer support
//! - **Responsive Cards**: Grouped metadata entries with consistent styling
//!
//! ## Interactive Features
//! - **Real-time Filtering**: Live search through metadata keys and values
//! - **Drag & Drop**: Direct file loading by dropping GGUF files onto the panel
//! - **Content Viewers**: Specialized panels for viewing large text content
//! - **Progress Tracking**: Visual progress bars during file loading operations
//!
//! ## Data Handling
//! - **Large Content**: Automatic detection and special handling of oversized values
//! - **Binary Detection**: Identifies and handles binary data appropriately
//! - **Tokenizer Content**: Special viewers for chat templates and token data
//! - **Temporary Files**: Handles dropped file bytes through temporary file creation

use eframe::egui;
use std::sync::{Arc, Mutex};
use crate::localization::LanguageProvider;
use crate::gui::layout::get_adaptive_font_size;
use crate::gui::theme::{INSPECTOR_BLUE, GADGET_YELLOW, TECH_GRAY};
use crate::gui::loader::{load_gguf_metadata_async, LoadingResult, MetadataEntry};
use crate::gui::export::show_base64_dialog;

/// Renders the main content panel with metadata display and interactive features.
///
/// This function creates the central content area that displays GGUF metadata in an
/// organized, searchable format. It handles various types of content including text,
/// binary data, and specialized tokenizer information, while providing interactive
/// features like filtering and drag-and-drop file loading.
///
/// # Panel Sections
///
/// 1. **Drop Zone**: Invisible area that handles drag-and-drop file operations
/// 2. **Progress Display**: Shows loading progress bar and status during file operations
/// 3. **Filter Toolbar**: Text input for real-time metadata filtering
/// 4. **Metadata Display**: Scrollable area with organized metadata entries
///
/// # Content Types
///
/// ## Standard Metadata
/// - **Key-Value Pairs**: Standard metadata displayed as labeled cards
/// - **Text Content**: Regular text values shown directly
/// - **Adaptive Sizing**: Font sizes and spacing adapt to screen size
///
/// ## Special Content
/// - **Chat Templates**: Large templates with dedicated viewer panels
/// - **Token Data**: GGML tokens and merges with specialized viewers
/// - **Binary Data**: Base64 encoding with external viewer support
/// - **Large Text**: Content over 1024 characters gets special handling
///
/// # Parameters
///
/// * `ctx` - egui context for screen calculations and input handling
/// * `ui` - UI context for rendering within the content panel
/// * `app` - Application instance implementing LanguageProvider
/// * `metadata` - Mutable reference to current metadata entries
/// * `filter` - Mutable reference to current filter text
/// * `loading` - Mutable loading state flag
/// * `loading_progress` - Shared progress indicator for async operations
/// * `loading_result` - Shared result container for async loading
/// * `selected_chat_template` - Mutable reference to selected chat template content
/// * `selected_ggml_tokens` - Mutable reference to selected token data
/// * `selected_ggml_merges` - Mutable reference to selected merge data
///
/// # Interactive Features
///
/// ## Drag and Drop
/// - **File Detection**: Automatically detects dropped GGUF files
/// - **Path Handling**: Supports both file paths and raw file bytes
/// - **Temporary Files**: Creates temporary files for byte data
/// - **Loading Integration**: Automatically starts async loading for dropped files
///
/// ## Filtering System
/// - **Real-time Search**: Filters metadata as user types
/// - **Key and Value Search**: Searches both metadata keys and display values
/// - **Clear Function**: Quick clear button when filter text is present
/// - **Responsive Layout**: Filter input adapts to available space
///
/// ## Content Interaction
/// - **View Buttons**: Special buttons for large content viewing
/// - **Panel Management**: Coordinates with right-side content panels
/// - **Base64 Viewer**: External viewer for binary data
/// - **Error Handling**: Graceful handling of export and viewing errors
///
/// # Examples
///
/// ## Basic Content Panel Usage
///
/// ```rust
/// use inspector_gguf::gui::panels::render_content_panel;
/// use inspector_gguf::localization::LanguageProvider;
/// use eframe::egui;
/// use std::sync::{Arc, Mutex};
///
/// fn render_main_content<T: LanguageProvider>(
///     ctx: &egui::Context,
///     app: &T,
///     // ... other parameters
/// ) {
///     egui::CentralPanel::default()
///         .show(ctx, |ui| {
///             // render_content_panel(ctx, ui, app, /* ... parameters */);
///         });
/// }
/// ```
#[allow(clippy::too_many_arguments, clippy::ptr_arg)]
pub fn render_content_panel<T: LanguageProvider>(
    ctx: &egui::Context,
    ui: &mut egui::Ui,
    app: &T,
    metadata: &mut Vec<MetadataEntry>,
    filter: &mut String,
    loading: &mut bool,
    loading_progress: &Arc<Mutex<f32>>,
    loading_result: &LoadingResult,
    selected_chat_template: &mut Option<String>,
    selected_ggml_tokens: &mut Option<String>,
    selected_ggml_merges: &mut Option<String>,
) {
    // Drop zone: поддержка drag-n-drop файлов
    let dropped = ctx.input(|i| i.raw.dropped_files.clone());
    if !dropped.is_empty() {
        for df in dropped {
            if !*loading {
                if let Some(path) = df.path {
                    *loading = true;
                    *loading_progress.lock().unwrap() = 0.0;
                    *loading_result.lock().unwrap() = None;
                    let progress_clone = Arc::clone(loading_progress);
                    let result_clone = Arc::clone(loading_result);
                    load_gguf_metadata_async(path, progress_clone, result_clone);
                } else if let Some(bytes) = df.bytes {
                    // Сохраняем во временный файл и загружаем
                    let tmp = std::env::temp_dir().join(&df.name);
                    match std::fs::write(&tmp, &*bytes) {
                        Ok(_) => {
                            *loading = true;
                            *loading_progress.lock().unwrap() = 0.0;
                            *loading_result.lock().unwrap() = None;
                            let progress_clone = Arc::clone(loading_progress);
                            let result_clone = Arc::clone(loading_result);
                            load_gguf_metadata_async(tmp, progress_clone, result_clone);
                        }
                        Err(e) => eprintln!("{}", app.t_with_args("messages.file_open_error", &[&e.to_string()])),
                    }
                }
            }
        }
    }

    // Get current progress
    let current_progress = if let Ok(progress) = loading_progress.try_lock() {
        *progress
    } else {
        0.0 // Default value if we can't get access
    };

    // Показываем progressbar если идет загрузка
    if *loading {
        ui.add(
            egui::ProgressBar::new(current_progress)
                .show_percentage()
                .fill(INSPECTOR_BLUE),
        );
        ui.label(egui::RichText::new(app.t("messages.loading")).color(TECH_GRAY).size(get_adaptive_font_size(14.0, ctx)));
    }

    // Filter toolbar
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new(format!("{}:", app.t("buttons.filter"))).color(TECH_GRAY).size(get_adaptive_font_size(14.0, ctx)));

        // Динамическая ширина поля фильтра в зависимости от размера окна
        let available_width = ui.available_width();
        let label_width = get_adaptive_font_size(50.0, ctx); // Примерная ширина лейбла "Filter:"
        let button_width = get_adaptive_font_size(120.0, ctx); // Фиксированная ширина кнопки

        // Рассчитываем ширину поля фильтра с учетом всех элементов
        let total_reserved_width = label_width + if !filter.is_empty() { button_width } else { 0.0 };
        let filter_width = (available_width - total_reserved_width).clamp(100.0, 400.0);

        ui.add_sized(
            [filter_width, get_adaptive_font_size(20.0, ctx)],
            egui::TextEdit::singleline(filter)
        );

        // Кнопка Clear filter показывается только когда есть текст в фильтре
        if !filter.is_empty() {
            ui.add_sized(
                [button_width, get_adaptive_font_size(20.0, ctx)],
                egui::Button::new(format!(
                    "{} {}",
                    egui_phosphor::regular::X,
                    app.t("buttons.clear")
                ))
            ).clicked().then(|| {
                filter.clear();
            });
        }
    });

    // Pre-compute translated strings to avoid borrowing issues
    let view_text = app.t("buttons.view");
    let no_metadata_text = app.t("messages.no_metadata");
    let binary_long_text = app.t("data.binary_long");
    let base64_text = app.t("data.base64");
    
    egui::ScrollArea::vertical()
        .auto_shrink([false; 2])
        .show(ui, |ui| {
            let mut first = true;
            for entry in metadata
                .iter()
                .filter(|entry| entry.key.contains(filter.as_str()) || entry.display_value.contains(filter.as_str()))
            {
                let k = &entry.key;
                let v = &entry.display_value;
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new(k).color(GADGET_YELLOW).strong().size(get_adaptive_font_size(14.0, ctx)));
                        ui.add_space(get_adaptive_font_size(4.0, ctx));
                        if k == "tokenizer.chat_template" {
                            // Специальная обработка для chat template - показываем кнопку Select
                            if ui
                                .button(format!(
                                    "{} {}",
                                    egui_phosphor::regular::EYE,
                                    view_text
                                ))
                                .clicked()
                            {
                                // Close other panels first
                                *selected_ggml_tokens = None;
                                *selected_ggml_merges = None;
                                *selected_chat_template = entry.full_value.clone();
                            }
                        } else if k == "tokenizer.ggml.tokens" {
                            // Специальная обработка для ggml tokens - показываем кнопку View
                            if ui
                                .button(format!(
                                    "{} {}",
                                    egui_phosphor::regular::EYE,
                                    view_text
                                ))
                                .clicked()
                            {
                                // Close other panels first
                                *selected_chat_template = None;
                                *selected_ggml_merges = None;
                                *selected_ggml_tokens = entry.full_value.clone();
                            }
                        } else if k == "tokenizer.ggml.merges" {
                            // Специальная обработка для ggml merges - показываем кнопку View
                            if ui
                                .button(format!(
                                    "{} {}",
                                    egui_phosphor::regular::EYE,
                                    view_text
                                ))
                                .clicked()
                            {
                                // Close other panels first
                                *selected_chat_template = None;
                                *selected_ggml_tokens = None;
                                *selected_ggml_merges = entry.full_value.clone();
                            }
                        } else if v.len() > 1024 || v.contains("\0") {
                            ui.horizontal(|ui| {
                                ui.label(
                                    egui::RichText::new(&binary_long_text)
                                        .color(egui::Color32::LIGHT_GRAY)
                                        .size(get_adaptive_font_size(12.0, ctx)),
                                );
                                if ui
                                    .button(format!(
                                        "{} {} {}",
                                        egui_phosphor::regular::EYE,
                                        view_text,
                                        base64_text
                                    ))
                                    .clicked()
                                    && let Err(e) = show_base64_dialog(v)
                                {
                                    eprintln!("Export failed: {}", e);
                                }
                            });
                        } else {
                            ui.label(
                                egui::RichText::new(v).color(egui::Color32::WHITE).size(get_adaptive_font_size(12.0, ctx)),
                            );
                        }
                    });
                });
                first = false;
                ui.add_space(get_adaptive_font_size(8.0, ctx));
            }
            if first {
                ui.label(
                    egui::RichText::new(&no_metadata_text).color(TECH_GRAY).size(get_adaptive_font_size(14.0, ctx)),
                );
            }
        });
}