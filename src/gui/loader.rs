//! Asynchronous GGUF file loading with progress tracking.
//!
//! This module provides background file loading capabilities for GGUF files with
//! real-time progress reporting and thread-safe result handling. The loading system
//! is designed to keep the UI responsive during potentially long-running file
//! operations while providing detailed progress feedback to users.
//!
//! # Architecture
//!
//! The loading system uses a multi-threaded approach:
//!
//! - **Main Thread**: Handles UI updates and progress display
//! - **Worker Thread**: Performs file I/O and GGUF parsing
//! - **Shared State**: Thread-safe progress and result communication
//!
//! # Progress Tracking
//!
//! Progress is reported through several phases:
//!
//! 1. **File Opening** (0-5%): Initial file access and validation
//! 2. **Reading** (5-80%): Chunked file reading with real-time updates
//! 3. **Parsing** (80-95%): GGUF format parsing and validation
//! 4. **Processing** (95-100%): Metadata extraction and formatting
//!
//! # Usage
//!
//! ## Basic Async Loading
//!
//! ```rust
//! use inspector_gguf::gui::loader::{load_gguf_metadata_async, LoadingResult};
//! use std::sync::{Arc, Mutex};
//! use std::path::PathBuf;
//!
//! let progress = Arc::new(Mutex::new(0.0f32));
//! let result: LoadingResult = Arc::new(Mutex::new(None));
//! let path = PathBuf::from("model.gguf");
//!
//! // Start async loading (non-blocking)
//! load_gguf_metadata_async(path, progress.clone(), result.clone());
//!
//! // Check progress in UI loop
//! let current_progress = *progress.lock().unwrap();
//! if current_progress >= 1.0 {
//!     if let Some(load_result) = result.lock().unwrap().take() {
//!         match load_result {
//!             Ok(metadata) => println!("Loaded {} entries", metadata.len()),
//!             Err(e) => eprintln!("Loading failed: {}", e),
//!         }
//!     }
//! }
//! ```

use std::fs::File;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use crate::format::{readable_value_for_key, get_full_tokenizer_content};

/// Type alias for thread-safe loading result container.
///
/// This type represents a shared, thread-safe container for loading results that can
/// be accessed from both the worker thread (for writing results) and the main thread
/// (for reading results). The nested structure provides:
///
/// - **Arc<Mutex<...>>**: Thread-safe shared ownership
/// - **Option<...>**: Indicates whether a result is available
/// - **Result<Vec<...>, String>**: Success with metadata or error with message
/// - **Vec<(String, String, `Option<String>`)>**: Metadata entries with key, display value, and optional full content
pub type LoadingResult = Arc<Mutex<Option<Result<Vec<(String, String, Option<String>)>, String>>>>;

/// Represents a single metadata entry from a GGUF file.
///
/// This structure contains both the display-optimized and full content versions
/// of metadata values, allowing the UI to show abbreviated content while preserving
/// access to complete data for detailed viewing or export operations.
///
/// # Fields
///
/// * `key` - The metadata key identifier (e.g., "model.name", "tokenizer.chat_template")
/// * `display_value` - Formatted value optimized for UI display (may be truncated or summarized)
/// * `full_value` - Complete original value for detailed viewing (None if same as display_value)
///
/// # Examples
///
/// ```rust
/// use inspector_gguf::gui::loader::MetadataEntry;
///
/// // Simple metadata entry
/// let entry = MetadataEntry {
///     key: "model.name".to_string(),
///     display_value: "llama-7b-chat".to_string(),
///     full_value: None, // Same as display value
/// };
///
/// // Large content with separate display and full values
/// let large_entry = MetadataEntry {
///     key: "tokenizer.chat_template".to_string(),
///     display_value: "Large template content...".to_string(),
///     full_value: Some("Full template content here...".to_string()),
/// };
/// ```
#[derive(Clone)]
pub struct MetadataEntry {
    /// The metadata key identifier (e.g., "model.name", "tokenizer.chat_template").
    pub key: String,
    /// Formatted value optimized for UI display (may be truncated or summarized).
    pub display_value: String,
    /// Complete original value for detailed viewing (None if same as display_value).
    pub full_value: Option<String>,
}

/// Loads GGUF metadata asynchronously with progress tracking.
///
/// This function initiates background loading of a GGUF file, providing real-time
/// progress updates and thread-safe result delivery. The operation is non-blocking,
/// allowing the UI to remain responsive during file processing.
///
/// # Loading Process
///
/// 1. **File Validation** (0-5%): Opens and validates file access
/// 2. **Chunked Reading** (5-80%): Reads file in 256KB chunks with progress updates
/// 3. **GGUF Parsing** (80-95%): Parses GGUF format using Candle library
/// 4. **Metadata Processing** (95-100%): Extracts and formats metadata entries
///
/// # Progress Reporting
///
/// Progress values have special meanings:
/// - **0.0 to 1.0**: Normal progress from start to completion
/// - **Negative values**: Indicate errors occurred during loading
/// - **1.0**: Loading completed successfully
///
/// # Parameters
///
/// * `path` - Path to the GGUF file to load
/// * `progress` - Shared progress indicator (0.0 to 1.0, negative for errors)
/// * `result` - Shared result container for metadata or error messages
///
/// # Thread Safety
///
/// This function spawns a new thread for file operations. The progress and result
/// parameters use Arc<Mutex<>> for safe cross-thread communication.
///
/// The function integrates with [`crate::format::load_gguf_metadata_with_full_content_sync`]
/// for file parsing and works with [`crate::gui::GgufApp`] for UI integration.
///
/// # Examples
///
/// ## Basic Usage
///
/// ```rust
/// use inspector_gguf::gui::loader::{load_gguf_metadata_async, LoadingResult};
/// use std::sync::{Arc, Mutex};
/// use std::path::PathBuf;
///
/// let progress = Arc::new(Mutex::new(0.0f32));
/// let result: LoadingResult = Arc::new(Mutex::new(None));
/// let path = PathBuf::from("model.gguf");
///
/// // Start loading (returns immediately)
/// load_gguf_metadata_async(path, progress.clone(), result.clone());
///
/// // Monitor progress in your UI loop
/// loop {
///     let current_progress = *progress.lock().unwrap();
///     
///     if current_progress < 0.0 {
///         println!("Loading failed");
///         break;
///     } else if current_progress >= 1.0 {
///         if let Some(load_result) = result.lock().unwrap().take() {
///             match load_result {
///                 Ok(metadata) => println!("Loaded {} entries", metadata.len()),
///                 Err(e) => println!("Error: {}", e),
///             }
///         }
///         break;
///     } else {
///         println!("Progress: {:.1}%", current_progress * 100.0);
///     }
///     
///     std::thread::sleep(std::time::Duration::from_millis(100));
/// }
/// ```
///
/// # Error Handling
///
/// Errors are communicated through both the progress indicator (negative values)
/// and the result container (Err variant). Common error scenarios include:
///
/// - File not found or inaccessible
/// - Invalid GGUF format
/// - Insufficient memory for large files
/// - I/O errors during reading
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