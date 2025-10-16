# Project Structure

## Source Organization

```
src/
├── main.rs              # Entry point, CLI parsing, profiling setup
├── lib.rs               # Public API and module exports
├── format.rs            # GGUF parsing and metadata extraction
├── gui/                 # GUI components (egui/eframe)
│   ├── mod.rs
│   ├── app.rs          # Main GgufApp state and eframe::App impl
│   ├── theme.rs        # UI theming and color palette
│   ├── layout.rs       # Responsive layout utilities
│   ├── export.rs       # Multi-format export functions
│   ├── loader.rs       # Async file loading with progress
│   ├── updater.rs      # GitHub release update checking
│   └── panels/         # Modular UI panels
│       ├── mod.rs
│       ├── sidebar.rs  # Left sidebar with actions
│       ├── content.rs  # Main content display
│       └── dialogs.rs  # Modal dialogs and right panels
└── localization/       # i18n system
    ├── mod.rs
    ├── manager.rs      # Central localization coordinator
    ├── loader.rs       # Translation file loading
    ├── detector.rs     # System locale detection
    ├── language.rs     # Language enum and metadata
    ├── settings.rs     # Localization settings
    └── error.rs        # Localization errors
```

## Supporting Files

```
assets/
├── fonts/              # Custom fonts for UI
└── icons/              # Application icons (various sizes)

translations/           # i18n JSON files
├── en.json            # English (default)
├── ru.json            # Russian
└── pt-BR.json         # Portuguese Brazilian

docs/                   # Comprehensive documentation
├── README.md          # Documentation index
├── USER_GUIDE.md      # End-user guide
├── API.md             # Library API documentation
├── ARCHITECTURE.md    # Technical architecture
├── DEPLOYMENT.md      # Build and deployment guide
└── FAQ.md             # Troubleshooting and FAQ

model/                 # Test model files (not in git)
```

## Architecture Patterns

### Modular Design
- Clear separation between GUI, CLI, and core functionality
- Panel-based UI architecture for maintainability
- Centralized state management in `GgufApp`

### Error Handling
- Custom error types using `thiserror`
- Module-specific error types (e.g., `localization::error`)
- Comprehensive error propagation with context

### Async Operations
- Background file loading with progress tracking
- Thread-safe state sharing via `Arc<Mutex<T>>`
- Non-blocking UI during long operations

### Localization
- JSON-based translation files
- Automatic system locale detection
- Runtime language switching without restart
- Fallback to English for missing translations

## Key Conventions

- All public APIs documented with rustdoc
- Unit tests inline with implementation
- Integration tests in `tests/` directory
- Profiling support via puffin (enabled with `--profile` flag)
- Windows-specific code gated with `#[cfg(target_os = "windows")]`
