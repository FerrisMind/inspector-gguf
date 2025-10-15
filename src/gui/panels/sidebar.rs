// Left sidebar panel functionality
// Handles action buttons, export buttons, and settings

use eframe::egui;
use rfd::FileDialog;
use std::sync::{Arc, Mutex};
use crate::localization::LanguageProvider;
use crate::gui::layout::{get_sidebar_width, get_adaptive_font_size, get_adaptive_button_width};
use crate::gui::theme::TECH_GRAY;
use crate::gui::export::{export_csv, export_yaml, export_markdown_to_file, export_html_to_file, export_markdown, export_pdf_from_markdown};
use crate::gui::loader::{load_gguf_metadata_async, LoadingResult, MetadataEntry};

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