// Central content panel functionality
// Handles metadata display, filtering, drag-and-drop, and progress tracking

use eframe::egui;
use std::sync::{Arc, Mutex};
use crate::localization::LanguageProvider;
use crate::gui::layout::get_adaptive_font_size;
use crate::gui::theme::{INSPECTOR_BLUE, GADGET_YELLOW, TECH_GRAY};
use crate::gui::loader::{load_gguf_metadata_async, LoadingResult, MetadataEntry};
use crate::gui::export::show_base64_dialog;

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