use std::fs;
use std::path::Path;
use regex::Regex;
use semver::Version;
use crate::versioning::error::VersioningError;

/// A robust Cargo.toml version management utility.
///
/// `CargoUpdater` provides safe and reliable methods for reading and updating version
/// information in Cargo.toml files while preserving the original file formatting and
/// structure. It uses regular expressions to locate and modify version fields without
/// disrupting other content or formatting.
///
/// The updater is designed to handle various Cargo.toml formatting styles and provides
/// comprehensive error handling for common failure scenarios such as missing files,
/// invalid version formats, and I/O errors.
///
/// # Examples
///
/// Basic usage for reading and updating versions:
///
/// ```
/// use inspector_gguf::versioning::CargoUpdater;
/// use semver::Version;
/// use tempfile::NamedTempFile;
/// use std::fs;
///
/// // Create a test Cargo.toml file
/// let temp_file = NamedTempFile::new()?;
/// let cargo_content = r#"[package]
/// name = "example"
/// version = "1.0.0"
/// edition = "2021"
/// "#;
/// fs::write(temp_file.path(), cargo_content)?;
///
/// // Create updater for the Cargo.toml file
/// let updater = CargoUpdater::new(temp_file.path());
///
/// // Read current version
/// let current_version = updater.read_current_version()?;
/// assert_eq!(current_version.to_string(), "1.0.0");
///
/// // Update to new version
/// let new_version = Version::parse("2.1.0")?;
/// updater.update_version(&new_version)?;
///
/// // Verify the update
/// let updated_version = updater.read_current_version()?;
/// assert_eq!(updated_version.to_string(), "2.1.0");
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// Advanced usage with error handling:
///
/// ```
/// use inspector_gguf::versioning::{CargoUpdater, VersioningError};
/// use semver::Version;
///
/// let updater = CargoUpdater::new("example/Cargo.toml");
///
/// match updater.read_current_version() {
///     Ok(version) => {
///         println!("Current version: {}", version);
///         
///         // Increment patch version
///         let new_version = Version::new(version.major, version.minor, version.patch + 1);
///         
///         match updater.update_version(&new_version) {
///             Ok(()) => println!("Updated to version {}", new_version),
///             Err(e) => eprintln!("Failed to update version: {}", e),
///         }
///     },
///     Err(VersioningError::CargoTomlNotFound(path)) => {
///         eprintln!("Cargo.toml not found at: {}", path);
///     },
///     Err(e) => eprintln!("Error reading version: {}", e),
/// }
/// ```
///
/// # Error Handling
///
/// All methods return [`Result`] types with [`VersioningError`] for comprehensive
/// error handling. Common error scenarios include:
///
/// - File not found or inaccessible
/// - Invalid version format in Cargo.toml
/// - Missing version field in Cargo.toml
/// - I/O errors during file operations
///
/// See [`VersioningError`] for detailed error type information, [`crate::versioning::VersionCli`]
/// for command-line operations, and [`crate::versioning::update_cargo_version`] for convenience functions.
pub struct CargoUpdater {
    cargo_path: String,
}

impl CargoUpdater {
    /// Creates a new `CargoUpdater` instance for the specified Cargo.toml file.
    ///
    /// # Arguments
    ///
    /// * `cargo_path` - Path to the Cargo.toml file to manage
    ///
    /// # Examples
    ///
    /// ```
    /// use inspector_gguf::versioning::CargoUpdater;
    ///
    /// let updater = CargoUpdater::new("Cargo.toml");
    /// let updater_with_path = CargoUpdater::new("path/to/Cargo.toml");
    /// ```
    pub fn new<P: AsRef<Path>>(cargo_path: P) -> Self {
        Self {
            cargo_path: cargo_path.as_ref().to_string_lossy().to_string(),
        }
    }

    /// Reads and parses the current version from the Cargo.toml file.
    ///
    /// This method reads the Cargo.toml file and extracts the version field using
    /// regular expressions. It supports various formatting styles and whitespace
    /// configurations around the version field.
    ///
    /// # Returns
    ///
    /// * `Ok(Version)` - The current version parsed as a semantic version
    /// * `Err(VersioningError)` - If the file cannot be read, parsed, or version not found
    ///
    /// # Examples
    ///
    /// ```
    /// use inspector_gguf::versioning::CargoUpdater;
    ///
    /// let updater = CargoUpdater::new("Cargo.toml");
    /// match updater.read_current_version() {
    ///     Ok(version) => println!("Current version: {}", version),
    ///     Err(e) => eprintln!("Failed to read version: {}", e),
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The Cargo.toml file does not exist or cannot be read
    /// - The file does not contain a valid version field
    /// - The version string cannot be parsed as a semantic version
    pub fn read_current_version(&self) -> Result<Version, VersioningError> {
        let content = self.read_cargo_toml()?;
        self.extract_version_from_content(&content)
    }

    /// Updates the version field in the Cargo.toml file while preserving formatting.
    ///
    /// This method reads the current Cargo.toml content, replaces the version field
    /// with the new version, and writes the updated content back to the file. The
    /// original formatting and structure of the file are preserved.
    ///
    /// # Arguments
    ///
    /// * `new_version` - The new semantic version to set in Cargo.toml
    ///
    /// # Examples
    ///
    /// ```
    /// use inspector_gguf::versioning::CargoUpdater;
    /// use semver::Version;
    ///
    /// let updater = CargoUpdater::new("Cargo.toml");
    /// let new_version = Version::parse("2.1.0")?;
    /// 
    /// updater.update_version(&new_version)?;
    /// println!("Version updated to {}", new_version);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The Cargo.toml file cannot be read or written
    /// - The version field is not found in the file
    /// - File I/O operations fail
    ///
    /// # Panics
    ///
    /// This function does not panic under normal circumstances.
    pub fn update_version(&self, new_version: &Version) -> Result<(), VersioningError> {
        let content = self.read_cargo_toml()?;
        let updated_content = self.replace_version_in_content(&content, new_version)?;
        self.write_cargo_toml(&updated_content)?;
        Ok(())
    }

    /// Read Cargo.toml file content
    fn read_cargo_toml(&self) -> Result<String, VersioningError> {
        if !Path::new(&self.cargo_path).exists() {
            return Err(VersioningError::CargoTomlNotFound(self.cargo_path.clone()));
        }
        
        fs::read_to_string(&self.cargo_path)
            .map_err(VersioningError::Io)
    }

    /// Write updated content to Cargo.toml
    fn write_cargo_toml(&self, content: &str) -> Result<(), VersioningError> {
        fs::write(&self.cargo_path, content)
            .map_err(VersioningError::Io)
    }

    /// Extract version from Cargo.toml content
    fn extract_version_from_content(&self, content: &str) -> Result<Version, VersioningError> {
        let version_regex = Regex::new(r#"version\s*=\s*"([^"]+)""#)
            .map_err(|e| VersioningError::VersionParseError(e.to_string()))?;

        if let Some(captures) = version_regex.captures(content)
            && let Some(version_str) = captures.get(1) {
                return Version::parse(version_str.as_str())
                    .map_err(|e| VersioningError::InvalidVersionFormat(e.to_string()));
            }

        Err(VersioningError::VersionLineNotFound)
    }

    /// Replace version in Cargo.toml content while preserving formatting
    fn replace_version_in_content(&self, content: &str, new_version: &Version) -> Result<String, VersioningError> {
        let version_regex = Regex::new(r#"(version\s*=\s*)"([^"]+)""#)
            .map_err(|e| VersioningError::VersionParseError(e.to_string()))?;

        if version_regex.is_match(content) {
            let updated_content = version_regex.replace(
                content,
                format!(r#"${{1}}"{}""#, new_version).as_str()
            );
            Ok(updated_content.to_string())
        } else {
            Err(VersioningError::VersionLineNotFound)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_version_from_content() {
        let content = r#"
[package]
name = "test-package"
version = "1.2.3"
edition = "2021"
"#;
        
        let updater = CargoUpdater::new("test");
        let version = updater.extract_version_from_content(content).unwrap();
        assert_eq!(version.to_string(), "1.2.3");
    }

    #[test]
    fn test_replace_version_in_content() {
        let content = r#"
[package]
name = "test-package"
version = "1.2.3"
edition = "2021"
"#;
        
        let updater = CargoUpdater::new("test");
        let new_version = Version::parse("2.0.0").unwrap();
        let updated_content = updater.replace_version_in_content(content, &new_version).unwrap();
        
        assert!(updated_content.contains(r#"version = "2.0.0""#));
        assert!(!updated_content.contains(r#"version = "1.2.3""#));
    }

    #[test]
    fn test_preserve_formatting() {
        let content = r#"
[package]
name = "test-package"
version    =    "1.2.3"
edition = "2021"
"#;
        
        let updater = CargoUpdater::new("test");
        let new_version = Version::parse("2.0.0").unwrap();
        let updated_content = updater.replace_version_in_content(content, &new_version).unwrap();
        
        // Should preserve the spacing around the equals sign
        assert!(updated_content.contains(r#"version    =    "2.0.0""#));
    }

    #[test]
    fn test_version_not_found() {
        let content = r#"
[package]
name = "test-package"
edition = "2021"
"#;
        
        let updater = CargoUpdater::new("test");
        let result = updater.extract_version_from_content(content);
        assert!(matches!(result, Err(VersioningError::VersionLineNotFound)));
    }
}