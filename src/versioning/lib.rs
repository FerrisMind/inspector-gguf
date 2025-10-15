use semver::Version;
use std::path::Path;
use crate::versioning::{CargoUpdater, VersioningError};

/// Update the version in Cargo.toml file
/// 
/// # Arguments
/// * `cargo_path` - Path to the Cargo.toml file
/// * `new_version` - New version to set
/// 
/// # Returns
/// * `Ok(())` if successful
/// * `Err(VersioningError)` if failed
/// 
/// # Example
/// ```
/// use semver::Version;
/// use inspector_gguf::versioning::update_cargo_version;
/// 
/// let new_version = Version::parse("1.2.3").unwrap();
/// update_cargo_version("Cargo.toml", &new_version).unwrap();
/// ```
pub fn update_cargo_version<P: AsRef<Path>>(cargo_path: P, new_version: &Version) -> Result<(), VersioningError> {
    let updater = CargoUpdater::new(cargo_path);
    updater.update_version(new_version)
}

/// Read the current version from Cargo.toml file
/// 
/// # Arguments
/// * `cargo_path` - Path to the Cargo.toml file
/// 
/// # Returns
/// * `Ok(Version)` with the current version if successful
/// * `Err(VersioningError)` if failed
/// 
/// # Example
/// ```
/// use inspector_gguf::versioning::read_cargo_version;
/// 
/// let current_version = read_cargo_version("Cargo.toml").unwrap();
/// println!("Current version: {}", current_version);
/// ```
pub fn read_cargo_version<P: AsRef<Path>>(cargo_path: P) -> Result<Version, VersioningError> {
    let updater = CargoUpdater::new(cargo_path);
    updater.read_current_version()
}