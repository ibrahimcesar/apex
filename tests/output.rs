// Integration tests for Output module

use xcargo::output::helpers;

#[test]
fn test_helpers_section() {
    // These functions output to stdout/stderr, so we just verify they don't panic
    helpers::section("Test Section");
    assert!(true);
}

#[test]
fn test_helpers_info() {
    helpers::info("Test info message");
    assert!(true);
}

#[test]
fn test_helpers_success() {
    helpers::success("Test success message");
    assert!(true);
}

#[test]
fn test_helpers_warning() {
    helpers::warning("Test warning message");
    assert!(true);
}

#[test]
fn test_helpers_error() {
    helpers::error("Test error message");
    assert!(true);
}

#[test]
fn test_helpers_with_string() {
    helpers::section(String::from("String Section"));
    helpers::info(String::from("String info"));
    helpers::success(String::from("String success"));
    helpers::warning(String::from("String warning"));
    helpers::error(String::from("String error"));
    assert!(true);
}

#[test]
fn test_helpers_with_format() {
    let count = 42;
    helpers::section(format!("Processing {} items", count));
    helpers::info(format!("Found {} targets", count));
    helpers::success(format!("Built {} packages", count));
    helpers::warning(format!("{} warnings found", count));
    helpers::error(format!("{} errors found", count));
    assert!(true);
}

#[test]
fn test_helpers_multiline() {
    helpers::section("Multi\nLine\nSection");
    helpers::info("Multi\nLine\nInfo");
    assert!(true);
}

#[test]
fn test_helpers_empty_string() {
    helpers::section("");
    helpers::info("");
    helpers::success("");
    helpers::warning("");
    helpers::error("");
    assert!(true);
}

#[test]
fn test_helpers_unicode() {
    helpers::section("Unicode: 🦀 Rust 🚀");
    helpers::info("Build: ✅ Success");
    helpers::success("🎉 Complete!");
    helpers::warning("⚠️  Warning");
    helpers::error("❌ Error");
    assert!(true);
}

#[test]
fn test_helpers_long_messages() {
    let long_message = "a".repeat(1000);
    helpers::section(&long_message);
    helpers::info(&long_message);
    assert!(true);
}

#[test]
fn test_helpers_special_characters() {
    helpers::section("Special: !@#$%^&*()");
    helpers::info("Path: /usr/local/bin");
    helpers::success("Target: x86_64-unknown-linux-gnu");
    assert!(true);
}

#[test]
fn test_helpers_sequential_calls() {
    // Simulate real usage
    helpers::section("xcargo build");
    helpers::info("Using target: x86_64-unknown-linux-gnu");
    helpers::info("Building in release mode");
    helpers::success("Build completed");
    assert!(true);
}

#[test]
fn test_helpers_nested_messages() {
    helpers::section("Main Task");
    helpers::info("  Subtask 1");
    helpers::info("  Subtask 2");
    helpers::success("  Subtask 3 complete");
    helpers::success("Main task complete");
    assert!(true);
}
