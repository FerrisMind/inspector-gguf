//! Left sidebar panel functionality.
//!
//! This module implements the left sidebar panel which serves as the primary
//! action center for the Inspector GGUF application. It provides quick access
//! to file operations, export functions, and application settings through a
//! vertically organized, scrollable interface.
//!
//! # Panel Features
//!
//! ## File Operations
//! - **Load Button**: Opens file dialog for GGUF file selection
//! - **Clear Button**: Removes currently loaded metadata
//! - **Drag & Drop Support**: Handled by the main content panel
//!
//! ## Export Functions
//! Multiple export formats with individual buttons:
//! - CSV export for spreadsheet applications
//! - YAML export for structured data
//! - Markdown export for documentation
//! - HTML export for web viewing
//! - PDF export for reports and printing
//!
//! ## Application Controls
//! - **Settings Button**: Opens configuration dialog
//! - **About Button**: Shows application information
//!
//! # Design Features
//!
//! - **Responsive Width**: Adapts to screen size automatically
//! - **Scrollable Content**: Handles overflow on small screens
//! - **Adaptive Typography**: Font sizes scale with display
//! - **Icon Integration**: Uses Phosphor icons for visual clarity

use eframe::egui;
use rfd::FileDialog;
use std::sync::{Arc, Mutex};
use crate::localization::LanguageProvider;
use crate::gui::layout::{get_sidebar_width, get_adaptive_font_size, get_adaptive_button_width};
use crate::gui::theme::TECH_GRAY;
use crate::gui::export::{export_csv, export_yaml, export_markdown_to_file, export_html_to_file, export_markdown, export_pdf_from_markdown};
use crate::gui::loader::{load_gguf_metadata_async, LoadingResult, MetadataEntry};

/// Renders the left sidebar panel with action buttons and export controls.
///
/// This function creates a comprehensive sidebar interface that provides access to
/// all major application functions including file operations, export capabilities,
/// and application settings. The sidebar uses adaptive sizing and scrolling to
/// work effectively across different screen sizes.
///
/// # Panel Layout
///
/// The sidebar is organized into logical sections:
///
/// 1. **File Operations** (top): Load and Clear buttons
/// 2. **Export Section** (middle): Multiple format export buttons
/// 3. **Application Controls** (bottom): Settings and About buttons
///
/// # Parameters
///
/// * `ctx` - egui context for screen size calculations and theming
/// * `ui` - UI context for rendering within the sidebar panel
/// * `app` - Application instance implementing LanguageProvider for text
/// * `metadata` - Mutable reference to current metadata for clearing
/// * `loading` - Mutable loading state flag
/// * `loading_progress` - Shared progress indicator for async operations
/// * `loading_result` - Shared result container for async loading
/// * `show_settings` - Mutable flag for settings dialog visibility
/// * `show_about` - Mutable flag for about dialog visibility
///
/// # Behavior
///
/// ## File Operations
/// - **Load Button**: Opens native file dialog, starts async loading if file selected
/// - **Clear Button**: Immediately clears all loaded metadata
/// - **Loading State**: Load button disabled during active loading operations
///
/// ## Export Operations
/// - **Format Selection**: Individual buttons for each supported export format
/// - **File Dialogs**: Native save dialogs with appropriate file extensions
/// - **Error Handling**: Displays localized error messages for failed exports
///
/// ## Responsive Design
/// - **Button Sizing**: Adapts to sidebar width with consistent margins
/// - **Font Scaling**: Text sizes adjust based on screen dimensions
/// - **Scrolling**: Vertical scroll area prevents content overflow
///
/// # Examples
///
/// ## Basic Usage in Application
///
/// ```rust
/// use inspector_gguf::gui::panels::render_sidebar;
/// use inspector_gguf::localization::LanguageProvider;
/// use eframe::egui;
/// use std::sync::{Arc, Mutex};
///
/// fn render_app_sidebar<T: LanguageProvider>(
///     ctx: &egui::Context,
///     app: &T,
///     // ... other parameters
/// ) {
///     egui::SidePanel::left("main_sidebar")
///         .resizable(false)
///         .show(ctx, |ui| {
///             // render_sidebar(ctx, ui, app, /* ... parameters */);
///         });
/// }
/// ```
#[allow(clippy::too_many_arguments)]
pub fn render_sidebar<T: LanguageProvider>(
    ctx: &egui::Context,
    ui: &mut egui::Ui,
    app: &T,
    metadata: &mut Vec<MetadataEntry>,
    loading: &mut bool,
    loading_progress: &Arc<Mutex<f32>>,
    loading_result: &LoadingResult,
    show_settings: &mut bool,
    show_about: &mut bool,
) {
    // Добавляем отступ от верхней границы
    ui.add_space(get_adaptive_font_size(16.0, ctx));

    // Добавляем прокрутку для остального содержимого
    egui::ScrollArea::vertical()
        .auto_shrink([false; 2])
        .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
        .show(ui, |ui| {

    let button_width = get_sidebar_width(ctx) - 20.0; // Отступы от краев
    let button_height = get_adaptive_font_size(34.0, ctx);
    
    // Dynamic button sizing based on text length
    let load_text = format!("{} {}", egui_phosphor::regular::FOLDER_OPEN, app.t("buttons.load"));
    
    if ui
        .add_sized(
            [button_width, button_height],
            egui::Button::new(
                egui::RichText::new(load_text)
                .size(get_adaptive_font_size(16.0, ctx)),
            ),
        )
        .clicked()
        && !*loading
        && let Some(path) = FileDialog::new().pick_file()
    {
        *loading = true;
        *loading_progress.lock().unwrap() = 0.0;
        *loading_result.lock().unwrap() = None;

        let progress_clone = Arc::clone(loading_progress);
        let result_clone = Arc::clone(loading_result);
        load_gguf_metadata_async(path, progress_clone, result_clone);
    }

    let clear_text = format!("{} {}", egui_phosphor::regular::BROOM, app.t("buttons.clear"));
    let clear_button_width = get_adaptive_button_width(ui, &clear_text, get_adaptive_font_size(16.0, ctx), button_width);
    
    if ui
        .add_sized(
            [clear_button_width, button_height],
            egui::Button::new(
                egui::RichText::new(clear_text)
                    .size(get_adaptive_font_size(16.0, ctx)),
            ),
        )
        .clicked()
    {
        metadata.clear();
    }

    ui.add_space(16.0);
    ui.label(
        egui::RichText::new(format!("{} {}:", egui_phosphor::regular::EXPORT, app.t("buttons.export")))
            .size(get_adaptive_font_size(16.0, ctx))
            .color(TECH_GRAY),
    );
    let small_button_height = get_adaptive_font_size(28.0, ctx);
    
    // CSV Export button
    let csv_text = format!("{} {}", egui_phosphor::regular::FILE_CSV, app.t("export.csv"));
    let csv_button_width = get_adaptive_button_width(ui, &csv_text, get_adaptive_font_size(16.0, ctx), button_width);
    
    if ui
        .add_sized(
            [csv_button_width, small_button_height],
            egui::Button::new(
                egui::RichText::new(csv_text)
                .size(get_adaptive_font_size(16.0, ctx)),
            ),
        )
        .clicked()
        && let Some(path) = FileDialog::new().save_file()
        && let Err(e) = export_csv(&metadata.iter().map(|entry| (&entry.key, &entry.display_value)).collect::<Vec<_>>(), &path)
    {
        eprintln!("{}", app.t_with_args("messages.export_failed", &[&e.to_string()]));
    }
    
    // YAML Export button
    let yaml_text = format!("{} {}", egui_phosphor::regular::FILE_CODE, app.t("export.yaml"));
    let yaml_button_width = get_adaptive_button_width(ui, &yaml_text, get_adaptive_font_size(16.0, ctx), button_width);
    
    if ui
        .add_sized(
            [yaml_button_width, small_button_height],
            egui::Button::new(
                egui::RichText::new(yaml_text)
                .size(get_adaptive_font_size(16.0, ctx)),
            ),
        )
        .clicked()
        && let Some(path) = FileDialog::new().save_file()
        && let Err(e) = export_yaml(&metadata.iter().map(|entry| (&entry.key, &entry.display_value)).collect::<Vec<_>>(), &path)
    {
        eprintln!("{}", app.t_with_args("messages.export_failed", &[&e.to_string()]));
    }
    
    if ui
        .add_sized(
            [button_width, small_button_height],
            egui::Button::new(
                egui::RichText::new(format!(
                    "{} {}",
                    egui_phosphor::regular::FILE_MD,
                    app.t("export.markdown")
                ))
                .size(get_adaptive_font_size(16.0, ctx)),
            ),
        )
        .clicked()
        && let Some(path) = FileDialog::new().save_file()
        && let Err(e) = export_markdown_to_file(&metadata.iter().map(|entry| (&entry.key, &entry.display_value)).collect::<Vec<_>>(), &path)
    {
        eprintln!("{}", app.t_with_args("messages.export_failed", &[&e.to_string()]));
    }
    
    if ui
        .add_sized(
            [button_width, small_button_height],
            egui::Button::new(
                egui::RichText::new(format!("{} {}", egui_phosphor::regular::FILE_HTML, app.t("export.html")))
                    .size(get_adaptive_font_size(16.0, ctx)),
            ),
        )
        .clicked()
        && let Some(path) = FileDialog::new().save_file()
        && let Err(e) = export_html_to_file(&metadata.iter().map(|entry| (&entry.key, &entry.display_value)).collect::<Vec<_>>(), &path)
    {
        eprintln!("{}", app.t_with_args("messages.export_failed", &[&e.to_string()]));
    }
    
    if ui
        .add_sized(
            [button_width, small_button_height],
            egui::Button::new(
                egui::RichText::new(format!(
                    "{} {}",
                    egui_phosphor::regular::FILE_PDF,
                    app.t("export.pdf")
                ))
                .size(get_adaptive_font_size(16.0, ctx)),
            ),
        )
        .clicked()
        && let Some(path) = FileDialog::new().save_file()
    {
        let md = export_markdown(&metadata.iter().map(|entry| (&entry.key, &entry.display_value)).collect::<Vec<_>>());
        if let Err(e) = export_pdf_from_markdown(&md, &path) {
            eprintln!("{}", app.t_with_args("messages.export_failed", &[&e.to_string()]));
        }
    }

    ui.add_space(16.0);

    // Кнопка настроек
    if ui
        .add_sized(
            [button_width, button_height],
            egui::Button::new(
                egui::RichText::new(format!(
                    "{} {}",
                    egui_phosphor::regular::GEAR,
                    app.t("buttons.settings")
                ))
                .size(get_adaptive_font_size(16.0, ctx)),
            ),
        )
        .clicked()
    {
        *show_settings = true;
    }

    // Кнопка "О программе"
    if ui
        .add_sized(
            [button_width, button_height],
            egui::Button::new(
                egui::RichText::new(format!("{} {}", egui_phosphor::regular::INFO, app.t("buttons.about")))
                    .size(get_adaptive_font_size(16.0, ctx)),
            ),
        )
        .clicked()
    {
        *show_about = true;
    }
    
    // Добавляем дополнительный отступ снизу для прокрутки
    ui.allocate_space(egui::vec2(0.0, get_adaptive_font_size(4.0, ctx)));
    });
}