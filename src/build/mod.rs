//! Build execution and management
//!
//! This module handles the actual build process, including invoking cargo
//! with the appropriate flags for cross-compilation.

mod options;
mod executor;
mod parallel;

// Re-export public types
pub use options::{BuildOptions, CargoOperation};
pub use executor::Builder;
