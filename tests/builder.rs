// Integration tests for Builder

use xcargo::build::{BuildOptions, Builder, CargoOperation};
use xcargo::config::Config;
use xcargo::error::Result;

#[test]
fn test_builder_new() -> Result<()> {
    let builder = Builder::new()?;
    // Just verify it can be created
    drop(builder);
    Ok(())
}

#[test]
fn test_builder_with_config() -> Result<()> {
    let config = Config::default();
    let builder = Builder::with_config(config)?;
    drop(builder);
    Ok(())
}

#[test]
fn test_build_options_default() {
    let options = BuildOptions::default();
    assert_eq!(options.operation, CargoOperation::Build);
    assert!(!options.release);
    assert!(options.target.is_none());
    assert!(options.use_zig.is_none());
    assert!(!options.use_container);
}

#[test]
fn test_build_options_fields() {
    let mut options = BuildOptions::default();
    options.target = Some("x86_64-unknown-linux-gnu".to_string());
    options.release = true;
    options.use_zig = Some(true);

    assert_eq!(options.target, Some("x86_64-unknown-linux-gnu".to_string()));
    assert!(options.release);
    assert_eq!(options.use_zig, Some(true));
}

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
fn test_cargo_operation_clone() {
    let op1 = CargoOperation::Build;
    let op2 = op1.clone();
    assert_eq!(op1, op2);
}

#[test]
fn test_build_options_clone() {
    let mut options1 = BuildOptions::default();
    options1.target = Some("x86_64-pc-windows-gnu".to_string());
    options1.release = true;

    let options2 = options1.clone();

    assert_eq!(options1.target, options2.target);
    assert_eq!(options1.release, options2.release);
    assert_eq!(options1.operation, options2.operation);
}

#[test]
fn test_build_options_various_operations() {
    let operations = vec![
        CargoOperation::Build,
        CargoOperation::Check,
        CargoOperation::Test,
    ];

    for op in operations {
        let mut options = BuildOptions::default();
        options.operation = op;
        assert_eq!(options.operation, op);
    }
}

#[test]
fn test_build_options_cargo_args() {
    let mut options = BuildOptions::default();
    options.cargo_args = vec!["--bins".to_string(), "--lib".to_string()];

    assert_eq!(options.cargo_args.len(), 2);
    assert!(options.cargo_args.contains(&"--bins".to_string()));
}

#[test]
fn test_build_options_verbose() {
    let mut options = BuildOptions::default();
    options.verbose = true;

    assert!(options.verbose);
}

#[test]
fn test_build_options_zig_none() {
    let options = BuildOptions::default();
    assert!(options.use_zig.is_none());
}

#[test]
fn test_build_options_zig_enabled() {
    let mut options = BuildOptions::default();
    options.use_zig = Some(true);
    assert_eq!(options.use_zig, Some(true));
}

#[test]
fn test_build_options_zig_disabled() {
    let mut options = BuildOptions::default();
    options.use_zig = Some(false);
    assert_eq!(options.use_zig, Some(false));
}

#[test]
fn test_build_options_container() {
    let mut options = BuildOptions::default();
    options.use_container = true;
    assert!(options.use_container);
}

#[test]
fn test_build_options_target_formats() {
    let targets = vec![
        "x86_64-unknown-linux-gnu",
        "x86_64-pc-windows-gnu",
        "aarch64-apple-darwin",
        "wasm32-unknown-unknown",
    ];

    for target in targets {
        let mut options = BuildOptions::default();
        options.target = Some(target.to_string());
        assert_eq!(options.target, Some(target.to_string()));
    }
}

#[test]
fn test_build_options_toolchain() {
    let mut options = BuildOptions::default();
    options.toolchain = Some("stable".to_string());
    assert_eq!(options.toolchain, Some("stable".to_string()));
}

#[test]
fn test_cargo_operation_default() {
    let op = CargoOperation::default();
    assert_eq!(op, CargoOperation::Build);
}
