# Inspector GGUF

[Русская версия](README.ru.md) | **English** | [Português (Brasil)](README.pt-BR.md)

A powerful, modern GGUF (GPT-Generated Unified Format) file inspection tool with an intuitive graphical interface and comprehensive command-line capabilities.

## 🚀 Overview

Inspector GGUF is a professional-grade tool designed for analyzing and exploring GGUF files used in machine learning and AI model development. Built with Rust and featuring a modern GUI powered by egui, it provides deep insights into model metadata, tokenizer configurations, and model architecture details.

## ✨ Features

### Core Functionality
- 🔍 **Deep GGUF Analysis** - Comprehensive metadata extraction and display
- 🖥️ **Modern GUI** - Intuitive interface with drag-and-drop support
- 📊 **Advanced Filtering** - Real-time search and filter capabilities
- 🎨 **Adaptive Design** - Responsive layout that scales with screen size

### Export Capabilities
- 📄 **Multiple Formats** - Export to CSV, YAML, Markdown, HTML, and PDF
- 💾 **Batch Processing** - Handle multiple files efficiently
- 🔧 **Custom Templates** - Flexible export formatting options

### Tokenizer Support
- 🧠 **Chat Templates** - View and analyze tokenizer chat templates
- 📝 **Token Analysis** - Inspect GGML tokens and merge operations
- 🔍 **Binary Data Handling** - Base64 encoding for large binary content

### Internationalization
- 🌍 **Multi-language Support** - English, Russian, Portuguese (Brazilian)
- 🔄 **Dynamic Language Switching** - Change language without restart
- 📱 **Localized UI** - Fully translated interface elements

### Developer Features
- ⚡ **Performance Profiling** - Built-in puffin profiler integration
- 🔄 **Auto-updates** - Automatic update checking from GitHub releases
- 🎯 **Error Handling** - Comprehensive error reporting and recovery

## 📦 Installation

### From Releases (Recommended)
Download the latest release from [GitHub Releases](https://github.com/FerrisMind/inspector-gguf/releases)

### From Source
```bash
git clone https://github.com/FerrisMind/inspector-gguf
cd inspector-gguf
cargo build --release
```

### From Crates.io
```bash
cargo install inspector-gguf
```

## 🎯 Usage

### Graphical Interface

Launch the GUI application:
```bash
inspector-gguf --gui
```

**GUI Features:**
- **Drag & Drop** - Simply drag GGUF files into the window
- **File Browser** - Use the "Load" button to browse for files
- **Export Options** - Multiple export formats available in the sidebar
- **Settings** - Language preferences and configuration options

### Command Line Interface

#### Basic Usage
```bash
# Analyze a single GGUF file
inspector-gguf path/to/model.gguf

# Export to specific format
inspector-gguf path/to/model.gguf --output metadata.json
```

#### Advanced Options
```bash
# Validate metadata directory
inspector-gguf --metadata-dir path/to/yaml/files

# Performance profiling
inspector-gguf --profile

# Check GGUF directory
inspector-gguf --check-dir path/to/gguf/models
```

## 🏗️ Architecture

### Project Structure
```
src/
├── gui/                    # GUI components
│   ├── app.rs             # Main application logic
│   ├── theme.rs           # UI theming and styling
│   ├── layout.rs          # Responsive layout utilities
│   ├── export.rs          # Export functionality
│   ├── loader.rs          # Async file loading
│   ├── updater.rs         # Update checking
│   └── panels/            # UI panels
│       ├── sidebar.rs     # Left sidebar with actions
│       ├── content.rs     # Main content display
│       └── dialogs.rs     # Modal dialogs
├── localization/          # Internationalization
│   ├── manager.rs         # Localization management
│   ├── loader.rs          # Translation loading
│   ├── detector.rs        # System locale detection
│   └── language.rs        # Language definitions
├── format.rs              # GGUF format handling
├── lib.rs                 # Library exports
└── main.rs                # Application entry point
```

### Key Components

#### GUI System
- **Modular Design** - Separated concerns with dedicated modules
- **Responsive Layout** - Adaptive sizing based on screen dimensions
- **Theme System** - Consistent color scheme and typography
- **Panel Architecture** - Reusable UI components

#### Localization System
- **Dynamic Loading** - Runtime language switching
- **Fallback Support** - Graceful degradation to English
- **System Detection** - Automatic locale detection
- **Extensible** - Easy addition of new languages

#### Export System
- **Format Abstraction** - Unified export interface
- **Error Handling** - Robust error recovery
- **File Management** - Automatic extension handling
- **Template System** - Customizable output formats

## 🔧 Configuration

### Language Settings
The application automatically detects your system language. Supported languages:
- **English** (en) - Default
- **Russian** (ru) - Русский
- **Portuguese (Brazilian)** (pt-BR) - Português (Brasil)

### Performance Tuning
For optimal performance with large models:
```bash
# Enable profiling mode
inspector-gguf --profile

# Access profiler at http://127.0.0.1:8585
```

## 🧪 Testing

Run the comprehensive test suite:
```bash
# Run all tests
cargo test

# Run with coverage
cargo test --all-features

# Run specific test modules
cargo test gui::export::tests
cargo test localization::tests
```

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guidelines](CONTRIBUTING.md) for details.

### Development Setup
```bash
git clone https://github.com/FerrisMind/inspector-gguf
cd inspector-gguf
cargo build
cargo test
```

### Adding New Languages
1. Create translation file in `translations/{language_code}.json`
2. Add language definition in `src/localization/language.rs`
3. Update language detection in `src/localization/detector.rs`
4. Test with `cargo test localization::tests`

## 📋 System Requirements

- **Rust** 1.70 or newer
- **Operating Systems** Windows, macOS, Linux
- **Memory** 512MB RAM minimum (2GB+ recommended for large models)
- **Storage** 50MB for application, additional space for model files

## 🐛 Troubleshooting

### Common Issues

**Application won't start:**
- Ensure Rust toolchain is properly installed
- Check system requirements
- Verify file permissions

**Large files loading slowly:**
- Enable profiling mode to identify bottlenecks
- Ensure sufficient system memory
- Consider using SSD storage for better I/O performance

**Export failures:**
- Check write permissions in target directory
- Ensure sufficient disk space
- Verify file path validity

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **Candle** - Rust-based ML framework for GGUF support
- **egui** - Immediate mode GUI framework
- **Community** - Contributors and users who make this project better

## 📚 Documentation

### Complete Documentation Suite
- **[📖 User Guide](docs/USER_GUIDE.md)** - Comprehensive usage instructions
- **[❓ FAQ](docs/FAQ.md)** - Frequently asked questions and troubleshooting
- **[🏗️ Architecture](docs/ARCHITECTURE.md)** - Technical architecture and design
- **[📋 API Documentation](docs/API.md)** - Library usage and integration
- **[🚀 Deployment Guide](docs/DEPLOYMENT.md)** - Building and deployment
- **[🤝 Contributing](CONTRIBUTING.md)** - How to contribute to the project

### Quick Links
- **[📥 Download Latest Release](https://github.com/FerrisMind/inspector-gguf/releases/latest)**
- **[🔄 Changelog](CHANGELOG.md)** - Version history and changes
- **[📜 License](LICENSE)** - MIT License details
- **[🤝 Code of Conduct](CODE_OF_CONDUCT.md)** - Community guidelines

## 📞 Support

- **[❓ FAQ](docs/FAQ.md)** - Quick answers to common questions
- **[🐛 Issues](https://github.com/FerrisMind/inspector-gguf/issues)** - Bug reports and feature requests
- **[💬 Discussions](https://github.com/FerrisMind/inspector-gguf/discussions)** - Community support and ideas
- **[📧 Email](mailto:contact@ferrismind.com)** - Direct contact for security issues

---

**Made with ❤️ by [FerrisMind](https://github.com/FerrisMind)**
