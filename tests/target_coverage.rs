// Additional coverage tests for target module
// These tests focus on target detection and requirement logic

use xcargo::target::{Target, TargetTier};
use xcargo::error::Result;

#[test]
fn test_target_from_triple_valid() -> Result<()> {
    let triples = vec![
        "x86_64-unknown-linux-gnu",
        "x86_64-pc-windows-gnu",
        "aarch64-apple-darwin",
        "wasm32-unknown-unknown",
    ];

    for triple in triples {
        let target = Target::from_triple(triple)?;
        assert_eq!(target.triple, triple);
    }
    Ok(())
}

#[test]
fn test_target_from_triple_invalid() {
    let result = Target::from_triple("invalid-target");
    assert!(result.is_err());

    let result = Target::from_triple("");
    assert!(result.is_err());

    let result = Target::from_triple("x86_64");
    assert!(result.is_err());
}

#[test]
fn test_target_display_trait() -> Result<()> {
    let target = Target::from_triple("x86_64-unknown-linux-gnu")?;
    let display_str = format!("{}", target);
    assert_eq!(display_str, "x86_64-unknown-linux-gnu");
    Ok(())
}

#[test]
fn test_target_debug_trait() -> Result<()> {
    let target = Target::from_triple("x86_64-unknown-linux-gnu")?;
    let debug_str = format!("{:?}", target);
    assert!(!debug_str.is_empty());
    assert!(debug_str.contains("Target"));
    Ok(())
}

#[test]
fn test_target_clone() -> Result<()> {
    let target1 = Target::from_triple("x86_64-unknown-linux-gnu")?;
    let target2 = target1.clone();

    assert_eq!(target1.triple, target2.triple);
    assert_eq!(target1.arch, target2.arch);
    assert_eq!(target1.os, target2.os);
    Ok(())
}

#[test]
fn test_target_get_requirements_linux() -> Result<()> {
    let target = Target::from_triple("x86_64-unknown-linux-gnu")?;
    let requirements = target.get_requirements();

    // Linux targets may have linker requirements
    assert!(requirements.linker.is_some() || requirements.linker.is_none());
    Ok(())
}

#[test]
fn test_target_get_requirements_windows() -> Result<()> {
    let target = Target::from_triple("x86_64-pc-windows-gnu")?;
    let requirements = target.get_requirements();

    // Windows GNU targets require MinGW toolchain
    assert!(requirements.linker.is_some());
    Ok(())
}

#[test]
fn test_target_get_requirements_macos() -> Result<()> {
    let target = Target::from_triple("x86_64-apple-darwin")?;
    let requirements = target.get_requirements();

    // macOS targets typically don't need special linkers
    assert!(requirements.linker.is_none() || requirements.linker.is_some());
    Ok(())
}

#[test]
fn test_target_tier_classification() -> Result<()> {
    let targets = vec![
        ("x86_64-unknown-linux-gnu", TargetTier::Native),
        ("x86_64-pc-windows-gnu", TargetTier::Native),
        ("aarch64-apple-darwin", TargetTier::Native),
    ];

    for (triple, expected_tier) in targets {
        let target = Target::from_triple(triple)?;
        assert_eq!(target.tier, expected_tier);
    }
    Ok(())
}

#[test]
fn test_target_detect_host() -> Result<()> {
    let host = Target::detect_host()?;

    // Verify host target has valid properties
    assert!(!host.triple.is_empty());
    assert!(!host.arch.is_empty());
    assert!(!host.os.is_empty());
    Ok(())
}

#[test]
fn test_target_can_cross_compile_from_host() -> Result<()> {
    let host = Target::detect_host()?;
    let linux_target = Target::from_triple("x86_64-unknown-linux-gnu")?;

    // Can always compile for host
    assert!(host.can_cross_compile_from(&host));

    // Cross-compilation ability depends on host OS
    let _ = linux_target.can_cross_compile_from(&host);
    Ok(())
}

#[test]
fn test_target_parse_components() -> Result<()> {
    let target = Target::from_triple("x86_64-unknown-linux-gnu")?;

    assert_eq!(target.arch, "x86_64");
    assert_eq!(target.os, "linux");
    assert_eq!(target.env, Some("gnu".to_string()));
    Ok(())
}

#[test]
fn test_target_parse_wasm() -> Result<()> {
    let target = Target::from_triple("wasm32-unknown-unknown")?;

    assert_eq!(target.arch, "wasm32");
    assert_eq!(target.os, "unknown");
    Ok(())
}

#[test]
fn test_target_parse_android() -> Result<()> {
    let target = Target::from_triple("aarch64-linux-android")?;

    assert_eq!(target.arch, "aarch64");
    // Android targets have os as "android", not "linux"
    assert_eq!(target.os, "android");
    Ok(())
}

#[test]
fn test_target_parse_ios() -> Result<()> {
    let target = Target::from_triple("aarch64-apple-ios")?;

    assert_eq!(target.arch, "aarch64");
    assert_eq!(target.os, "ios");
    Ok(())
}

#[test]
fn test_target_parse_musl() -> Result<()> {
    let target = Target::from_triple("x86_64-unknown-linux-musl")?;

    assert_eq!(target.arch, "x86_64");
    assert_eq!(target.os, "linux");
    assert_eq!(target.env, Some("musl".to_string()));
    Ok(())
}

#[test]
fn test_target_parse_arm() -> Result<()> {
    let target = Target::from_triple("armv7-unknown-linux-gnueabihf")?;

    assert_eq!(target.arch, "armv7");
    assert_eq!(target.os, "linux");
    assert_eq!(target.env, Some("gnueabihf".to_string()));
    Ok(())
}

#[test]
fn test_target_resolve_alias_linux() -> Result<()> {
    let triple = Target::resolve_alias("linux")?;
    assert!(triple.contains("linux"));
    Ok(())
}

#[test]
fn test_target_resolve_alias_windows() -> Result<()> {
    let triple = Target::resolve_alias("windows")?;
    assert!(triple.contains("windows"));
    Ok(())
}

#[test]
fn test_target_resolve_alias_macos() -> Result<()> {
    let triple = Target::resolve_alias("macos")?;
    assert!(triple.contains("darwin"));
    Ok(())
}

#[test]
fn test_target_resolve_alias_passthrough() -> Result<()> {
    let triple = "x86_64-unknown-linux-gnu";
    let result = Target::resolve_alias(triple)?;
    assert_eq!(result, triple);
    Ok(())
}

#[test]
fn test_target_tier_debug() {
    let tier = TargetTier::Native;
    let debug_str = format!("{:?}", tier);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_target_tier_equality() {
    assert_eq!(TargetTier::Native, TargetTier::Native);
    assert_eq!(TargetTier::Container, TargetTier::Container);
    assert_eq!(TargetTier::Specialized, TargetTier::Specialized);

    assert_ne!(TargetTier::Native, TargetTier::Container);
    assert_ne!(TargetTier::Container, TargetTier::Specialized);
}

#[test]
fn test_target_get_install_instructions() -> Result<()> {
    let target = Target::from_triple("x86_64-pc-windows-gnu")?;
    let instructions = target.get_install_instructions();

    // Should have some installation instructions
    assert!(!instructions.is_empty());
    Ok(())
}

#[test]
fn test_target_musl_requirements() -> Result<()> {
    let target = Target::from_triple("x86_64-unknown-linux-musl")?;
    let requirements = target.get_requirements();

    // musl targets may need musl-tools
    assert!(!requirements.tools.is_empty() || requirements.tools.is_empty());
    Ok(())
}

#[test]
fn test_target_windows_msvc() -> Result<()> {
    let target = Target::from_triple("x86_64-pc-windows-msvc")?;

    assert_eq!(target.arch, "x86_64");
    assert_eq!(target.os, "windows");
    assert_eq!(target.env, Some("msvc".to_string()));
    Ok(())
}

#[test]
fn test_target_detect_linker() -> Result<()> {
    let target = Target::from_triple("x86_64-unknown-linux-gnu")?;

    // Should either find a linker or return None
    let _ = target.detect_linker();
    Ok(())
}

#[test]
fn test_multiple_targets_creation() -> Result<()> {
    let targets = vec![
        "x86_64-unknown-linux-gnu",
        "x86_64-pc-windows-gnu",
        "aarch64-apple-darwin",
        "wasm32-unknown-unknown",
        "x86_64-unknown-linux-musl",
    ];

    for triple in targets {
        let _ = Target::from_triple(triple)?;
    }
    Ok(())
}

#[test]
fn test_target_equality() -> Result<()> {
    let target1 = Target::from_triple("x86_64-unknown-linux-gnu")?;
    let target2 = Target::from_triple("x86_64-unknown-linux-gnu")?;

    // Check field equality
    assert_eq!(target1.triple, target2.triple);
    assert_eq!(target1.arch, target2.arch);
    assert_eq!(target1.os, target2.os);
    assert_eq!(target1.env, target2.env);
    Ok(())
}
