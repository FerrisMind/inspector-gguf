//! Version management functionality for Cargo.toml files.
//!
//! This module provides comprehensive version management capabilities for Rust projects,
//! including programmatic updating of Cargo.toml version fields, CLI integration for
//! version manipulation, and robust error handling for version-related operations.
//!
//! The versioning system is designed to support automated release workflows, version
//! bumping operations, and integration with CI/CD pipelines. It maintains compatibility
//! with semantic versioning (semver) standards and preserves Cargo.toml formatting.
//!
//! # Key Features
//!
//! - **Cargo.toml Version Updates**: Programmatic reading and writing of version fields
//! - **CLI Integration**: Command-line interface for version management operations
//! - **Semantic Versioning**: Full support for semver version parsing and manipulation
//! - **Format Preservation**: Maintains original Cargo.toml formatting and structure
//! - **Error Handling**: Comprehensive error types for robust error management
//!
//! # Examples
//!
//! Basic version reading and updating:
//!
//! ```
//! use inspector_gguf::versioning::{read_cargo_version, update_cargo_version};
//! use semver::Version;
//!
//! // Read current version from Cargo.toml
//! let current_version = read_cargo_version("Cargo.toml")?;
//! println!("Current version: {}", current_version);
//!
//! // Update to a new version
//! let new_version = Version::parse("2.0.0")?;
//! update_cargo_version("Cargo.toml", &new_version)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! CLI-based version management:
//!
//! ```
//! use inspector_gguf::versioning::VersionCli;
//!
//! // Increment patch version
//! let new_version = VersionCli::increment_version("Cargo.toml", "patch")?;
//! println!("Updated to version: {}", new_version);
//!
//! // Set specific version
//! VersionCli::update_version("Cargo.toml", "3.1.0")?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! # Organization
//!
//! This module is organized into several key components:
//!
//! - [`CargoUpdater`] - Core functionality for reading and writing Cargo.toml versions with [`CargoUpdater::read_current_version`] and [`CargoUpdater::update_version`]
//! - [`VersionCli`] - Command-line interface for version management operations including [`VersionCli::increment_version`] and [`VersionCli::update_version`]
//! - [`VersioningError`] - Comprehensive error types for version-related failures
//! - [`update_cargo_version`] - Convenience function for programmatic version updates using [`CargoUpdater`]
//! - [`read_cargo_version`] - Convenience function for reading current versions via [`CargoUpdater`]
//!
//! # Integration with Inspector GGUF
//!
//! The versioning system integrates with Inspector GGUF's release management workflow,
//! supporting automated version updates during the build and release process. It can be
//! used both as a library component and through CLI commands for manual version management.
//!
//! The system works alongside [`crate::gui::updater`] for checking updates and can be
//! integrated with [`crate::gui::GgufApp`] for version display in the about dialog.

/// Cargo.toml version updating utilities
pub mod cargo_updater;
/// Error types for versioning operations
pub mod error;
/// Command-line interface for version management
pub mod cli;
mod lib;

#[cfg(test)]
mod integration_test;

pub use cargo_updater::CargoUpdater;
pub use error::VersioningError;
pub use lib::{update_cargo_version, read_cargo_version};
pub use cli::VersionCli;