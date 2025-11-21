# xcargo - Path to Production (v1.0.0)

**Current Version:** v0.3.0
**Target:** v1.0.0 (Production Ready)
**Last Updated:** 2025-11-21

---

## Executive Summary

xcargo is a zero-configuration cross-compilation tool for Rust. This document outlines the roadmap from v0.3.0 to a production-ready v1.0.0 release.

### Current State (v0.3.0)

| Feature | Status | Notes |
|---------|--------|-------|
| Basic build wrapper | ✅ Done | `xcargo build --target <triple>` |
| Target management | ✅ Done | `xcargo target add/list/info` |
| Parallel builds | ✅ Done | `build.parallel = true` |
| Zig cross-compilation | ✅ Done | macOS/Windows → Linux |
| Linker configuration | ✅ Done | Per-target linker in config |
| Interactive setup | ✅ Done | `xcargo init --interactive` |
| `xcargo check` | ✅ Done | Type checking without building |
| `xcargo test` | ✅ Done | Run tests for targets |
| Container builds | ⚠️ Partial | Feature-gated, basic Docker/Podman |
| Progress bars | ❌ Missing | indicatif is in deps but unused |
| Build caching | ❌ Missing | No caching layer |
| Bundled toolchains | ❌ Missing | No on-demand download |

---

## Priority Tiers

### P0 - Critical for v1.0.0 (Must Have)

These features are required for a stable, production-ready release.

#### 1. Error Handling & Recovery
**Status:** Needs Work
**Effort:** 2-3 days

- [ ] Replace `anyhow` with structured error types in critical paths
- [ ] Graceful degradation when tools are missing
- [ ] Clear, actionable error messages for all failure modes, with Tips/Hintts
- [ ] Exit codes that CI systems can rely on

#### 2. Comprehensive Testing
**Status:** Minimal coverage
**Effort:** 3-5 days

- [ ] Unit tests for all modules (target 80%+ coverage)
- [ ] Integration tests for CLI commands
- [ ] Cross-platform CI testing (Linux, macOS, Windows)
- [ ] Test Zig cross-compilation in CI
- [ ] Test container builds in CI

#### 3. Documentation
**Status:** README exists, needs expansion
**Effort:** 2-3 days

- [ ] Complete README with all commands
- [ ] `xcargo doctor` command for system diagnostics
- [ ] Troubleshooting guide
- [ ] Examples for common scenarios
- [ ] API documentation (rustdoc)

#### 4. Stability & Polish
**Status:** Functional but rough edges
**Effort:** 2-3 days

- [ ] Handle edge cases (no Cargo.toml, workspace projects)
- [ ] Consistent output formatting
- [ ] Proper signal handling (Ctrl+C)
- [ ] No panics in any code path

---

### P1 - High Priority (Should Have)

Features that significantly improve user experience.

#### 5. Progress Bars & Better Output
**Status:** Not implemented (deps available)
**Effort:** 1 day

- [ ] Use `indicatif` for build progress
- [ ] Show compilation phases (compiling, linking)
- [ ] Multi-target progress in parallel builds
- [ ] Spinner during toolchain installation

#### 6. `xcargo doctor` Command
**Status:** Not implemented
**Effort:** 1 day

- [ ] Check rustup installation
- [ ] Check installed targets
- [ ] Check linkers for configured targets
- [ ] Check Zig availability
- [ ] Check Docker/Podman availability
- [ ] Suggest fixes for missing tools

#### 7. Build Caching
**Status:** Not implemented
**Effort:** 2-3 days

- [ ] Track build artifacts per target
- [ ] Skip unchanged targets (hash-based)
- [ ] `xcargo clean` command
- [ ] Cache configuration options

#### 8. Container Improvements
**Status:** Basic implementation
**Effort:** 2-3 days

- [ ] Better image selection logic
- [ ] Support custom Dockerfiles
- [ ] Volume caching for cargo registry
- [ ] Support for podman machine on macOS

---

### P2 - Medium Priority (Nice to Have)

Features that enhance the tool but aren't blockers.

#### 9. Bundled Toolchains
**Status:** Not implemented
**Effort:** 3-5 days

- [ ] On-demand toolchain download
- [ ] Support Bootlin toolchains (Linux hosts)
- [ ] Support musl-cross-make toolchains
- [ ] Version management for toolchains
- [ ] Offline mode with pre-downloaded toolchains

#### 10. Build Profiles
**Status:** Config exists, not fully used
**Effort:** 1-2 days

- [ ] `xcargo build --profile release-all`
- [ ] Built-in profiles: minimal, mobile, server
- [ ] Profile inheritance
- [ ] Profile-specific flags

#### 11. CI/CD Integrations
**Status:** Not implemented
**Effort:** 2-3 days

- [ ] GitHub Action (`uses: xcargo/action@v1`)
- [ ] GitLab CI template
- [ ] Example workflows for common scenarios
- [ ] Matrix build support

#### 12. Release Automation
**Status:** Basic GitHub Actions
**Effort:** 1-2 days

- [ ] `xcargo release` command
- [ ] Automatic changelog generation
- [ ] Asset upload to GitHub Releases
- [ ] homebrew formula auto-update

---

### P3 - Low Priority (Future)

Post-1.0 features.

- TUI interface (ratatui)
- Plugin system
- Custom builders
- Telemetry (opt-in)
- Workspace support improvements
- Cross-testing with emulators

---

## Architecture Improvements

### Current Module Structure

```
src/
├── lib.rs          # Re-exports
├── main.rs         # CLI entry point
├── build/mod.rs    # Build orchestration
├── config/         # Configuration handling
├── container/      # Docker/Podman integration
├── output/mod.rs   # Terminal output helpers
├── target/mod.rs   # Target platform handling
└── toolchain/      # Toolchain management
    ├── mod.rs
    └── zig.rs      # Zig integration
```

### Recommended Changes

1. **Split `build/mod.rs`** - Currently 700+ lines, split into:
   - `build/executor.rs` - Build execution
   - `build/options.rs` - BuildOptions struct
   - `build/parallel.rs` - Parallel build logic

2. **Add `error/` module** - Structured error types:
   - `error/mod.rs` - Error enum
   - `error/suggestions.rs` - Error-to-suggestion mapping

3. **Add `cache/` module** - Build caching:
   - `cache/mod.rs` - Cache API
   - `cache/hash.rs` - File hashing

---

## Release Milestones

### v0.4.0 - Stability Release
**Goal:** Polish existing features, improve reliability

- [ ] P0: Error handling improvements
- [ ] P0: Comprehensive testing
- [ ] P1: Progress bars
- [ ] P1: `xcargo doctor` command

### v0.5.0 - Caching & Containers
**Goal:** Performance and container improvements

- [ ] P1: Build caching
- [ ] P1: Container improvements
- [ ] P2: Build profiles

### v0.6.0 - CI/CD Ready
**Goal:** First-class CI/CD support

- [ ] P0: Documentation complete
- [ ] P2: GitHub Action
- [ ] P2: Example workflows

### v1.0.0 - Production Release
**Goal:** Stable, documented, battle-tested

- [ ] All P0 items complete
- [ ] All P1 items complete
- [ ] API stability guarantee
- [ ] Published to crates.io
- [ ] homebrew formula
- [ ] Minimum 50 GitHub stars

---

## Testing Strategy

### Unit Tests
- Target parsing and validation
- Config loading and merging
- Toolchain detection
- Zig wrapper generation

### Integration Tests
- Full CLI command execution
- Build artifact verification
- Error message verification
- Config file handling

### CI Matrix

| OS | Rust | Targets Tested |
|----|------|----------------|
| ubuntu-latest | stable, beta | native, windows-gnu |
| macos-latest | stable | native, linux via Zig |
| windows-latest | stable | native, linux-gnu |

---

## Dependencies Audit

### Current Dependencies (18 total)

| Crate | Version | Purpose | Notes |
|-------|---------|---------|-------|
| clap | 4.5 | CLI parsing | Essential |
| anyhow | 1.0 | Error handling | Consider replacing for P0 |
| thiserror | 1.0 | Error derive | Keep |
| serde | 1.0 | Serialization | Essential |
| toml | 0.8 | Config parsing | Essential |
| tokio | 1.41 | Async runtime | Essential for parallel |
| colored | 2.1 | Terminal colors | Keep |
| indicatif | 0.17 | Progress bars | **Not used yet** |
| inquire | 0.7 | Interactive prompts | Keep |
| which | 6.0 | Executable detection | Keep |
| dirs | 5.0 | Home directory | Keep |
| walkdir | 2.5 | Directory traversal | Keep |

### Unused/Commented Dependencies
- reqwest (commented) - For toolchain download feature
- flate2/tar (commented) - For archive extraction

---

## Success Criteria for v1.0.0

### Functionality
- [ ] Can build for all Tier 1 Rust targets
- [ ] Can build for common Tier 2 targets (ARM Linux, Windows GNU)
- [ ] Works on Linux, macOS, and Windows hosts
- [ ] Container builds work with Docker and Podman
- [ ] Zig cross-compilation works for Linux targets

### Quality
- [ ] No known critical bugs
- [ ] 80%+ test coverage
- [ ] All public APIs documented
- [ ] No `unwrap()` in library code
- [ ] Clippy clean (`-D warnings`)

### Documentation
- [ ] README covers all features
- [ ] Troubleshooting guide exists
- [ ] Examples for 5+ common scenarios
- [ ] API docs on docs.rs

### Community
- [ ] CONTRIBUTING.md
- [ ] Issue templates
- [ ] PR template
- [ ] Code of conduct

---

## Quick Wins (Can Do Today)

1. **Use indicatif** - Already in deps, just needs integration
2. **Add `xcargo doctor`** - Simple diagnostic command
3. **Improve error messages** - Low-hanging fruit
4. **Add more tests** - Start with target parsing
5. **Update TODO.md** - Currently outdated (says v0.0.1)

---

## Risk Assessment

| Risk | Impact | Mitigation |
|------|--------|------------|
| Zig compatibility breaks | High | Pin Zig version, test in CI |
| Container API changes | Medium | Use stable Docker CLI |
| Cross-platform differences | Medium | Comprehensive CI matrix |
| Dependency vulnerabilities | Low | Regular `cargo audit` |

---

*This document should be reviewed and updated after each release.*
