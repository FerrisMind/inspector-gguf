use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use eframe::egui;
use egui::{FontData, FontDefinitions, FontFamily};
use inspector_gguf::format::readable_value;
use rfd::FileDialog;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

// Type alias для сложного типа результата загрузки
type LoadingResult = Arc<Mutex<Option<Result<Vec<(String, String)>, String>>>>;

// Theme colors (Inspector Gadget palette)
const INSPECTOR_BLUE: egui::Color32 = egui::Color32::from_rgb(30, 58, 138);
const GADGET_YELLOW: egui::Color32 = egui::Color32::from_rgb(251, 191, 36);
const TECH_GRAY: egui::Color32 = egui::Color32::from_rgb(148, 163, 184);
#[allow(dead_code)]
const DANGER_RED: egui::Color32 = egui::Color32::from_rgb(239, 68, 68);
#[allow(dead_code)]
const SUCCESS_GREEN: egui::Color32 = egui::Color32::from_rgb(16, 185, 129);

const SIDEBAR_WIDTH: f32 = 120.0;

fn load_custom_font(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        "rubik_distressed".to_owned(),
        std::sync::Arc::new(FontData::from_static(include_bytes!(
            "../assets/fonts/RubikDistressed-Regular.ttf"
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

    ctx.set_fonts(fonts);
}

fn apply_inspector_theme(ctx: &egui::Context) {
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
    visuals.selection.bg_fill = GADGET_YELLOW;
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

    // Типографика
    let mut text_styles = BTreeMap::new();
    text_styles.insert(
        egui::TextStyle::Heading,
        egui::FontId::new(16.0, egui::FontFamily::Proportional),
    );
    text_styles.insert(
        egui::TextStyle::Body,
        egui::FontId::new(14.0, egui::FontFamily::Proportional),
    );
    text_styles.insert(
        egui::TextStyle::Button,
        egui::FontId::new(14.0, egui::FontFamily::Proportional),
    );
    text_styles.insert(
        egui::TextStyle::Small,
        egui::FontId::new(12.0, egui::FontFamily::Proportional),
    );
    style.text_styles = text_styles;

    // Отступы и размеры
    style.spacing.item_spacing = egui::vec2(12.0, 12.0);
    style.spacing.button_padding = egui::vec2(12.0, 8.0);
    style.spacing.indent = 20.0;
    style.spacing.slider_width = 160.0;
    style.spacing.interact_size = egui::vec2(80.0, 32.0);

    // Применяем визуальные настройки через Style
    style.visuals = visuals;
    ctx.set_style(style);
}

pub struct GgufApp {
    pub metadata: Vec<(String, String)>,
    pub filter: String,
    pub loading: bool,
    pub loading_progress: Arc<Mutex<f32>>,
    pub loading_result: LoadingResult,
}

impl Default for GgufApp {
    fn default() -> Self {
        Self {
            metadata: Vec::new(),
            filter: String::new(),
            loading: false,
            loading_progress: Arc::new(Mutex::new(0.0)),
            loading_result: Arc::new(Mutex::new(None)),
        }
    }
}

impl eframe::App for GgufApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        puffin::GlobalProfiler::lock().new_frame();

        // Загружаем кастомный шрифт
        load_custom_font(ctx);

        // Обновляем прогресс
        let current_progress = if let Ok(progress) = self.loading_progress.try_lock() {
            *progress
        } else {
            0.0 // Значение по умолчанию если не удается получить доступ
        };

        // Применяем тему Inspector Gadget
        apply_inspector_theme(ctx);

        // Inspector Gadget Header
        egui::TopBottomPanel::top("inspector_header").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Логотип Inspector Gadget (увеличительное стекло)
                ui.add_space(8.0);
                ui.label(egui::RichText::new(egui_phosphor::regular::MAGNIFYING_GLASS).size(20.0));
                ui.add_space(8.0);

                // Заголовок приложения
                ui.vertical(|ui| {
                    ui.heading(
                        egui::RichText::new("Inspector GGUF")
                            .color(egui::Color32::WHITE)
                            .size(16.0),
                    );
                    ui.label(
                        egui::RichText::new("Case Analysis Tool")
                            .color(GADGET_YELLOW)
                            .size(12.0),
                    );
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Статус операции
                    if self.loading {
                        ui.label(egui::RichText::new("🔄 Scanning...").color(GADGET_YELLOW));
                    } else if !self.metadata.is_empty() {
                        ui.label(egui::RichText::new("✅ Case Loaded").color(SUCCESS_GREEN));
                    } else {
                        ui.label(
                            egui::RichText::new("📋 Ready for Investigation").color(GADGET_YELLOW),
                        );
                    }
                });
            });
        });

        if self.loading {
            if current_progress < 0.0 {
                self.loading = false; // Ошибка
            } else if current_progress >= 1.0 {
                // Проверяем результат загрузки
                if let Ok(mut result) = self.loading_result.try_lock()
                    && let Some(load_result) = result.take()
                {
                    self.loading = false;
                    match load_result {
                        Ok(metadata) => {
                            self.metadata = metadata;
                        }
                        Err(e) => {
                            eprintln!("Ошибка загрузки: {}", e);
                        }
                    }
                }
            }
        }

        egui::SidePanel::left("inspector_toolkit")
            .resizable(false)
            .exact_width(SIDEBAR_WIDTH)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(egui::RichText::new(egui_phosphor::regular::TARGET).size(16.0));
                    ui.heading(
                        egui::RichText::new("Mission Control")
                            .color(GADGET_YELLOW)
                            .size(12.0),
                    );
                    ui.label(
                        egui::RichText::new("Inspector's Toolkit")
                            .color(TECH_GRAY)
                            .size(12.0),
                    );
                });
                ui.add_space(8.0);

                if ui
                    .add_sized([100.0, 34.0], egui::Button::new("Load"))
                    .clicked()
                    && !self.loading
                    && let Some(path) = FileDialog::new().pick_file()
                {
                    self.loading = true;
                    *self.loading_progress.lock().unwrap() = 0.0;
                    *self.loading_result.lock().unwrap() = None;

                    let progress_clone = Arc::clone(&self.loading_progress);
                    let result_clone = Arc::clone(&self.loading_result);
                    load_gguf_metadata_async(path, progress_clone, result_clone);
                }

                if ui
                    .add_sized([100.0, 34.0], egui::Button::new("Clear"))
                    .clicked()
                {
                    self.metadata.clear();
                }

                ui.separator();
                if ui
                    .add_sized([100.0, 34.0], egui::Button::new("Profiler"))
                    .clicked()
                    && let Err(e) = opener::open("http://127.0.0.1:8585")
                {
                    eprintln!("Failed to open profiler: {}", e);
                }

                ui.add_space(8.0);
                ui.label(egui::RichText::new("Export:").color(TECH_GRAY));
                if ui
                    .add_sized([100.0, 28.0], egui::Button::new("CSV"))
                    .clicked()
                    && let Some(path) = FileDialog::new().save_file()
                    && let Err(e) = export_csv(&self.metadata, &path)
                {
                    eprintln!("CSV export failed: {}", e);
                }
                if ui
                    .add_sized([100.0, 28.0], egui::Button::new("YAML"))
                    .clicked()
                    && let Some(path) = FileDialog::new().save_file()
                    && let Err(e) = export_yaml(&self.metadata, &path)
                {
                    eprintln!("YAML export failed: {}", e);
                }
                if ui
                    .add_sized([100.0, 28.0], egui::Button::new("MD"))
                    .clicked()
                    && let Some(path) = FileDialog::new().save_file()
                    && let Err(e) = export_markdown_to_file(&self.metadata, &path)
                {
                    eprintln!("Markdown export failed: {}", e);
                }
                if ui
                    .add_sized([100.0, 28.0], egui::Button::new("HTML"))
                    .clicked()
                    && let Some(path) = FileDialog::new().save_file()
                    && let Err(e) = export_html_to_file(&self.metadata, &path)
                {
                    eprintln!("HTML export failed: {}", e);
                }
                if ui
                    .add_sized([100.0, 28.0], egui::Button::new("PDF"))
                    .clicked()
                    && let Some(path) = FileDialog::new().save_file()
                {
                    let md = export_markdown(&self.metadata);
                    if let Err(e) = export_pdf_from_markdown(&md, &path) {
                        eprintln!("PDF export failed: {}", e);
                    }
                }
            });

        egui::CentralPanel::default().frame(egui::Frame::central_panel(&ctx.style()).fill(egui::Color32::from_rgb(12, 18, 26))).show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(egui::RichText::new(egui_phosphor::regular::CHART_BAR).size(16.0));
                ui.heading(egui::RichText::new("Investigation Dashboard").color(GADGET_YELLOW).size(14.0));
                ui.label(egui::RichText::new("Case Evidence & Analysis").color(TECH_GRAY).size(12.0));
            });
            ui.add_space(12.0);

            // Drop zone: поддержка drag-n-drop файлов
            let dropped = ctx.input(|i| i.raw.dropped_files.clone());
            if !dropped.is_empty() {
                for df in dropped {
                    if !self.loading {
                        if let Some(path) = df.path {
                            self.loading = true;
                            *self.loading_progress.lock().unwrap() = 0.0;
                            *self.loading_result.lock().unwrap() = None;
                            let progress_clone = Arc::clone(&self.loading_progress);
                            let result_clone = Arc::clone(&self.loading_result);
                            load_gguf_metadata_async(path, progress_clone, result_clone);
                        } else if let Some(bytes) = df.bytes {
                            // Сохраняем во временный файл и загружаем
                            let tmp = std::env::temp_dir().join(&df.name);
                            match std::fs::write(&tmp, &*bytes) {
                                Ok(_) => {
                                    self.loading = true;
                                    *self.loading_progress.lock().unwrap() = 0.0;
                                    *self.loading_result.lock().unwrap() = None;
                                    let progress_clone = Arc::clone(&self.loading_progress);
                                    let result_clone = Arc::clone(&self.loading_result);
                                    load_gguf_metadata_async(tmp, progress_clone, result_clone);
                                }
                                Err(e) => eprintln!("Failed to write dropped file: {}", e),
                            }
                        }
                    }
                }
            }

            // Показываем progressbar если идет загрузка
            if self.loading {
                ui.add(
                    egui::ProgressBar::new(current_progress)
                        .show_percentage()
                        .fill(INSPECTOR_BLUE),
                );
                ui.label(egui::RichText::new("Загрузка файла...").color(TECH_GRAY));
            }

            // Toolbar moved to Mission Control side panel

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Filter:").color(TECH_GRAY));
                ui.text_edit_singleline(&mut self.filter);
                if ui.add(egui::Button::new("Apply")).clicked() { /* filter applied via iterator below */ }
                if ui.add(egui::Button::new("Clear filter")).clicked() {
                    self.filter.clear();
                }
            });

            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                let mut first = true;
                for (k, v) in self
                    .metadata
                    .iter()
                    .filter(|(k, v)| k.contains(&self.filter) || v.contains(&self.filter))
                {
                    ui.group(|ui| {
                        ui.vertical(|ui| {
                            ui.label(
                                egui::RichText::new(k)
                                    .color(GADGET_YELLOW)
                                    .strong(),
                            );
                            ui.add_space(4.0);
                            if v.len() > 1024 || v.contains("\0") {
                                ui.horizontal(|ui| {
                                    ui.label(
                                        egui::RichText::new("<binary> (long)")
                                            .color(egui::Color32::LIGHT_GRAY),
                                    );
                                    if ui.button("View Base64").clicked()
                                        && let Err(e) = show_base64_dialog(v)
                                    {
                                        eprintln!("Base64 view failed: {}", e);
                                    }
                                });
                            } else {
                                ui.label(
                                    egui::RichText::new(v)
                                        .color(egui::Color32::WHITE),
                                );
                            }
                        });
                    });
                    first = false;
                    ui.add_space(8.0);
                }
                if first {
                    ui.label(
                        egui::RichText::new("Метаданные отсутствуют")
                            .color(TECH_GRAY),
                    );
                }
            });
        });
    }
}

#[allow(dead_code)]
fn show_base64_dialog(data: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Encode string as base64 (assume original bytes are the utf-8 of data)
    let b64 = STANDARD.encode(data.as_bytes());
    // Save to temp file and open with default editor
    let tmp = std::env::temp_dir().join("gguf_metadata_base64.txt");
    std::fs::write(&tmp, b64)?;
    opener::open(&tmp)?;
    Ok(())
}

fn export_csv(
    metadata: &[(String, String)],
    path: &std::path::Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = ensure_extension(path, "csv");
    let mut wtr = csv::Writer::from_path(&path)?;
    wtr.write_record(["key", "value"])?;
    for (k, v) in metadata {
        wtr.write_record([k, v])?;
    }
    wtr.flush()?;
    Ok(())
}

fn sanitize_for_markdown(s: &str) -> String {
    // Убираем управляющие символы кроме перевода строки и таба
    s.chars()
        .map(|c| {
            if c.is_control() && c != '\n' && c != '\t' {
                ' '
            } else {
                c
            }
        })
        .collect()
}

fn escape_markdown_text(s: &str) -> String {
    // Escape characters that can break Markdown structure in headings
    s.chars()
        .map(|c| match c {
            '*' | '_' | '`' | '[' | ']' | '<' | '>' | '#' => format!("\\{}", c),
            other => other.to_string(),
        })
        .collect::<Vec<_>>()
        .join("")
}

fn ensure_extension(path: &std::path::Path, ext: &str) -> std::path::PathBuf {
    if path.extension().is_none() {
        let mut p = path.to_path_buf();
        p.set_extension(ext);
        p
    } else {
        path.to_path_buf()
    }
}

fn export_markdown(metadata: &[(String, String)]) -> String {
    let mut out = String::new();
    out.push_str("# GGUF Metadata\n\n");
    for (k, v) in metadata {
        out.push_str(&format!("## {}\n\n", escape_markdown_text(k)));
        out.push('\n');
        if v.len() > 1024 || v.contains('\0') {
            // Для больших/бинарных полей — Base64
            let b64 = STANDARD.encode(v.as_bytes());
            out.push_str("```base64\n");
            out.push_str(&b64);
            out.push_str("\n```\n\n");
        } else {
            let safe = sanitize_for_markdown(v);
            out.push_str("```\n");
            out.push_str(&safe.replace("```", "` ` `"));
            out.push_str("\n```\n\n");
        }
    }
    out
}

fn export_html(metadata: &[(String, String)]) -> Result<String, Box<dyn std::error::Error>> {
    let md = export_markdown(metadata);
    let parser = pulldown_cmark::Parser::new(&md);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    Ok(html_output)
}

fn export_markdown_to_file(
    metadata: &[(String, String)],
    path: &std::path::Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let md = export_markdown(metadata);
    let path = ensure_extension(path, "md");
    std::fs::write(&path, md)?;
    Ok(())
}

fn export_html_to_file(
    metadata: &[(String, String)],
    path: &std::path::Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let html = export_html(metadata)?;
    let path = ensure_extension(path, "html");
    std::fs::write(&path, html)?;
    Ok(())
}

fn export_pdf_from_markdown(
    md: &str,
    out_path: &std::path::Path,
) -> Result<(), Box<dyn std::error::Error>> {
    // Ensure .pdf extension and pass &str to markdown2pdf
    let out_path = ensure_extension(out_path, "pdf");
    let out_str = out_path.to_str().ok_or("output path is not valid UTF-8")?;
    // markdown2pdf can error on unexpected tokens — provide sanitized markdown
    let safe_md = sanitize_for_markdown(md);
    markdown2pdf::parse_into_file(
        safe_md.to_string(),
        out_str,
        markdown2pdf::config::ConfigSource::Default,
    )?;
    Ok(())
}

fn export_yaml(
    metadata: &[(String, String)],
    path: &std::path::Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let map: std::collections::HashMap<_, _> = metadata.iter().cloned().collect();
    let yaml = serde_yaml::to_string(&map)?;
    std::fs::write(path, yaml)?;
    Ok(())
}

fn load_gguf_metadata_async(
    path: std::path::PathBuf,
    progress: Arc<Mutex<f32>>,
    result: LoadingResult,
) {
    puffin::profile_scope!("load_gguf_metadata_async");

    thread::spawn(move || {
        puffin::profile_scope!("file_loading_thread");
        // Начало загрузки
        *progress.lock().unwrap() = 0.0;

        // Попытка открыть файл
        let mut f = {
            puffin::profile_scope!("file_open");
            match File::open(&path) {
                Ok(file) => file,
                Err(e) => {
                    *progress.lock().unwrap() = -1.0;
                    *result.lock().unwrap() = Some(Err(format!("Не удалось открыть файл: {}", e)));
                    return;
                }
            }
        };

        // Получаем размер файла для расчета прогресса
        let file_size = {
            puffin::profile_scope!("file_metadata");
            match f.metadata() {
                Ok(metadata) => metadata.len(),
                Err(e) => {
                    *progress.lock().unwrap() = -1.0;
                    *result.lock().unwrap() =
                        Some(Err(format!("Не удалось получить размер файла: {}", e)));
                    return;
                }
            }
        };

        *progress.lock().unwrap() = 0.05;

        // Чтение файла в память по частям для отображения реального прогресса
        let mut buf = Vec::new();
        let mut bytes_read = 0u64;
        let chunk_size = 256 * 1024; // 256KB chunks для лучшей производительности
        let mut chunk = vec![0u8; chunk_size];
        let mut last_progress_update = Instant::now();
        let mut last_progress_value = 0.05;

        {
            puffin::profile_scope!("file_reading");
            loop {
                match f.read(&mut chunk) {
                    Ok(0) => break, // EOF
                    Ok(n) => {
                        buf.extend_from_slice(&chunk[..n]);
                        bytes_read += n as u64;

                        // Обновляем прогресс чтения (от 5% до 80%), но не чаще чем раз в 50мс
                        let read_progress = (bytes_read as f32 / file_size as f32) * 0.75 + 0.05;
                        let current_progress = read_progress.min(0.8);

                        // Обновляем прогресс только если прошло достаточно времени или изменение значительное
                        if last_progress_update.elapsed() > Duration::from_millis(50)
                            || (current_progress - last_progress_value).abs() > 0.01
                        {
                            *progress.lock().unwrap() = current_progress;
                            last_progress_value = current_progress;
                            last_progress_update = Instant::now();
                        }
                    }
                    Err(e) => {
                        *progress.lock().unwrap() = -1.0;
                        *result.lock().unwrap() = Some(Err(format!("Ошибка чтения файла: {}", e)));
                        return;
                    }
                }
            }
        }

        *progress.lock().unwrap() = 0.85;

        // Парсинг GGUF
        let content = {
            puffin::profile_scope!("gguf_parsing");
            let mut cursor = std::io::Cursor::new(&buf);
            match candle::quantized::gguf_file::Content::read(&mut cursor) {
                Ok(content) => content,
                Err(e) => {
                    *progress.lock().unwrap() = -1.0;
                    *result.lock().unwrap() = Some(Err(format!("Ошибка парсинга GGUF: {}", e)));
                    return;
                }
            }
        };

        *progress.lock().unwrap() = 0.95;

        // Обработка метаданных
        let mut out = Vec::new();
        {
            puffin::profile_scope!("metadata_processing");
            for (k, v) in content.metadata.iter() {
                let s = readable_value(v);
                out.push((k.clone(), s));
            }
        }

        *progress.lock().unwrap() = 1.0;
        *result.lock().unwrap() = Some(Ok(out));
    });
}
