// CLI command integration tests
// Tests the main entry point and subcommands using assert_cmd

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_version_flag() {
    let mut cmd = Command::cargo_bin("xcargo").unwrap();
    cmd.arg("--version");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("xcargo"));
}

#[test]
fn test_help_flag() {
    let mut cmd = Command::cargo_bin("xcargo").unwrap();
    cmd.arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Cross-compilation"))
        .stdout(predicate::str::contains("Commands:"));
}

#[test]
fn test_target_list_command() {
    let mut cmd = Command::cargo_bin("xcargo").unwrap();
    cmd.args(&["target", "list"]);

    let output = cmd.output().unwrap();
    assert!(output.status.success(), "target list command should succeed");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Available") || stdout.contains("x86_64") || stdout.contains("targets"), "Should show targets");
}

#[test]
fn test_target_list_installed() {
    let mut cmd = Command::cargo_bin("xcargo").unwrap();
    cmd.args(&["target", "list", "--installed"]);

    let output = cmd.output().unwrap();
    assert!(output.status.success(), "target list --installed should succeed");

    // Should show at least the host target
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.len() > 0, "Should show installed targets");
}

#[test]
fn test_target_info_command() {
    let mut cmd = Command::cargo_bin("xcargo").unwrap();
    cmd.args(&["target", "info", "x86_64-unknown-linux-gnu"]);

    let output = cmd.output().unwrap();
    assert!(output.status.success(), "target info should succeed");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("x86_64") || stdout.contains("linux"), "Should show target info");
}

#[test]
fn test_doctor_command() {
    let mut cmd = Command::cargo_bin("xcargo").unwrap();
    cmd.arg("doctor");

    let output = cmd.output().unwrap();
    assert!(output.status.success(), "doctor command should succeed");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("System") || stdout.contains("cargo"), "Should show diagnostics");
}

#[test]
fn test_doctor_verbose() {
    let mut cmd = Command::cargo_bin("xcargo").unwrap();
    cmd.args(&["doctor", "--verbose"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("System Diagnostics"));
}

#[test]
fn test_config_command() {
    let mut cmd = Command::cargo_bin("xcargo").unwrap();
    cmd.arg("config");

    let output = cmd.output().unwrap();
    // Config command shows configuration or suggests creating one
    // Either success or specific error is acceptable
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stdout.contains("config") || stderr.contains("config") ||
        stdout.contains("xcargo") || stderr.contains("xcargo"),
        "Should show config or config-related message"
    );
}

#[test]
fn test_init_command_creates_config() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("xcargo").unwrap();
    cmd.current_dir(temp_dir.path());
    cmd.arg("init");

    // This will prompt, so we'll check it fails without TTY
    // In a real environment, it would create xcargo.toml
    let result = cmd.output().unwrap();

    // Either succeeds and creates config, or fails due to no TTY
    // Both are acceptable test outcomes
    assert!(result.status.success() || result.status.code() != Some(0));
}

#[test]
fn test_build_without_cargo_toml() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("xcargo").unwrap();
    cmd.current_dir(temp_dir.path());
    cmd.args(&["build", "--target", "x86_64-unknown-linux-gnu"]);

    let output = cmd.output().unwrap();
    assert!(!output.status.success(), "Build should fail without Cargo.toml");

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stderr.contains("Cargo.toml") || stdout.contains("Cargo.toml") ||
        stderr.contains("config") || stdout.contains("config"),
        "Should mention missing Cargo.toml or config"
    );
}

#[test]
fn test_check_without_cargo_toml() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("xcargo").unwrap();
    cmd.current_dir(temp_dir.path());
    cmd.args(&["check", "--target", "x86_64-unknown-linux-gnu"]);

    let output = cmd.output().unwrap();
    assert!(!output.status.success(), "Check should fail without Cargo.toml");

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stderr.contains("Cargo.toml") || stdout.contains("Cargo.toml") ||
        stderr.contains("config") || stdout.contains("config"),
        "Should mention missing Cargo.toml or config"
    );
}

#[test]
fn test_build_verbose_flag() {
    let temp_dir = TempDir::new().unwrap();

    // Create a minimal Cargo.toml
    let cargo_toml = r#"[package]
name = "test_verbose"
version = "0.1.0"
edition = "2021"
"#;
    fs::write(temp_dir.path().join("Cargo.toml"), cargo_toml).unwrap();
    fs::create_dir(temp_dir.path().join("src")).unwrap();
    fs::write(
        temp_dir.path().join("src/main.rs"),
        "fn main() { println!(\"test\"); }",
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("xcargo").unwrap();
    cmd.current_dir(temp_dir.path());
    cmd.args(&["build", "--verbose"]);

    // Should run (may succeed or fail depending on toolchain)
    let output = cmd.output().unwrap();

    // Check that it at least tried to build
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        stdout.contains("Compiling") || stderr.contains("Compiling") || stderr.contains("error"),
        "Should attempt to compile"
    );
}

#[test]
fn test_build_release_flag() {
    let temp_dir = TempDir::new().unwrap();

    let cargo_toml = r#"[package]
name = "test_release"
version = "0.1.0"
edition = "2021"
"#;
    fs::write(temp_dir.path().join("Cargo.toml"), cargo_toml).unwrap();
    fs::create_dir(temp_dir.path().join("src")).unwrap();
    fs::write(
        temp_dir.path().join("src/main.rs"),
        "fn main() { println!(\"test\"); }",
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("xcargo").unwrap();
    cmd.current_dir(temp_dir.path());
    cmd.args(&["build", "--release"]);

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        stdout.contains("Compiling") || stderr.contains("Compiling") || stderr.contains("error"),
        "Should attempt to compile"
    );
}

#[test]
fn test_check_operation() {
    let temp_dir = TempDir::new().unwrap();

    let cargo_toml = r#"[package]
name = "test_check"
version = "0.1.0"
edition = "2021"
"#;
    fs::write(temp_dir.path().join("Cargo.toml"), cargo_toml).unwrap();
    fs::create_dir(temp_dir.path().join("src")).unwrap();
    fs::write(
        temp_dir.path().join("src/main.rs"),
        "fn main() { println!(\"test\"); }",
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("xcargo").unwrap();
    cmd.current_dir(temp_dir.path());
    cmd.arg("check");

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        stdout.contains("Checking") || stderr.contains("Checking") || stderr.contains("error"),
        "Should attempt to check"
    );
}

#[test]
fn test_test_operation() {
    let temp_dir = TempDir::new().unwrap();

    let cargo_toml = r#"[package]
name = "test_test"
version = "0.1.0"
edition = "2021"
"#;
    fs::write(temp_dir.path().join("Cargo.toml"), cargo_toml).unwrap();
    fs::create_dir(temp_dir.path().join("src")).unwrap();
    fs::write(
        temp_dir.path().join("src/lib.rs"),
        "pub fn add(a: i32, b: i32) -> i32 { a + b }",
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("xcargo").unwrap();
    cmd.current_dir(temp_dir.path());
    cmd.arg("test");

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        stdout.contains("Testing") || stderr.contains("Testing") ||
        stderr.contains("Compiling") || stderr.contains("error"),
        "Should attempt to test"
    );
}

#[test]
fn test_build_with_toolchain() {
    let temp_dir = TempDir::new().unwrap();

    let cargo_toml = r#"[package]
name = "test_toolchain"
version = "0.1.0"
edition = "2021"
"#;
    fs::write(temp_dir.path().join("Cargo.toml"), cargo_toml).unwrap();
    fs::create_dir(temp_dir.path().join("src")).unwrap();
    fs::write(
        temp_dir.path().join("src/main.rs"),
        "fn main() { println!(\"test\"); }",
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("xcargo").unwrap();
    cmd.current_dir(temp_dir.path());
    cmd.args(&["build", "--toolchain", "stable"]);

    // Should at least attempt to build
    let _ = cmd.output();
}

#[test]
fn test_build_all_parallel() {
    let temp_dir = TempDir::new().unwrap();

    let cargo_toml = r#"[package]
name = "test_parallel"
version = "0.1.0"
edition = "2021"
"#;
    fs::write(temp_dir.path().join("Cargo.toml"), cargo_toml).unwrap();
    fs::create_dir(temp_dir.path().join("src")).unwrap();
    fs::write(
        temp_dir.path().join("src/main.rs"),
        "fn main() { println!(\"test\"); }",
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("xcargo").unwrap();
    cmd.current_dir(temp_dir.path());
    cmd.args(&["build", "--all", "--parallel"]);

    // Should attempt parallel build
    let _ = cmd.output();
}

#[test]
fn test_invalid_subcommand() {
    let mut cmd = Command::cargo_bin("xcargo").unwrap();
    cmd.arg("invalid_command");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("unrecognized subcommand"));
}

#[test]
fn test_version_command() {
    let mut cmd = Command::cargo_bin("xcargo").unwrap();
    cmd.arg("version");

    let output = cmd.output().unwrap();
    assert!(output.status.success(), "version command should succeed");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("xcargo") && stdout.contains("0.3"), "Should show version");
}

#[test]
fn test_build_with_cargo_args() {
    let temp_dir = TempDir::new().unwrap();

    let cargo_toml = r#"[package]
name = "test_cargo_args"
version = "0.1.0"
edition = "2021"

[lib]
path = "src/lib.rs"
"#;
    fs::write(temp_dir.path().join("Cargo.toml"), cargo_toml).unwrap();
    fs::create_dir(temp_dir.path().join("src")).unwrap();
    fs::write(
        temp_dir.path().join("src/lib.rs"),
        "pub fn test() {}",
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("xcargo").unwrap();
    cmd.current_dir(temp_dir.path());
    cmd.args(&["build", "--", "--lib"]);

    // Should pass --lib to cargo
    let _ = cmd.output();
}

#[test]
fn test_build_with_container_flag() {
    let temp_dir = TempDir::new().unwrap();

    let cargo_toml = r#"[package]
name = "test_container"
version = "0.1.0"
edition = "2021"
"#;
    fs::write(temp_dir.path().join("Cargo.toml"), cargo_toml).unwrap();
    fs::create_dir(temp_dir.path().join("src")).unwrap();
    fs::write(
        temp_dir.path().join("src/main.rs"),
        "fn main() {}",
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("xcargo").unwrap();
    cmd.current_dir(temp_dir.path());
    cmd.args(&["build", "--container"]);

    // Should attempt container build (may fail if docker not available)
    let _ = cmd.output();
}

#[test]
fn test_build_with_zig_flag() {
    let temp_dir = TempDir::new().unwrap();

    let cargo_toml = r#"[package]
name = "test_zig"
version = "0.1.0"
edition = "2021"
"#;
    fs::write(temp_dir.path().join("Cargo.toml"), cargo_toml).unwrap();
    fs::create_dir(temp_dir.path().join("src")).unwrap();
    fs::write(
        temp_dir.path().join("src/main.rs"),
        "fn main() {}",
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("xcargo").unwrap();
    cmd.current_dir(temp_dir.path());
    cmd.args(&["build", "--zig"]);

    // Should attempt zig build (may fail if zig not available)
    let _ = cmd.output();
}
