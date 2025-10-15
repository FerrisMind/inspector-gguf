use std::fs;
use std::path::Path;
use regex::Regex;
use semver::Version;
use crate::versioning::error::VersioningError;

/// Cargo.toml updater for version management
pub struct CargoUpdater {
    cargo_path: String,
}

impl CargoUpdater {
    /// Create a new CargoUpdater instance
    pub fn new<P: AsRef<Path>>(cargo_path: P) -> Self {
        Self {
            cargo_path: cargo_path.as_ref().to_string_lossy().to_string(),
        }
    }

    /// Read the current version from Cargo.toml
    pub fn read_current_version(&self) -> Result<Version, VersioningError> {
        let content = self.read_cargo_toml()?;
        self.extract_version_from_content(&content)
    }

    /// Update the version in Cargo.toml
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