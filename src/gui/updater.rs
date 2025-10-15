//! Update checking functionality via GitHub API
//! Handles version comparison and update status management

use reqwest::{blocking, StatusCode};
use semver::Version;
use std::error::Error;

// Update checking constants
const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");
const GITHUB_REPO: &str = "FerrisMind/inspector-gguf";

/// Checks for updates by querying the GitHub API for the latest release
/// 
/// Returns a status string that can be used for localization:
/// - "new_version_available:{tag}" if a newer version is available
/// - "latest_version" if the current version is up to date
/// - "releases_not_found" if no releases are found
/// 
/// # Errors
/// 
/// Returns an error if:
/// - Network request fails
/// - GitHub API returns an error status
/// - JSON parsing fails
/// - Version parsing fails
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