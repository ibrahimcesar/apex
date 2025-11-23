---
sidebar_position: 2
---

# Installation

Learn how to install xcargo on your system.

## Quick Install (Recommended)

The fastest way to install xcargo is using our installer scripts:

### macOS / Linux

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/ibrahimcesar/xcargo/releases/latest/download/xcargo-installer.sh | sh
```

### Windows (PowerShell)

```powershell
irm https://github.com/ibrahimcesar/xcargo/releases/latest/download/xcargo-installer.ps1 | iex
```

## Package Managers

### Homebrew (macOS / Linux)

```bash
# Install from our tap
brew install ibrahimcesar/tap/xcargo
```

**Updating:**
```bash
brew upgrade xcargo
```

## Install from crates.io

:::info Coming Soon
xcargo is not yet published to crates.io. Use one of the installation methods above.
:::

Once published, you'll be able to install with:

```bash
cargo install xcargo
```

## Prebuilt Binaries

Download prebuilt binaries from the [latest release](https://github.com/ibrahimcesar/xcargo/releases/latest):

| Platform | Download |
|----------|----------|
| **macOS (Apple Silicon)** | [xcargo-aarch64-apple-darwin.tar.xz](https://github.com/ibrahimcesar/xcargo/releases/latest/download/xcargo-aarch64-apple-darwin.tar.xz) |
| **macOS (Intel)** | [xcargo-x86_64-apple-darwin.tar.xz](https://github.com/ibrahimcesar/xcargo/releases/latest/download/xcargo-x86_64-apple-darwin.tar.xz) |
| **Linux (glibc)** | [xcargo-x86_64-unknown-linux-gnu.tar.xz](https://github.com/ibrahimcesar/xcargo/releases/latest/download/xcargo-x86_64-unknown-linux-gnu.tar.xz) |
| **Linux (musl)** | [xcargo-x86_64-unknown-linux-musl.tar.xz](https://github.com/ibrahimcesar/xcargo/releases/latest/download/xcargo-x86_64-unknown-linux-musl.tar.xz) |
| **Windows (MSVC)** | [xcargo-x86_64-pc-windows-msvc.zip](https://github.com/ibrahimcesar/xcargo/releases/latest/download/xcargo-x86_64-pc-windows-msvc.zip) |

All downloads include SHA256 checksums for verification.

## Install from Source

Clone the repository and build from source:

```bash
# Clone the repository
git clone https://github.com/ibrahimcesar/xcargo
cd xcargo

# Build and install
cargo install --path .
```

## Verify Installation

Check that xcargo is installed correctly:

```bash
xcargo --version
```

You should see output like:

```
xcargo 0.3.0
```

Run system diagnostics to verify your setup:

```bash
xcargo doctor
```

Expected output:
```
xcargo System Diagnostics
========================

✓ Rust Toolchain
  rustc 1.75.0 (stable)
  cargo 1.75.0

✓ Host Platform
  x86_64-unknown-linux-gnu

✓ Cross-Compilation Tools
  zig: 0.11.0 (optional)

System is ready for cross-compilation!
```

## Prerequisites

xcargo requires these tools to be installed:

| Tool | Required For | Installation |
|------|--------------|--------------|
| **cargo** | All builds | Install Rust from [rustup.rs](https://rustup.rs) |
| **rustup** | Target management | Included with Rust installation |
| **zig** (optional) | Linux cross-compilation from macOS/Windows | [ziglang.org](https://ziglang.org/download/) |
| **docker/podman** (optional) | Container-based builds | [docker.com](https://www.docker.com) or [podman.io](https://podman.io) |

## Platform-Specific Notes

### Linux

xcargo works best on Linux for cross-compilation. Most cross-compilation toolchains are readily available:

```bash
# Debian/Ubuntu - Install common cross-compilation tools
sudo apt-get install gcc-aarch64-linux-gnu gcc-arm-linux-gnueabihf mingw-w64

# Fedora/RHEL
sudo dnf install gcc-aarch64-linux-gnu gcc-arm-linux-gnu mingw64-gcc
```

### macOS

On macOS, **Zig** is recommended for Linux cross-compilation (zero-config, no Docker required):

```bash
# Install Zig (recommended)
brew install zig

# Optional: Install mingw for Windows cross-compilation
brew install mingw-w64
```

With Zig installed, you can cross-compile to Linux targets without any additional setup:

```bash
xcargo build --target x86_64-unknown-linux-gnu
xcargo build --target aarch64-unknown-linux-gnu
```

### Windows

On Windows, **Zig** is recommended for Linux cross-compilation:

```powershell
# Install Zig using Scoop
scoop install zig

# Or download from: https://ziglang.org/download/
```

Alternatively, consider using WSL2 for the best cross-compilation experience.

## Next Steps

- [Quick Start](./quick-start.md) - Learn basic usage
- [Target Management](./guides/target-management.md) - Manage compilation targets
