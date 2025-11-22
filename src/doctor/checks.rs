//! Individual diagnostic checks

use crate::config::ConfigDiscovery;
use crate::toolchain::ToolchainManager;
use std::process::Command;
use which::which;

/// Status of a diagnostic check
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CheckStatus {
    /// Check passed successfully
    Pass,
    /// Check passed with warnings
    Warning,
    /// Check failed (non-critical)
    Fail,
    /// Check failed (critical - blocks usage)
    Critical,
}

/// Result of a diagnostic check
#[derive(Debug, Clone)]
pub struct CheckResult {
    /// Name of the check
    pub name: String,
    /// Status of the check
    pub status: CheckStatus,
    /// Detailed message
    pub message: String,
    /// Optional suggestion for fixing issues
    pub suggestion: Option<String>,
}

impl CheckResult {
    /// Create a passing check result
    pub fn pass(name: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            status: CheckStatus::Pass,
            message: message.into(),
            suggestion: None,
        }
    }

    /// Create a warning check result
    pub fn warning(
        name: impl Into<String>,
        message: impl Into<String>,
        suggestion: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            status: CheckStatus::Warning,
            message: message.into(),
            suggestion: Some(suggestion.into()),
        }
    }

    /// Create a failing check result
    pub fn fail(
        name: impl Into<String>,
        message: impl Into<String>,
        suggestion: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            status: CheckStatus::Fail,
            message: message.into(),
            suggestion: Some(suggestion.into()),
        }
    }

    /// Create a critical failure check result
    pub fn critical(
        name: impl Into<String>,
        message: impl Into<String>,
        suggestion: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            status: CheckStatus::Critical,
            message: message.into(),
            suggestion: Some(suggestion.into()),
        }
    }
}

/// Trait for diagnostic checks
pub trait Check {
    /// Run the check and return the result
    fn run(&self) -> CheckResult;
}

/// Check if rustup is installed
pub fn check_rustup() -> CheckResult {
    match which("rustup") {
        Ok(path) => {
            // Get rustup version
            if let Ok(output) = Command::new("rustup").arg("--version").output() {
                let version = String::from_utf8_lossy(&output.stdout);
                let version_line = version.lines().next().unwrap_or("unknown");
                CheckResult::pass(
                    "rustup",
                    format!("Found at {:?}: {}", path, version_line),
                )
            } else {
                CheckResult::pass("rustup", format!("Found at {:?}", path))
            }
        }
        Err(_) => CheckResult::critical(
            "rustup",
            "rustup not found in PATH",
            "Install rustup from https://rustup.rs/",
        ),
    }
}

/// Check if cargo is installed
pub fn check_cargo() -> CheckResult {
    match which("cargo") {
        Ok(path) => {
            if let Ok(output) = Command::new("cargo").arg("--version").output() {
                let version = String::from_utf8_lossy(&output.stdout);
                let version_line = version.lines().next().unwrap_or("unknown");
                CheckResult::pass("cargo", format!("Found at {:?}: {}", path, version_line))
            } else {
                CheckResult::pass("cargo", format!("Found at {:?}", path))
            }
        }
        Err(_) => CheckResult::critical(
            "cargo",
            "cargo not found in PATH",
            "Install Rust toolchain from https://rustup.rs/",
        ),
    }
}

/// Check default toolchain
pub fn check_default_toolchain() -> CheckResult {
    let manager = match ToolchainManager::new() {
        Ok(m) => m,
        Err(_) => {
            return CheckResult::fail(
                "default toolchain",
                "Could not initialize toolchain manager",
                "Ensure rustup is properly installed",
            )
        }
    };

    match manager.get_default_toolchain() {
        Ok(Some(toolchain)) => CheckResult::pass(
            "default toolchain",
            format!("Using toolchain: {}", toolchain.name),
        ),
        Ok(None) => CheckResult::warning(
            "default toolchain",
            "No default toolchain set",
            "Run: rustup default stable",
        ),
        Err(_) => CheckResult::warning(
            "default toolchain",
            "Could not determine default toolchain",
            "Run: rustup default stable",
        ),
    }
}

/// Check installed targets
pub fn check_installed_targets() -> CheckResult {
    let manager = match ToolchainManager::new() {
        Ok(m) => m,
        Err(_) => {
            return CheckResult::fail(
                "installed targets",
                "Could not check installed targets",
                "Ensure rustup is properly installed",
            )
        }
    };

    // Get default toolchain first
    let toolchain = match manager.get_default_toolchain() {
        Ok(Some(tc)) => tc.name,
        Ok(None) => "stable".to_string(), // Fallback to stable
        Err(_) => {
            return CheckResult::fail(
                "installed targets",
                "Could not determine toolchain",
                "Run: rustup default stable",
            )
        }
    };

    match manager.list_targets(&toolchain) {
        Ok(targets) => {
            let installed_count = targets.len();

            if installed_count <= 1 {
                // Only host target
                CheckResult::warning(
                    "installed targets",
                    "No additional targets installed (only host target)",
                    "Install targets with: rustup target add <target>",
                )
            } else {
                CheckResult::pass(
                    "installed targets",
                    format!("{} target(s) installed for {}", installed_count, toolchain),
                )
            }
        }
        Err(_) => CheckResult::fail(
            "installed targets",
            "Could not list installed targets",
            "Check rustup installation",
        ),
    }
}

/// Check if Zig is available
pub fn check_zig() -> CheckResult {
    match which("zig") {
        Ok(path) => {
            if let Ok(output) = Command::new("zig").arg("version").output() {
                let version = String::from_utf8_lossy(&output.stdout);
                CheckResult::pass(
                    "zig",
                    format!("Found at {:?}: v{}", path, version.trim()),
                )
            } else {
                CheckResult::pass("zig", format!("Found at {:?}", path))
            }
        }
        Err(_) => CheckResult::warning(
            "zig",
            "Zig not found (optional)",
            "Install Zig for easy Linux cross-compilation: https://ziglang.org/download/",
        ),
    }
}

/// Check if Docker is available
pub fn check_docker() -> CheckResult {
    match which("docker") {
        Ok(path) => {
            // Check if Docker daemon is running
            if let Ok(output) = Command::new("docker").arg("info").output() {
                if output.status.success() {
                    CheckResult::pass("docker", format!("Found and running at {:?}", path))
                } else {
                    CheckResult::warning(
                        "docker",
                        format!("Found at {:?} but daemon not running", path),
                        "Start Docker daemon",
                    )
                }
            } else {
                CheckResult::warning(
                    "docker",
                    format!("Found at {:?} but status unknown", path),
                    "Verify Docker installation",
                )
            }
        }
        Err(_) => CheckResult::warning(
            "docker",
            "Docker not found (optional)",
            "Install Docker for container-based builds: https://docker.com/",
        ),
    }
}

/// Check if Podman is available
pub fn check_podman() -> CheckResult {
    match which("podman") {
        Ok(path) => {
            if let Ok(output) = Command::new("podman").arg("--version").output() {
                let version = String::from_utf8_lossy(&output.stdout);
                let version_line = version.lines().next().unwrap_or("unknown");
                CheckResult::pass("podman", format!("Found at {:?}: {}", path, version_line))
            } else {
                CheckResult::pass("podman", format!("Found at {:?}", path))
            }
        }
        Err(_) => CheckResult::warning(
            "podman",
            "Podman not found (optional)",
            "Install Podman as Docker alternative: https://podman.io/",
        ),
    }
}

/// Check for common linkers
pub fn check_common_linkers() -> CheckResult {
    let linkers = vec![
        ("gcc", "GNU Compiler Collection"),
        ("clang", "LLVM Clang"),
        ("x86_64-w64-mingw32-gcc", "MinGW-w64 for Windows"),
        ("aarch64-linux-gnu-gcc", "ARM64 Linux cross-compiler"),
    ];

    let mut found = Vec::new();
    let mut missing = Vec::new();

    for (linker, description) in linkers {
        if which(linker).is_ok() {
            found.push(description);
        } else {
            missing.push((linker, description));
        }
    }

    if found.is_empty() {
        CheckResult::warning(
            "common linkers",
            "No common cross-compilation linkers found",
            "Install build tools for your platform (build-essential, mingw-w64, etc.)",
        )
    } else {
        let message = format!("Found {} linker(s): {}", found.len(), found.join(", "));

        if missing.is_empty() {
            CheckResult::pass("common linkers", message)
        } else {
            let suggestion = format!(
                "Missing: {}. Install as needed for your targets.",
                missing
                    .iter()
                    .map(|(_, desc)| *desc)
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            CheckResult::warning("common linkers", message, suggestion)
        }
    }
}

/// Check for xcargo configuration file
pub fn check_config_file() -> CheckResult {
    match ConfigDiscovery::find() {
        Ok(Some(path)) => CheckResult::pass(
            "xcargo.toml",
            format!("Found configuration at: {}", path.display()),
        ),
        Ok(None) => CheckResult::warning(
            "xcargo.toml",
            "No xcargo.toml found in current directory or parents",
            "Run 'xcargo init' to create a configuration file",
        ),
        Err(e) => CheckResult::fail(
            "xcargo.toml",
            format!("Error checking configuration: {}", e),
            "Check file permissions",
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_result_constructors() {
        let pass = CheckResult::pass("test", "success");
        assert_eq!(pass.status, CheckStatus::Pass);
        assert!(pass.suggestion.is_none());

        let warning = CheckResult::warning("test", "issue", "fix");
        assert_eq!(warning.status, CheckStatus::Warning);
        assert!(warning.suggestion.is_some());

        let fail = CheckResult::fail("test", "failed", "suggestion");
        assert_eq!(fail.status, CheckStatus::Fail);
        assert!(fail.suggestion.is_some());

        let critical = CheckResult::critical("test", "critical", "urgent fix");
        assert_eq!(critical.status, CheckStatus::Critical);
        assert!(critical.suggestion.is_some());
    }

    #[test]
    fn test_check_rustup() {
        let result = check_rustup();
        // Should either pass or fail, but not panic
        assert!(!result.name.is_empty());
    }

    #[test]
    fn test_check_cargo() {
        let result = check_cargo();
        // cargo should be available in test environment
        assert_eq!(result.status, CheckStatus::Pass);
    }

    #[test]
    fn test_check_default_toolchain() {
        let result = check_default_toolchain();
        // Should not panic
        assert!(!result.name.is_empty());
    }

    #[test]
    fn test_check_zig() {
        let result = check_zig();
        // Zig may or may not be installed
        assert!(
            matches!(result.status, CheckStatus::Pass | CheckStatus::Warning)
        );
    }

    #[test]
    fn test_check_config_file() {
        let result = check_config_file();
        // Config may or may not exist, but check should work
        assert!(!result.name.is_empty());
    }
}
