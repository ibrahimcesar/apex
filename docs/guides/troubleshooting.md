---
sidebar_position: 7
---

# Troubleshooting Guide

Common issues and solutions when using xcargo for cross-compilation.

## Quick Diagnosis

Start with the doctor command to identify issues:

```bash
xcargo doctor
```

This will check your environment and provide specific suggestions for any problems found.

## Build Failures

### Linker Not Found

**Error**:
```
error: linker `x86_64-w64-mingw32-gcc` not found
```

**Cause**: Cross-compilation linker is not installed.

**Solution**:

```bash
# macOS - Install MinGW for Windows cross-compilation
brew install mingw-w64

# Linux (Debian/Ubuntu)
sudo apt install mingw-w64

# Linux (Fedora/RHEL)
sudo dnf install mingw64-gcc
```

**Alternative**: Use Zig for automatic cross-compilation:

```bash
xcargo build --target x86_64-pc-windows-gnu --zig
```

### Target Not Installed

**Error**:
```
error: target 'x86_64-pc-windows-gnu' not installed
```

**Cause**: Rust target is not added to your toolchain.

**Solution**:

```bash
# Add the target
rustup target add x86_64-pc-windows-gnu

# Verify installation
rustup target list --installed
```

###  Zig Not Found

**Error**:
```
error: Zig compiler not found
```

**Cause**: Zig is not installed or not in PATH.

**Solution**:

```bash
# macOS
brew install zig

# Linux - Download from ziglang.org
wget https://ziglang.org/download/0.15.2/zig-linux-x86_64-0.15.2.tar.xz
tar xf zig-linux-x86_64-0.15.2.tar.xz
sudo mv zig-linux-x86_64-0.15.2 /usr/local/zig
export PATH="/usr/local/zig:$PATH"

# Windows - Use Scoop
scoop install zig
```

### Container Runtime Not Found

**Error**:
```
error: No container runtime available (docker/podman)
```

**Cause**: Docker or Podman is not installed when using `--container`.

**Solution**:

```bash
# Install Docker
# macOS/Windows: Download Docker Desktop from docker.com
# Linux: Follow instructions at docs.docker.com/engine/install/

# Or install Podman
brew install podman  # macOS
sudo apt install podman  # Linux
```

## Configuration Issues

### No xcargo.toml Found

**Warning**:
```
Warning: No xcargo.toml found
```

**Cause**: xcargo configuration file is missing.

**Solution**:

```bash
# Create default configuration
xcargo init

# Or create with interactive wizard
xcargo init --interactive
```

### Invalid Configuration

**Error**:
```
error: Failed to parse xcargo.toml: missing field `targets`
```

**Cause**: Configuration file has syntax errors or missing required fields.

**Solution**:

```bash
# View the default config for reference
xcargo config --default

# Validate your config
xcargo config
```

**Example valid configuration**:

```toml
[project]
name = "my-project"

[[targets]]
triple = "x86_64-pc-windows-gnu"

[[targets]]
triple = "x86_64-unknown-linux-gnu"

[build]
parallel = true
```

## Target-Specific Issues

### Windows (MinGW) Cross-Compilation

**Problem**: Windows builds fail from macOS/Linux

**Solutions**:

1. **Use Zig** (easiest):
```bash
xcargo build --target x86_64-pc-windows-gnu --zig
```

2. **Install MinGW**:
```bash
# macOS
brew install mingw-w64

# Linux
sudo apt install mingw-w64
```

3. **Use Container**:
```bash
xcargo build --target x86_64-pc-windows-gnu --container
```

### Linux Cross-Compilation from macOS

**Problem**: Linux builds fail on macOS

**Solutions**:

1. **Use Zig**:
```bash
xcargo build --target x86_64-unknown-linux-gnu --zig
```

2. **Use Container**:
```bash
xcargo build --target x86_64-unknown-linux-gnu --container
```

### ARM Cross-Compilation

**Problem**: ARM builds fail with linker errors

**Solutions**:

1. **Install ARM toolchain**:
```bash
# macOS
brew install --cask gcc-arm-embedded

# Linux (Debian/Ubuntu)
sudo apt install gcc-aarch64-linux-gnu

# Linux (Fedora/RHEL)
sudo dnf install gcc-aarch64-linux-gnu
```

2. **Configure linker in xcargo.toml**:
```toml
[[targets]]
triple = "aarch64-unknown-linux-gnu"
linker = "aarch64-linux-gnu-gcc"
```

## Environment Issues

### rustup Not Found

**Error**:
```
error: rustup not found in PATH
```

**Cause**: Rustup is not installed or shell hasn't been restarted after installation.

**Solution**:

```bash
# Install rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Restart shell or source profile
source $HOME/.cargo/env

# Verify installation
rustup --version
```

### No Default Toolchain

**Error**:
```
error: No default toolchain configured
```

**Cause**: No Rust toolchain is set as default.

**Solution**:

```bash
# Set stable as default
rustup default stable

# Or use nightly
rustup default nightly

# Verify
rustup show
```

### Permission Denied

**Error**:
```
error: Permission denied (os error 13)
```

**Cause**: Insufficient permissions for cargo/rustup directories.

**Solution**:

```bash
# Fix cargo directory permissions
chmod -R u+w ~/.cargo

# Fix rustup directory permissions
chmod -R u+w ~/.rustup

# Never run cargo with sudo!
```

## Performance Issues

### Slow Builds

**Problem**: Cross-compilation is slow

**Solutions**:

1. **Enable parallel builds**:
```bash
# In xcargo.toml
[build]
parallel = true

# Or via CLI
xcargo build --all
```

2. **Use build cache**:
```bash
# Cache is automatic, but you can clear it if needed
cargo clean
```

3. **Use native toolchain when possible**:
- Zig is faster than container builds
- Native cross-compilers are fastest

### Container Builds are Slow

**Problem**: Container-based builds take a long time

**Solutions**:

1. **Use Zig instead**:
```bash
xcargo build --target x86_64-unknown-linux-gnu --zig
```

2. **Cache cargo registry in container**:
```bash
# This is automatic with xcargo containers
# But ensure Docker Desktop has enough resources allocated
```

3. **Use native toolchain**:
```bash
# Install cross-compiler and avoid containers
brew install mingw-w64  # macOS
xcargo build --target x86_64-pc-windows-gnu  # No container needed
```

## Common Pitfalls

### Mixing --zig and --no-zig

**Error**: Conflicting flags

**Solution**: Use only one:
```bash
# Use Zig
xcargo build --zig

# Don't use Zig
xcargo build --no-zig

# Default: xcargo decides based on availability
xcargo build
```

### Building for Host Platform

**Tip**: You don't need xcargo for host platform builds

```bash
# For host platform, just use cargo
cargo build

# xcargo is for cross-compilation
xcargo build --target x86_64-pc-windows-gnu
```

### Forgetting to Add Target

**Problem**: Build fails immediately

**Solution**: Always add target first:
```bash
# Add target
rustup target add x86_64-pc-windows-gnu

# Then build
xcargo build --target x86_64-pc-windows-gnu
```

## Debugging Tips

### Enable Verbose Output

Get more details about what xcargo is doing:

```bash
xcargo -v build --target x86_64-pc-windows-gnu
```

### Check Environment

View current configuration:

```bash
# View xcargo config
xcargo config

# Check installed targets
rustup target list --installed

# Check available toolchains
rustup toolchain list

# Run diagnostics
xcargo doctor
```

### Test Individual Components

```bash
# Test target is installed
rustup target list --installed | grep x86_64-pc-windows-gnu

# Test linker is available
which x86_64-w64-mingw32-gcc

# Test Zig is available
zig version

# Test Docker is running
docker info
```

## Getting Help

### Check Documentation

- [xcargo doctor Reference](../reference/doctor.md) - Diagnostic tool
- [CLI Commands](../reference/cli-commands.md) - All available commands
- [Configuration Reference](../reference/configuration.md) - Config file format
- [Target Management](target-management.md) - Managing targets

### Report Issues

If you're still stuck:

1. Run `xcargo doctor` and save the output
2. Try with `-v` flag for verbose output
3. Check [GitHub Issues](https://github.com/ibrahimcesar/xcargo/issues)
4. Open a new issue with:
   - xcargo version (`xcargo version`)
   - Output of `xcargo doctor`
   - Verbose build output (`xcargo -v build ...`)
   - Your OS and Rust version (`rustup show`)

## Quick Reference

### Essential Commands

```bash
# Diagnose environment
xcargo doctor

# Initialize project
xcargo init

# Add a target
rustup target add <triple>

# Build for target
xcargo build --target <triple>

# Build for all targets
xcargo build --all

# Use Zig for cross-compilation
xcargo build --target <triple> --zig

# Use container
xcargo build --target <triple> --container
```

### Common Target Triples

```bash
# Windows
x86_64-pc-windows-gnu       # 64-bit Windows (MinGW)
x86_64-pc-windows-msvc      # 64-bit Windows (MSVC)
i686-pc-windows-gnu         # 32-bit Windows

# Linux
x86_64-unknown-linux-gnu    # 64-bit Linux (glibc)
x86_64-unknown-linux-musl   # 64-bit Linux (musl)
aarch64-unknown-linux-gnu   # ARM64 Linux

# macOS
x86_64-apple-darwin         # Intel macOS
aarch64-apple-darwin        # Apple Silicon

# WebAssembly
wasm32-unknown-unknown      # WebAssembly

# Mobile
aarch64-linux-android       # Android ARM64
aarch64-apple-ios           # iOS ARM64
```

## See Also

- [Cross-Compilation Guide](cross-compilation.md) - Detailed guide
- [Target Management](target-management.md) - Managing targets
- [CI/CD Integration](ci-cd-integration.md) - Using xcargo in CI/CD
