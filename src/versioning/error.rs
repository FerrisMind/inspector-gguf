use thiserror::Error;

/// Comprehensive error types for version management operations.
///
/// `VersioningError` provides detailed error information for all failure scenarios
/// that can occur during Cargo.toml version management operations. Each variant
/// includes contextual information to help diagnose and resolve issues.
///
/// The error types are designed to be both human-readable and programmatically
/// actionable, allowing applications to handle different error scenarios appropriately.
///
/// # Examples
///
/// Handling different error types:
///
/// ```
/// use inspector_gguf::versioning::{VersioningError, read_cargo_version};
///
/// match read_cargo_version("nonexistent.toml") {
///     Ok(version) => println!("Version: {}", version),
///     Err(VersioningError::CargoTomlNotFound(path)) => {
///         eprintln!("Cargo.toml not found at: {}", path);
///     },
///     Err(VersioningError::VersionLineNotFound) => {
///         eprintln!("No version field found in Cargo.toml");
///     },
///     Err(VersioningError::InvalidVersionFormat(msg)) => {
///         eprintln!("Invalid version format: {}", msg);
///     },
///     Err(e) => eprintln!("Other error: {}", e),
/// }
/// ```
#[derive(Error, Debug)]
pub enum VersioningError {
    /// I/O operation failed during file reading or writing.
    ///
    /// This error occurs when the underlying file system operations fail,
    /// such as when reading from or writing to Cargo.toml files. The wrapped
    /// [`std::io::Error`] provides detailed information about the specific I/O failure.
    ///
    /// # Common Causes
    /// - Insufficient file permissions
    /// - Disk space exhaustion
    /// - Network file system issues
    /// - File locked by another process
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    /// The specified Cargo.toml file was not found at the given path.
    ///
    /// This error indicates that the file path provided to version management
    /// operations does not exist or is not accessible. The error includes the
    /// full path that was attempted for easier debugging.
    ///
    /// # Common Causes
    /// - Incorrect file path specification
    /// - File moved or deleted
    /// - Working directory different than expected
    /// - Insufficient permissions to access the file
    #[error("Cargo.toml not found at path: {0}")]
    CargoTomlNotFound(String),
    
    /// No version field was found in the Cargo.toml file.
    ///
    /// This error occurs when the Cargo.toml file exists and can be read, but
    /// does not contain a recognizable version field in the expected format.
    /// The version field should be in the `[package]` section as `version = "x.y.z"`.
    ///
    /// # Common Causes
    /// - Malformed Cargo.toml file
    /// - Version field in unexpected location
    /// - Non-standard version field formatting
    /// - Corrupted or incomplete Cargo.toml file
    #[error("Version line not found in Cargo.toml")]
    VersionLineNotFound,
    
    /// The version string format is invalid or cannot be parsed.
    ///
    /// This error occurs when a version string does not conform to semantic
    /// versioning standards or contains invalid characters or structure.
    /// The error message includes details about what was invalid.
    ///
    /// # Common Causes
    /// - Non-numeric version components
    /// - Missing version components (major, minor, patch)
    /// - Invalid pre-release or build metadata format
    /// - Unsupported version string format
    #[error("Invalid version format: {0}")]
    InvalidVersionFormat(String),
    
    /// Failed to parse version string using the semver parser.
    ///
    /// This error occurs when the version string appears to be in the correct
    /// format but fails semantic version parsing due to subtle formatting issues
    /// or edge cases in the semver specification.
    ///
    /// # Common Causes
    /// - Edge cases in semver parsing
    /// - Unusual pre-release version formats
    /// - Build metadata parsing issues
    /// - Version string encoding problems
    #[error("Failed to parse version: {0}")]
    VersionParseError(String),

    /// Git operation failed during commit analysis or repository operations.
    ///
    /// This error occurs when Git commands fail during commit analysis, tag
    /// retrieval, or other repository operations required for automatic
    /// version determination.
    ///
    /// # Common Causes
    /// - Git not installed or not in PATH
    /// - Not in a Git repository
    /// - Repository has no commits or tags
    /// - Git command execution failure
    /// - Insufficient permissions for Git operations
    #[error("Git operation failed: {0}")]
    GitError(String),
}