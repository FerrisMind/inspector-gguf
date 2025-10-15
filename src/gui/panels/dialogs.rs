// Dialog panels functionality
// Handles settings dialog, about dialog, and right-side content panels

use eframe::egui;
use crate::localization::{LanguageProvider, LocalizationManager};
use crate::gui::layout::get_adaptive_font_size;
use crate::gui::theme::{GADGET_YELLOW, TECH_GRAY};
use crate::gui::updater::check_for_updates;

pub fn render_settings_dialog<T: LanguageProvider>(
    ctx: &egui::Context,
    _ui: &mut egui::Ui,
    app: &mut T,
    show_settings: &mut bool,
    localization_manager: &mut LocalizationManager,
) {
    // Calculate adaptive window size based on content
    let base_width: f32 = if ctx.screen_rect().width() >= 1440.0 { 500.0 } else { 400.0 };
    let base_height: f32 = if ctx.screen_rect().width() >= 1440.0 { 400.0 } else { 300.0 };
    let window_size = [base_width, base_height];
    
    egui::Window::new(app.t("settings.title"))
        .resizable(true) // Allow resizing for better adaptation
        .collapsible(false)
        .default_size(window_size)
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.add_space(get_adaptive_font_size(8.0, ctx));
                
                // Language selection section
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(format!("{}:", app.t("settings.language")))
                        .size(get_adaptive_font_size(14.0, ctx))
                        .color(GADGET_YELLOW));
                });
                
                ui.add_space(get_adaptive_font_size(4.0, ctx));
                
                // Language dropdown
                let current_language = localization_manager.get_current_language();
                let current_display_name = current_language.display_name();
                
                egui::ComboBox::from_label("")
                    .selected_text(egui::RichText::new(current_display_name).size(get_adaptive_font_size(14.0, ctx)))
                    .show_ui(ui, |ui| {
                        for language in localization_manager.get_available_languages() {
                            let display_name = language.display_name();
                            let is_selected = language == current_language;
                            
                            if ui.selectable_label(is_selected, 
                                egui::RichText::new(display_name).size(get_adaptive_font_size(14.0, ctx))
                            ).clicked() && language != current_language {
                                // Change language immediately
                                if let Err(e) = localization_manager.set_language_with_persistence(language) {
                                    eprintln!("Failed to change language: {}", e);
                                } else {
                                    // Request repaint to update all UI text immediately
                                    ctx.request_repaint();
                                }
                            }
                        }
                    });
                
                ui.add_space(get_adaptive_font_size(4.0, ctx));
                ui.label(egui::RichText::new(app.t("settings.language_description"))
                    .size(get_adaptive_font_size(12.0, ctx))
                    .color(TECH_GRAY));
                
                ui.add_space(get_adaptive_font_size(16.0, ctx));
                
                // Close button
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button(egui::RichText::new(app.t("buttons.close")).size(get_adaptive_font_size(14.0, ctx))).clicked() {
                        *show_settings = false;
                    }
                });
            });
        });
}

pub fn render_about_dialog<T: LanguageProvider>(
    ctx: &egui::Context,
    _ui: &mut egui::Ui,
    app: &mut T,
    show_about: &mut bool,
    update_status: &mut Option<String>,
) {
    // Calculate adaptive window size based on content
    let base_width: f32 = if ctx.screen_rect().width() >= 1440.0 { 550.0 } else { 450.0 };
    let base_height: f32 = if ctx.screen_rect().width() >= 1440.0 { 450.0 } else { 380.0 };
    let window_size = [base_width, base_height];
    
    egui::Window::new(app.t("about.title"))
        .resizable(true) // Allow resizing for better adaptation
        .collapsible(false)
        .default_size(window_size)
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.heading(egui::RichText::new(app.t("app.title")).size(get_adaptive_font_size(18.0, ctx)));
                ui.label(egui::RichText::new(format!("{}: 0.1.0", app.t("app.version"))).size(get_adaptive_font_size(14.0, ctx)));
                ui.label(egui::RichText::new(app.t("about.description")).size(get_adaptive_font_size(14.0, ctx)));
                ui.label(egui::RichText::new(app.t("about.built_with")).size(get_adaptive_font_size(14.0, ctx)));
                ui.add_space(get_adaptive_font_size(8.0, ctx));

                // Информация о лицензиях
                ui.label(egui::RichText::new(app.t("about.license")).size(get_adaptive_font_size(12.0, ctx)).color(GADGET_YELLOW));
                ui.label(egui::RichText::new(app.t("info.third_party_components")).size(get_adaptive_font_size(12.0, ctx)));
                ui.label(egui::RichText::new(app.t("info.open_source_licenses")).size(get_adaptive_font_size(12.0, ctx)));
                ui.add_space(get_adaptive_font_size(4.0, ctx));
                ui.label(egui::RichText::new(app.t("actions.run_cargo_license")).size(get_adaptive_font_size(11.0, ctx)).color(TECH_GRAY));
                ui.add_space(get_adaptive_font_size(8.0, ctx));

                ui.label(egui::RichText::new(app.t("about.copyright")).size(get_adaptive_font_size(12.0, ctx)));

                // Update status display
                if let Some(status) = update_status {
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new(status.as_str()).size(get_adaptive_font_size(12.0, ctx)));
                        if status.contains(app.t("messages.update_available").split(':').next().unwrap_or(""))
                            && ui.button(egui::RichText::new(app.t("actions.download")).size(get_adaptive_font_size(12.0, ctx))).clicked() {
                            let _ = opener::open("https://github.com/FerrisMind/inspector-gguf/releases/latest");
                        }
                    });
                }

                ui.horizontal(|ui| {
                    // Кнопка проверки обновлений
                    if ui.button(egui::RichText::new(format!("{} {}", egui_phosphor::regular::ARROW_CLOCKWISE, app.t("about.check_updates"))).size(get_adaptive_font_size(14.0, ctx))).clicked() {
                        *update_status = Some(app.t("messages.checking_updates"));
                        ctx.request_repaint();

                        match check_for_updates() {
                            Ok(status) => {
                                // Translate the status message based on content
                                if status.starts_with("new_version_available:") {
                                    let version = status.split(':').nth(1).unwrap_or("");
                                    *update_status = Some(app.t_with_args("messages.update_available", &[version]));
                                } else if status == "latest_version" {
                                    *update_status = Some(app.t("messages.up_to_date"));
                                } else if status == "releases_not_found" {
                                    *update_status = Some(app.t("errors.releases_not_found"));
                                } else {
                                    *update_status = Some(status);
                                }
                            }
                            Err(e) => {
                                let error_msg = e.to_string();
                                if error_msg.starts_with("github_api_failed:") {
                                    let status_code = error_msg.split(':').nth(1).unwrap_or("");
                                    *update_status = Some(app.t_with_args("errors.github_api_failed", &[status_code]));
                                } else if error_msg == "parse_tag_failed" {
                                    *update_status = Some(app.t("errors.parse_tag_failed"));
                                } else {
                                    *update_status = Some(app.t_with_args("messages.update_error", &[&error_msg]));
                                }
                                eprintln!("Update check failed: {}", e);
                            }
                        }
                    }

                    // Кнопка GitHub
                    if ui.button(egui::RichText::new(format!("{} {}", egui_phosphor::regular::GITHUB_LOGO, app.t("about.github"))).size(get_adaptive_font_size(14.0, ctx))).clicked() {
                        let _ = opener::open("https://github.com/FerrisMind/inspector-gguf");
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button(egui::RichText::new(app.t("buttons.close")).size(get_adaptive_font_size(14.0, ctx))).clicked() {
                            *show_about = false;
                        }
                    });
                });
            });
        });
}

pub fn render_right_side_panels(
    ctx: &egui::Context,
    selected_chat_template: &mut Option<String>,
    selected_ggml_tokens: &mut Option<String>,
    selected_ggml_merges: &mut Option<String>,
    t_chat_template: &str,
    t_ggml_tokens: &str,
    t_ggml_merges: &str,
) {
    // Панель для chat template
    if selected_chat_template.is_some() {
        let right_panel_width = if ctx.screen_rect().width() >= 1920.0 {
            500.0
        } else if ctx.screen_rect().width() >= 1440.0 {
            450.0
        } else {
            400.0
        };
        // Адаптивная минимальная ширина панели
        let right_panel_min_width = if ctx.screen_rect().width() >= 1920.0 {
            450.0 // На больших экранах минимум 450px
        } else if ctx.screen_rect().width() >= 1440.0 {
            400.0 // На средних экранах минимум 400px
        } else if ctx.screen_rect().width() >= 1024.0 {
            350.0 // На планшетах минимум 350px
        } else {
            300.0 // На маленьких экранах минимум 300px
        };
        egui::SidePanel::right("chat_template_panel")
            .resizable(true)
            .default_width(right_panel_width)
            .min_width(right_panel_min_width)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.add_space(4.0); // Отступ сверху для заголовка

                    // Заголовок с кнопками Copy и X
                    ui.horizontal(|ui| {
                        // Кнопка Copy слева
                        #[allow(clippy::collapsible_if)]
                        if ui.button(egui_phosphor::regular::COPY).clicked() {
                            if let Some(content) = selected_chat_template {
                                ctx.copy_text(content.clone());
                            }
                        }

                        // Центрируем заголовок в оставшемся пространстве
                        let available_size = ui.available_size_before_wrap();
                        ui.allocate_ui_with_layout(
                            available_size,
                            egui::Layout::left_to_right(egui::Align::Center),
                            |ui| {
                        ui.heading(
                            egui::RichText::new(t_chat_template).color(GADGET_YELLOW).size(get_adaptive_font_size(16.0, ctx)),
                        );
                            },
                        );

                        // Кнопка X прижата к правому краю
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button(egui_phosphor::regular::X).clicked() {
                                *selected_chat_template = None;
                            }
                        });
                    });
                    ui.add_space(8.0);

                    // ScrollArea для содержимого
                    if let Some(content) = selected_chat_template {
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            ui.label(egui::RichText::new(content.as_str()).monospace().color(TECH_GRAY).size(get_adaptive_font_size(12.0, ctx)));
                        });
                    }
                });
            });
    }

    // Панель для ggml tokens
    if selected_ggml_tokens.is_some() {
        let right_panel_width = if ctx.screen_rect().width() >= 1920.0 {
            500.0
        } else if ctx.screen_rect().width() >= 1440.0 {
            450.0
        } else {
            400.0
        };
        // Адаптивная минимальная ширина панели
        let right_panel_min_width = if ctx.screen_rect().width() >= 1920.0 {
            450.0 // На больших экранах минимум 450px
        } else if ctx.screen_rect().width() >= 1440.0 {
            400.0 // На средних экранах минимум 400px
        } else if ctx.screen_rect().width() >= 1024.0 {
            350.0 // На планшетах минимум 350px
        } else {
            300.0 // На маленьких экранах минимум 300px
        };
        egui::SidePanel::right("ggml_tokens_panel")
            .resizable(true)
            .default_width(right_panel_width)
            .min_width(right_panel_min_width)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.add_space(4.0); // Отступ сверху для заголовка

                    // Заголовок с кнопками Copy и X
                    ui.horizontal(|ui| {
                        // Кнопка Copy слева
                        #[allow(clippy::collapsible_if)]
                        if ui.button(egui_phosphor::regular::COPY).clicked() {
                            if let Some(content) = selected_ggml_tokens {
                                ctx.copy_text(content.clone());
                            }
                        }

                        // Центрируем заголовок в оставшемся пространстве
                        let available_size = ui.available_size_before_wrap();
                        ui.allocate_ui_with_layout(
                            available_size,
                            egui::Layout::left_to_right(egui::Align::Center),
                            |ui| {
                        ui.heading(
                            egui::RichText::new(t_ggml_tokens).color(GADGET_YELLOW).size(get_adaptive_font_size(16.0, ctx)),
                        );
                            },
                        );

                        // Кнопка X прижата к правому краю
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button(egui_phosphor::regular::X).clicked() {
                                *selected_ggml_tokens = None;
                            }
                        });
                    });
                    ui.add_space(8.0);

                    // ScrollArea для содержимого
                    if let Some(content) = selected_ggml_tokens {
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            ui.label(egui::RichText::new(content.as_str()).monospace().color(TECH_GRAY).size(get_adaptive_font_size(12.0, ctx)));
                        });
                    }
                });
            });
    }

    // Панель для ggml merges
    if selected_ggml_merges.is_some() {
        let right_panel_width = if ctx.screen_rect().width() >= 1920.0 {
            500.0
        } else if ctx.screen_rect().width() >= 1440.0 {
            450.0
        } else {
            400.0
        };
        // Адаптивная минимальная ширина панели
        let right_panel_min_width = if ctx.screen_rect().width() >= 1920.0 {
            450.0 // На больших экранах минимум 450px
        } else if ctx.screen_rect().width() >= 1440.0 {
            400.0 // На средних экранах минимум 400px
        } else if ctx.screen_rect().width() >= 1024.0 {
            350.0 // На планшетах минимум 350px
        } else {
            300.0 // На маленьких экранах минимум 300px
        };
        egui::SidePanel::right("ggml_merges_panel")
            .resizable(true)
            .default_width(right_panel_width)
            .min_width(right_panel_min_width)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.add_space(4.0); // Отступ сверху для заголовка

                    // Заголовок с кнопками Copy и X
                    ui.horizontal(|ui| {
                        // Кнопка Copy слева
                        #[allow(clippy::collapsible_if)]
                        if ui.button(egui_phosphor::regular::COPY).clicked() {
                            if let Some(content) = selected_ggml_merges {
                                ctx.copy_text(content.clone());
                            }
                        }

                        // Центрируем заголовок в оставшемся пространстве
                        let available_size = ui.available_size_before_wrap();
                        ui.allocate_ui_with_layout(
                            available_size,
                            egui::Layout::left_to_right(egui::Align::Center),
                            |ui| {
                        ui.heading(
                            egui::RichText::new(t_ggml_merges).color(GADGET_YELLOW).size(get_adaptive_font_size(16.0, ctx)),
                        );
                            },
                        );

                        // Кнопка X прижата к правому краю
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button(egui_phosphor::regular::X).clicked() {
                                *selected_ggml_merges = None;
                            }
                        });
                    });
                    ui.add_space(8.0);

                    // ScrollArea для содержимого
                    if let Some(content) = selected_ggml_merges {
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            ui.label(egui::RichText::new(content.as_str()).monospace().color(TECH_GRAY).size(get_adaptive_font_size(12.0, ctx)));
                        });
                    }
                });
            });
    }
}