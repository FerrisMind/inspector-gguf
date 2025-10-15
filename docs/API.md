# Inspector GGUF API Documentation

This document describes the public API and interfaces provided by Inspector GGUF for library usage and integration.

## 📚 Library Usage

Inspector GGUF can be used as a library in other Rust projects. Add it to your `Cargo.toml`:

```toml
[dependencies]
inspector-gguf = "0.1.0"
```

## 🔧 Core API

### GGUF Format Handling

#### `load_gguf_metadata_with_full_content_sync`
Synchronously loads GGUF metadata with full content extraction.

```rust
use inspector_gguf::format::load_gguf_metadata_with_full_content_sync;
use std::path::Path;

pub fn load_gguf_metadata_with_full_content_sync(
    path: &Path
) -> Result<Vec<(String, String, Option<String>)>, Box<dyn std::error::Error>>

// Usage example
let path = Path::new("model.gguf");
match load_gguf_metadata_with_full_content_sync(&path) {
    Ok(metadata) => {
        for (key, display_value, full_value) in metadata {
            println!("{}: {}", key, display_value);
            if let Some(full) = full_value {
                println!("Full content available: {} bytes", full.len());
            }
        }
    }
    Err(e) => eprintln!("Failed to load metadata: {}", e),
}
```

#### `readable_value_for_key`
Converts raw GGUF values to human-readable format.

```rust
use inspector_gguf::format::readable_value_for_key;

pub fn readable_value_for_key(
    key: &str,
    value: &candle::quantized::gguf_file::Value
) -> String

// Usage example
let readable = readable_value_for_key("general.name", &gguf_value);
println!("Model name: {}", readable);
```

#### `get_full_tokenizer_content`
Extracts complete tokenizer content from GGUF metadata.

```rust
use inspector_gguf::format::get_full_tokenizer_content;

pub fn get_full_tokenizer_content(
    key: &str,
    value: &candle::quantized::gguf_file::Value
) -> Option<String>

// Usage example
if let Some(tokenizer_data) = get_full_tokenizer_content("tokenizer.json", &value) {
    println!("Tokenizer JSON: {}", tokenizer_data);
}
```

## 🎨 GUI Components

### Application State

#### `GgufApp`
Main application struct implementing `eframe::App`.

```rust
use inspector_gguf::gui::GgufApp;

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
    pub update_status: Option<String>,
    pub localization_manager: LocalizationManager,
}

// Usage example
let app = GgufApp::default();
let native_options = eframe::NativeOptions::default();
eframe::run_native("Inspector GGUF", native_options, Box::new(|_cc| Ok(Box::new(app))));
```

#### `MetadataEntry`
Represents a single metadata entry with display and full values.

```rust
pub struct MetadataEntry {
    pub key: String,
    pub display_value: String,
    pub full_value: Option<String>,
}
```

### Theme System

#### Theme Colors
Predefined color constants for consistent UI theming.

```rust
use inspector_gguf::gui::theme::{
    INSPECTOR_BLUE,
    GADGET_YELLOW,
    TECH_GRAY,
    DANGER_RED,
    SUCCESS_GREEN
};

// Usage in UI components
ui.label(egui::RichText::new("Title").color(GADGET_YELLOW));
```

#### Theme Functions

```rust
use inspector_gguf::gui::theme::{apply_inspector_theme, load_custom_font};

// Apply theme to egui context
apply_inspector_theme(ctx);

// Load custom fonts
load_custom_font(ctx);
```

### Layout Utilities

#### Responsive Sizing Functions

```rust
use inspector_gguf::gui::layout::{
    get_sidebar_width,
    get_adaptive_font_size,
    get_adaptive_button_width
};

// Get adaptive dimensions
let sidebar_width = get_sidebar_width(ctx);
let font_size = get_adaptive_font_size(14.0, ctx);
let button_width = get_adaptive_button_width(ui, "Button Text", font_size, max_width);
```

## 📤 Export System

### Export Functions

#### CSV Export
```rust
use inspector_gguf::gui::export::export_csv;
use std::path::Path;

pub fn export_csv(
    metadata: &[(&str, &str)],
    path: &Path
) -> Result<(), Box<dyn std::error::Error>>

// Usage example
let metadata_pairs: Vec<(&str, &str)> = metadata
    .iter()
    .map(|entry| (entry.key.as_str(), entry.display_value.as_str()))
    .collect();

export_csv(&metadata_pairs, Path::new("output.csv"))?;
```

#### YAML Export
```rust
use inspector_gguf::gui::export::export_yaml;

pub fn export_yaml(
    metadata: &[(&str, &str)],
    path: &Path
) -> Result<(), Box<dyn std::error::Error>>

// Usage example
export_yaml(&metadata_pairs, Path::new("output.yaml"))?;
```

#### Markdown Export
```rust
use inspector_gguf::gui::export::{export_markdown, export_markdown_to_file};

// Generate markdown string
pub fn export_markdown(metadata: &[(&str, &str)]) -> String

// Export directly to file
pub fn export_markdown_to_file(
    metadata: &[(&str, &str)],
    path: &Path
) -> Result<(), Box<dyn std::error::Error>>

// Usage example
let markdown_content = export_markdown(&metadata_pairs);
export_markdown_to_file(&metadata_pairs, Path::new("output.md"))?;
```

#### HTML Export
```rust
use inspector_gguf::gui::export::{export_html, export_html_to_file};

// Generate HTML string
pub fn export_html(metadata: &[(&str, &str)]) -> String

// Export directly to file
pub fn export_html_to_file(
    metadata: &[(&str, &str)],
    path: &Path
) -> Result<(), Box<dyn std::error::Error>>
```

#### PDF Export
```rust
use inspector_gguf::gui::export::export_pdf_from_markdown;

pub fn export_pdf_from_markdown(
    markdown_content: &str,
    path: &Path
) -> Result<(), Box<dyn std::error::Error>>

// Usage example
let markdown = export_markdown(&metadata_pairs);
export_pdf_from_markdown(&markdown, Path::new("output.pdf"))?;
```

### Utility Functions

#### File Extension Management
```rust
use inspector_gguf::gui::export::ensure_extension;

pub fn ensure_extension(path: &Path, extension: &str) -> PathBuf

// Usage example
let path_with_ext = ensure_extension(Path::new("output"), "csv");
// Returns: "output.csv"
```

#### Text Processing
```rust
use inspector_gguf::gui::export::{sanitize_for_markdown, escape_markdown_text};

// Sanitize text for markdown output
pub fn sanitize_for_markdown(text: &str) -> String

// Escape markdown special characters
pub fn escape_markdown_text(text: &str) -> String
```

#### Binary Data Handling
```rust
use inspector_gguf::gui::export::show_base64_dialog;

pub fn show_base64_dialog(data: &str) -> Result<(), Box<dyn std::error::Error>>

// Usage example
if data.len() > 1024 {
    show_base64_dialog(data)?;
}
```

## 🌍 Localization System

### Language Management

#### `LocalizationManager`
Central manager for all localization functionality.

```rust
use inspector_gguf::localization::LocalizationManager;

// Create new manager
let manager = LocalizationManager::new()?;

// Get translated text
let text = manager.get_text("buttons.load");

// Get text with arguments
let text = manager.get_text_with_args("messages.export_failed", &["File not found"]);

// Change language
manager.set_language_with_persistence(Language::Russian)?;

// Get available languages
let languages = manager.get_available_languages();
```

#### `LanguageProvider` Trait
Trait for components that need localization support.

```rust
use inspector_gguf::localization::LanguageProvider;

pub trait LanguageProvider {
    fn t(&self, key: &str) -> String;
    fn t_with_args(&self, key: &str, args: &[&str]) -> String;
}

// Implementation example
impl LanguageProvider for MyComponent {
    fn t(&self, key: &str) -> String {
        self.localization_manager.get_text(key)
    }
    
    fn t_with_args(&self, key: &str, args: &[&str]) -> String {
        let mut text = self.t(key);
        for (i, arg) in args.iter().enumerate() {
            text = text.replace(&format!("{{{}}}", i), arg);
        }
        text
    }
}
```

### Language Definitions

#### `Language` Enum
Supported languages enumeration.

```rust
use inspector_gguf::localization::Language;

pub enum Language {
    English,
    Russian,
    PortugueseBrazilian,
}

impl Language {
    pub fn code(&self) -> &'static str;
    pub fn display_name(&self) -> &'static str;
    pub fn native_name(&self) -> &'static str;
}

// Usage example
let lang = Language::Russian;
println!("Code: {}", lang.code()); // "ru"
println!("Display: {}", lang.display_name()); // "Russian"
println!("Native: {}", lang.native_name()); // "Русский"
```

### System Locale Detection

#### `SystemLocaleDetector`
Automatic system locale detection.

```rust
use inspector_gguf::localization::SystemLocaleDetector;

// Detect system language
let detected_language = SystemLocaleDetector::detect_system_language();

// Parse locale string
let language = SystemLocaleDetector::parse_locale_string("en_US");

// Validate locale
let is_valid = SystemLocaleDetector::is_valid_locale("ru_RU");
```

## 🔄 Async Operations

### File Loading

#### `load_gguf_metadata_async`
Asynchronous file loading with progress tracking.

```rust
use inspector_gguf::gui::loader::{load_gguf_metadata_async, LoadingResult};
use std::sync::{Arc, Mutex};

pub fn load_gguf_metadata_async(
    path: PathBuf,
    progress: Arc<Mutex<f32>>,
    result: LoadingResult,
)

// Usage example
let progress = Arc::new(Mutex::new(0.0));
let result = Arc::new(Mutex::new(None));

load_gguf_metadata_async(path, progress.clone(), result.clone());

// Monitor progress
loop {
    let current_progress = *progress.lock().unwrap();
    if current_progress >= 1.0 {
        break;
    }
    std::thread::sleep(std::time::Duration::from_millis(100));
}

// Get result
if let Some(load_result) = result.lock().unwrap().take() {
    match load_result {
        Ok(metadata) => println!("Loaded {} entries", metadata.len()),
        Err(e) => eprintln!("Loading failed: {}", e),
    }
}
```

## 🔄 Update System

### Update Checking

#### `check_for_updates`
Check for application updates from GitHub releases.

```rust
use inspector_gguf::gui::updater::check_for_updates;

// Check for updates
match check_for_updates() {
    Ok(status) => {
        if status.starts_with("new_version_available:") {
            let version = status.split(':').nth(1).unwrap_or("");
            println!("New version available: {}", version);
        } else if status == "latest_version" {
            println!("You have the latest version");
        }
    }
    Err(e) => eprintln!("Update check failed: {}", e),
}
```

## 🧪 Testing Utilities

### Test Data Creation

```rust
// Create test metadata for testing export functions
fn create_test_metadata() -> Vec<(&'static str, &'static str)> {
    vec![
        ("general.name", "Test Model"),
        ("general.version", "1.0"),
        ("general.description", "A test model for validation"),
    ]
}

// Create temporary file for testing
fn create_temp_file(extension: &str) -> PathBuf {
    let mut temp = std::env::temp_dir();
    temp.push(format!("test_file.{}", extension));
    temp
}
```

## 🚨 Error Handling

### Custom Error Types

```rust
use inspector_gguf::error::{InspectorError, Result};

// Custom error handling
fn process_gguf_file(path: &Path) -> Result<Vec<MetadataEntry>> {
    let metadata = load_gguf_metadata_with_full_content_sync(path)
        .map_err(|e| InspectorError::LoadError(e.to_string()))?;
    
    Ok(metadata.into_iter()
        .map(|(key, display_value, full_value)| MetadataEntry {
            key,
            display_value,
            full_value,
        })
        .collect())
}
```

## 📋 Integration Examples

### Basic CLI Tool

```rust
use inspector_gguf::format::load_gguf_metadata_with_full_content_sync;
use inspector_gguf::gui::export::export_csv;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_path = Path::new("model.gguf");
    let output_path = Path::new("metadata.csv");
    
    // Load metadata
    let metadata = load_gguf_metadata_with_full_content_sync(input_path)?;
    
    // Convert to export format
    let export_data: Vec<(&str, &str)> = metadata
        .iter()
        .map(|(k, v, _)| (k.as_str(), v.as_str()))
        .collect();
    
    // Export to CSV
    export_csv(&export_data, output_path)?;
    
    println!("Metadata exported to {}", output_path.display());
    Ok(())
}
```

### Custom GUI Integration

```rust
use inspector_gguf::gui::{GgufApp, theme::apply_inspector_theme};
use eframe::egui;

struct CustomApp {
    gguf_app: GgufApp,
    custom_data: String,
}

impl eframe::App for CustomApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        apply_inspector_theme(ctx);
        
        // Custom UI elements
        egui::TopBottomPanel::top("custom_top").show(ctx, |ui| {
            ui.label("Custom Application Header");
        });
        
        // Integrate GGUF functionality
        self.gguf_app.update(ctx, frame);
    }
}
```

---

This API documentation is automatically generated from the source code and kept up-to-date with each release.