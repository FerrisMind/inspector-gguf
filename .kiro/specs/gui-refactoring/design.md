# GUI Refactoring Design Document

## Overview

This design document outlines the modular architecture for refactoring the large `gui.rs` file into smaller, maintainable modules. The design follows Rust's module system best practices and ensures clear separation of concerns while maintaining all existing functionality.

## Architecture

The refactored GUI will be organized into the following module structure:

```
src/gui/
├── mod.rs              # Main module file with re-exports
├── app.rs              # Main GgufApp struct and eframe::App implementation
├── theme.rs            # Theme system and styling
├── export.rs           # Export functionality for various formats
├── loader.rs           # File loading and processing
├── updater.rs          # Update checking functionality
├── panels/
│   ├── mod.rs          # Panel module re-exports
│   ├── sidebar.rs      # Left sidebar panel
│   ├── content.rs      # Central content panel
│   └── dialogs.rs      # Settings and about dialogs
└── layout.rs           # Adaptive layout utilities
```

## Components and Interfaces

### 1. Main Module (`src/gui/mod.rs`)

**Purpose**: Central module file that re-exports all public interfaces and provides a clean API for the rest of the application.

**Public Interface**:
```rust
pub use app::GgufApp;
pub use theme::{apply_inspector_theme, load_custom_font};
pub use export::*;
pub use loader::{load_gguf_metadata_async, LoadingResult, MetadataEntry};
pub use updater::check_for_updates;
```

### 2. Application Core (`src/gui/app.rs`)

**Purpose**: Contains the main `GgufApp` struct and `eframe::App` implementation. Acts as the orchestrator for all GUI functionality.

**Responsibilities**:
- Main application state management
- eframe::App trait implementation
- Coordination between different subsystems
- Event handling and state updates

**Key Components**:
- `GgufApp` struct definition
- `Default` implementation for `GgufApp`
- `eframe::App::update()` method implementation
- `LanguageProvider` trait implementation

### 3. Theme System (`src/gui/theme.rs`)

**Purpose**: Manages all visual styling, theming, and adaptive layout calculations.

**Responsibilities**:
- Color constants and theme definitions
- Font loading and configuration
- Adaptive sizing calculations
- Theme application to egui context

**Key Components**:
- Color constants (INSPECTOR_BLUE, GADGET_YELLOW, etc.)
- `load_custom_font()` function
- `apply_inspector_theme()` function
- Adaptive sizing functions (`get_sidebar_width`, `get_adaptive_font_size`, etc.)

### 4. Export System (`src/gui/export.rs`)

**Purpose**: Handles all export functionality for different file formats.

**Responsibilities**:
- Export to CSV, YAML, Markdown, HTML, PDF formats
- File path utilities and extension handling
- Content sanitization for different formats
- Error handling for export operations

**Key Components**:
- Export functions for each format
- Utility functions (`ensure_extension`, `sanitize_for_markdown`, etc.)
- Base64 dialog functionality

### 5. File Loader (`src/gui/loader.rs`)

**Purpose**: Manages asynchronous file loading and GGUF metadata processing.

**Responsibilities**:
- Asynchronous file loading with progress tracking
- GGUF file parsing and metadata extraction
- Progress reporting and error handling
- Thread management for background operations

**Key Components**:
- `LoadingResult` type alias
- `MetadataEntry` struct
- `load_gguf_metadata_async()` function
- Progress tracking and result management

### 6. Update Checker (`src/gui/updater.rs`)

**Purpose**: Handles application update checking via GitHub API.

**Responsibilities**:
- GitHub API integration
- Version comparison logic
- Update status management
- Network error handling

**Key Components**:
- `check_for_updates()` function
- Version constants and repository information
- HTTP client configuration and error handling

### 7. Panel Management (`src/gui/panels/`)

**Purpose**: Manages different UI panels and their rendering logic.

#### 7.1 Sidebar Panel (`src/gui/panels/sidebar.rs`)
- Left sidebar with action buttons
- Export buttons and functionality
- Settings and about buttons
- Adaptive button sizing

#### 7.2 Content Panel (`src/gui/panels/content.rs`)
- Central metadata display area
- Filter functionality
- Metadata entry rendering
- Drag-and-drop support

#### 7.3 Dialog Panels (`src/gui/panels/dialogs.rs`)
- Settings dialog
- About dialog
- Right-side content panels (chat template, tokens, merges)

### 8. Layout Utilities (`src/gui/layout.rs`)

**Purpose**: Provides adaptive layout calculations and utilities.

**Responsibilities**:
- Screen size detection and adaptation
- Responsive sizing calculations
- Layout helper functions

## Data Models

### MetadataEntry
```rust
#[derive(Clone)]
pub struct MetadataEntry {
    pub key: String,
    pub display_value: String,
    pub full_value: Option<String>,
}
```

### LoadingResult
```rust
type LoadingResult = Arc<Mutex<Option<Result<Vec<(String, String, Option<String>)>, String>>>>;
```

## Error Handling

Each module will maintain its existing error handling patterns:
- Export functions return `Result<(), Box<dyn std::error::Error>>`
- File loading uses mutex-protected result sharing
- Update checking returns structured error messages
- UI operations handle errors gracefully with user feedback

## Testing Strategy

The modular design enables focused testing:
- Unit tests for individual export functions
- Integration tests for file loading workflows
- Theme application testing with mock contexts
- Panel rendering tests with egui test framework

## Migration Strategy

The refactoring will be performed incrementally:
1. Create new module structure
2. Move functions to appropriate modules
3. Update imports and dependencies
4. Ensure all functionality remains intact
5. Remove original large file

## Performance Considerations

- No performance impact expected from modularization
- Existing async file loading preserved
- Theme application remains efficient
- Module boundaries designed to minimize cross-module calls

## Dependencies

The refactoring maintains all existing dependencies:
- `eframe` and `egui` for GUI framework
- `base64`, `reqwest`, `semver` for utilities
- `rfd` for file dialogs
- Export-related crates (`csv`, `serde_yaml`, etc.)
- Localization system integration preserved