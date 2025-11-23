# Contributing to xcargo 🎯

Thank you for your interest in contributing to xcargo! We welcome contributions of all kinds - from bug reports to code contributions to documentation improvements.

## 🚀 Project Status

xcargo is approaching **v1.0.0 - Production Ready**. We're focused on stability, test coverage, and professional distribution.

**Current priorities:**
- Test coverage (target: 80%, current: 69%)
- Bug fixes and stability improvements
- Documentation improvements
- Platform testing and verification

## 🤝 How to Contribute

### Reporting Issues

When reporting issues, please include:

**For bugs:**
- xcargo version (`xcargo --version`)
- Host platform (`xcargo doctor`)
- Target triple you're building for
- Steps to reproduce
- Expected vs actual behavior
- Error messages and logs

**For feature requests:**
- Use case description
- Proposed solution or API
- Alternative approaches considered
- Impact on existing functionality

**For target support requests:**
- Target triple (e.g., `riscv64gc-unknown-linux-gnu`)
- Toolchain availability
- Host OS you're building from
- Use case (embedded, server, mobile, etc.)

### Code Contributions

1. **Fork and clone:**
   ```bash
   git clone https://github.com/yourusername/xcargo
   cd xcargo
   ```

2. **Create a feature branch:**
   ```bash
   git checkout -b feature/amazing-feature
   # Or for bugs: git checkout -b fix/issue-123
   ```

3. **Make your changes:**
   - Follow Rust conventions
   - Add tests for new functionality
   - Update documentation as needed

4. **Run quality checks:**
   ```bash
   # Format code
   cargo fmt

   # Run tests
   cargo test

   # Run linter
   cargo clippy -- -D warnings

   # Check test coverage (if cargo-tarpaulin installed)
   cargo tarpaulin --out Html
   ```

5. **Commit your changes:**
   ```bash
   git add .
   git commit -m "feat: add amazing feature

   Detailed description of what this commit does.

   Closes #123"
   ```

   **Commit message format:**
   - `feat:` - New feature
   - `fix:` - Bug fix
   - `docs:` - Documentation changes
   - `test:` - Test additions/changes
   - `refactor:` - Code refactoring
   - `perf:` - Performance improvements
   - `chore:` - Maintenance tasks

6. **Push and create PR:**
   ```bash
   git push origin feature/amazing-feature
   ```
   Then open a Pull Request on GitHub.

### Documentation Contributions

Documentation improvements are highly valued! Areas that need help:

- User guides and tutorials
- API documentation
- Troubleshooting guides
- Platform-specific setup instructions
- Example projects

Documentation lives in:
- `docs/` - Docusaurus-based documentation site
- `README.md` - Project overview
- Code comments - Public API documentation

## 📋 Development Setup

### Prerequisites

- **Rust 1.70+**: Install from [rustup.rs](https://rustup.rs)
- **Git**: For version control
- **cargo-tarpaulin** (optional): For test coverage
  ```bash
  cargo install cargo-tarpaulin
  ```
- **cargo-dist** (optional): For testing releases
  ```bash
  cargo install cargo-dist
  ```

### Building from Source

```bash
# Clone the repository
git clone https://github.com/ibrahimcesar/xcargo
cd xcargo

# Build debug version
cargo build

# Build release version
cargo build --release

# Run from source
cargo run -- --help

# Install locally for testing
cargo install --path .
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture

# Run tests for specific module
cargo test target::

# Check test coverage
cargo tarpaulin --out Html --output-dir coverage/
open coverage/index.html
```

### Testing Cross-Compilation

```bash
# Test building for a different target
cargo run -- build --target x86_64-unknown-linux-gnu

# Test with Zig
cargo run -- build --target x86_64-unknown-linux-musl --zig

# Test parallel builds
cargo run -- build --target x86_64-unknown-linux-gnu,aarch64-unknown-linux-gnu

# Run diagnostics
cargo run -- doctor
```

## 🎯 Areas Needing Help

### High Priority (v1.0.0 blockers)

- **Test Coverage** - Need 20-25 more tests to reach 80%
  - `src/main.rs` (currently 47%)
  - `src/target/mod.rs` (currently 64%)
  - `src/doctor/report.rs` (currently 77%)

### Medium Priority (v1.1+)

- **Windows Native Support** - Better cross-compilation from Windows
- **Container Optimizations** - Faster builds with volume caching
- **Build Caching** - Hash-based incremental cross-compilation
- **CI/CD Templates** - GitHub Actions, GitLab CI examples

### Documentation

- **Video Tutorials** - Screen recordings of common workflows
- **Real-World Examples** - Production use cases
- **Troubleshooting Guide** - Common errors and solutions
- **Platform Testing** - Verification on different OS/architectures

## 📝 Code Style

### Rust Conventions

- Use `cargo fmt` (enforced in CI)
- Pass `cargo clippy -- -D warnings`
- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Add documentation for public APIs
- Use meaningful variable and function names

### Error Handling

- Use `anyhow::Result` for application errors
- Use `thiserror` for library errors
- Provide helpful error messages with suggestions
- Use `anyhow::Context` to add context to errors

Example:
```rust
use anyhow::{Context, Result};

fn load_config() -> Result<Config> {
    Config::from_file("xcargo.toml")
        .context("Failed to load xcargo.toml")
        .context("Run 'xcargo init' to create a configuration file")?
}
```

### Testing Guidelines

- Write tests for all public APIs
- Use descriptive test names: `test_target_from_triple_parses_linux`
- Test edge cases and error conditions
- Use temporary directories for filesystem tests
- Clean up resources in tests

Example:
```rust
#[test]
fn test_config_from_nonexistent_file() -> Result<()> {
    let result = Config::from_file("nonexistent.toml");
    assert!(result.is_err());
    Ok(())
}
```

## 🧪 Testing

### Platform Testing

Test on multiple platforms when possible:

| Platform | Priority | Notes |
|----------|----------|-------|
| **Linux x86_64** | High | Primary development platform |
| **macOS ARM64** | High | Apple Silicon testing |
| **macOS x86_64** | Medium | Intel Mac testing |
| **Windows 11** | Medium | Native Windows support |
| **Linux ARM64** | Low | CI/Docker testing |

### Cross-Compilation Testing

Test building for various targets:
```bash
# Linux targets
xcargo build --target x86_64-unknown-linux-gnu
xcargo build --target x86_64-unknown-linux-musl
xcargo build --target aarch64-unknown-linux-gnu

# macOS targets
xcargo build --target x86_64-apple-darwin
xcargo build --target aarch64-apple-darwin

# Windows targets
xcargo build --target x86_64-pc-windows-msvc
xcargo build --target x86_64-pc-windows-gnu
```

## 🚢 Release Process

See [RELEASE_PROCESS.md](.github/RELEASE_PROCESS.md) for detailed release procedures.

**Quick overview:**
1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Run tests: `cargo test`
4. Create tag: `git tag -a v0.x.0 -m "Release v0.x.0"`
5. Push tag: `git push origin v0.x.0`
6. GitHub Actions automatically builds and publishes

## 📦 Package Managers

### Current Status

| Package Manager | Status | Maintainer |
|----------------|--------|------------|
| **Homebrew** | ✅ Live | Automated via cargo-dist |
| **Shell installer** | ✅ Live | Automated via cargo-dist |
| **PowerShell** | ✅ Live | Automated via cargo-dist |
| **crates.io** | 🚧 Planned | Manual (v1.0.0) |
| **Scoop** | 📋 Planned | Community (v1.1+) |
| **APT** | 📋 Planned | Community (v1.2+) |
| **AUR** | 📋 Planned | Community (v1.2+) |

### Contributing Package Manager Support

To add xcargo to a package manager:

1. **Check existing installers:**
   - Shell: https://github.com/ibrahimcesar/xcargo/releases/latest/download/xcargo-installer.sh
   - Homebrew: https://github.com/ibrahimcesar/homebrew-tap

2. **Create package manifest:**
   - Follow package manager guidelines
   - Use official release artifacts
   - Include SHA256 verification

3. **Test installation:**
   - Install in clean environment
   - Verify `xcargo --version`
   - Test basic functionality

4. **Submit package:**
   - Open issue describing the package
   - Link to package repository
   - Provide maintenance plan

## 🐛 Debugging

### Enable verbose logging

```bash
# Set environment variable
RUST_LOG=debug xcargo build --target x86_64-unknown-linux-gnu

# Or use --verbose flag
xcargo build --verbose --target x86_64-unknown-linux-gnu
```

### Common issues

**Issue:** Tests fail with "target not installed"

**Solution:**
```bash
rustup target add x86_64-unknown-linux-gnu
```

**Issue:** Container builds fail

**Solution:**
```bash
# Check Docker is running
docker ps

# Or use Zig instead
xcargo build --target x86_64-unknown-linux-gnu --zig
```

## 💬 Communication

- **GitHub Issues:** Bug reports, feature requests
- **GitHub Discussions:** Questions, ideas, showcase
- **Pull Requests:** Code contributions

## 📜 License

By contributing to xcargo, you agree that your contributions will be licensed under the MIT License.

Your contributions may be redistributed under this license, and you retain copyright of your contributions.

## 🙏 Recognition

Contributors are recognized in:
- `CONTRIBUTORS.md` (alphabetically)
- Release notes for their contributions
- Git commit history

Significant contributions may be highlighted in:
- Project README
- Release announcements
- Project website

---

## Quick Links

- [Project Roadmap](ROADMAP.md)
- [Release Process](.github/RELEASE_PROCESS.md)
- [Homebrew Tap Setup](.github/HOMEBREW_TAP_SETUP.md)
- [cargo-dist TODO](.github/CARGO_DIST_TODO.md)

---

**Thank you for contributing to xcargo!** 🎯

Every contribution, no matter how small, helps make Rust cross-compilation easier for everyone.
