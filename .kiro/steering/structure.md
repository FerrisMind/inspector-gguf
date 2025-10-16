# Inspector GGUF Project Structure

## Root Directory Layout
```
inspector-gguf/
├── src/                    # Source code
├── docs/                   # Documentation
├── examples/               # Usage examples
├── translations/           # Localization files
├── assets/                 # Static assets (icons, fonts)
├── model/                  # Test model files
├── target/                 # Build artifacts (ignored)
├── Cargo.toml             # Project configuration
├── build.rs               # Build script
└── README.md              # Project documentation
```

## Source Code Organization (`src/`)
```
src/
├── main.rs                # Application entry point and CLI
├── lib.rs                 # Library exports and public API
├── format.rs              # GGUF file format handling
├── gui/                   # GUI components and logic
│   ├── app.rs            # Main application state (GgufApp)
│   ├── theme.rs          # UI theming and styling
│   ├── layout.rs         # Responsive layout utilities
│   ├── export.rs         # Multi-format export functionality
│   ├── loader.rs         # Async file loading with progress
│   ├── updater.rs        # Update checking from GitHub
│   └── panels/           # UI panel components
│       ├── sidebar.rs    # Left sidebar with actions
│       ├── content.rs    # Main content display area
│       └── dialogs.rs    # Modal dialogs and panels
├── localization/         # Internationalization system
│   ├── mod.rs           # Module exports
│   ├── manager.rs       # Central localization management
│   ├── loader.rs        # Translation file loading
│   ├── detector.rs      # System locale detection
│   ├── language.rs      # Language definitions
│   └── settings.rs      # Persistent language settings
├── versioning/          # Version management (future)
└── documentation/       # Documentation automation (future)
```

## Module Responsibilities

### Core Modules
- **`main.rs`**: CLI parsing, application initialization, profiling setup
- **`lib.rs`**: Public API exports for library usage
- **`format.rs`**: GGUF parsing using Candle, metadata extraction

### GUI System (`gui/`)
- **`app.rs`**: Main application state, eframe::App implementation
- **`theme.rs`**: Inspector Gadget color scheme, font management
- **`layout.rs`**: Responsive sizing functions, adaptive UI elements
- **`export.rs`**: CSV, YAML, Markdown, HTML, PDF export functions
- **`loader.rs`**: Background file loading with Arc<Mutex<>> progress tracking
- **`updater.rs`**: GitHub API integration for version checking

### Panel Architecture (`gui/panels/`)
- **`sidebar.rs`**: Load/Clear/Export buttons, settings access
- **`content.rs`**: Metadata display, filtering, special viewers
- **`dialogs.rs`**: Settings modal, about dialog, right-side panels

### Localization System (`localization/`)
- **`manager.rs`**: LocalizationManager with translation caching
- **`loader.rs`**: JSON translation file parsing and validation
- **`detector.rs`**: Platform-specific locale detection (Windows/Unix)
- **`language.rs`**: Language enum (English, Russian, PortugueseBrazilian)
- **`settings.rs`**: Persistent language preference storage

## Asset Organization
```
assets/
├── fonts/                # Custom fonts (if any)
└── icons/               # Application icons
    ├── 128x128@2x.png   # High-DPI icon
    └── ...              # Various sizes
```

## Translation Files (`translations/`)
```
translations/
├── en.json              # English (default)
├── ru.json              # Russian
└── pt-BR.json           # Portuguese (Brazilian)
```

## Documentation Structure (`docs/`)
```
docs/
├── README.md            # Documentation index
├── USER_GUIDE.md        # End-user documentation
├── API.md               # Library API reference
├── ARCHITECTURE.md      # Technical architecture
├── DEPLOYMENT.md        # Build and deployment
└── FAQ.md               # Frequently asked questions
```

## Naming Conventions
- **Files**: snake_case (e.g., `cargo_updater.rs`)
- **Modules**: snake_case (e.g., `localization`)
- **Structs**: PascalCase (e.g., `GgufApp`, `LocalizationManager`)
- **Functions**: snake_case (e.g., `load_gguf_metadata_sync`)
- **Constants**: SCREAMING_SNAKE_CASE (e.g., `INSPECTOR_BLUE`)

## Import Organization
```rust
// Standard library imports first
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// External crate imports
use eframe::egui;
use serde::{Deserialize, Serialize};

// Internal crate imports
use crate::localization::LocalizationManager;
use crate::gui::theme::apply_inspector_theme;
```

## Test Organization
- **Unit tests**: Inline with `#[cfg(test)]` modules
- **Integration tests**: `tests/` directory (if needed)
- **Test utilities**: Shared test helpers in test modules
- **Mock data**: Standardized test datasets for consistent testing