# xcargo Development TODO

> **Project Goal:** Build a zero-configuration cross-compilation tool for Rust that automates toolchain management, intelligently uses containers only when needed, and makes cross-compilation boring (in a good way).

**Current Status:** v0.3.0 (Zig cross-compilation support)
**Next Milestone:** v0.4.0 (Stability & Polish)
**Roadmap:** See [PATH_TO_PRODUCTION.md](./PATH_TO_PRODUCTION.md) for detailed v1.0.0 roadmap

---

## ✅ Phase 1: Core Foundation (v0.1.0) - COMPLETED

### 1.1 Module Structure Setup
- [x] Create `src/target/mod.rs` - Target platform definitions and detection
- [x] Create `src/toolchain/mod.rs` - Toolchain detection and management
- [x] Create `src/build/mod.rs` - Build orchestration
- [x] Create `src/container/mod.rs` - Container runtime integration
- [x] Create `src/config/mod.rs` - Configuration file handling
- [x] Create `src/output/mod.rs` - User-facing output formatting
- [x] Update `src/lib.rs` prelude to re-export core types

### 1.2 Target Detection (`src/target/`)
- [x] Define `Target` struct with triple, OS, arch, environment
- [x] Implement `Target::from_triple()` parser
- [x] Implement `Target::detect_host()` for current platform
- [x] Implement `Target::detect_installed()` via rustup
- [x] Add target validation against rustup's supported targets
- [x] Implement target tier classification (Tier 1/2/3)
- [x] Add target aliases (e.g., "linux" → "x86_64-unknown-linux-gnu")
- [x] Unit tests for target parsing and detection

### 1.3 Toolchain Management (`src/toolchain/`)
- [x] Detect `rustup` availability and version
- [x] Check if target is installed via `rustup target list --installed`
- [x] Implement auto-installation of Rust targets
- [x] Detect system linkers (gcc, mingw-w64, clang)
- [x] Detect cross-compilation tools per target
- [x] Generate installation suggestions for missing tools

### 1.4 Configuration System (`src/config/`)
- [x] Define config struct matching `xcargo.toml` schema
- [x] Implement TOML parsing with serde
- [x] Support `[targets]`, `[build]`, `[container]`, `[profiles]` sections
- [x] Load config from `xcargo.toml`
- [x] Merge with defaults (native-first strategy)
- [x] Implement `xcargo init` to generate default config
- [x] Add `xcargo config` command to show current/default config

### 1.5 Build System - Native Only (`src/build/`)
- [x] Implement basic cargo wrapper for single target
- [x] Pass `--target` flag correctly to cargo
- [x] Handle `--release` and `--debug` modes
- [x] Capture and format cargo output
- [x] Implement build for multiple targets sequentially
- [x] Add basic error handling and reporting
- [x] Track build artifacts locations

### 1.6 CLI Implementation (`src/main.rs`)
- [x] Implement `xcargo init` - create default `xcargo.toml`
- [x] Implement `xcargo target list` - show available/installed targets
- [x] Implement `xcargo target add <target>` - install via rustup
- [x] Implement `xcargo build --target <target>` - single target build
- [x] Implement `xcargo build --all` - build all configured targets
- [x] Implement `xcargo check` - type checking without building
- [x] Implement `xcargo test` - run tests for targets
- [x] Add proper error messages and colored output

### 1.7 Output & UX (`src/output/`)
- [x] Implement progress indicators (spinners, progress bars)
- [x] Add colored output for success/warning/error
- [x] Format toolchain suggestions nicely
- [x] Add verbose mode logging
- [x] Add build summary (time, artifacts, locations)

### 1.8 Testing & Documentation
- [x] Create `examples/basic_build.rs` - working example
- [x] Create `CHANGELOG.md` - track changes
- [x] Update README with installation instructions
- [ ] Create `tests/integration.rs` - integration tests (needs work)
- [ ] Add unit tests for each module (>70% coverage) (needs work)

---

## ✅ Phase 2: Container Integration (v0.2.0) - COMPLETED

### 2.1 Container Runtime Detection
- [x] Detect Docker availability and version
- [x] Detect Podman availability and version
- [x] Implement runtime selection (auto/docker/podman)
- [x] Implement `use-when` condition parsing (e.g., `target.os != host.os`)

### 2.2 Docker/Podman Support
- [x] Use pre-built cross-compilation images (rust-cross/cross)
- [x] Handle volume mounting for project directory
- [x] Support custom container images via config
- [x] Add `--container` flag for forced container builds

### 2.3 Build Strategy Logic
- [x] Implement decision tree: native vs. container
- [x] Check if linker is available for native build
- [x] Respect `container.use-when` config
- [x] Feature-gated container support (`--features container`)

---

## ✅ Phase 3: Advanced Features (v0.3.0) - COMPLETED

### 3.1 Parallel Builds
- [x] Implement parallel builds for multiple targets
- [x] Use tokio for parallelization
- [x] Respect `build.parallel` config
- [x] Handle failures gracefully (continue or stop)

### 3.2 Zig Cross-Compilation (NEW)
- [x] Auto-detect Zig installation
- [x] Generate CC/AR/LINKER wrapper scripts
- [x] Support x86_64/aarch64/armv7 Linux targets
- [x] Cross-compile from macOS/Windows to Linux
- [x] Add `--zig` and `--no-zig` CLI flags

### 3.3 Per-Target Configuration
- [x] Add custom environment variables per target
- [x] Support custom linker per target
- [x] Support custom RUSTFLAGS per target

### 3.4 Additional CLI Commands
- [x] `xcargo check` - type checking without building
- [x] `xcargo test` - run tests for targets

### Remaining for Phase 3:
- [ ] Build profiles (`xcargo build --profile <name>`)
- [ ] Build caching system
- [ ] `xcargo clean` command
- [ ] Progress bars in parallel builds (infrastructure added)

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
