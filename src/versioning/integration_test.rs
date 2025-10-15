use semver::Version;
use std::fs;
use tempfile::NamedTempFile;
use crate::versioning::{update_cargo_version, read_cargo_version};

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_full_cargo_toml_update_cycle() {
        // Create a temporary Cargo.toml file
        let temp_file = NamedTempFile::new().unwrap();
        let cargo_content = r#"[package]
name = "test-package"
version = "1.0.0"
edition = "2021"
description = "A test package"

[dependencies]
serde = "1.0"
"#;
        
        fs::write(temp_file.path(), cargo_content).unwrap();
        
        // Read current version
        let current_version = read_cargo_version(temp_file.path()).unwrap();
        assert_eq!(current_version.to_string(), "1.0.0");
        
        // Update to new version
        let new_version = Version::parse("2.1.0").unwrap();
        update_cargo_version(temp_file.path(), &new_version).unwrap();
        
        // Verify the update
        let updated_version = read_cargo_version(temp_file.path()).unwrap();
        assert_eq!(updated_version.to_string(), "2.1.0");
        
        // Verify the file content still has proper formatting
        let updated_content = fs::read_to_string(temp_file.path()).unwrap();
        assert!(updated_content.contains(r#"version = "2.1.0""#));
        assert!(updated_content.contains(r#"name = "test-package""#));
        assert!(updated_content.contains(r#"edition = "2021""#));
        assert!(!updated_content.contains(r#"version = "1.0.0""#));
    }

    #[test]
    fn test_current_project_cargo_toml() {
        // Test with the actual project Cargo.toml
        let current_version = read_cargo_version("Cargo.toml").unwrap();
        
        // Should be able to read the current version (0.2.0 as seen in the file)
        assert_eq!(current_version.major, 0);
        assert_eq!(current_version.minor, 2);
        assert_eq!(current_version.patch, 0);
    }
}