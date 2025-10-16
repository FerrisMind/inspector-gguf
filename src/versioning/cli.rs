use semver::Version;
use std::path::Path;
use crate::versioning::{update_cargo_version, read_cargo_version, VersioningError};

/// Command-line interface for Cargo.toml version management operations.
///
/// `VersionCli` provides a high-level interface for common version management tasks
/// that can be easily integrated into CLI applications, build scripts, and automated
/// workflows. It offers convenient methods for version reading, updating, and
/// semantic version incrementing.
///
/// This struct is designed as a stateless utility with static methods, making it
/// suitable for use in command-line tools and scripts where you need to perform
/// version operations without maintaining state between calls.
///
/// # Examples
///
/// Basic version operations:
///
/// ```
/// use inspector_gguf::versioning::VersionCli;
/// use tempfile::NamedTempFile;
/// use std::fs;
///
/// // Create a test Cargo.toml file
/// let temp_file = NamedTempFile::new()?;
/// let cargo_content = r#"[package]
/// name = "test-cli"
/// version = "1.2.3"
/// edition = "2021"
/// "#;
/// fs::write(temp_file.path(), cargo_content)?;
///
/// // Show current version
/// let current = VersionCli::show_current_version(temp_file.path())?;
/// assert_eq!(current, "1.2.3");
///
/// // Update to specific version
/// VersionCli::update_version(temp_file.path(), "2.1.0")?;
/// let updated = VersionCli::show_current_version(temp_file.path())?;
/// assert_eq!(updated, "2.1.0");
///
/// // Increment patch version (2.1.0 -> 2.1.1)
/// let new_version = VersionCli::increment_version(temp_file.path(), "patch")?;
/// assert_eq!(new_version, "2.1.1");
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// Version increment workflow:
///
/// ```
/// use inspector_gguf::versioning::VersionCli;
/// use tempfile::NamedTempFile;
/// use std::fs;
///
/// let temp_file = NamedTempFile::new()?;
/// let cargo_content = r#"[package]
/// name = "increment-test"
/// version = "1.0.0"
/// "#;
/// fs::write(temp_file.path(), cargo_content)?;
///
/// // Increment patch: 1.0.0 -> 1.0.1
/// let patch_version = VersionCli::increment_version(temp_file.path(), "patch")?;
/// assert_eq!(patch_version, "1.0.1");
///
/// // Increment minor: 1.0.1 -> 1.1.0
/// let minor_version = VersionCli::increment_version(temp_file.path(), "minor")?;
/// assert_eq!(minor_version, "1.1.0");
///
/// // Increment major: 1.1.0 -> 2.0.0
/// let major_version = VersionCli::increment_version(temp_file.path(), "major")?;
/// assert_eq!(major_version, "2.0.0");
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// Error handling example:
///
/// ```
/// use inspector_gguf::versioning::{VersionCli, VersioningError};
///
/// // Test invalid increment type
/// let temp_file = tempfile::NamedTempFile::new()?;
/// let cargo_content = r#"[package]
/// name = "error-test"
/// version = "1.0.0"
/// "#;
/// std::fs::write(temp_file.path(), cargo_content)?;
///
/// match VersionCli::increment_version(temp_file.path(), "invalid") {
///     Ok(_) => panic!("Should have failed"),
///     Err(VersioningError::InvalidVersionFormat(msg)) => {
///         assert!(msg.contains("Invalid increment type"));
///     },
///     Err(e) => panic!("Unexpected error: {}", e),
/// }
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Integration with Build Systems
///
/// The CLI interface is particularly useful for integration with build systems
/// and CI/CD pipelines where version management needs to be automated:
///
/// ```bash
/// # Example usage in build scripts
/// cargo run --bin version-tool -- increment patch
/// cargo run --bin version-tool -- set 2.0.0
/// ```
pub struct VersionCli;

impl VersionCli {
    /// Updates the Cargo.toml version field with a new version string.
    ///
    /// This method parses the provided version string and updates the Cargo.toml
    /// file accordingly. The version string must be a valid semantic version.
    ///
    /// # Arguments
    ///
    /// * `cargo_path` - Path to the Cargo.toml file to update
    /// * `version_str` - New version string in semantic version format (e.g., "2.1.0")
    ///
    /// # Examples
    ///
    /// ```
    /// use inspector_gguf::versioning::VersionCli;
    ///
    /// // Update to a specific version
    /// VersionCli::update_version("Cargo.toml", "2.1.0")?;
    ///
    /// // Update with pre-release version
    /// VersionCli::update_version("Cargo.toml", "2.1.0-beta.1")?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The version string is not a valid semantic version
    /// - The Cargo.toml file cannot be read or written
    /// - The version field is not found in the Cargo.toml file
    pub fn update_version<P: AsRef<Path>>(cargo_path: P, version_str: &str) -> Result<(), VersioningError> {
        let new_version = Version::parse(version_str)
            .map_err(|e| VersioningError::InvalidVersionFormat(e.to_string()))?;
        
        update_cargo_version(cargo_path, &new_version)
    }

    /// Reads and returns the current version from Cargo.toml as a string.
    ///
    /// This method provides a convenient way to retrieve the current version
    /// for display purposes or further processing in CLI applications.
    ///
    /// # Arguments
    ///
    /// * `cargo_path` - Path to the Cargo.toml file to read
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - The current version as a string representation
    /// * `Err(VersioningError)` - If the version cannot be read or parsed
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use inspector_gguf::versioning::VersionCli;
    ///
    /// let current_version = VersionCli::show_current_version("Cargo.toml")?;
    /// println!("Current version: {}", current_version);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The Cargo.toml file does not exist or cannot be read
    /// - The version field is missing or invalid
    /// - The version cannot be parsed as a semantic version
    pub fn show_current_version<P: AsRef<Path>>(cargo_path: P) -> Result<String, VersioningError> {
        let version = read_cargo_version(cargo_path)?;
        Ok(version.to_string())
    }

    /// Increments the version according to semantic versioning rules.
    ///
    /// This method reads the current version, increments the specified component
    /// (major, minor, or patch), and updates the Cargo.toml file with the new version.
    /// When incrementing major or minor versions, lower-order components are reset to zero.
    ///
    /// # Arguments
    ///
    /// * `cargo_path` - Path to the Cargo.toml file to update
    /// * `increment_type` - Type of increment: "major", "minor", or "patch"
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - The new version string after incrementing
    /// * `Err(VersioningError)` - If the operation fails
    ///
    /// # Examples
    ///
    /// ```
    /// use inspector_gguf::versioning::VersionCli;
    ///
    /// // Increment patch version (1.2.3 -> 1.2.4)
    /// let new_version = VersionCli::increment_version("Cargo.toml", "patch")?;
    /// println!("New patch version: {}", new_version);
    ///
    /// // Increment minor version (1.2.3 -> 1.3.0)
    /// let new_version = VersionCli::increment_version("Cargo.toml", "minor")?;
    /// println!("New minor version: {}", new_version);
    ///
    /// // Increment major version (1.2.3 -> 2.0.0)
    /// let new_version = VersionCli::increment_version("Cargo.toml", "major")?;
    /// println!("New major version: {}", new_version);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// # Semantic Versioning Rules
    ///
    /// - **Major**: Increments major version, resets minor and patch to 0
    /// - **Minor**: Increments minor version, resets patch to 0, keeps major unchanged
    /// - **Patch**: Increments patch version, keeps major and minor unchanged
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The increment type is not "major", "minor", or "patch"
    /// - The Cargo.toml file cannot be read or written
    /// - The current version cannot be parsed
    pub fn increment_version<P: AsRef<Path>>(cargo_path: P, increment_type: &str) -> Result<String, VersioningError> {
        let current_version = read_cargo_version(&cargo_path)?;
        
        let new_version = match increment_type.to_lowercase().as_str() {
            "major" => Version::new(current_version.major + 1, 0, 0),
            "minor" => Version::new(current_version.major, current_version.minor + 1, 0),
            "patch" => Version::new(current_version.major, current_version.minor, current_version.patch + 1),
            _ => return Err(VersioningError::InvalidVersionFormat(
                format!("Invalid increment type: {}. Use 'major', 'minor', or 'patch'", increment_type)
            )),
        };

        update_cargo_version(cargo_path, &new_version)?;
        Ok(new_version.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::fs;

    #[test]
    fn test_cli_update_version() {
        let temp_file = NamedTempFile::new().unwrap();
        let cargo_content = r#"[package]
name = "test"
version = "1.0.0"
"#;
        fs::write(temp_file.path(), cargo_content).unwrap();

        // Test updating version
        VersionCli::update_version(temp_file.path(), "2.1.0").unwrap();
        let updated_version = VersionCli::show_current_version(temp_file.path()).unwrap();
        assert_eq!(updated_version, "2.1.0");
    }

    #[test]
    fn test_cli_increment_version() {
        let temp_file = NamedTempFile::new().unwrap();
        let cargo_content = r#"[package]
name = "test"
version = "1.2.3"
"#;
        fs::write(temp_file.path(), cargo_content).unwrap();

        // Test major increment
        let new_version = VersionCli::increment_version(temp_file.path(), "major").unwrap();
        assert_eq!(new_version, "2.0.0");

        // Reset and test minor increment
        fs::write(temp_file.path(), cargo_content).unwrap();
        let new_version = VersionCli::increment_version(temp_file.path(), "minor").unwrap();
        assert_eq!(new_version, "1.3.0");

        // Reset and test patch increment
        fs::write(temp_file.path(), cargo_content).unwrap();
        let new_version = VersionCli::increment_version(temp_file.path(), "patch").unwrap();
        assert_eq!(new_version, "1.2.4");
    }

    #[test]
    fn test_cli_invalid_increment_type() {
        let temp_file = NamedTempFile::new().unwrap();
        let cargo_content = r#"[package]
name = "test"
version = "1.0.0"
"#;
        fs::write(temp_file.path(), cargo_content).unwrap();

        let result = VersionCli::increment_version(temp_file.path(), "invalid");
        assert!(matches!(result, Err(VersioningError::InvalidVersionFormat(_))));
    }
}