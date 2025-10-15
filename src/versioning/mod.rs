pub mod cargo_updater;
pub mod error;
pub mod cli;
mod lib;

#[cfg(test)]
mod integration_test;

pub use cargo_updater::CargoUpdater;
pub use error::VersioningError;
pub use lib::{update_cargo_version, read_cargo_version};
pub use cli::VersionCli;