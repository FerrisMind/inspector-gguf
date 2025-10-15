use inspector_gguf::versioning::{read_cargo_version, VersionCli};
use semver::Version;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Cargo.toml Version Updater Example");
    println!("==================================");

    // Read current version
    let current_version = read_cargo_version("Cargo.toml")?;
    println!("Current version: {}", current_version);

    // Example 1: Update to a specific version
    println!("\nExample 1: Updating to a specific version (0.2.1)");
    let new_version = Version::parse("0.2.1")?;
    
    // Note: We're not actually updating the real Cargo.toml in this example
    // In a real scenario, you would call:
    // update_cargo_version("Cargo.toml", &new_version)?;
    println!("Would update version to: {}", new_version);

    // Example 2: Using CLI interface for version increments
    println!("\nExample 2: Version increment operations");
    
    // Simulate incrementing patch version
    let patch_version = Version::new(current_version.major, current_version.minor, current_version.patch + 1);
    println!("Patch increment would result in: {}", patch_version);
    
    // Simulate incrementing minor version
    let minor_version = Version::new(current_version.major, current_version.minor + 1, 0);
    println!("Minor increment would result in: {}", minor_version);
    
    // Simulate incrementing major version
    let major_version = Version::new(current_version.major + 1, 0, 0);
    println!("Major increment would result in: {}", major_version);

    println!("\nExample 3: CLI interface usage");
    println!("Current version via CLI: {}", VersionCli::show_current_version("Cargo.toml")?);

    println!("\nNote: This example reads the current version but doesn't modify Cargo.toml");
    println!("To actually update the version, uncomment the update_cargo_version calls");

    Ok(())
}