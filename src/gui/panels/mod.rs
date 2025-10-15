// Panel management modules
// This module provides organized UI panel functionality

pub mod sidebar;
pub mod content;
pub mod dialogs;

// Re-export panel functionality for clean API
pub use sidebar::render_sidebar;
pub use content::render_content_panel;
pub use dialogs::{render_settings_dialog, render_about_dialog, render_right_side_panels};