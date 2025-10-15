// Main application struct and eframe::App implementation
// This module acts as the orchestrator for all GUI functionality

use std::sync::{Arc, Mutex};
use eframe::egui;
use crate::localization::{LocalizationManager, LanguageProvider};
use crate::gui::loader::{LoadingResult, MetadataEntry};
use crate::gui::theme::{apply_inspector_theme, load_custom_font, TECH_GRAY, GADGET_YELLOW};
use crate::gui::layout::{get_sidebar_width, get_adaptive_font_size};
use crate::gui::updater::check_for_updates;
use crate::gui::panels::dialogs;
use rfd;

/// Main application struct that orchestrates all GUI functionality
pub struct GgufApp {
    pub metadata: Vec<MetadataEntry>,
    pub filter: String,
    pub loading: bool,
    pub loading_progress: Arc<Mutex<f32>>,
    pub loading_result: LoadingResult,
    pub show_settings: bool,
    pub show_about: bool,
    pub selected_chat_template: Option<String>,
    pub selected_ggml_tokens: Option<String>,
    pub selected_ggml_merges: Option<String>,
    // Update checking fields
    pub update_status: Option<String>,
    // Localization
    pub localization_manager: LocalizationManager,
}

impl Default for GgufApp {
    fn default() -> Self {
        let localization_manager = LocalizationManager::new()
            .unwrap_or_else(|e| {
                eprintln!("Warning: Failed to initialize localization manager: {}", e);
                LocalizationManager::default()
            });
            
        Self {
            metadata: Vec::new(),
            filter: String::new(),
            loading: false,
            loading_progress: Arc::new(Mutex::new(0.0)),
            loading_result: Arc::new(Mutex::new(None)),
            show_settings: false,
            show_about: false,
            selected_chat_template: None,
            selected_ggml_tokens: None,
            selected_ggml_merges: None,
            update_status: None,
            localization_manager,
        }
    }
}

impl eframe::App for GgufApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        puffin::GlobalProfiler::lock().new_frame();

        // Load custom font and apply theme
        load_custom_font(ctx);
        apply_inspector_theme(ctx);

        // Update loading progress
        let current_progress = if let Ok(progress) = self.loading_progress.try_lock() {
            *progress
        } else {
            0.0
        };

        // Handle loading completion
        if self.loading {
            if current_progress < 0.0 {
                self.loading = false; // Error
            } else if current_progress >= 1.0 {
                // Check loading result
                if let Ok(mut result) = self.loading_result.try_lock()
                    && let Some(load_result) = result.take()
                {
                    self.loading = false;
                    match load_result {
                        Ok(metadata) => {
                            self.metadata = metadata.into_iter()
                                .map(|(key, display_value, full_value)| MetadataEntry {
                                    key,
                                    display_value,
                                    full_value,
                                })
                                .collect();
                        }
                        Err(e) => {
                            eprintln!("{}", self.t_with_args("messages.parsing_error", &[&e.to_string()]));
                        }
                    }
                }
            }
        }

        // Pre-compute translation strings to avoid borrowing issues
        let t_chat_template = self.t("panels.chat_template");
        let t_ggml_tokens = self.t("panels.ggml_tokens");
        let t_ggml_merges = self.t("panels.ggml_merges");

        // Render right-side panels for special content
        dialogs::render_right_side_panels(
            ctx,
            &mut self.selected_chat_template,
            &mut self.selected_ggml_tokens,
            &mut self.selected_ggml_merges,
            &t_chat_template,
            &t_ggml_tokens,
            &t_ggml_merges,
        );

        // Render sidebar panel using the dedicated function
        egui::SidePanel::left("inspector_toolkit")
            .resizable(false)
            .exact_width(get_sidebar_width(ctx))
            .show(ctx, |ui| {
                // Render sidebar directly to avoid borrowing issues
                // Add top spacing
                ui.add_space(get_adaptive_font_size(16.0, ctx));

                // Add scroll area for content
                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                    .show(ui, |ui| {
                        let button_width = get_sidebar_width(ctx) - 20.0;
                        let button_height = get_adaptive_font_size(34.0, ctx);
                        
                        // Load button
                        let load_text = format!("{} {}", egui_phosphor::regular::FOLDER_OPEN, self.t("buttons.load"));
                        
                        if ui
                            .add_sized(
                                [button_width, button_height],
                                egui::Button::new(
                                    egui::RichText::new(load_text)
                                    .size(get_adaptive_font_size(16.0, ctx)),
                                ),
                            )
                            .clicked()
                            && !self.loading
                            && let Some(path) = rfd::FileDialog::new().pick_file()
                        {
                            self.loading = true;
                            *self.loading_progress.lock().unwrap() = 0.0;
                            *self.loading_result.lock().unwrap() = None;

                            let progress_clone = Arc::clone(&self.loading_progress);
                            let result_clone = Arc::clone(&self.loading_result);
                            crate::gui::loader::load_gguf_metadata_async(path, progress_clone, result_clone);
                        }

                        // Clear button
                        let clear_text = format!("{} {}", egui_phosphor::regular::BROOM, self.t("buttons.clear"));
                        
                        if ui
                            .add_sized(
                                [button_width, button_height],
                                egui::Button::new(
                                    egui::RichText::new(clear_text)
                                        .size(get_adaptive_font_size(16.0, ctx)),
                                ),
                            )
                            .clicked()
                        {
                            self.metadata.clear();
                        }

                        ui.add_space(16.0);
                        ui.label(
                            egui::RichText::new(format!("{} {}:", egui_phosphor::regular::EXPORT, self.t("buttons.export")))
                                .size(get_adaptive_font_size(16.0, ctx))
                                .color(TECH_GRAY),
                        );
                        
                        let small_button_height = get_adaptive_font_size(28.0, ctx);
                        
                        // CSV Export button
                        let csv_text = format!("{} {}", egui_phosphor::regular::FILE_CSV, self.t("export.csv"));
                        
                        if ui
                            .add_sized(
                                [button_width, small_button_height],
                                egui::Button::new(
                                    egui::RichText::new(csv_text)
                                    .size(get_adaptive_font_size(16.0, ctx)),
                                ),
                            )
                            .clicked()
                            && let Some(path) = rfd::FileDialog::new().save_file()
                            && let Err(e) = crate::gui::export::export_csv(&self.metadata.iter().map(|entry| (&entry.key, &entry.display_value)).collect::<Vec<_>>(), &path)
                        {
                            eprintln!("{}", self.t_with_args("messages.export_failed", &[&e.to_string()]));
                        }
                        
                        // YAML Export button
                        let yaml_text = format!("{} {}", egui_phosphor::regular::FILE_CODE, self.t("export.yaml"));
                        
                        if ui
                            .add_sized(
                                [button_width, small_button_height],
                                egui::Button::new(
                                    egui::RichText::new(yaml_text)
                                    .size(get_adaptive_font_size(16.0, ctx)),
                                ),
                            )
                            .clicked()
                            && let Some(path) = rfd::FileDialog::new().save_file()
                            && let Err(e) = crate::gui::export::export_yaml(&self.metadata.iter().map(|entry| (&entry.key, &entry.display_value)).collect::<Vec<_>>(), &path)
                        {
                            eprintln!("{}", self.t_with_args("messages.export_failed", &[&e.to_string()]));
                        }
                        
                        // Markdown Export button
                        if ui
                            .add_sized(
                                [button_width, small_button_height],
                                egui::Button::new(
                                    egui::RichText::new(format!(
                                        "{} {}",
                                        egui_phosphor::regular::FILE_MD,
                                        self.t("export.markdown")
                                    ))
                                    .size(get_adaptive_font_size(16.0, ctx)),
                                ),
                            )
                            .clicked()
                            && let Some(path) = rfd::FileDialog::new().save_file()
                            && let Err(e) = crate::gui::export::export_markdown_to_file(&self.metadata.iter().map(|entry| (&entry.key, &entry.display_value)).collect::<Vec<_>>(), &path)
                        {
                            eprintln!("{}", self.t_with_args("messages.export_failed", &[&e.to_string()]));
                        }
                        
                        // HTML Export button
                        if ui
                            .add_sized(
                                [button_width, small_button_height],
                                egui::Button::new(
                                    egui::RichText::new(format!("{} {}", egui_phosphor::regular::FILE_HTML, self.t("export.html")))
                                        .size(get_adaptive_font_size(16.0, ctx)),
                                ),
                            )
                            .clicked()
                            && let Some(path) = rfd::FileDialog::new().save_file()
                            && let Err(e) = crate::gui::export::export_html_to_file(&self.metadata.iter().map(|entry| (&entry.key, &entry.display_value)).collect::<Vec<_>>(), &path)
                        {
                            eprintln!("{}", self.t_with_args("messages.export_failed", &[&e.to_string()]));
                        }
                        
                        // PDF Export button
                        if ui
                            .add_sized(
                                [button_width, small_button_height],
                                egui::Button::new(
                                    egui::RichText::new(format!(
                                        "{} {}",
                                        egui_phosphor::regular::FILE_PDF,
                                        self.t("export.pdf")
                                    ))
                                    .size(get_adaptive_font_size(16.0, ctx)),
                                ),
                            )
                            .clicked()
                            && let Some(path) = rfd::FileDialog::new().save_file()
                        {
                            let md = crate::gui::export::export_markdown(&self.metadata.iter().map(|entry| (&entry.key, &entry.display_value)).collect::<Vec<_>>());
                            if let Err(e) = crate::gui::export::export_pdf_from_markdown(&md, &path) {
                                eprintln!("{}", self.t_with_args("messages.export_failed", &[&e.to_string()]));
                            }
                        }

                        ui.add_space(16.0);

                        // Settings button
                        if ui
                            .add_sized(
                                [button_width, button_height],
                                egui::Button::new(
                                    egui::RichText::new(format!(
                                        "{} {}",
                                        egui_phosphor::regular::GEAR,
                                        self.t("buttons.settings")
                                    ))
                                    .size(get_adaptive_font_size(16.0, ctx)),
                                ),
                            )
                            .clicked()
                        {
                            self.show_settings = true;
                        }

                        // About button
                        if ui
                            .add_sized(
                                [button_width, button_height],
                                egui::Button::new(
                                    egui::RichText::new(format!("{} {}", egui_phosphor::regular::INFO, self.t("buttons.about")))
                                        .size(get_adaptive_font_size(16.0, ctx)),
                                ),
                            )
                            .clicked()
                        {
                            self.show_about = true;
                        }
                        
                        // Add bottom spacing for scrolling
                        ui.allocate_space(egui::vec2(0.0, get_adaptive_font_size(4.0, ctx)));
                    });
            });

        // Render central content panel using the dedicated function
        egui::CentralPanel::default()
            .frame(
                egui::Frame::central_panel(&ctx.style()).fill(egui::Color32::from_rgb(12, 18, 26)),
            )
            .show(ctx, |ui| {
                // Handle drag and drop
                let dropped = ctx.input(|i| i.raw.dropped_files.clone());
                if !dropped.is_empty() {
                    for df in dropped {
                        if !self.loading
                            && let Some(path) = df.path
                        {
                            self.loading = true;
                            *self.loading_progress.lock().unwrap() = 0.0;
                            *self.loading_result.lock().unwrap() = None;
                            let progress_clone = Arc::clone(&self.loading_progress);
                            let result_clone = Arc::clone(&self.loading_result);
                            crate::gui::loader::load_gguf_metadata_async(path, progress_clone, result_clone);
                        } else if let Some(bytes) = df.bytes {
                            // Save to temporary file and load
                            let tmp = std::env::temp_dir().join(&df.name);
                            match std::fs::write(&tmp, &*bytes) {
                                Ok(_) => {
                                    self.loading = true;
                                    *self.loading_progress.lock().unwrap() = 0.0;
                                    *self.loading_result.lock().unwrap() = None;
                                    let progress_clone = Arc::clone(&self.loading_progress);
                                    let result_clone = Arc::clone(&self.loading_result);
                                    crate::gui::loader::load_gguf_metadata_async(tmp, progress_clone, result_clone);
                                }
                                Err(e) => eprintln!("{}", self.t_with_args("messages.file_open_error", &[&e.to_string()])),
                            }
                        }
                    }
                }

                // Show progress bar if loading
                if self.loading {
                    ui.add(
                        egui::ProgressBar::new(current_progress)
                            .show_percentage()
                            .fill(egui::Color32::from_rgb(30, 58, 138)),
                    );
                    ui.label(egui::RichText::new(self.t("messages.loading")).color(TECH_GRAY).size(get_adaptive_font_size(14.0, ctx)));
                }

                // Filter section
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(format!("{}:", self.t("buttons.filter"))).color(TECH_GRAY).size(get_adaptive_font_size(14.0, ctx)));

                    let available_width = ui.available_width();
                    let label_width = get_adaptive_font_size(50.0, ctx);
                    let button_width = get_adaptive_font_size(120.0, ctx);

                    let total_reserved_width = label_width + if !self.filter.is_empty() { button_width } else { 0.0 };
                    let filter_width = (available_width - total_reserved_width).clamp(100.0, 400.0);

                    ui.add_sized(
                        [filter_width, get_adaptive_font_size(20.0, ctx)],
                        egui::TextEdit::singleline(&mut self.filter)
                    );

                    if !self.filter.is_empty()
                        && ui.add_sized(
                            [button_width, get_adaptive_font_size(20.0, ctx)],
                            egui::Button::new(format!(
                                "{} {}",
                                egui_phosphor::regular::X,
                                self.t("buttons.clear")
                            ))
                        ).clicked()
                    {
                        self.filter.clear();
                    }
                });

                // Pre-compute translated strings to avoid borrowing issues
                let view_text = self.t("buttons.view");
                let no_metadata_text = self.t("messages.no_metadata");
                let binary_long_text = self.t("data.binary_long");
                let base64_text = self.t("data.base64");
                
                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        let mut first = true;
                        for entry in self
                            .metadata
                            .iter()
                            .filter(|entry| entry.key.contains(&self.filter) || entry.display_value.contains(&self.filter))
                        {
                            let k = &entry.key;
                            let v = &entry.display_value;
                            ui.group(|ui| {
                                ui.vertical(|ui| {
                                    ui.label(egui::RichText::new(k).color(GADGET_YELLOW).strong().size(get_adaptive_font_size(14.0, ctx)));
                                    ui.add_space(get_adaptive_font_size(4.0, ctx));
                                    
                                    if k == "tokenizer.chat_template" {
                                        if ui
                                            .button(format!(
                                                "{} {}",
                                                egui_phosphor::regular::EYE,
                                                view_text
                                            ))
                                            .clicked()
                                        {
                                            self.selected_ggml_tokens = None;
                                            self.selected_ggml_merges = None;
                                            self.selected_chat_template = entry.full_value.clone();
                                        }
                                    } else if k == "tokenizer.ggml.tokens" {
                                        if ui
                                            .button(format!(
                                                "{} {}",
                                                egui_phosphor::regular::EYE,
                                                view_text
                                            ))
                                            .clicked()
                                        {
                                            self.selected_chat_template = None;
                                            self.selected_ggml_merges = None;
                                            self.selected_ggml_tokens = entry.full_value.clone();
                                        }
                                    } else if k == "tokenizer.ggml.merges" {
                                        if ui
                                            .button(format!(
                                                "{} {}",
                                                egui_phosphor::regular::EYE,
                                                view_text
                                            ))
                                            .clicked()
                                        {
                                            self.selected_chat_template = None;
                                            self.selected_ggml_tokens = None;
                                            self.selected_ggml_merges = entry.full_value.clone();
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
                                                && let Err(e) = crate::gui::export::show_base64_dialog(v)
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
            });

        // Render dialog windows - these create their own windows so no ui parameter needed
        // We'll implement these directly here for now since the panel functions expect ui parameter
        
        // Settings dialog
        if self.show_settings {
            let base_width: f32 = if ctx.screen_rect().width() >= 1440.0 { 500.0 } else { 400.0 };
            let base_height: f32 = if ctx.screen_rect().width() >= 1440.0 { 400.0 } else { 300.0 };
            let window_size = [base_width, base_height];
            
            egui::Window::new(self.t("settings.title"))
                .resizable(true)
                .collapsible(false)
                .default_size(window_size)
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.add_space(get_adaptive_font_size(8.0, ctx));
                        
                        // Language selection section
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new(format!("{}:", self.t("settings.language")))
                                .size(get_adaptive_font_size(14.0, ctx))
                                .color(GADGET_YELLOW));
                        });
                        
                        ui.add_space(get_adaptive_font_size(4.0, ctx));
                        
                        // Language dropdown
                        let current_language = self.localization_manager.get_current_language();
                        let current_display_name = current_language.display_name();
                        
                        egui::ComboBox::from_label("")
                            .selected_text(egui::RichText::new(current_display_name).size(get_adaptive_font_size(14.0, ctx)))
                            .show_ui(ui, |ui| {
                                for language in self.localization_manager.get_available_languages() {
                                    let display_name = language.display_name();
                                    let is_selected = language == current_language;
                                    
                                    if ui.selectable_label(is_selected, 
                                        egui::RichText::new(display_name).size(get_adaptive_font_size(14.0, ctx))
                                    ).clicked() && language != current_language {
                                        // Change language immediately
                                        if let Err(e) = self.localization_manager.set_language_with_persistence(language) {
                                            eprintln!("Failed to change language: {}", e);
                                        } else {
                                            // Request repaint to update all UI text immediately
                                            ctx.request_repaint();
                                        }
                                    }
                                }
                            });
                        
                        ui.add_space(get_adaptive_font_size(4.0, ctx));
                        ui.label(egui::RichText::new(self.t("settings.language_description"))
                            .size(get_adaptive_font_size(12.0, ctx))
                            .color(TECH_GRAY));
                        
                        ui.add_space(get_adaptive_font_size(16.0, ctx));
                        
                        // Close button
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button(egui::RichText::new(self.t("buttons.close")).size(get_adaptive_font_size(14.0, ctx))).clicked() {
                                self.show_settings = false;
                            }
                        });
                    });
                });
        }

        // About dialog
        if self.show_about {
            let base_width: f32 = if ctx.screen_rect().width() >= 1440.0 { 550.0 } else { 450.0 };
            let base_height: f32 = if ctx.screen_rect().width() >= 1440.0 { 450.0 } else { 380.0 };
            let window_size = [base_width, base_height];
            
            egui::Window::new(self.t("about.title"))
                .resizable(true)
                .collapsible(false)
                .default_size(window_size)
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.heading(egui::RichText::new(self.t("app.title")).size(get_adaptive_font_size(18.0, ctx)));
                        ui.label(egui::RichText::new(format!("{}: 0.1.0", self.t("app.version"))).size(get_adaptive_font_size(14.0, ctx)));
                        ui.label(egui::RichText::new(self.t("about.description")).size(get_adaptive_font_size(14.0, ctx)));
                        ui.label(egui::RichText::new(self.t("about.built_with")).size(get_adaptive_font_size(14.0, ctx)));
                        ui.add_space(get_adaptive_font_size(8.0, ctx));

                        // License information
                        ui.label(egui::RichText::new(self.t("about.license")).size(get_adaptive_font_size(12.0, ctx)).color(GADGET_YELLOW));
                        ui.label(egui::RichText::new(self.t("info.third_party_components")).size(get_adaptive_font_size(12.0, ctx)));
                        ui.label(egui::RichText::new(self.t("info.open_source_licenses")).size(get_adaptive_font_size(12.0, ctx)));
                        ui.add_space(get_adaptive_font_size(4.0, ctx));
                        ui.label(egui::RichText::new(self.t("actions.run_cargo_license")).size(get_adaptive_font_size(11.0, ctx)).color(TECH_GRAY));
                        ui.add_space(get_adaptive_font_size(8.0, ctx));

                        ui.label(egui::RichText::new(self.t("about.copyright")).size(get_adaptive_font_size(12.0, ctx)));

                        // Update status display
                        if let Some(ref status) = self.update_status {
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new(status).size(get_adaptive_font_size(12.0, ctx)));
                                if status.contains(self.t("messages.update_available").split(':').next().unwrap_or(""))
                                    && ui.button(egui::RichText::new(self.t("actions.download")).size(get_adaptive_font_size(12.0, ctx))).clicked() {
                                    let _ = opener::open("https://github.com/FerrisMind/inspector-gguf/releases/latest");
                                }
                            });
                        }

                        ui.horizontal(|ui| {
                            // Update check button
                            if ui.button(egui::RichText::new(format!("{} {}", egui_phosphor::regular::ARROW_CLOCKWISE, self.t("about.check_updates"))).size(get_adaptive_font_size(14.0, ctx))).clicked() {
                                self.update_status = Some(self.t("messages.checking_updates"));
                                ctx.request_repaint();

                                match check_for_updates() {
                                    Ok(status) => {
                                        // Translate the status message based on content
                                        if status.starts_with("new_version_available:") {
                                            let version = status.split(':').nth(1).unwrap_or("");
                                            self.update_status = Some(self.t_with_args("messages.update_available", &[version]));
                                        } else if status == "latest_version" {
                                            self.update_status = Some(self.t("messages.up_to_date"));
                                        } else if status == "releases_not_found" {
                                            self.update_status = Some(self.t("errors.releases_not_found"));
                                        } else {
                                            self.update_status = Some(status);
                                        }
                                    }
                                    Err(e) => {
                                        let error_msg = e.to_string();
                                        if error_msg.starts_with("github_api_failed:") {
                                            let status_code = error_msg.split(':').nth(1).unwrap_or("");
                                            self.update_status = Some(self.t_with_args("errors.github_api_failed", &[status_code]));
                                        } else if error_msg == "parse_tag_failed" {
                                            self.update_status = Some(self.t("errors.parse_tag_failed"));
                                        } else {
                                            self.update_status = Some(self.t_with_args("messages.update_error", &[&error_msg]));
                                        }
                                        eprintln!("Update check failed: {}", e);
                                    }
                                }
                            }

                            // GitHub button
                            if ui.button(egui::RichText::new(format!("{} {}", egui_phosphor::regular::GITHUB_LOGO, self.t("about.github"))).size(get_adaptive_font_size(14.0, ctx))).clicked() {
                                let _ = opener::open("https://github.com/FerrisMind/inspector-gguf");
                            }

                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.button(egui::RichText::new(self.t("buttons.close")).size(get_adaptive_font_size(14.0, ctx))).clicked() {
                                    self.show_about = false;
                                }
                            });
                        });
                    });
                });
        }
    }
}

impl LanguageProvider for GgufApp {
    fn t(&self, key: &str) -> String {
        self.localization_manager.get_text(key)
    }
    
    fn t_with_args(&self, key: &str, args: &[&str]) -> String {
        let mut text = self.localization_manager.get_text(key);
        
        // Simple argument substitution using {0}, {1}, etc.
        for (i, arg) in args.iter().enumerate() {
            let placeholder = format!("{{{}}}", i);
            text = text.replace(&placeholder, arg);
        }
        
        text
    }
}