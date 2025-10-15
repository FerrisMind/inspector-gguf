# Inspector GGUF - Frequently Asked Questions

This document answers the most common questions about Inspector GGUF. If you don't find your answer here, please check our [GitHub Issues](https://github.com/FerrisMind/inspector-gguf/issues) or create a new issue.

## 🚀 Getting Started

### Q: What is Inspector GGUF?
**A:** Inspector GGUF is a powerful tool for analyzing and viewing metadata of GGUF (GPT-Generated Unified Format) files used in machine learning. It provides both a modern graphical interface and command-line tools for comprehensive GGUF file inspection.

### Q: What are GGUF files?
**A:** GGUF (GPT-Generated Unified Format) is a file format used to store large language models and their metadata. It's designed to be efficient for loading and using AI models, containing model weights, configuration, tokenizer information, and other metadata in a single file.

### Q: Do I need to install anything special to run Inspector GGUF?
**A:** No additional dependencies are required. Inspector GGUF is distributed as a standalone executable that includes all necessary components. Simply download and run the application.

### Q: Which operating systems are supported?
**A:** Inspector GGUF supports:
- **Windows** 10/11 (x64)
- **macOS** 10.15+ (Intel and Apple Silicon)
- **Linux** (Ubuntu 18.04+, CentOS 7+, and other modern distributions)

## 📁 File Handling

### Q: What file formats does Inspector GGUF support?
**A:** Inspector GGUF specifically supports GGUF files (`.gguf` extension). It cannot read other model formats like ONNX, PyTorch, or TensorFlow files.

### Q: How large files can Inspector GGUF handle?
**A:** Inspector GGUF can handle GGUF files of any size, from small test models (a few MB) to large production models (100GB+). The application uses efficient streaming and memory management to handle large files without consuming excessive system memory.

### Q: Why is my file loading slowly?
**A:** Large GGUF files may take time to load due to:
- **File size**: Larger files naturally take longer to process
- **Storage speed**: Files on HDD load slower than SSD
- **System memory**: Limited RAM may cause slower processing
- **File complexity**: Files with extensive metadata take longer to parse

**Solutions:**
- Use SSD storage for better performance
- Ensure sufficient system memory (8GB+ recommended for large models)
- Close other applications to free resources
- Enable profiling mode to identify bottlenecks

### Q: Can I load multiple GGUF files at once?
**A:** Currently, Inspector GGUF loads one file at a time in the GUI. For batch processing multiple files, use the command-line interface:
```bash
for file in *.gguf; do
    inspector-gguf "$file" --output "${file%.gguf}.json"
done
```

## 🖥️ User Interface

### Q: How do I change the interface language?
**A:** 
1. Click the **"⚙️ Settings"** button in the sidebar
2. Select your preferred language from the dropdown
3. The interface will update immediately
4. Your language preference is automatically saved

### Q: The text is too small/large. Can I adjust the font size?
**A:** Inspector GGUF automatically adjusts font sizes based on your screen resolution and DPI settings. The interface uses adaptive sizing to ensure readability across different displays. If you need different sizing, you can adjust your system's display scaling settings.

### Q: Can I resize the application window?
**A:** Yes, the Inspector GGUF window is fully resizable. The interface adapts to different window sizes, with responsive layout that adjusts sidebar width, font sizes, and button dimensions based on available space.

### Q: What are the special viewers for?
**A:** Special viewers provide enhanced display for specific types of data:
- **Chat Template Viewer**: Shows tokenizer chat templates in a readable format
- **GGML Tokens Viewer**: Displays token lists with search functionality
- **GGML Merges Viewer**: Shows merge operations and patterns
- **Base64 Viewer**: Displays binary data in Base64 encoding

## 📤 Export Features

### Q: What export formats are available?
**A:** Inspector GGUF supports five export formats:
- **CSV**: Comma-separated values for spreadsheet applications
- **YAML**: Human-readable structured data format
- **Markdown**: Documentation format with tables and headers
- **HTML**: Web-compatible format with styling
- **PDF**: Professional document format for printing/sharing

### Q: Which export format should I use?
**A:** Choose based on your needs:
- **CSV**: For data analysis in Excel, Google Sheets, or databases
- **YAML**: For configuration files or human-readable data storage
- **Markdown**: For documentation, GitHub README files, or wikis
- **HTML**: For web viewing or embedding in websites
- **PDF**: For formal documentation, printing, or archival

### Q: Can I customize the export format?
**A:** Currently, export formats use predefined templates optimized for readability and compatibility. Custom templates are planned for future releases. You can modify the exported files after generation if needed.

### Q: Why did my export fail?
**A:** Common export failure reasons:
- **Insufficient permissions**: Cannot write to the selected directory
- **Disk space**: Not enough free space for the export file
- **Invalid path**: The selected path contains invalid characters
- **File in use**: The target file is open in another application

**Solutions:**
- Choose a different output directory
- Free up disk space
- Close applications that might be using the target file
- Check file path for special characters

## 🔧 Technical Issues

### Q: The application won't start. What should I do?
**A:** Try these troubleshooting steps:

1. **Check system requirements**: Ensure your OS is supported
2. **Run from command line**: Open terminal/command prompt and run the executable to see error messages
3. **Check permissions**: Ensure the executable has run permissions
4. **Antivirus software**: Some antivirus programs may block the application
5. **System libraries**: On Linux, ensure required libraries are installed

**Linux users may need:**
```bash
sudo apt install libgtk-3-0 libssl3 ca-certificates
```

### Q: I'm getting "Permission denied" errors. How do I fix this?
**A:** This usually indicates file permission issues:

**On Linux/macOS:**
```bash
chmod +x inspector-gguf
```

**On Windows:**
- Right-click the executable → Properties → Security
- Ensure your user account has "Full control"
- Try running as Administrator if necessary

### Q: The application crashes when loading large files. What can I do?
**A:** Large file crashes are usually memory-related:

1. **Close other applications** to free memory
2. **Increase virtual memory** (swap space) on your system
3. **Use a machine with more RAM** for very large models
4. **Try the command-line interface** which uses less memory
5. **Enable profiling mode** to identify the specific bottleneck

### Q: Can I run Inspector GGUF on a server without a GUI?
**A:** Yes! Use the command-line interface:
```bash
# Analyze a file and export to JSON
inspector-gguf model.gguf --output metadata.json

# Process multiple files
inspector-gguf --metadata-dir /path/to/yaml/files

# Validate GGUF directory
inspector-gguf --check-dir /path/to/gguf/models
```

## 🌍 Localization

### Q: How do I add support for my language?
**A:** We welcome new language contributions! Here's how:

1. **Create translation file**: Copy `translations/en.json` to `translations/{your_language_code}.json`
2. **Translate all strings**: Maintain the same JSON structure
3. **Add language definition**: Update `src/localization/language.rs`
4. **Test your translation**: Build and test the application
5. **Submit a pull request**: Share your translation with the community

### Q: Some text isn't translated. Is this a bug?
**A:** This could be:
- **Missing translation**: The text key isn't in your language file
- **Fallback behavior**: The application falls back to English for missing translations
- **Hard-coded text**: Some technical terms might be intentionally not translated

Please report untranslated text as an issue with the specific text and context.

### Q: Can I change the date/time format for my locale?
**A:** Currently, Inspector GGUF uses standard international formats. Locale-specific date/time formatting is planned for future releases.

## 🔄 Updates and Maintenance

### Q: How do I check for updates?
**A:** 
1. Click **"ℹ️ About"** in the sidebar
2. Click **"🔄 Check for updates"**
3. The application will check GitHub for new releases
4. Follow the download link if an update is available

### Q: Does Inspector GGUF update automatically?
**A:** No, Inspector GGUF does not update automatically. You need to manually download and install new versions. This ensures you have control over when updates are applied.

### Q: How often are updates released?
**A:** Release frequency depends on:
- **Bug fixes**: Critical issues are addressed quickly
- **Feature requests**: New features are added based on community feedback
- **Security updates**: Security issues are prioritized
- **Platform updates**: Updates for new OS versions or dependencies

Follow our [GitHub repository](https://github.com/FerrisMind/inspector-gguf) for release announcements.

### Q: Can I use older versions of Inspector GGUF?
**A:** Yes, all previous versions remain available on the [Releases page](https://github.com/FerrisMind/inspector-gguf/releases). However, we recommend using the latest version for bug fixes and new features.

## 🛡️ Security and Privacy

### Q: Does Inspector GGUF send my data anywhere?
**A:** No, Inspector GGUF processes all files locally on your machine. The only network activity is:
- **Update checking**: Contacts GitHub to check for new versions
- **No file upload**: Your GGUF files never leave your computer
- **No telemetry**: No usage data is collected or transmitted

### Q: Is it safe to analyze GGUF files from unknown sources?
**A:** While Inspector GGUF itself is safe, you should be cautious with files from untrusted sources:
- **Scan files** with antivirus software before analysis
- **Use isolated environment** for suspicious files
- **Verify file sources** when possible
- **Backup important data** before processing unknown files

### Q: Can malicious GGUF files harm my system?
**A:** Inspector GGUF only reads and displays file metadata. It doesn't execute any code from GGUF files. However, as with any file format, theoretically crafted files could potentially exploit parser vulnerabilities. We use the well-tested Candle library for GGUF parsing to minimize such risks.

## 🚀 Performance

### Q: Why is Inspector GGUF using so much memory?
**A:** Memory usage depends on:
- **File size**: Larger GGUF files require more memory to process
- **Metadata complexity**: Files with extensive metadata use more memory
- **GUI rendering**: The graphical interface requires additional memory
- **System overhead**: Operating system and other applications also use memory

**To reduce memory usage:**
- Close other applications
- Use command-line interface for batch processing
- Process files one at a time
- Restart the application periodically for long sessions

### Q: Can I improve performance for large files?
**A:** Yes, several strategies can help:

**Hardware:**
- Use SSD storage instead of HDD
- Ensure sufficient RAM (8GB+ recommended)
- Use faster CPU for better processing speed

**Software:**
- Close unnecessary applications
- Use command-line interface for batch processing
- Enable profiling mode to identify bottlenecks
- Process files during low system activity

### Q: What is profiling mode and should I use it?
**A:** Profiling mode enables detailed performance monitoring:
```bash
inspector-gguf --profile
```

**Benefits:**
- Identifies performance bottlenecks
- Provides detailed timing information
- Helps optimize processing for your system
- Useful for troubleshooting slow operations

**When to use:**
- Experiencing slow performance
- Processing very large files
- Optimizing batch operations
- Troubleshooting system issues

## 🤝 Community and Support

### Q: How can I get help with Inspector GGUF?
**A:** Several support options are available:

1. **Documentation**: Check this FAQ and the User Guide
2. **GitHub Issues**: Search existing issues or create a new one
3. **GitHub Discussions**: Ask questions and share ideas
4. **Community**: Connect with other users and contributors

### Q: How can I contribute to Inspector GGUF?
**A:** We welcome contributions in many forms:

- **Bug reports**: Help us identify and fix issues
- **Feature requests**: Suggest new functionality
- **Code contributions**: Submit pull requests
- **Documentation**: Improve guides and documentation
- **Translations**: Add support for new languages
- **Testing**: Test new releases and provide feedback

See our [Contributing Guide](CONTRIBUTING.md) for detailed information.

### Q: Can I use Inspector GGUF in my commercial project?
**A:** Yes! Inspector GGUF is released under the MIT License, which allows:
- **Commercial use**: Use in commercial projects
- **Modification**: Modify the source code
- **Distribution**: Distribute modified or unmodified versions
- **Private use**: Use privately without sharing changes

The only requirement is to include the original license notice.

### Q: How can I report a security vulnerability?
**A:** For security issues, please:
1. **Do not** create a public GitHub issue
2. **Email** the maintainers directly at security@ferrismind.com
3. **Include** detailed information about the vulnerability
4. **Allow** reasonable time for response and fix

We take security seriously and will respond promptly to legitimate security reports.

---

## 📞 Still Need Help?

If your question isn't answered here:

1. **Search** [GitHub Issues](https://github.com/FerrisMind/inspector-gguf/issues)
2. **Check** [GitHub Discussions](https://github.com/FerrisMind/inspector-gguf/discussions)
3. **Create** a new issue with detailed information
4. **Join** our community discussions

We're here to help and appreciate your feedback to improve Inspector GGUF!