# inspector-gguf

[Русская версия](README.ru.md) | English

A powerful GGUF (GPT-Generated Unified Format) file inspection tool with graphical and command-line interface.

## Description

`inspector-gguf` is a tool for analyzing and viewing metadata of GGUF files used in machine learning. It uses the `candle` library for file reading and provides a convenient interface for viewing model information.

## Features

- 🔍 Read and display GGUF file metadata
- 🖥️ Graphical interface based on egui
- 💾 Export metadata to JSON, CSV, and YAML formats
- 📊 Tokenizer and model vocabulary analysis
- 🎨 Modern and intuitive interface

## Installation

From crates.io:

```bash
cargo install inspector-gguf
```

From source:

```bash
git clone https://github.com/FerrisMind/inspector-gguf
cd inspector-gguf
cargo build --release
```

## Usage

### Graphical Interface

Run the application without arguments to open the GUI:

```bash
inspector-gguf
```

### Command Line

Specify the path to a GGUF file:

```bash
inspector-gguf <path/to/model.gguf>
```

## Examples

View model metadata:

```bash
inspector-gguf models/llama-2-7b.gguf
```

## System Requirements

- Rust 1.70 or newer
- Windows, macOS, or Linux

## License

MIT License. See [LICENSE](LICENSE) file for details.

## Contributing

Pull requests are welcome! For major changes, please open an issue first to discuss what you would like to change.

## Author

FerrisMind
