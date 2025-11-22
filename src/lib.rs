//! # xcargo 🚀
//!
//! > Cross-compilation made simple
//!
//! **xcargo** is a Rust cross-compilation tool that simplifies building for
//! multiple targets through automatic toolchain management and intelligent
//! container usage.
//!
//! ## Features
//!
//! - **Zero Configuration**: Works out of the box for common cross-compilation scenarios
//! - **Multiple Strategies**: Native toolchains, Zig, or container-based builds
//! - **Parallel Builds**: Build for multiple targets simultaneously
//! - **Target Detection**: Automatic discovery of installed Rust targets
//! - **Plugin System**: Extensible architecture for custom toolchains
//! - **System Diagnostics**: Built-in `doctor` command for troubleshooting
//!
//! ## Quick Start
//!
//! ### CLI Usage
//!
//! ```bash
//! # Install xcargo
//! cargo install xcargo
//!
//! # Add a target
//! rustup target add x86_64-pc-windows-gnu
//!
//! # Build for Windows from macOS/Linux
//! xcargo build --target x86_64-pc-windows-gnu --zig
//!
//! # Build for all configured targets in parallel
//! xcargo build --all
//!
//! # Check system setup
//! xcargo doctor
//! ```
//!
//! ### Library Usage
//!
//! ```rust,ignore
//! use xcargo::prelude::*;
//!
//! // Detect available targets
//! let targets = Target::detect_installed()?;
//! for target in targets {
//!     println!("Target: {} (Tier {})", target.triple, target.tier);
//! }
//!
//! // Configure and execute a build
//! let options = BuildOptions::new()
//!     .target("x86_64-pc-windows-gnu")
//!     .release(true)
//!     .use_zig(true);
//!
//! let builder = Builder::new(options)?;
//! builder.execute()?;
//! ```
//!
//! ## Configuration
//!
//! Create an `xcargo.toml` file to configure cross-compilation targets:
//!
//! ```toml
//! [project]
//! name = "my-app"
//!
//! [[targets]]
//! triple = "x86_64-pc-windows-gnu"
//!
//! [[targets]]
//! triple = "x86_64-unknown-linux-gnu"
//! linker = "x86_64-linux-gnu-gcc"
//!
//! [build]
//! parallel = true
//! profile = "release"
//! ```
//!
//! ## Architecture
//!
//! xcargo is organized into several key modules:
//!
//! - [`target`] - Target platform detection and requirements
//! - [`config`] - Configuration file parsing and management
//! - [`toolchain`] - Rust toolchain and cross-compiler management
//! - [`build`] - Build orchestration and execution
//! - [`container`] - Docker/Podman container runtime integration
//! - [`plugin`] - Plugin system for extensibility
//! - [`doctor`] - System diagnostics and health checks
//! - [`error`] - Error types and handling
//! - [`cache`] - Build caching for faster incremental builds
//!
//! ## Cross-Compilation Strategies
//!
//! xcargo supports three cross-compilation strategies:
//!
//! ### 1. Native Toolchains (Fastest)
//!
//! ```bash
//! # Install MinGW for Windows cross-compilation
//! brew install mingw-w64  # macOS
//! xcargo build --target x86_64-pc-windows-gnu
//! ```
//!
//! ### 2. Zig Cross-Compilation (Recommended)
//!
//! ```bash
//! # Install Zig
//! brew install zig
//! xcargo build --target x86_64-unknown-linux-gnu --zig
//! ```
//!
//! ### 3. Container-Based Builds
//!
//! ```bash
//! # Requires Docker or Podman
//! xcargo build --target x86_64-unknown-linux-gnu --container
//! ```
//!
//! ## Examples
//!
//! ### Building for Windows from macOS
//!
//! ```bash
//! rustup target add x86_64-pc-windows-gnu
//! xcargo build --target x86_64-pc-windows-gnu --zig --release
//! ```
//!
//! ### Multi-Platform Release Build
//!
//! ```bash
//! # Configure targets in xcargo.toml
//! xcargo build --all --release
//! ```
//!
//! ### Custom Plugin
//!
//! ```rust,ignore
//! use xcargo::plugin::{Plugin, PluginContext};
//! use xcargo::error::Result;
//!
//! struct NotificationPlugin;
//!
//! impl Plugin for NotificationPlugin {
//!     fn name(&self) -> &str {
//!         "notification-plugin"
//!     }
//!
//!     fn on_post_build(&self, ctx: &PluginContext) -> Result<()> {
//!         println!("✅ Build completed for {}", ctx.target);
//!         Ok(())
//!     }
//! }
//! ```
//!
//! ## Platform Support
//!
//! | Host Platform | Supported Targets |
//! |--------------|-------------------|
//! | macOS | Windows (GNU/MSVC), Linux (glibc/musl), macOS, ARM |
//! | Linux | Windows (GNU), Linux (all variants), ARM |
//! | Windows | Windows (native), Linux (via WSL2 or containers) |
//!
//! ## See Also
//!
//! - [Cross-Compilation Guide](https://xcargo.dev/guides/cross-compilation)
//! - [Plugin Development](https://xcargo.dev/guides/plugin-development)
//! - [Troubleshooting](https://xcargo.dev/guides/troubleshooting)
//! - [GitHub Repository](https://github.com/ibrahimcesar/xcargo)

#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::match_same_arms)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::assigning_clones)]
#![allow(clippy::manual_let_else)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unused_self)]
#![allow(clippy::field_reassign_with_default)]
#![allow(clippy::unnecessary_unwrap)]
#![cfg_attr(test, allow(deprecated))]

/// Target platform definitions and detection
pub mod target;

/// Configuration file handling
pub mod config;

/// Toolchain installation and management
pub mod toolchain;

/// Build orchestration
pub mod build;

/// Container runtime integration
#[cfg(feature = "container")]
pub mod container;

/// Dependency management (OpenSSL, etc.)
pub mod deps {}

/// Output and logging
pub mod output;

/// Error types
pub mod error;

/// Build caching
pub mod cache;

/// Plugin system for extensibility
pub mod plugin;

/// System diagnostics
pub mod doctor;

/// Prelude for convenient imports
pub mod prelude {
    //! Convenient re-exports
    //!
    //! ```rust
    //! use xcargo::prelude::*;
    //! ```
    #![allow(clippy::mixed_attributes_style)]

    pub use crate::build::{BuildOptions, Builder, CargoOperation};
    pub use crate::config::Config;
    pub use crate::error::{Error, ExitCode, Result};
    pub use crate::target::{Target, TargetRequirements, TargetTier};
    pub use crate::toolchain::{Toolchain, ToolchainManager};
}

// Re-exports
pub use error::{Error, ExitCode, Result};

#[cfg(test)]
mod tests {
    #[test]
    fn test_placeholder() {
        // Placeholder test
        assert_eq!(2 + 2, 4);
    }
}
