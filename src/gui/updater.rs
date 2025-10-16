//! Update checking functionality via GitHub API.
//!
//! This module provides automated update checking capabilities by querying the GitHub
//! API for the latest release information. It handles version comparison, network
//! communication, and status reporting to keep users informed about available updates.
//!
//! # Features
//!
//! - **Automatic Version Detection**: Compares current version with latest GitHub release
//! - **Semantic Version Parsing**: Uses semver for accurate version comparison
//! - **Network Error Handling**: Graceful handling of network and API failures
//! - **Localization Support**: Returns status keys for multi-language support
//!
//! # Update Check Process
//!
//! 1. **API Request**: Queries GitHub API for latest release information
//! 2. **Version Parsing**: Extracts and parses version numbers using semver
//! 3. **Comparison**: Compares current version with latest available version
//! 4. **Status Generation**: Returns appropriate status message for UI display
//!
//! # Usage
//!
//! ## Basic Update Check
//!
//! ```rust
//! use inspector_gguf::gui::updater::check_for_updates;
//!
//! match check_for_updates() {
//!     Ok(status) => {
//!         if status.starts_with("new_version_available:") {
//!             let version = status.split(':').nth(1).unwrap_or("");
//!             println!("Update available: {}", version);
//!         } else if status == "latest_version" {
//!             println!("You have the latest version");
//!         }
//!     }
//!     Err(e) => eprintln!("Update check failed: {}", e),
//! }
//! ```
//!
//! ## Integration with Localization
//!
//! ```rust
//! use inspector_gguf::gui::updater::check_for_updates;
//! use inspector_gguf::localization::LanguageProvider;
//!
//! fn check_updates_with_localization<T: LanguageProvider>(app: &T) -> String {
//!     match check_for_updates() {
//!         Ok(status) => {
//!             if status.starts_with("new_version_available:") {
//!                 let version = status.split(':').nth(1).unwrap_or("");
//!                 app.t_with_args("messages.update_available", &[version])
//!             } else if status == "latest_version" {
//!                 app.t("messages.up_to_date")
//!             } else {
//!                 status
//!             }
//!         }
//!         Err(e) => app.t_with_args("messages.update_error", &[&e.to_string()]),
//!     }
//! }
//! ```

use reqwest::{blocking, StatusCode};
use semver::Version;
use std::error::Error;

/// Current application version extracted from Cargo.toml at compile time.
///
/// This constant contains the version string as defined in the package manifest,
/// automatically updated during the build process to ensure version consistency.
const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// GitHub repository identifier for update checking.
///
/// Format: "owner/repository" - used to construct GitHub API URLs for
/// querying release information and download links.
const GITHUB_REPO: &str = "FerrisMind/inspector-gguf";

/// Checks for updates by querying the GitHub API for the latest release.
///
/// This function performs a network request to the GitHub API to retrieve information
/// about the latest release, compares it with the current version, and returns a
/// status string suitable for localization and user display.
///
/// # Return Values
///
/// The function returns different status strings based on the comparison result:
///
/// - `"new_version_available:{tag}"` - A newer version is available (tag includes version)
/// - `"latest_version"` - Current version is up to date
/// - `"releases_not_found"` - No releases found in the repository
///
/// # Network Behavior
///
/// - **User Agent**: Identifies as "Inspector-GGUF-App" for API requests
/// - **Timeout**: Uses default reqwest timeout settings
/// - **Rate Limiting**: Respects GitHub API rate limits
/// - **HTTPS**: All requests use secure HTTPS connections
///
/// # Version Comparison
///
/// Uses semantic versioning (semver) for accurate version comparison:
/// - Handles version tags with or without 'v' prefix
/// - Compares major.minor.patch versions correctly
/// - Supports pre-release and build metadata
///
/// # Examples
///
/// ## Basic Update Check
///
/// ```rust
/// use inspector_gguf::gui::updater::check_for_updates;
///
/// match check_for_updates() {
///     Ok(status) => {
///         match status.as_str() {
///             "latest_version" => println!("You're up to date!"),
///             "releases_not_found" => println!("No releases available"),
///             s if s.starts_with("new_version_available:") => {
///                 let version = s.split(':').nth(1).unwrap_or("unknown");
///                 println!("Update available: {}", version);
///             }
///             _ => println!("Unknown status: {}", status),
///         }
///     }
///     Err(e) => eprintln!("Update check failed: {}", e),
/// }
/// ```
///
/// ## Async Update Check (in UI context)
///
/// ```rust
/// use inspector_gguf::gui::updater::check_for_updates;
/// use std::thread;
///
/// fn check_updates_async() {
///     thread::spawn(|| {
///         match check_for_updates() {
///             Ok(status) => {
///                 // Update UI with status
///                 println!("Update status: {}", status);
///             }
///             Err(e) => {
///                 eprintln!("Update check error: {}", e);
///             }
///         }
///     });
/// }
/// ```
///
/// # Errors
///
/// This function returns an error in the following cases:
///
/// - **Network Failures**: No internet connection, DNS resolution failures
/// - **API Errors**: GitHub API returns non-success HTTP status codes
/// - **Parsing Errors**: Invalid JSON response or malformed version strings
/// - **Version Errors**: Unable to parse current or remote version numbers
///
/// # Error Types
///
/// Common error scenarios and their meanings:
///
/// - `reqwest::Error` - Network or HTTP request failures
/// - `serde_json::Error` - JSON parsing failures
/// - `semver::Error` - Version string parsing failures
/// - Custom errors for API-specific issues
pub fn check_for_updates() -> Result<String, Box<dyn Error>> {
    let url = format!("https://api.github.com/repos/{}/releases/latest", GITHUB_REPO);

    let client = blocking::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "Inspector-GGUF-App")
        .send()?;

    if response.status() == StatusCode::NOT_FOUND {
        return Ok("releases_not_found".to_string());
    }

    if !response.status().is_success() {
        return Err(format!("github_api_failed:{}", response.status()).into());
    }

    let release_data: serde_json::Value = response.json()?;
    let latest_tag = release_data["tag_name"]
        .as_str()
        .ok_or("parse_tag_failed")?;

    // Remove 'v' prefix if present
    let latest_version_str = latest_tag.strip_prefix('v').unwrap_or(latest_tag);

    let current_version = Version::parse(CURRENT_VERSION)?;
    let latest_version = Version::parse(latest_version_str)?;

    if latest_version > current_version {
        Ok(format!("new_version_available:{}", latest_tag))
    } else {
        Ok("latest_version".to_string())
    }
}