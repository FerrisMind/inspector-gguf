use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use eframe::egui;
use inspector_gguf::format::readable_value;
use rfd::FileDialog;
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

fn apply_inspector_theme(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();

    // Widgets
    style.visuals.widgets.inactive.bg_fill = TECH_GRAY;
    style.visuals.widgets.inactive.corner_radius = egui::CornerRadius::same(6);
    style.visuals.widgets.active.bg_fill = INSPECTOR_BLUE;
    style.visuals.widgets.active.corner_radius = egui::CornerRadius::same(6);
    style.visuals.widgets.hovered.bg_fill = GADGET_YELLOW;

    // Selection and accents
    style.visuals.selection.bg_fill = GADGET_YELLOW;
    style.visuals.override_text_color = Some(egui::Color32::WHITE);

    // Slightly tinted panels
    style.visuals.faint_bg_color = egui::Color32::from_rgb(20, 28, 36);

    ctx.set_style(style);
}

pub struct GgufApp {
    pub metadata: Vec<(String, String)>,
    pub filter: String,
    pub loading: bool,
    pub loading_progress: Arc<Mutex<f32>>,
    pub loading_result: LoadingResult,
    // theme applied flag
    pub theme_applied: bool,
}

impl Default for GgufApp {
    fn default() -> Self {
        Self {
            metadata: Vec::new(),
            filter: String::new(),
            loading: false,
            loading_progress: Arc::new(Mutex::new(0.0)),
            loading_result: Arc::new(Mutex::new(None)),
            theme_applied: false,
        }
    }
}

impl eframe::App for GgufApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        puffin::GlobalProfiler::lock().new_frame();
        // Обновляем прогресс
        let current_progress = if let Ok(progress) = self.loading_progress.try_lock() {
            *progress
        } else {
            0.0 // Значение по умолчанию если не удается получить доступ
        };

        if !self.theme_applied {
            apply_inspector_theme(ctx);
            self.theme_applied = true;
        }

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
            .default_width(120.0)
            .resizable(true)
            .show(ctx, |ui| {
                ui.heading("Mission\nControl");
                ui.add_space(6.0);

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
                ui.label("Export:");
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

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🔎 Inspector GGUF - Case Analysis Tool");

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
                ui.label("Загрузка файла...");
            }

            // Toolbar moved to Mission Control side panel

            ui.horizontal(|ui| {
                ui.label("Filter:");
                ui.text_edit_singleline(&mut self.filter);
                if ui.button("Apply").clicked() { /* filter applied via iterator below */ }
                if ui.button("Clear filter").clicked() {
                    self.filter.clear();
                }
            });

            egui::ScrollArea::vertical().show(ui, |ui| {
                for (k, v) in self
                    .metadata
                    .iter()
                    .filter(|(k, v)| k.contains(&self.filter) || v.contains(&self.filter))
                {
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.label(egui::RichText::new(k).strong());
                                ui.add_space(4.0);
                                if v.len() > 1024 || v.contains("\0") {
                                    ui.horizontal(|ui| {
                                        ui.label("<binary> (long)");
                                        if ui.button("View Base64").clicked()
                                            && let Err(e) = show_base64_dialog(v)
                                        {
                                            eprintln!("Base64 view failed: {}", e);
                                        }
                                    });
                                } else {
                                    ui.label(v);
                                }
                            });
                        });
                    });
                    ui.add_space(6.0);
                }
            });
        });
    }
}

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
