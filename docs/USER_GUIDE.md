# Inspector GGUF User Guide

Welcome to Inspector GGUF! This comprehensive guide will help you get the most out of this powerful GGUF file inspection tool.

## 🚀 Getting Started

### Installation

#### Option 1: Download Pre-built Binary (Recommended)
1. Visit the [Releases page](https://github.com/FerrisMind/inspector-gguf/releases)
2. Download the latest version for your operating system
3. Extract the archive and run the executable

#### Option 2: Install from Crates.io
```bash
cargo install inspector-gguf
```

#### Option 3: Build from Source
```bash
git clone https://github.com/FerrisMind/inspector-gguf
cd inspector-gguf
cargo build --release
```

### First Launch

Run Inspector GGUF with the GUI interface:
```bash
inspector-gguf --gui
```

## 🖥️ Graphical User Interface

### Main Interface Overview

The Inspector GGUF GUI consists of three main areas:

```
┌─────────────┬─────────────────────────────────────┬─────────────┐
│             │                                     │             │
│   Sidebar   │           Main Content              │ Right Panel │
│             │                                     │ (Optional)  │
│   Actions   │         Metadata Display            │             │
│   Export    │         Filter Controls             │   Special   │
│   Settings  │         Progress Tracking           │   Viewers   │
│             │                                     │             │
└─────────────┴─────────────────────────────────────┴─────────────┘
```

### Loading GGUF Files

#### Method 1: Drag and Drop
1. Simply drag a GGUF file from your file manager
2. Drop it anywhere in the Inspector GGUF window
3. The file will automatically start loading

#### Method 2: Load Button
1. Click the **"📁 Load"** button in the sidebar
2. Browse and select your GGUF file
3. Click "Open" to start loading

#### Method 3: Command Line
```bash
inspector-gguf --gui path/to/your/model.gguf
```

### Understanding the Interface

#### Sidebar (Left Panel)
- **📁 Load**: Open file browser to select GGUF files
- **🧹 Clear**: Clear all loaded metadata
- **📤 Export**: Multiple export format options
  - **📄 CSV**: Comma-separated values
  - **📋 YAML**: Human-readable data format
  - **📝 MD**: Markdown documentation
  - **🌐 HTML**: Web-compatible format
  - **📑 PDF**: Portable document format
- **⚙️ Settings**: Application preferences
- **ℹ️ About**: Version and update information

#### Main Content Area
- **Progress Bar**: Shows loading progress for large files
- **Filter Controls**: Search and filter metadata entries
- **Metadata Display**: Key-value pairs from the GGUF file
- **Special Viewers**: Enhanced display for specific data types

#### Right Panel (Contextual)
Appears when viewing special content:
- **Chat Templates**: Tokenizer chat template viewer
- **GGML Tokens**: Token list viewer
- **GGML Merges**: Merge operations viewer

### Filtering and Search

#### Basic Filtering
1. Use the **Filter** text box at the top of the main content
2. Type any text to filter metadata entries
3. Filtering searches both keys and values
4. Click the **"✖ Clear"** button to remove filters

#### Advanced Search Tips
- **Partial matches**: Type part of a key or value
- **Case insensitive**: Search works regardless of case
- **Real-time**: Results update as you type
- **Multiple terms**: Space-separated terms work as AND search

### Viewing Special Content

#### Chat Templates
1. Look for entries with key `tokenizer.chat_template`
2. Click the **"👁 View"** button next to the entry
3. The chat template opens in the right panel
4. Scroll through the template content
5. Click the **"✖"** button to close

#### GGML Tokens
1. Find entries with key `tokenizer.ggml.tokens`
2. Click **"👁 View"** to open the token viewer
3. Browse through the token list
4. Use the search functionality within the viewer

#### GGML Merges
1. Locate `tokenizer.ggml.merges` entries
2. Click **"👁 View"** to see merge operations
3. Examine the merge rules and patterns

#### Binary Data
- Large binary data is automatically detected
- Shows as **"<binary> (long)"** with a **"👁 View Base64"** button
- Click to view the Base64-encoded representation
- Useful for examining embedded files and data

## 📤 Exporting Data

### Export Formats

#### CSV (Comma-Separated Values)
- **Best for**: Spreadsheet applications, data analysis
- **Contains**: Key-value pairs in tabular format
- **Usage**: Open in Excel, Google Sheets, or any CSV viewer

#### YAML (YAML Ain't Markup Language)
- **Best for**: Configuration files, human-readable data
- **Contains**: Hierarchical structure of metadata
- **Usage**: Easy to read and edit in text editors

#### Markdown
- **Best for**: Documentation, README files
- **Contains**: Formatted metadata with headers and tables
- **Usage**: GitHub, documentation systems, static site generators

#### HTML
- **Best for**: Web viewing, sharing via browser
- **Contains**: Styled metadata with CSS formatting
- **Usage**: Open in any web browser, embed in websites

#### PDF
- **Best for**: Printing, formal documentation
- **Contains**: Professional-looking formatted document
- **Usage**: Print, share as final document, archival

### Export Process

1. **Load your GGUF file** using any of the loading methods
2. **Choose export format** by clicking the appropriate button in the sidebar
3. **Select output location** in the file dialog
4. **Wait for completion** - larger files may take a moment
5. **Check the output** - success/error messages appear in the console

### Export Tips

- **File extensions**: Automatically added if not specified
- **Overwrite protection**: Confirms before overwriting existing files
- **Error handling**: Clear error messages if export fails
- **Large files**: Progress indication for big exports

## ⚙️ Settings and Preferences

### Language Settings

#### Changing Language
1. Click the **"⚙️ Settings"** button in the sidebar
2. Find the **Language** dropdown
3. Select your preferred language:
   - **English** - Default language
   - **Русский** - Russian interface
   - **Português (Brasil)** - Brazilian Portuguese
4. Click **"Close"** to apply changes

#### Supported Languages
- **English**: Complete translation, default fallback
- **Russian**: Full interface translation
- **Portuguese (Brazilian)**: Complete localization

### Application Preferences

#### Auto-save Settings
- Language preferences are automatically saved
- Window size and position remembered
- Export format preferences stored

#### Reset to Defaults
- Delete configuration files to reset all settings
- Restart the application for clean state

## 🔄 Updates and Maintenance

### Checking for Updates

#### Automatic Check
1. Click **"ℹ️ About"** in the sidebar
2. Click **"🔄 Check for updates"**
3. Wait for the update check to complete
4. Follow download links if updates are available

#### Update Information
- **Current version**: Displayed in About dialog
- **Latest version**: Fetched from GitHub releases
- **Download links**: Direct links to new releases
- **Release notes**: Available on GitHub

### Troubleshooting

#### Common Issues

**Application won't start:**
- Verify system requirements are met
- Check file permissions
- Try running from command line for error messages

**Files won't load:**
- Ensure file is a valid GGUF format
- Check file permissions and accessibility
- Try with a smaller test file first

**Export failures:**
- Verify write permissions in target directory
- Ensure sufficient disk space
- Check file path validity

**Performance issues:**
- Close other applications to free memory
- Use SSD storage for better I/O performance
- Enable profiling mode to identify bottlenecks

#### Getting Help

1. **Check this user guide** for common solutions
2. **Search existing issues** on GitHub
3. **Create a new issue** with detailed information:
   - Operating system and version
   - Application version
   - Steps to reproduce the problem
   - Error messages or screenshots

## 💡 Tips and Best Practices

### Performance Optimization

#### For Large Files
- **Close unnecessary applications** to free memory
- **Use fast storage** (SSD preferred) for better I/O
- **Enable profiling** to identify bottlenecks
- **Filter results** to reduce display overhead

#### Memory Management
- **Clear metadata** when switching between large files
- **Close special viewers** when not needed
- **Restart application** periodically for long sessions

### Workflow Recommendations

#### Model Analysis Workflow
1. **Load the model** using drag-and-drop
2. **Review general information** (name, version, description)
3. **Examine architecture details** (layers, parameters)
4. **Check tokenizer configuration** if available
5. **Export relevant data** for documentation or analysis

#### Batch Processing
1. **Use command-line interface** for multiple files
2. **Script the export process** for automation
3. **Organize output files** in structured directories
4. **Document your analysis** using exported data

### Security Considerations

#### File Safety
- **Verify file sources** before loading unknown GGUF files
- **Scan files** with antivirus if from untrusted sources
- **Backup important data** before processing
- **Use isolated environment** for suspicious files

#### Privacy
- **Local processing**: All analysis happens on your machine
- **No data transmission**: Files are not sent to external servers
- **Update checks only**: Only version checking contacts GitHub

## 🎯 Advanced Usage

### Command Line Interface

#### Basic Commands
```bash
# Analyze single file
inspector-gguf model.gguf

# Export to specific format
inspector-gguf model.gguf --output metadata.json

# Validate metadata directory
inspector-gguf --metadata-dir /path/to/yaml/files

# Performance profiling
inspector-gguf --profile
```

#### Batch Processing
```bash
# Process multiple files
for file in *.gguf; do
    inspector-gguf "$file" --output "${file%.gguf}.json"
done
```

### Integration with Other Tools

#### Scripting
- **JSON output**: Easy to parse with scripts
- **CSV format**: Import into data analysis tools
- **YAML format**: Use in configuration management

#### Development Workflow
- **Model validation**: Verify model metadata
- **Documentation generation**: Create model documentation
- **Quality assurance**: Check model consistency

## 📚 Additional Resources

### Documentation
- **Architecture Guide**: Technical implementation details
- **API Documentation**: Library usage and integration
- **Contributing Guide**: How to contribute to the project

### Community
- **GitHub Repository**: Source code and issue tracking
- **Discussions**: Community questions and ideas
- **Releases**: Download latest versions and changelogs

### Support
- **User Guide**: This comprehensive guide
- **FAQ**: Frequently asked questions
- **Issue Tracker**: Bug reports and feature requests

---

Thank you for using Inspector GGUF! We hope this guide helps you make the most of the application. If you have suggestions for improving this guide, please let us know through our GitHub repository.