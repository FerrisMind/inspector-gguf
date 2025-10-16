# Technical Stack

## Language & Edition

- **Rust** edition 2024
- Minimum Rust version: 1.70+

## Core Dependencies

- **candle-core** (0.9.1) - GGUF file parsing and ML framework
- **egui** (0.32) / **eframe** (0.32) - Immediate mode GUI framework
- **serde** / **serde_json** - Serialization and JSON handling
- **structopt** (0.3) - CLI argument parsing
- **thiserror** (1.0) - Error handling
- **reqwest** (0.11) - HTTP client for update checking
- **semver** (1.0) - Version comparison

## Export & Format Support

- **csv** (1.3) - CSV export
- **serde_yaml** (0.9) - YAML export
- **pulldown-cmark** (0.13) / **markdown2pdf** (0.1) - Markdown and PDF export
- **base64** (0.22) - Binary data encoding

## Platform-Specific

- **winapi** (0.3) - Windows console and locale detection
- **winres** (0.1) - Windows resource embedding (build-time)

## Profiling & Monitoring

- **puffin** (0.19) / **puffin_http** (0.16) - Performance profiling
- **sysinfo** (0.30) - System resource monitoring

## Common Commands

```bash
# Build release version
cargo build --release

# Run GUI mode
cargo run --release -- --gui

# Run with profiling
cargo run --release -- --profile

# Run tests
cargo test

# Run tests with all features
cargo test --all-features

# Check for unused dependencies
cargo machete

# Check for errors
cargo clippy --lib -- -D warnings

# Generate documentation
cargo doc --all-features --open
```

## Build Configuration

Release builds use aggressive optimization:
- `opt-level = 3` - Maximum optimization
- `lto = "thin"` - Link-time optimization
- `codegen-units = 1` - Single codegen unit for better optimization
- `strip = true` - Remove debug symbols
- No debug info or assertions in release builds
