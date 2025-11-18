# xcargo Development TODO

> **Project Goal:** Build a zero-configuration cross-compilation tool for Rust that automates toolchain management, intelligently uses containers only when needed, and makes cross-compilation boring (in a good way).

**Current Status:** v0.0.1 (preparing for crates.io on crates.io)
**Next Milestone:** v0.1.0-alpha (Basic functionality)

---

## 🎯 Phase 1: Core Foundation (v0.1.0-alpha)

### 1.1 Module Structure Setup
- [ ] Create `src/target/mod.rs` - Target platform definitions and detection
- [ ] Create `src/toolchain/mod.rs` - Toolchain detection and management
- [ ] Create `src/build/mod.rs` - Build orchestration
- [ ] Create `src/container/mod.rs` - Container runtime integration
- [ ] Create `src/config/mod.rs` - Configuration file handling
- [ ] Create `src/deps/mod.rs` - Native dependency handling
- [ ] Create `src/output/mod.rs` - User-facing output formatting
- [ ] Update `src/lib.rs` prelude to re-export core types once modules are implemented

### 1.2 Target Detection (`src/target/`)
- [ ] Define `Target` struct with triple, OS, arch, environment
- [ ] Implement `Target::from_triple()` parser
- [ ] Implement `Target::detect_host()` for current platform
- [ ] Implement `Target::detect_installed()` via rustup
- [ ] Add target validation against rustup's supported targets
- [ ] Implement target tier classification (Tier 1/2/3)
- [ ] Add target aliases (e.g., "linux" → "x86_64-unknown-linux-gnu")
- [ ] Unit tests for target parsing and detection

### 1.3 Toolchain Management (`src/toolchain/`)
- [ ] Detect `rustup` availability and version
- [ ] Check if target is installed via `rustup target list --installed`
- [ ] Implement auto-installation of Rust targets
- [ ] Detect system linkers (gcc, mingw-w64, clang)
- [ ] Detect cross-compilation tools per target
- [ ] Generate installation suggestions for missing tools
- [ ] Add dry-run mode for toolchain operations
- [ ] Integration tests for toolchain detection

### 1.4 Configuration System (`src/config/`)
- [ ] Define `ApexConfig` struct matching `apex.toml` schema
- [ ] Implement TOML parsing with serde
- [ ] Support `[targets]`, `[build]`, `[container]`, `[profiles]` sections
- [ ] Load config from `apex.toml` or `.apex/config.toml`
- [ ] Merge with defaults (native-first strategy)
- [ ] Implement `apex init` to generate default config
- [ ] Add config validation
- [ ] Add `apex config` command to show current/default config

### 1.5 Build System - Native Only (`src/build/`)
- [ ] Implement basic cargo wrapper for single target
- [ ] Pass `--target` flag correctly to cargo
- [ ] Handle `--release` and `--debug` modes
- [ ] Capture and format cargo output
- [ ] Implement build for multiple targets sequentially
- [ ] Add basic error handling and reporting
- [ ] Track build artifacts locations
- [ ] Integration test: build simple binary for host target

### 1.6 CLI Implementation (`src/main.rs`)
- [ ] Implement `apex init` - create default `apex.toml`
- [ ] Implement `apex target list` - show available/installed targets
- [ ] Implement `apex target add <target>` - install via rustup
- [ ] Implement `apex target remove <target>` - uninstall via rustup
- [ ] Implement `apex build --target <target>` - single target build
- [ ] Implement `apex build --all` - build all configured targets
- [ ] Implement `apex doctor` - check system dependencies
- [ ] Implement `apex doctor --target <target>` - check specific target
- [ ] Add proper error messages and colored output

### 1.7 Output & UX (`src/output/`)
- [ ] Implement progress indicators (spinners, progress bars)
- [ ] Add colored output for success/warning/error
- [ ] Format toolchain suggestions nicely
- [ ] Add verbose mode logging
- [ ] Implement `doctor` output with checkmarks/crosses
- [ ] Add build summary (time, artifacts, locations)

### 1.8 Testing & Documentation
- [ ] Create `tests/integration.rs` - integration tests
- [ ] Create `examples/basic_build.rs` - working example
- [ ] Add unit tests for each module (>70% coverage)
- [ ] Create `docs/guide.md` - user guide
- [ ] Create `CHANGELOG.md` - track changes
- [ ] Update README with installation instructions
- [ ] Add rustdoc comments to all public APIs
- [ ] Test on Linux, macOS, Windows (WSL2)

---

## 🐳 Phase 2: Container Integration (v0.2.0)

### 2.1 Container Runtime Detection
- [ ] Detect Docker availability and version
- [ ] Detect Podman availability and version
- [ ] Implement runtime selection (auto/docker/podman/youki)
- [ ] Add fallback logic (youki → docker → podman)
- [ ] Implement `use-when` condition parsing (e.g., `target.os != host.os`)

### 2.2 Youki Integration (Embedded Runtime)
- [ ] Research youki integration options
- [ ] Evaluate embedding youki vs. external dependency
- [ ] Implement basic container creation via youki
- [ ] Map build directories into container
- [ ] Execute cargo commands in container
- [ ] Capture container output
- [ ] Clean up containers after build

### 2.3 Docker/Podman Fallback
- [ ] Use pre-built cross-compilation images (rust-cross/cross)
- [ ] Implement custom Dockerfiles for missing targets
- [ ] Handle volume mounting for project directory
- [ ] Implement caching layer for container images
- [ ] Support custom container images via config

### 2.4 Build Strategy Logic
- [ ] Implement decision tree: native vs. container
- [ ] Check if linker is available for native build
- [ ] Fallback to container if native tools missing
- [ ] Respect `container.use-when` config
- [ ] Add `--force-native` and `--force-container` flags

### 2.5 Container Testing
- [ ] Test Docker builds on all Tier 2 targets
- [ ] Test Podman builds
- [ ] Test youki basic functionality
- [ ] Verify artifact ownership and permissions
- [ ] Integration tests for container builds

---

## ⚡ Phase 3: Advanced Features (v0.3.0)

### 3.1 Parallel Builds
- [ ] Implement parallel builds for multiple targets
- [ ] Use rayon or tokio for parallelization
- [ ] Respect `build.parallel` config
- [ ] Show progress for all builds simultaneously
- [ ] Handle failures gracefully (continue or stop)

### 3.2 Build Profiles
- [ ] Implement `profiles` section in config
- [ ] Add built-in profiles: `release-all`, `mobile`, `embedded`
- [ ] Support `apex build --profile <name>`
- [ ] Allow profile composition and inheritance

### 3.3 Caching System
- [ ] Implement build cache per target
- [ ] Cache toolchain installations
- [ ] Cache container images
- [ ] Implement cache invalidation strategy
- [ ] Add `apex cache clean` command
- [ ] Respect `build.cache` config

### 3.4 Dependency Management
- [ ] Detect native dependencies (OpenSSL, SQLite, libpq)
- [ ] Auto-install or suggest installation commands
- [ ] Handle static linking preferences
- [ ] Support vendoring native dependencies
- [ ] Add OpenSSL cross-compilation support
- [ ] Document dependency handling per target

### 3.5 Custom Target Definitions
- [ ] Allow custom target JSON definitions
- [ ] Support custom linker scripts
- [ ] Add custom environment variables per target
- [ ] Support pre-build and post-build hooks

---

## 🔗 Phase 4: CI/CD & Distribution (v0.4.0)

### 4.1 GitHub Actions Integration
- [ ] Create `apex-action` GitHub Action
- [ ] Publish to GitHub Marketplace
- [ ] Add matrix build support
- [ ] Create example workflow templates
- [ ] Support artifact upload to releases

### 4.2 GitLab CI Integration
- [ ] Create GitLab CI template
- [ ] Add to GitLab template library
- [ ] Support GitLab artifact registry
- [ ] Create example `.gitlab-ci.yml`

### 4.3 Release Automation
- [ ] Implement `apex release` command
- [ ] Support GitHub Releases API
- [ ] Auto-generate release notes from CHANGELOG
- [ ] Upload build artifacts
- [ ] Support multi-platform binary distribution

### 4.4 Distribution
- [ ] Publish to crates.io (full release)
- [ ] Create homebrew formula
- [ ] Create installation script (curl | sh)
- [ ] Build pre-compiled binaries for major platforms
- [ ] Set up automatic release pipeline

---

## 🎨 Phase 5: Polish & Extras (v1.0.0)

### 5.1 GUI/TUI Interface
- [ ] Research TUI frameworks (ratatui, cursive)
- [ ] Design interactive target selector
- [ ] Add live build progress visualization
- [ ] Implement `apex ui` command

### 5.2 Advanced Tooling
- [ ] Add `apex clean` - clean all target artifacts
- [ ] Add `apex check` - quick check all targets
- [ ] Add `apex benchmark` - benchmark build times
- [ ] Add `apex test --target <target>` - cross-testing
- [ ] Support custom cargo commands

### 5.3 Documentation
- [ ] Create comprehensive docs site (mdBook)
- [ ] Add architecture diagrams
- [ ] Create video tutorials
- [ ] Add troubleshooting guide
- [ ] Document all supported targets in detail
- [ ] Add migration guides from cross/cargo-zigbuild

### 5.4 Performance Optimization
- [ ] Profile build process
- [ ] Optimize parallel builds
- [ ] Reduce container startup time
- [ ] Implement incremental builds
- [ ] Add build metrics and reporting

### 5.5 Community & Ecosystem
- [ ] Create contribution guidelines
- [ ] Set up issue templates
- [ ] Create Discord/Zulip community
- [ ] Add plugin/extension system
- [ ] Support custom builders

---

## 📝 Immediate Next Steps (This Week)

1. **Create Module Files**
   - [ ] Create all empty module files with basic structure
   - [ ] Add module documentation headers
   - [ ] Update lib.rs prelude as modules are populated

2. **Implement Target Detection**
   - [ ] Start with `src/target/mod.rs`
   - [ ] Implement core `Target` struct and parsing
   - [ ] Add tests for target detection

3. **Basic Build Wrapper**
   - [ ] Implement `src/build/mod.rs` with simple cargo wrapper
   - [ ] Make `apex build --target <target>` work for native builds

4. **Doctor Command**
   - [ ] Implement toolchain detection
   - [ ] Make `apex doctor` show current system status

---

## 🐛 Known Issues / Tech Debt

- [ ] Add proper error types instead of anyhow everywhere
- [ ] Add logging framework (tracing/log)
- [ ] Improve error messages with suggestions
- [ ] Add telemetry (opt-in) for usage analytics
- [ ] Improve Windows support (beyond WSL2)

---

## 📊 Success Metrics

**v0.1.0 Success Criteria:**
- [ ] Can build for Windows from Linux (mingw)
- [ ] Can build for Linux ARM from x86_64
- [ ] `apex doctor` correctly identifies missing tools
- [ ] Works on Linux and macOS
- [ ] Has at least 5 examples in `examples/`
- [ ] Has integration tests for core workflows

**v1.0.0 Success Criteria:**
- [ ] Supports 20+ targets across all tiers
- [ ] Has 1000+ GitHub stars
- [ ] Used in production by multiple projects
- [ ] Has comprehensive documentation
- [ ] CI/CD integrations are published
- [ ] Community is active and growing

---

## 🎯 Vision

**Make cross-compilation in Rust as simple as:**
```bash
apex build --all
```

**And it just works.** No Docker Desktop. No manual toolchain setup. No copying linker paths. No reading hour-long guides.

Just. Works. ✨

---

*Last Updated: 2025-11-18*
*Next Review: When Phase 1 is 50% complete*
