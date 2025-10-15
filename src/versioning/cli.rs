use semver::Version;
use std::path::Path;
use crate::versioning::{update_cargo_version, read_cargo_version, VersioningError};

/// CLI interface for Cargo.toml version management
pub struct VersionCli;

impl VersionCli {
    /// Update Cargo.toml version with a new version string
    pub fn update_version<P: AsRef<Path>>(cargo_path: P, version_str: &str) -> Result<(), VersioningError> {
        let new_version = Version::parse(version_str)
            .map_err(|e| VersioningError::InvalidVersionFormat(e.to_string()))?;
        
        update_cargo_version(cargo_path, &new_version)
    }

    /// Read and display current version from Cargo.toml
    pub fn show_current_version<P: AsRef<Path>>(cargo_path: P) -> Result<String, VersioningError> {
        let version = read_cargo_version(cargo_path)?;
        Ok(version.to_string())
    }

    /// Increment version based on type (major, minor, patch)
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