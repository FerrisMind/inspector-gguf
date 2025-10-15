# Contributing to Inspector GGUF

Thank you for your interest in contributing to Inspector GGUF! This document provides guidelines and information for contributors.

## 🚀 Getting Started

### Prerequisites
- Rust 1.70 or newer
- Git
- Basic knowledge of Rust and GUI development

### Development Setup
```bash
# Clone the repository
git clone https://github.com/FerrisMind/inspector-gguf
cd inspector-gguf

# Build the project
cargo build

# Run tests
cargo test

# Start the GUI application
cargo run -- --gui
```

## 🏗️ Project Architecture

### Code Organization
```
src/
├── gui/                    # GUI components and logic
├── localization/          # Internationalization system
├── format.rs              # GGUF file format handling
├── lib.rs                 # Library exports
└── main.rs                # Application entry point
```

### Key Design Principles
1. **Modularity** - Each component has a single responsibility
2. **Testability** - All functionality is unit tested
3. **Internationalization** - All user-facing text is translatable
4. **Performance** - Efficient handling of large GGUF files
5. **User Experience** - Intuitive and responsive interface

## 🔧 Development Guidelines

### Code Style
- Follow Rust standard formatting (`cargo fmt`)
- Use `cargo clippy` for linting
- Write comprehensive documentation
- Include unit tests for new functionality

### Commit Messages
Use conventional commit format:
```
type(scope): description

feat(gui): add new export format
fix(localization): resolve translation loading issue
docs(readme): update installation instructions
test(export): add CSV export tests
```

### Branch Naming
- `feature/description` - New features
- `fix/description` - Bug fixes
- `docs/description` - Documentation updates
- `refactor/description` - Code refactoring

## 🧪 Testing

### Running Tests
```bash
# Run all tests
cargo test

# Run specific test module
cargo test gui::export::tests

# Run with output
cargo test -- --nocapture

# Run tests with coverage (requires cargo-tarpaulin)
cargo tarpaulin --out Html
```

### Test Categories
1. **Unit Tests** - Test individual functions and modules
2. **Integration Tests** - Test component interactions
3. **GUI Tests** - Test user interface components
4. **Localization Tests** - Test translation loading and formatting

### Writing Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_name() {
        // Arrange
        let input = create_test_data();
        
        // Act
        let result = function_under_test(input);
        
        // Assert
        assert_eq!(result, expected_value);
    }
}
```

## 🌍 Internationalization

### Adding New Languages

1. **Create Translation File**
   ```bash
   # Create new translation file
   cp translations/en.json translations/{language_code}.json
   ```

2. **Translate Content**
   - Translate all strings in the JSON file
   - Maintain the same key structure
   - Use appropriate formatting for your language

3. **Add Language Definition**
   ```rust
   // In src/localization/language.rs
   pub enum Language {
       English,
       Russian,
       PortugueseBrazilian,
       YourNewLanguage, // Add here
   }
   ```

4. **Update Language Detection**
   ```rust
   // In src/localization/detector.rs
   // Add locale detection logic for your language
   ```

5. **Test Translation**
   ```bash
   cargo test localization::tests
   ```

### Translation Guidelines
- Keep translations concise and clear
- Maintain consistent terminology
- Consider cultural context
- Test UI layout with translated text

## 🎨 GUI Development

### Adding New UI Components

1. **Create Component Module**
   ```rust
   // src/gui/components/new_component.rs
   use eframe::egui;
   
   pub fn render_new_component(ui: &mut egui::Ui, data: &ComponentData) {
       // Component implementation
   }
   ```

2. **Follow UI Patterns**
   - Use consistent spacing and sizing
   - Apply theme colors and fonts
   - Implement responsive design
   - Add proper error handling

3. **Test Component**
   - Test with different screen sizes
   - Verify accessibility
   - Test with all supported languages

### Theme System
```rust
// Use predefined theme colors
use crate::gui::theme::{INSPECTOR_BLUE, GADGET_YELLOW, TECH_GRAY};

// Apply adaptive sizing
let font_size = get_adaptive_font_size(14.0, ctx);
let button_width = get_adaptive_button_width(ui, &text, font_size, max_width);
```

## 📦 Export System

### Adding New Export Formats

1. **Implement Export Function**
   ```rust
   // In src/gui/export.rs
   pub fn export_new_format(
       metadata: &[(&str, &str)],
       path: &Path,
   ) -> Result<(), Box<dyn std::error::Error>> {
       // Implementation
   }
   ```

2. **Add UI Button**
   ```rust
   // In sidebar or appropriate UI component
   if ui.button("New Format").clicked() {
       if let Some(path) = FileDialog::new().save_file() {
           if let Err(e) = export_new_format(&metadata, &path) {
               eprintln!("Export failed: {}", e);
           }
       }
   }
   ```

3. **Add Tests**
   ```rust
   #[test]
   fn test_export_new_format() {
       let metadata = create_test_metadata();
       let temp_file = create_temp_file();
       
       let result = export_new_format(&metadata, &temp_file);
       assert!(result.is_ok());
       
       // Verify file content
   }
   ```

## 🐛 Bug Reports

### Before Reporting
1. Check existing issues
2. Reproduce the bug
3. Gather system information
4. Create minimal reproduction case

### Bug Report Template
```markdown
**Bug Description**
Clear description of the bug

**Steps to Reproduce**
1. Step one
2. Step two
3. Step three

**Expected Behavior**
What should happen

**Actual Behavior**
What actually happens

**Environment**
- OS: [Windows/macOS/Linux]
- Rust version: [output of `rustc --version`]
- Application version: [version number]

**Additional Context**
Any other relevant information
```

## 🚀 Feature Requests

### Feature Request Template
```markdown
**Feature Description**
Clear description of the proposed feature

**Use Case**
Why is this feature needed?

**Proposed Implementation**
How should this feature work?

**Alternatives Considered**
Other approaches you've considered

**Additional Context**
Any other relevant information
```

## 📋 Pull Request Process

### Before Submitting
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add/update tests
5. Update documentation
6. Run full test suite
7. Check code formatting

### Pull Request Template
```markdown
**Description**
Brief description of changes

**Type of Change**
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

**Testing**
- [ ] Tests pass locally
- [ ] New tests added
- [ ] Manual testing completed

**Checklist**
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Documentation updated
- [ ] No breaking changes (or documented)
```

### Review Process
1. Automated checks must pass
2. Code review by maintainers
3. Address feedback
4. Final approval and merge

## 🏆 Recognition

Contributors are recognized in:
- GitHub contributors list
- Release notes
- Project documentation

## 📞 Getting Help

- **GitHub Discussions** - General questions and ideas
- **GitHub Issues** - Bug reports and feature requests
- **Code Review** - Pull request discussions

## 📄 License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to Inspector GGUF! 🎉