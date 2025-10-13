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
        // Обновляем прогресс
        let current_progress = if let Ok(progress) = self.loading_progress.try_lock() {
            *progress
        } else {
            0.0 // Значение по умолчанию если не удается получить доступ
        };

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

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("GGUF Metadata Viewer");

            // Показываем progressbar если идет загрузка
            if self.loading {
                ui.add(egui::ProgressBar::new(current_progress).show_percentage());
                ui.label("Загрузка файла...");
            }

            ui.horizontal(|ui| {
                if ui.button("Load GGUF File").clicked()
                    && !self.loading
                    && let Some(path) = FileDialog::new().pick_file()
                {
                    self.loading = true;
                    *self.loading_progress.lock().unwrap() = 0.0;
                    *self.loading_result.lock().unwrap() = None;

                    // Клонируем Arc для передачи в поток
                    let progress_clone = Arc::clone(&self.loading_progress);
                    let result_clone = Arc::clone(&self.loading_result);

                    load_gguf_metadata_async(path, progress_clone, result_clone);
                }
                if ui.button("Clear").clicked() {
                    self.metadata.clear();
                }

                ui.separator();
                if ui.button("Open Profiler (Web)").clicked()
                    && let Err(e) = opener::open("http://127.0.0.1:8585")
                {
                    eprintln!("Failed to open profiler: {}", e);
                }
                if ui.button("Export CSV").clicked()
                    && let Some(path) = FileDialog::new().save_file()
                    && let Err(e) = export_csv(&self.metadata, &path)
                {
                    eprintln!("CSV export failed: {}", e);
                }
                if ui.button("Export YAML").clicked()
                    && let Some(path) = FileDialog::new().save_file()
                    && let Err(e) = export_yaml(&self.metadata, &path)
                {
                    eprintln!("YAML export failed: {}", e);
                }
            });

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
                    ui.label(format!("{}:", k));
                    // If value looks binary (non-utf8 or too long), offer Base64 view
                    if v.len() > 1024 || v.contains("\0") {
                        ui.horizontal(|ui| {
                            ui.label("<binary> (long)");
                            if ui.button("View Base64").clicked() {
                                // show in separate window
                                if let Err(e) = show_base64_dialog(v) {
                                    eprintln!("Base64 view failed: {}", e);
                                }
                            }
                        });
                    } else {
                        ui.label(v);
                    }
                    ui.separator();
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
    let mut wtr = csv::Writer::from_path(path)?;
    wtr.write_record(["key", "value"])?;
    for (k, v) in metadata {
        wtr.write_record([k, v])?;
    }
    wtr.flush()?;
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
