// Target triple parsing and detection tests
// Tests comprehensive target parsing logic including:
// - Target triple parsing (arch-vendor-os-env)
// - Platform detection edge cases
// - Target requirement determination
// - Target tier classification
// - Exotic and embedded target support

use xcargo::target::{Target, TargetRequirements, TargetTier};
use xcargo::Result;

#[test]
fn test_parse_standard_linux_triple() -> Result<()> {
    let target = Target::from_triple("x86_64-unknown-linux-gnu")?;
    assert_eq!(target.arch, "x86_64");
    assert_eq!(target.vendor, "unknown");
    assert_eq!(target.os, "linux");
    assert_eq!(target.env, Some("gnu".to_string()));
    Ok(())
}

#[test]
fn test_parse_musl_linux_triple() -> Result<()> {
    let target = Target::from_triple("x86_64-unknown-linux-musl")?;
    assert_eq!(target.arch, "x86_64");
    assert_eq!(target.vendor, "unknown");
    assert_eq!(target.os, "linux");
    assert_eq!(target.env, Some("musl".to_string()));
    Ok(())
}

#[test]
fn test_parse_macos_triple() -> Result<()> {
    let target = Target::from_triple("aarch64-apple-darwin")?;
    assert_eq!(target.arch, "aarch64");
    assert_eq!(target.vendor, "apple");
    assert_eq!(target.os, "darwin");
    assert!(target.env.is_none(), "macOS targets don't have env component");
    Ok(())
}

#[test]
fn test_parse_windows_msvc_triple() -> Result<()> {
    let target = Target::from_triple("x86_64-pc-windows-msvc")?;
    assert_eq!(target.arch, "x86_64");
    assert_eq!(target.vendor, "pc");
    assert_eq!(target.os, "windows");
    assert_eq!(target.env, Some("msvc".to_string()));
    Ok(())
}

#[test]
fn test_parse_windows_gnu_triple() -> Result<()> {
    let target = Target::from_triple("x86_64-pc-windows-gnu")?;
    assert_eq!(target.arch, "x86_64");
    assert_eq!(target.vendor, "pc");
    assert_eq!(target.os, "windows");
    assert_eq!(target.env, Some("gnu".to_string()));
    Ok(())
}

#[test]
fn test_parse_wasm_triple() -> Result<()> {
    let target = Target::from_triple("wasm32-unknown-unknown")?;
    assert_eq!(target.arch, "wasm32");
    assert_eq!(target.vendor, "unknown");
    assert_eq!(target.os, "unknown");
    assert!(target.env.is_none());
    Ok(())
}

#[test]
fn test_parse_android_triple() -> Result<()> {
    let target = Target::from_triple("aarch64-linux-android")?;
    assert_eq!(target.arch, "aarch64");
    assert_eq!(target.vendor, "linux");
    assert_eq!(target.os, "android");
    assert!(target.env.is_none());
    Ok(())
}

#[test]
fn test_parse_ios_triple() -> Result<()> {
    let target = Target::from_triple("aarch64-apple-ios")?;
    assert_eq!(target.arch, "aarch64");
    assert_eq!(target.vendor, "apple");
    assert_eq!(target.os, "ios");
    assert!(target.env.is_none());
    Ok(())
}

#[test]
fn test_parse_armv7_triple() -> Result<()> {
    let target = Target::from_triple("armv7-unknown-linux-gnueabihf")?;
    assert_eq!(target.arch, "armv7");
    assert_eq!(target.vendor, "unknown");
    assert_eq!(target.os, "linux");
    assert_eq!(target.env, Some("gnueabihf".to_string()));
    Ok(())
}

#[test]
fn test_parse_freebsd_triple() -> Result<()> {
    let target = Target::from_triple("x86_64-unknown-freebsd")?;
    assert_eq!(target.arch, "x86_64");
    assert_eq!(target.vendor, "unknown");
    assert_eq!(target.os, "freebsd");
    assert!(target.env.is_none());
    Ok(())
}

#[test]
fn test_parse_netbsd_triple() -> Result<()> {
    let target = Target::from_triple("x86_64-unknown-netbsd")?;
    assert_eq!(target.arch, "x86_64");
    assert_eq!(target.os, "netbsd");
    Ok(())
}

#[test]
fn test_parse_riscv_triple() -> Result<()> {
    let target = Target::from_triple("riscv64gc-unknown-linux-gnu")?;
    assert_eq!(target.arch, "riscv64gc");
    assert_eq!(target.os, "linux");
    Ok(())
}

#[test]
fn test_parse_embedded_arm_triple() -> Result<()> {
    let target = Target::from_triple("thumbv7em-none-eabi")?;
    assert_eq!(target.arch, "thumbv7em");
    assert_eq!(target.vendor, "none");
    assert_eq!(target.os, "eabi");
    assert!(target.env.is_none());
    Ok(())
}

#[test]
fn test_parse_powerpc_triple() -> Result<()> {
    let target = Target::from_triple("powerpc64le-unknown-linux-gnu")?;
    assert_eq!(target.arch, "powerpc64le");
    assert_eq!(target.os, "linux");
    Ok(())
}

#[test]
fn test_parse_s390x_triple() -> Result<()> {
    let target = Target::from_triple("s390x-unknown-linux-gnu")?;
    assert_eq!(target.arch, "s390x");
    assert_eq!(target.os, "linux");
    Ok(())
}

#[test]
fn test_invalid_triple_too_few_parts() {
    let result = Target::from_triple("x86_64-linux");
    assert!(result.is_err(), "Should error on triple with < 3 parts");
}

#[test]
fn test_invalid_triple_empty() {
    let result = Target::from_triple("");
    assert!(result.is_err(), "Should error on empty triple");
}

#[test]
fn test_invalid_triple_single_part() {
    let result = Target::from_triple("x86_64");
    assert!(result.is_err(), "Should error on single-part triple");
}

#[test]
fn test_target_requirements_none() {
    let reqs = TargetRequirements::none();
    assert!(reqs.linker.is_none());
    assert!(reqs.tools.is_empty());
    assert!(reqs.system_libs.is_empty());
    assert!(reqs.env_vars.is_empty());
}

#[test]
fn test_target_requirements_satisfied_empty() {
    let reqs = TargetRequirements::none();
    assert!(reqs.are_satisfied(), "Empty requirements should be satisfied");
}

#[test]
fn test_target_requirements_with_linker() {
    let mut reqs = TargetRequirements::none();
    reqs.linker = Some("gcc".to_string());

    // Result depends on whether gcc is installed
    // Just verify it doesn't panic
    let _ = reqs.are_satisfied();
}

#[test]
fn test_target_requirements_with_tools() {
    let mut reqs = TargetRequirements::none();
    reqs.tools = vec!["cargo".to_string()]; // cargo should be available

    assert!(reqs.are_satisfied(), "cargo should be available");
}

#[test]
fn test_target_requirements_with_missing_tool() {
    let mut reqs = TargetRequirements::none();
    reqs.tools = vec!["nonexistent-tool-xyz123".to_string()];

    assert!(!reqs.are_satisfied(), "Nonexistent tool should not be satisfied");
}

#[test]
fn test_detect_host_target() -> Result<()> {
    let host = Target::detect_host()?;

    // Verify host target is valid
    assert!(!host.triple.is_empty());
    assert!(!host.arch.is_empty());
    assert!(!host.os.is_empty());

    // Host should be a known architecture
    assert!(
        host.arch == "x86_64" ||
        host.arch == "aarch64" ||
        host.arch == "i686" ||
        host.arch.starts_with("arm"),
        "Host arch should be recognized: {}", host.arch
    );

    Ok(())
}

#[test]
fn test_detect_installed_targets() -> Result<()> {
    let installed = Target::detect_installed()?;

    // Should have at least the host target installed
    assert!(!installed.is_empty(), "Should have at least one target installed");

    // All installed targets should be valid
    for target in &installed {
        assert!(!target.triple.is_empty());
        assert!(!target.arch.is_empty());
        assert!(!target.os.is_empty());
    }

    Ok(())
}

#[test]
fn test_target_tier_classification() -> Result<()> {
    // Native tier targets
    let native = Target::from_triple("x86_64-unknown-linux-gnu")?;
    assert_eq!(native.tier, TargetTier::Native);

    // Container tier targets
    let container = Target::from_triple("aarch64-unknown-linux-gnu")?;
    // Tier depends on host, just verify it's set
    let _ = container.tier;

    Ok(())
}

#[test]
fn test_target_equality() -> Result<()> {
    let target1 = Target::from_triple("x86_64-unknown-linux-gnu")?;
    let target2 = Target::from_triple("x86_64-unknown-linux-gnu")?;

    assert_eq!(target1, target2);
    assert_eq!(target1.triple, target2.triple);

    Ok(())
}

#[test]
fn test_target_inequality() -> Result<()> {
    let target1 = Target::from_triple("x86_64-unknown-linux-gnu")?;
    let target2 = Target::from_triple("aarch64-unknown-linux-gnu")?;

    assert_ne!(target1, target2);

    Ok(())
}

#[test]
fn test_target_clone() -> Result<()> {
    let target = Target::from_triple("x86_64-unknown-linux-gnu")?;
    let cloned = target.clone();

    assert_eq!(target, cloned);
    assert_eq!(target.triple, cloned.triple);

    Ok(())
}

#[test]
fn test_target_debug_format() -> Result<()> {
    let target = Target::from_triple("x86_64-unknown-linux-gnu")?;
    let debug_str = format!("{:?}", target);

    assert!(debug_str.contains("x86_64"));
    assert!(debug_str.contains("linux"));

    Ok(())
}

#[test]
fn test_parse_multipart_env() -> Result<()> {
    // Some targets have multi-part env like "gnueabihf"
    let target = Target::from_triple("arm-unknown-linux-gnueabihf")?;
    assert_eq!(target.arch, "arm");
    assert_eq!(target.os, "linux");
    assert_eq!(target.env, Some("gnueabihf".to_string()));

    Ok(())
}

#[test]
fn test_get_requirements_for_host() -> Result<()> {
    let host = Target::detect_host()?;
    let reqs = host.get_requirements();

    // Host target should have minimal requirements
    // (may or may not need linker depending on platform)
    let _ = reqs;

    Ok(())
}

#[test]
fn test_get_requirements_for_cross_target() -> Result<()> {
    let target = Target::from_triple("aarch64-unknown-linux-gnu")?;
    let reqs = target.get_requirements();

    // Cross-compilation target should have some requirements
    // Just verify it doesn't panic
    let _ = reqs;

    Ok(())
}

#[test]
fn test_windows_gnu_requirements() -> Result<()> {
    let target = Target::from_triple("x86_64-pc-windows-gnu")?;
    let reqs = target.get_requirements();

    // Windows GNU should suggest mingw linker
    assert!(reqs.linker.is_some(), "Windows GNU should have linker requirement");

    Ok(())
}

#[test]
fn test_musl_target_requirements() -> Result<()> {
    let target = Target::from_triple("x86_64-unknown-linux-musl")?;
    let reqs = target.get_requirements();

    // musl targets may have specific requirements
    let _ = reqs;

    Ok(())
}

#[test]
fn test_parse_i686_triple() -> Result<()> {
    let target = Target::from_triple("i686-unknown-linux-gnu")?;
    assert_eq!(target.arch, "i686");
    assert_eq!(target.os, "linux");

    Ok(())
}

#[test]
fn test_parse_mips_triple() -> Result<()> {
    let target = Target::from_triple("mips-unknown-linux-gnu")?;
    assert_eq!(target.arch, "mips");
    assert_eq!(target.os, "linux");

    Ok(())
}

#[test]
fn test_hash_target() -> Result<()> {
    use std::collections::HashSet;

    let mut set = HashSet::new();
    let target1 = Target::from_triple("x86_64-unknown-linux-gnu")?;
    let target2 = Target::from_triple("x86_64-unknown-linux-gnu")?;

    set.insert(target1.clone());
    set.insert(target2);

    // Should only have one unique target
    assert_eq!(set.len(), 1);

    Ok(())
}
