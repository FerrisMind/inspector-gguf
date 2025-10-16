# Inspector GGUF Technology Stack

## Core Technologies
- **Language**: Rust (Edition 2024, MSRV 1.70+)
- **GUI Framework**: egui/eframe 0.32 - Immediate mode GUI
- **GGUF Parsing**: Candle (candle-core 0.9.1) - Rust ML framework
- **Build System**: Cargo with custom build.rs for Windows resources

## Key Dependencies
- **CLI**: structopt 0.3 - Command-line argument parsing
- **Serialization**: serde 1.0 + serde_json + serde_yaml
- **File Dialogs**: rfd 0.15 - Native file dialogs
- **Async/Threading**: std::sync (Arc, Mutex) for thread-safe operations
- **Profiling**: puffin 0.19 + puffin_http 0.16 - Performance profiling
- **Updates**: reqwest 0.11 + semver 1.0 - GitHub API integration
- **Export Formats**: csv, pulldown-cmark, markdown2pdf
- **Icons**: egui-phosphor 0.10 - Icon library

## Architecture Patterns
- **Modular Structure**: Clear separation of concerns across modules
- **Error Handling**: thiserror for structured error types
- **Async Operations**: Background loading with progress tracking
- **State Management**: Centralized state in main app struct
- **Localization**: JSON-based translation system with runtime switching

## Build Configuration
```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Run GUI application
cargo run -- --gui

# Run with profiling
cargo run -- --profile

# Run tests
cargo test

# Format code
cargo fmt

# Lint code
cargo clippy
```

## Performance Optimizations
- **Release Profile**: High optimization (opt-level = 3, LTO enabled)
- **Memory Management**: Efficient handling of large GGUF files
- **Async Loading**: Non-blocking file operations with progress tracking
- **Profiling Integration**: Built-in puffin profiler for performance monitoring

## Platform Support
- **Windows**: Native support with winres build dependency
- **macOS**: Full compatibility
- **Linux**: Complete support
- **Cross-compilation**: Rust's native cross-platform capabilities

## Development Tools
- **Testing**: Built-in cargo test with tempfile for integration tests
- **Profiling**: Puffin web interface at http://127.0.0.1:8585
- **Documentation**: Comprehensive inline docs and external markdown
- **CI/CD**: GitHub Actions for automated testing and releases