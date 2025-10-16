use semver::Version;
use std::path::Path;
use crate::versioning::{CargoUpdater, VersioningError};

/// Updates the version field in a Cargo.toml file with a new semantic version.
///
/// This is a convenience function that creates a [`CargoUpdater`] instance and
/// performs the version update operation. It provides a simple interface for
/// programmatic version updates without requiring direct interaction with the
/// [`CargoUpdater`] struct.
///
/// The function preserves the original formatting and structure of the Cargo.toml
/// file while only modifying the version field.
///
/// # Arguments
///
/// * `cargo_path` - Path to the Cargo.toml file to update
/// * `new_version` - The new semantic version to set
///
/// # Returns
///
/// * `Ok(())` - If the version was successfully updated
/// * `Err(VersioningError)` - If the operation failed
///
/// # Examples
///
/// Basic version update:
///
/// ```
/// use semver::Version;
/// use inspector_gguf::versioning::update_cargo_version;
/// use tempfile::NamedTempFile;
/// use std::fs;
///
/// // Create a temporary Cargo.toml file for testing
/// let temp_file = NamedTempFile::new()?;
/// let cargo_content = r#"[package]
/// name = "test-package"
/// version = "1.0.0"
/// edition = "2021"
/// "#;
/// fs::write(temp_file.path(), cargo_content)?;
///
/// // Update the version
/// let new_version = Version::parse("2.1.0")?;
/// update_cargo_version(temp_file.path(), &new_version)?;
///
/// // Verify the update
/// let updated_content = fs::read_to_string(temp_file.path())?;
/// assert!(updated_content.contains(r#"version = "2.1.0""#));
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// Error handling example:
///
/// ```
/// use semver::Version;
/// use inspector_gguf::versioning::{update_cargo_version, VersioningError};
///
/// let new_version = Version::parse("2.0.0")?;
/// 
/// match update_cargo_version("nonexistent.toml", &new_version) {
///     Ok(()) => println!("Version updated successfully"),
///     Err(VersioningError::CargoTomlNotFound(path)) => {
///         eprintln!("Cargo.toml not found: {}", path);
///     },
///     Err(e) => eprintln!("Update failed: {}", e),
/// }
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Errors
///
/// Returns an error if:
/// - The Cargo.toml file does not exist or cannot be read
/// - The file does not contain a valid version field
/// - File I/O operations fail
/// - The version field cannot be updated
///
/// See [`VersioningError`] for detailed error information.
pub fn update_cargo_version<P: AsRef<Path>>(cargo_path: P, new_version: &Version) -> Result<(), VersioningError> {
    let updater = CargoUpdater::new(cargo_path);
    updater.update_version(new_version)
}

/// Reads and parses the current version from a Cargo.toml file.
///
/// This is a convenience function that creates a [`CargoUpdater`] instance and
/// reads the current version. It provides a simple interface for reading version
/// information without requiring direct interaction with the [`CargoUpdater`] struct.
///
/// # Arguments
///
/// * `cargo_path` - Path to the Cargo.toml file to read
///
/// # Returns
///
/// * `Ok(Version)` - The current version parsed as a semantic version
/// * `Err(VersioningError)` - If the version cannot be read or parsed
///
/// # Examples
///
/// Basic version reading:
///
/// ```
/// use inspector_gguf::versioning::read_cargo_version;
/// use tempfile::NamedTempFile;
/// use std::fs;
///
/// // Create a temporary Cargo.toml file for testing
/// let temp_file = NamedTempFile::new()?;
/// let cargo_content = r#"[package]
/// name = "test-package"
/// version = "1.2.3"
/// edition = "2021"
/// "#;
/// fs::write(temp_file.path(), cargo_content)?;
///
/// // Read the version
/// let current_version = read_cargo_version(temp_file.path())?;
/// assert_eq!(current_version.to_string(), "1.2.3");
/// println!("Current version: {}", current_version);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// Version comparison example:
///
/// ```
/// use inspector_gguf::versioning::read_cargo_version;
/// use semver::Version;
/// use tempfile::NamedTempFile;
/// use std::fs;
///
/// let temp_file = NamedTempFile::new()?;
/// let cargo_content = r#"[package]
/// name = "test-package"
/// version = "1.2.3"
/// "#;
/// fs::write(temp_file.path(), cargo_content)?;
///
/// let current_version = read_cargo_version(temp_file.path())?;
/// let target_version = Version::parse("2.0.0")?;
///
/// if current_version < target_version {
///     println!("Version {} is older than target {}", current_version, target_version);
/// }
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// Error handling example:
///
/// ```
/// use inspector_gguf::versioning::{read_cargo_version, VersioningError};
///
/// match read_cargo_version("missing.toml") {
///     Ok(version) => println!("Current version: {}", version),
///     Err(VersioningError::CargoTomlNotFound(path)) => {
///         eprintln!("File not found: {}", path);
///     },
///     Err(VersioningError::VersionLineNotFound) => {
///         eprintln!("No version field found in Cargo.toml");
///     },
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
///
/// See [`VersioningError`] for detailed error information.
pub fn read_cargo_version<P: AsRef<Path>>(cargo_path: P) -> Result<Version, VersioningError> {
    let updater = CargoUpdater::new(cargo_path);
    updater.read_current_version()
}