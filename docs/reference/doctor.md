---
sidebar_position: 5
---

# xcargo doctor

System diagnostics and troubleshooting command.

## Overview

The `xcargo doctor` command runs comprehensive system diagnostics to verify your cross-compilation environment is properly configured. It checks for required tools, installed targets, and optional dependencies.

## Usage

```bash
xcargo doctor
```

## What It Checks

The doctor command performs 9 diagnostic checks:

### 1. rustup
Verifies rustup installation and version.

**Pass**: rustup is installed and accessible
**Critical**: rustup not found - xcargo requires rustup

### 2. cargo
Checks Rust toolchain installation.

**Pass**: cargo is installed and accessible
**Critical**: cargo not found - xcargo requires cargo

### 3. Default Toolchain
Verifies a default Rust toolchain is configured.

**Pass**: Default toolchain is set (stable/beta/nightly)
**Warning**: No default toolchain configured

**Fix**:
```bash
rustup default stable
```

### 4. Installed Targets
Lists cross-compilation targets installed for the default toolchain.

**Pass**: One or more additional targets installed
**Warning**: Only host target available

**Fix**:
```bash
# Install specific target
rustup target add x86_64-pc-windows-gnu

# Install multiple targets
rustup target add x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu
```

### 5. Zig (Optional)
Checks for Zig compiler, used for easy C/C++ cross-compilation.

**Pass**: Zig is installed
**Warning**: Zig not found (optional feature)

**Install**:
```bash
# macOS
brew install zig

# Linux (download from ziglang.org)
wget https://ziglang.org/download/0.15.2/zig-linux-x86_64-0.15.2.tar.xz
tar xf zig-linux-x86_64-0.15.2.tar.xz
sudo mv zig-linux-x86_64-0.15.2 /usr/local/zig
export PATH="/usr/local/zig:$PATH"
```

### 6. Docker (Optional)
Checks for Docker, used for container-based builds.

**Pass**: Docker is installed and daemon is running
**Warning**: Docker found but daemon not running
**Warning**: Docker not found (optional feature)

**Install**:
- macOS/Windows: [Docker Desktop](https://docker.com/)
- Linux: Follow [official instructions](https://docs.docker.com/engine/install/)

### 7. Podman (Optional)
Checks for Podman, an alternative container runtime.

**Pass**: Podman is installed
**Warning**: Podman not found (optional feature)

**Install**:
```bash
# macOS
brew install podman

# Linux
sudo apt install podman  # Debian/Ubuntu
sudo dnf install podman  # Fedora/RHEL
```

### 8. Common Linkers
Checks for cross-compilation linkers:
- `gcc` - GNU Compiler Collection
- `clang` - LLVM Clang
- `x86_64-w64-mingw32-gcc` - MinGW-w64 for Windows
- `aarch64-linux-gnu-gcc` - ARM64 Linux cross-compiler

**Pass**: All common linkers found
**Warning**: Some linkers missing

**Install**:
```bash
# macOS
brew install mingw-w64        # Windows cross-compilation
brew install --cask gcc-arm-embedded  # ARM cross-compilation

# Linux (Debian/Ubuntu)
sudo apt install build-essential mingw-w64 gcc-aarch64-linux-gnu

# Linux (Fedora/RHEL)
sudo dnf install gcc mingw64-gcc gcc-aarch64-linux-gnu
```

### 9. xcargo.toml
Checks for xcargo configuration file in current directory or parents.

**Pass**: Configuration file found
**Warning**: No xcargo.toml found

**Fix**:
```bash
xcargo init
```

## Output Format

The doctor command uses color-coded status indicators:

- **✓ [PASS]** - Check passed (green)
- **⚠ [WARN]** - Optional feature unavailable or non-critical issue (yellow)
- **✗ [FAIL]** - Feature unavailable, some functionality limited (red)
- **✗ [CRIT]** - Critical issue, xcargo will not work (bright red, bold)

## Exit Codes

- **0** - All critical checks passed (system functional)
- **2** - Critical checks failed (xcargo will not work)

## Example Output

```
xcargo doctor - System Diagnostics
──────────────────────────────────
Checking your cross-compilation environment...

✓ [PASS] rustup
  Found at "/Users/user/.cargo/bin/rustup": rustup 1.28.2

✓ [PASS] cargo
  Found at "/Users/user/.cargo/bin/cargo": cargo 1.91.1

✓ [PASS] default toolchain
  Using toolchain: stable-x86_64-apple-darwin

✓ [PASS] installed targets
  4 target(s) installed for stable

✓ [PASS] zig
  Found at "/opt/homebrew/bin/zig": v0.15.2

⚠ [WARN] docker
  Docker not found (optional)
  → Install Docker for container-based builds: https://docker.com/

⚠ [WARN] podman
  Podman not found (optional)
  → Install Podman as Docker alternative: https://podman.io/

⚠ [WARN] common linkers
  Found 2 linker(s): GNU Compiler Collection, LLVM Clang
  → Missing: MinGW-w64 for Windows, ARM64 Linux cross-compiler.

✓ [PASS] xcargo.toml
  Found configuration at: /Users/user/project/xcargo.toml


============================================================
Summary
============================================================
  Total checks:      9
  ✓ Passed:          5
  ⚠ Warnings:        4

✓ System is functional. Some optional features unavailable.
```

## Use Cases

### Before Starting a Project

Run `xcargo doctor` to verify your environment is ready:

```bash
cd my-project
xcargo doctor
```

### Troubleshooting Build Issues

If builds fail, run doctor to identify missing dependencies:

```bash
xcargo doctor
# Install missing tools based on suggestions
xcargo build --target x86_64-pc-windows-gnu
```

### CI/CD Health Checks

Use in CI pipelines to verify runner configuration:

```yaml
# GitHub Actions
- name: Check xcargo environment
  run: xcargo doctor
```

### New Machine Setup

After installing Rust on a new machine:

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify setup
xcargo doctor

# Install recommended tools
brew install zig mingw-w64  # macOS
```

## Common Issues

### "rustup not found"

**Problem**: rustup is not installed or not in PATH

**Solution**:
```bash
# Install rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Restart shell or source profile
source $HOME/.cargo/env
```

### "No default toolchain set"

**Problem**: No Rust toolchain configured as default

**Solution**:
```bash
rustup default stable
```

### "Docker daemon not running"

**Problem**: Docker is installed but daemon is not active

**Solution**:
- **macOS/Windows**: Start Docker Desktop application
- **Linux**: `sudo systemctl start docker`

### "No additional targets installed"

**Problem**: Only host target available, can't cross-compile

**Solution**:
```bash
# Common targets
rustup target add x86_64-pc-windows-gnu      # Windows
rustup target add x86_64-unknown-linux-gnu   # Linux
rustup target add aarch64-unknown-linux-gnu  # ARM64 Linux
rustup target add wasm32-unknown-unknown     # WebAssembly
```

## Related Commands

- [`xcargo target list`](cli-commands.md#target-list) - List available targets
- [`xcargo target add`](cli-commands.md#target-add) - Install a target
- [`xcargo init`](cli-commands.md#init) - Initialize xcargo configuration
- [`xcargo config`](cli-commands.md#config) - View configuration

## See Also

- [Troubleshooting Guide](../guides/troubleshooting.md) - Common issues and solutions
- [Installation Guide](../installation.md) - Installing xcargo and dependencies
- [Target Management](../guides/target-management.md) - Managing Rust targets
