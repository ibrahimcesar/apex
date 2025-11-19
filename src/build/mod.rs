//! Build execution and management
//!
//! This module handles the actual build process, including invoking cargo
//! with the appropriate flags for cross-compilation.

use crate::config::Config;
use crate::error::{Error, Result};
use crate::output::{helpers, tips};
use crate::target::Target;
use crate::toolchain::ToolchainManager;
use std::process::Command;
use std::sync::{Arc, Mutex};
use tokio::task;

/// Build options and configuration
#[derive(Debug, Clone)]
pub struct BuildOptions {
    /// Target triple to build for
    pub target: Option<String>,

    /// Release mode
    pub release: bool,

    /// Additional cargo arguments
    pub cargo_args: Vec<String>,

    /// Toolchain to use (defaults to active)
    pub toolchain: Option<String>,

    /// Verbose output
    pub verbose: bool,
}

impl Default for BuildOptions {
    fn default() -> Self {
        Self {
            target: None,
            release: false,
            cargo_args: Vec::new(),
            toolchain: None,
            verbose: false,
        }
    }
}

/// Build executor
pub struct Builder {
    /// Toolchain manager
    toolchain_manager: ToolchainManager,

    /// Configuration
    config: Config,
}

impl Builder {
    /// Create a new builder
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use xcargo::build::Builder;
    ///
    /// # fn example() -> xcargo::Result<()> {
    /// let builder = Builder::new()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new() -> Result<Self> {
        let toolchain_manager = ToolchainManager::new()?;
        let config = Config::discover()?.map(|(c, _)| c).unwrap_or_default();

        Ok(Self {
            toolchain_manager,
            config,
        })
    }

    /// Create a builder with a specific configuration
    pub fn with_config(config: Config) -> Result<Self> {
        let toolchain_manager = ToolchainManager::new()?;

        Ok(Self {
            toolchain_manager,
            config,
        })
    }

    /// Build the current project
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use xcargo::build::{Builder, BuildOptions};
    ///
    /// # fn example() -> xcargo::Result<()> {
    /// let builder = Builder::new()?;
    /// let options = BuildOptions {
    ///     target: Some("x86_64-pc-windows-gnu".to_string()),
    ///     release: true,
    ///     ..Default::default()
    /// };
    /// builder.build(&options)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn build(&self, options: &BuildOptions) -> Result<()> {
        helpers::section("xcargo build");

        // Determine target
        let target_triple = if let Some(target) = &options.target {
            target.clone()
        } else if let Some(default_target) = self.config.targets.default.first() {
            helpers::info(format!("Using default target from config: {}", default_target));
            default_target.clone()
        } else {
            let host = Target::detect_host()?;
            helpers::info(format!("No target specified, using host: {}", host.triple));
            host.triple
        };

        // Parse target
        let target = Target::from_triple(&target_triple)?;
        helpers::progress(format!("Building for target: {}", target.triple));

        // Determine toolchain
        let toolchain = if let Some(tc) = &options.toolchain {
            tc.clone()
        } else {
            "stable".to_string()
        };

        // Ensure target is installed
        helpers::progress(format!("Checking toolchain and target..."));
        self.toolchain_manager.prepare_target(&toolchain, &target)?;
        helpers::success("Toolchain and target ready");

        // Show tips based on target
        if target.os != Target::detect_host()?.os {
            helpers::tip("Cross-compiling to a different OS");
            if self.config.container.use_when == "target.os != host.os" {
                helpers::hint("Container builds not yet implemented - using native toolchain");
            }
        }

        // Get linker suggestion
        let requirements = target.get_requirements();
        if let Some(linker) = requirements.linker {
            helpers::hint(format!("Recommended linker: {}", linker));
            helpers::tip(format!("Set linker in xcargo.toml: [targets.\"{}\"] linker = \"{}\"",
                target.triple, linker));
        }

        // Build cargo command
        helpers::progress("Running cargo build...");
        let mut cmd = Command::new("cargo");

        // Add toolchain override if specified
        if options.toolchain.is_some() {
            cmd.arg(format!("+{}", toolchain));
        }

        cmd.arg("build");

        // Add target
        cmd.arg("--target").arg(&target.triple);

        // Add release flag
        if options.release {
            cmd.arg("--release");
        }

        // Add verbose flag
        if options.verbose || self.config.build.cargo_flags.contains(&"--verbose".to_string()) {
            cmd.arg("--verbose");
        }

        // Add additional cargo flags from config
        for flag in &self.config.build.cargo_flags {
            if flag != "--verbose" || !options.verbose {
                cmd.arg(flag);
            }
        }

        // Add additional args from options
        for arg in &options.cargo_args {
            cmd.arg(arg);
        }

        if options.verbose {
            helpers::info(format!("Executing: {:?}", cmd));
        }

        // Execute build
        let status = cmd.status()
            .map_err(|e| Error::Build(format!("Failed to execute cargo: {}", e)))?;

        if status.success() {
            println!(); // Empty line for spacing
            helpers::success(format!("Build completed for {}", target.triple));

            // Show helpful tips
            if options.release {
                helpers::tip(format!("Release build artifacts are in target/{}/release/", target.triple));
            } else {
                helpers::tip(format!("Debug build artifacts are in target/{}/debug/", target.triple));
            }

            // Additional tips based on target
            if target.os == "windows" && Target::detect_host()?.os != "windows" {
                helpers::tip(format!("Test Windows binaries with Wine: wine target/{}/debug/your-app.exe", target.triple));
            }

            Ok(())
        } else {
            Err(Error::Build(format!(
                "Build failed for target {}",
                target.triple
            )))
        }
    }

    /// Build for multiple targets (sequential)
    pub fn build_all(&self, targets: &[String], options: &BuildOptions) -> Result<()> {
        helpers::section("xcargo build (multiple targets)");
        helpers::info(format!("Building for {} targets", targets.len()));

        let mut successes = Vec::new();
        let mut failures = Vec::new();

        for (idx, target) in targets.iter().enumerate() {
            println!("\n[{}/{}] Target: {}", idx + 1, targets.len(), target);
            println!("{}", "─".repeat(50));

            let mut target_options = options.clone();
            target_options.target = Some(target.clone());

            match self.build(&target_options) {
                Ok(()) => successes.push(target.clone()),
                Err(e) => {
                    helpers::error(format!("Failed to build {}: {}", target, e));
                    failures.push(target.clone());
                }
            }
        }

        println!("\n");
        helpers::section("Build Summary");
        helpers::success(format!("{} target(s) built successfully", successes.len()));

        if !failures.is_empty() {
            helpers::error(format!("{} target(s) failed", failures.len()));
            for target in &failures {
                helpers::error(format!("  - {}", target));
            }
            return Err(Error::Build("Some targets failed to build".to_string()));
        }

        helpers::tip(tips::PARALLEL_BUILDS);
        Ok(())
    }

    /// Build for multiple targets in parallel
    pub async fn build_all_parallel(&self, targets: &[String], options: &BuildOptions) -> Result<()> {
        helpers::section("xcargo build (parallel)");
        helpers::info(format!("Building for {} targets in parallel", targets.len()));

        let successes = Arc::new(Mutex::new(Vec::new()));
        let failures = Arc::new(Mutex::new(Vec::new()));

        let mut handles = Vec::new();

        for (idx, target) in targets.iter().enumerate() {
            let target = target.clone();
            let mut target_options = options.clone();
            target_options.target = Some(target.clone());

            let successes = Arc::clone(&successes);
            let failures = Arc::clone(&failures);

            let handle = task::spawn_blocking(move || {
                println!("\n[{}] Starting build for: {}", idx + 1, target);
                println!("{}", "─".repeat(50));

                // Create a new builder for this task
                let builder = match Builder::new() {
                    Ok(b) => b,
                    Err(e) => {
                        let mut failures = failures.lock().unwrap();
                        failures.push(target.clone());
                        eprintln!("Failed to create builder for {}: {}", target, e);
                        return;
                    }
                };

                match builder.build(&target_options) {
                    Ok(()) => {
                        let mut successes = successes.lock().unwrap();
                        successes.push(target.clone());
                    }
                    Err(e) => {
                        let mut failures = failures.lock().unwrap();
                        failures.push(target.clone());
                        eprintln!("Failed to build {}: {}", target, e);
                    }
                }
            });

            handles.push(handle);
        }

        // Wait for all builds to complete
        for handle in handles {
            handle.await.map_err(|e| Error::Build(format!("Task join error: {}", e)))?;
        }

        let successes = successes.lock().unwrap();
        let failures = failures.lock().unwrap();

        println!("\n");
        helpers::section("Build Summary");
        helpers::success(format!("{} target(s) built successfully", successes.len()));

        if !failures.is_empty() {
            helpers::error(format!("{} target(s) failed", failures.len()));
            for target in failures.iter() {
                helpers::error(format!("  - {}", target));
            }
            return Err(Error::Build("Some targets failed to build".to_string()));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_options_default() {
        let options = BuildOptions::default();
        assert!(options.target.is_none());
        assert!(!options.release);
        assert!(options.cargo_args.is_empty());
    }

    #[test]
    fn test_builder_new() {
        // This test will succeed if rustup is installed
        let builder = Builder::new();
        if builder.is_err() {
            // Skip test if rustup is not available
            return;
        }
        assert!(builder.is_ok());
    }
}
