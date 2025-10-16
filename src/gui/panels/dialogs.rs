//! Dialog panels and specialized content viewers.
//!
//! This module implements modal dialogs and specialized content viewing panels
//! for the Inspector GGUF application. It provides settings management, application
//! information display, and dedicated viewers for large content such as chat
//! templates and tokenizer data.
//!
//! # Dialog Types
//!
//! ## Modal Dialogs
//! - **Settings Dialog**: Language preferences and application configuration
//! - **About Dialog**: Application information, version details, and update checking
//!
//! ## Content Panels
//! - **Chat Template Viewer**: Dedicated panel for viewing large chat templates
//! - **Token Data Viewer**: Specialized viewer for GGML tokens and merges
//! - **Right-Side Panels**: Resizable panels that don't block main content
//!
//! # Design Features
//!
//! ## Responsive Design
//! - **Adaptive Sizing**: Dialog dimensions adjust to screen size
//! - **Minimum Sizes**: Ensures usability on small screens
//! - **Scalable Typography**: Font sizes adapt to display density
//!
//! ## User Experience
//! - **Modal Behavior**: Settings and About dialogs block interaction with main UI
//! - **Non-Modal Panels**: Content viewers allow simultaneous main UI interaction
//! - **Copy Functionality**: Easy copying of large content to clipboard
//! - **Keyboard Navigation**: Standard dialog keyboard shortcuts

use eframe::egui;
use crate::localization::{LanguageProvider, LocalizationManager};
use crate::gui::layout::get_adaptive_font_size;
use crate::gui::theme::{GADGET_YELLOW, TECH_GRAY};
use crate::gui::updater::check_for_updates;

/// Renders the settings dialog for application configuration.
///
/// This function creates a modal dialog window that allows users to configure
/// application settings, primarily language preferences. The dialog provides
/// an intuitive interface for changing settings with immediate effect and
/// persistent storage of user preferences.
///
/// # Dialog Features
///
/// ## Language Selection
/// - **Dropdown Interface**: ComboBox showing available languages with display names
/// - **Immediate Application**: Language changes take effect immediately
/// - **Persistent Storage**: Settings are automatically saved to disk
/// - **Visual Feedback**: UI updates immediately to reflect language changes
///
/// ## Responsive Design
/// - **Adaptive Sizing**: Dialog size adjusts based on screen dimensions
/// - **Minimum Usability**: Maintains usable size on small screens
/// - **Scalable Elements**: All UI elements scale with screen size
///
/// # Parameters
///
/// * `ctx` - egui context for window creation and screen size detection
/// * `_ui` - UI context (unused as this creates its own window)
/// * `app` - Application instance implementing LanguageProvider for text
/// * `show_settings` - Mutable flag controlling dialog visibility
/// * `localization_manager` - Mutable reference to localization system
///
/// # Behavior
///
/// ## Window Management
/// - **Modal Dialog**: Blocks interaction with main application
/// - **Resizable**: Users can resize for better visibility
/// - **Non-Collapsible**: Prevents accidental minimization
/// - **Close Button**: Standard close button in bottom-right corner
///
/// ## Language Management
/// - **Current Selection**: Shows currently active language
/// - **Available Options**: Lists all supported languages with native names
/// - **Immediate Changes**: Language switches immediately upon selection
/// - **Error Handling**: Graceful handling of language switching failures
///
/// # Examples
///
/// ## Usage in Main Application Loop
///
/// ```rust
/// use inspector_gguf::gui::panels::render_settings_dialog;
/// use inspector_gguf::localization::{LanguageProvider, LocalizationManager};
/// use eframe::egui;
///
/// fn handle_settings_dialog<T: LanguageProvider>(
///     ctx: &egui::Context,
///     app: &mut T,
///     show_settings: &mut bool,
///     localization_manager: &mut LocalizationManager,
/// ) {
///     if *show_settings {
///         // render_settings_dialog(ctx, ui, app, show_settings, localization_manager);
///     }
/// }
/// ```
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

/// Renders the about dialog with application information and update checking.
///
/// This function creates a comprehensive about dialog that displays application
/// information, version details, licensing information, and provides update
/// checking functionality. The dialog serves as both an information source
/// and a gateway to application updates and external resources.
///
/// # Dialog Content
///
/// ## Application Information
/// - **Title and Version**: Application name and current version number
/// - **Description**: Brief description of application purpose and capabilities
/// - **Technology Stack**: Information about underlying technologies (Rust, egui)
///
/// ## Legal Information
/// - **License Details**: MIT license information and copyright notice
/// - **Third-Party Components**: Information about open source dependencies
/// - **License Commands**: Instructions for viewing detailed license information
///
/// ## Update Management
/// - **Version Checking**: Manual update check with GitHub API integration
/// - **Status Display**: Current update status with localized messages
/// - **Download Links**: Direct links to latest releases when updates are available
///
/// # Parameters
///
/// * `ctx` - egui context for window creation and screen size detection
/// * `_ui` - UI context (unused as this creates its own window)
/// * `app` - Application instance implementing LanguageProvider for text
/// * `show_about` - Mutable flag controlling dialog visibility
/// * `update_status` - Mutable reference to current update check status
///
/// # Interactive Features
///
/// ## Update Checking
/// - **Manual Check**: Button to trigger update check via GitHub API
/// - **Status Messages**: Localized status messages for different scenarios
/// - **Download Integration**: Direct browser opening for update downloads
/// - **Error Handling**: Graceful handling of network and API failures
///
/// ## External Links
/// - **GitHub Repository**: Direct link to project repository
/// - **Release Downloads**: Links to latest release downloads
/// - **Browser Integration**: Uses system default browser for external links
///
/// # Examples
///
/// ## Usage in Main Application Loop
///
/// ```rust
/// use inspector_gguf::gui::panels::render_about_dialog;
/// use inspector_gguf::localization::LanguageProvider;
/// use eframe::egui;
///
/// fn handle_about_dialog<T: LanguageProvider>(
///     ctx: &egui::Context,
///     app: &mut T,
///     show_about: &mut bool,
///     update_status: &mut Option<String>,
/// ) {
///     if *show_about {
///         // render_about_dialog(ctx, ui, app, show_about, update_status);
///     }
/// }
/// ```
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

/// Renders specialized right-side panels for viewing large content.
///
/// This function manages the display of resizable right-side panels that provide
/// dedicated viewers for large text content such as chat templates, tokenizer
/// data, and other substantial metadata values. The panels are non-modal and
/// allow simultaneous interaction with the main application interface.
///
/// # Panel Types
///
/// ## Chat Template Panel
/// - **Large Template Display**: Dedicated viewer for chat template content
/// - **Monospace Formatting**: Preserves template structure and formatting
/// - **Copy Functionality**: One-click copying to system clipboard
///
/// ## Token Data Panels
/// - **GGML Tokens**: Viewer for tokenizer vocabulary data
/// - **GGML Merges**: Viewer for byte-pair encoding merge rules
/// - **Structured Display**: Organized presentation of tokenizer information
///
/// # Panel Features
///
/// ## Responsive Design
/// - **Adaptive Width**: Panel width adjusts to screen size automatically
/// - **Minimum Width**: Ensures usability across different screen sizes
/// - **Resizable Interface**: Users can adjust panel width as needed
///
/// ## User Interface
/// - **Header Controls**: Copy button and close button in panel header
/// - **Scrollable Content**: Vertical scrolling for large content
/// - **Monospace Text**: Preserves formatting for structured data
/// - **Consistent Styling**: Matches application theme and color scheme
///
/// # Parameters
///
/// * `ctx` - egui context for panel creation and screen size calculations
/// * `selected_chat_template` - Mutable reference to chat template content
/// * `selected_ggml_tokens` - Mutable reference to token data content
/// * `selected_ggml_merges` - Mutable reference to merge data content
/// * `t_chat_template` - Localized title for chat template panel
/// * `t_ggml_tokens` - Localized title for tokens panel
/// * `t_ggml_merges` - Localized title for merges panel
///
/// # Panel Management
///
/// ## Exclusive Display
/// - **Single Panel**: Only one content panel is shown at a time
/// - **Automatic Switching**: Opening one panel closes others
/// - **Clean State**: Closing a panel clears its content reference
///
/// ## Content Handling
/// - **Large Text**: Optimized for displaying substantial text content
/// - **Copy Integration**: System clipboard integration for easy copying
/// - **Scroll Management**: Automatic scrolling for content navigation
///
/// # Examples
///
/// ## Usage in Main Application
///
/// ```rust
/// use inspector_gguf::gui::panels::render_right_side_panels;
/// use eframe::egui;
///
/// fn handle_content_panels(
///     ctx: &egui::Context,
///     selected_chat_template: &mut Option<String>,
///     selected_ggml_tokens: &mut Option<String>,
///     selected_ggml_merges: &mut Option<String>,
/// ) {
///     let t_chat_template = "Chat Template";
///     let t_ggml_tokens = "GGML Tokens";
///     let t_ggml_merges = "GGML Merges";
///
///     render_right_side_panels(
///         ctx,
///         selected_chat_template,
///         selected_ggml_tokens,
///         selected_ggml_merges,
///         &t_chat_template,
///         &t_ggml_tokens,
///         &t_ggml_merges,
///     );
/// }
/// ```
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