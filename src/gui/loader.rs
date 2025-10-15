use std::fs::File;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use crate::format::{readable_value_for_key, get_full_tokenizer_content};

// Type alias for complex loading result type
pub type LoadingResult = Arc<Mutex<Option<Result<Vec<(String, String, Option<String>)>, String>>>>;

#[derive(Clone)]
pub struct MetadataEntry {
    pub key: String,
    pub display_value: String,
    pub full_value: Option<String>,
}

pub fn load_gguf_metadata_async(
    path: std::path::PathBuf,
    progress: Arc<Mutex<f32>>,
    result: LoadingResult,
) {
    puffin::profile_scope!("load_gguf_metadata_async");

    thread::spawn(move || {
        puffin::profile_scope!("file_loading_thread");
        // Start loading
        *progress.lock().unwrap() = 0.0;

        // Try to open file
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

        // Get file size for progress calculation
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

        // Read file into memory in chunks to show real progress
        let mut buf = Vec::new();
        let mut bytes_read = 0u64;
        let chunk_size = 256 * 1024; // 256KB chunks for better performance
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

                        // Update reading progress (from 5% to 80%), but not more often than once per 50ms
                        let read_progress = (bytes_read as f32 / file_size as f32) * 0.75 + 0.05;
                        let current_progress = read_progress.min(0.8);

                        // Update progress only if enough time has passed or change is significant
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

        // GGUF parsing
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

        // Process metadata
        let mut out = Vec::new();
        {
            puffin::profile_scope!("metadata_processing");
            for (k, v) in content.metadata.iter() {
                let s = readable_value_for_key(k, v);
                let full_content = get_full_tokenizer_content(k, v);
                out.push((k.clone(), s, full_content));
            }
        }

        *progress.lock().unwrap() = 1.0;
        *result.lock().unwrap() = Some(Ok(out));
    });
}