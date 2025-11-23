// Toolchain management tests
// Tests toolchain detection, installation, and version checking

use xcargo::target::Target;
use xcargo::toolchain::ToolchainManager;
use xcargo::Result;

#[test]
fn test_toolchain_manager_creation() -> Result<()> {
    let manager = ToolchainManager::new()?;
    let _ = manager;
    Ok(())
}

#[test]
fn test_check_target_installed_host() -> Result<()> {
    let manager = ToolchainManager::new()?;
    let host = Target::detect_host()?;

    let is_installed = manager.is_target_installed("stable", &host.triple)?;
    // Host target should be installed
    assert!(is_installed, "Host target should be installed");

    Ok(())
}

#[test]
fn test_check_nonexistent_target() -> Result<()> {
    let manager = ToolchainManager::new()?;

    // Use a very unlikely target
    let is_installed = manager.is_target_installed("stable", "fake-unknown-nonexistent")?;
    assert!(!is_installed, "Fake target should not be installed");

    Ok(())
}

#[test]
fn test_list_targets() -> Result<()> {
    let manager = ToolchainManager::new()?;

    let targets = manager.list_targets("stable")?;
    assert!(!targets.is_empty(), "Should have targets available");

    Ok(())
}

#[test]
fn test_toolchain_detection() -> Result<()> {
    let manager = ToolchainManager::new()?;

    // Verify stable toolchain exists
    let is_installed = manager.is_toolchain_installed("stable")?;
    assert!(is_installed, "Stable toolchain should be available");

    Ok(())
}

#[test]
fn test_prepare_target_host() -> Result<()> {
    let manager = ToolchainManager::new()?;
    let host = Target::detect_host()?;

    // Preparing host target should succeed
    let result = manager.prepare_target("stable", &host);
    assert!(result.is_ok(), "Preparing host target should succeed");

    Ok(())
}

#[test]
fn test_ensure_target_already_installed() -> Result<()> {
    let manager = ToolchainManager::new()?;
    let host = Target::detect_host()?;

    // Ensuring an already-installed target should succeed quickly
    let result = manager.ensure_target("stable", &host.triple);
    assert!(result.is_ok(), "Ensuring installed target should succeed");

    Ok(())
}

#[test]
fn test_toolchain_list() -> Result<()> {
    let manager = ToolchainManager::new()?;

    // Should be able to list toolchains
    let toolchains = manager.list_toolchains()?;
    assert!(!toolchains.is_empty(), "Should have at least one toolchain");

    Ok(())
}

#[test]
fn test_multiple_toolchain_checks() -> Result<()> {
    let manager = ToolchainManager::new()?;

    // Check multiple toolchains
    let toolchains = vec!["stable", "beta", "nightly"];

    for tc in &toolchains {
        let result = manager.is_toolchain_installed(tc);
        // May or may not be installed, just verify it doesn't panic
        let _ = result;
    }

    Ok(())
}

#[test]
fn test_target_add_simulation() -> Result<()> {
    let manager = ToolchainManager::new()?;

    // We won't actually add a target, but verify the manager can handle the request
    // (it may succeed if target already installed, or try to install)
    let result = manager.ensure_target("stable", "wasm32-unknown-unknown");

    // Don't assert success - depends on what's installed
    // Just verify it returns a Result
    let _ = result;

    Ok(())
}

#[test]
fn test_prepare_with_different_toolchains() -> Result<()> {
    let manager = ToolchainManager::new()?;
    let host = Target::detect_host()?;

    // Try different toolchains
    let toolchains = vec!["stable"];

    for tc in &toolchains {
        let result = manager.prepare_target(tc, &host);
        // stable should succeed
        if tc == &"stable" {
            assert!(result.is_ok(), "Stable toolchain should work");
        }
    }

    Ok(())
}

#[test]
fn test_rustup_detection() -> Result<()> {
    let manager = ToolchainManager::new()?;

    // If manager was created, rustup should be available
    let rustup_available = which::which("rustup").is_ok();
    assert!(rustup_available, "rustup should be available if ToolchainManager was created");

    let _ = manager;
    Ok(())
}

#[test]
fn test_cargo_detection() -> Result<()> {
    // cargo should always be available if we're running tests
    let cargo_available = which::which("cargo").is_ok();
    assert!(cargo_available, "cargo should be available");

    Ok(())
}

#[test]
fn test_rustc_detection() -> Result<()> {
    // rustc should always be available if we're running tests
    let rustc_available = which::which("rustc").is_ok();
    assert!(rustc_available, "rustc should be available");

    Ok(())
}

#[test]
fn test_toolchain_manager_reuse() -> Result<()> {
    // Create multiple manager instances
    let manager1 = ToolchainManager::new()?;
    let manager2 = ToolchainManager::new()?;

    // Both should work independently
    let _ = manager1.is_toolchain_installed("stable")?;
    let _ = manager2.is_toolchain_installed("stable")?;

    Ok(())
}

#[test]
fn test_list_available_targets() -> Result<()> {
    let manager = ToolchainManager::new()?;

    let all_targets = manager.list_targets("stable")?;
    assert!(!all_targets.is_empty(), "Should have many available targets");

    // Should include common targets
    let all_str = all_targets.join(" ");
    assert!(
        all_str.contains("x86_64-unknown-linux-gnu") ||
        all_str.contains("aarch64-apple-darwin") ||
        all_str.contains("windows"),
        "Should include common targets"
    );

    Ok(())
}

#[test]
fn test_active_toolchain() -> Result<()> {
    let manager = ToolchainManager::new()?;

    // Should be able to get active toolchain
    let active = manager.show_active_toolchain()?;
    assert!(!active.is_empty(), "Active toolchain should not be empty");

    Ok(())
}

#[test]
fn test_prepare_multiple_targets_sequentially() -> Result<()> {
    let manager = ToolchainManager::new()?;
    let host = Target::detect_host()?;

    // Prepare same target multiple times (should be idempotent)
    manager.prepare_target("stable", &host)?;
    manager.prepare_target("stable", &host)?;
    manager.prepare_target("stable", &host)?;

    Ok(())
}
