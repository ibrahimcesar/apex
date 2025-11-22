//! Build options and cargo operations

/// Cargo operation type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CargoOperation {
    /// cargo build
    #[default]
    Build,
    /// cargo check
    Check,
    /// cargo test
    Test,
}

impl CargoOperation {
    /// Get the cargo subcommand name
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            CargoOperation::Build => "build",
            CargoOperation::Check => "check",
            CargoOperation::Test => "test",
        }
    }

    /// Get a human-readable description
    #[must_use]
    pub fn description(&self) -> &'static str {
        match self {
            CargoOperation::Build => "Building",
            CargoOperation::Check => "Checking",
            CargoOperation::Test => "Testing",
        }
    }
}

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

    /// Use container for build
    pub use_container: bool,

    /// Zig preference: None = auto, Some(true) = force, Some(false) = disable
    pub use_zig: Option<bool>,

    /// Cargo operation (build, check, test)
    pub operation: CargoOperation,
}

impl Default for BuildOptions {
    fn default() -> Self {
        Self {
            target: None,
            release: false,
            cargo_args: Vec::new(),
            toolchain: None,
            verbose: false,
            use_container: false,
            use_zig: None,
            operation: CargoOperation::Build,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cargo_operation_as_str() {
        assert_eq!(CargoOperation::Build.as_str(), "build");
        assert_eq!(CargoOperation::Check.as_str(), "check");
        assert_eq!(CargoOperation::Test.as_str(), "test");
    }

    #[test]
    fn test_cargo_operation_description() {
        assert_eq!(CargoOperation::Build.description(), "Building");
        assert_eq!(CargoOperation::Check.description(), "Checking");
        assert_eq!(CargoOperation::Test.description(), "Testing");
    }

    #[test]
    fn test_build_options_default() {
        let options = BuildOptions::default();
        assert_eq!(options.target, None);
        assert!(!options.release);
        assert!(options.cargo_args.is_empty());
    }
}
