// Main GUI module with re-exports for clean API
// This module orchestrates all GUI functionality

pub mod app;
pub mod theme;
pub mod export;
pub mod loader;
pub mod updater;
pub mod layout;
pub mod panels;

// Re-export main application struct and key functionality
pub use app::GgufApp;

// Theme system re-exports
pub use theme::{
    apply_inspector_theme, 
    load_custom_font, 
    INSPECTOR_BLUE, 
    GADGET_YELLOW, 
    TECH_GRAY, 
    DANGER_RED, 
    SUCCESS_GREEN
};

// Layout utilities re-exports
pub use layout::{
    get_sidebar_width, 
    get_adaptive_font_size, 
    get_adaptive_button_width
};

// Export system re-exports (all public functions)
pub use export::{
    ensure_extension,
    sanitize_for_markdown,
    escape_markdown_text,
    show_base64_dialog,
    export_csv,
    export_yaml,
    export_markdown,
    export_markdown_to_file,
    export_html,
    export_html_to_file,
    export_pdf_from_markdown
};

// File loader re-exports
pub use loader::{
    load_gguf_metadata_async, 
    LoadingResult, 
    MetadataEntry
};

// Update checker re-exports
pub use updater::check_for_updates;

// Panel system re-exports
pub use panels::{
    render_sidebar,
    render_content_panel,
    render_settings_dialog,
    render_about_dialog,
    render_right_side_panels
};