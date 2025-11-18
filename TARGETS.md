# Supported Targets

This document lists all target platforms supported by apex and their implementation status.

## Target Tiers

- **Tier 1**: Native compilation (fast, no containers)
- **Tier 2**: Container-based (automatic fallback)
- **Tier 3**: Specialized (mobile, embedded, etc.)

---

## Tier 1: Native Compilation

These targets can be compiled natively on the host system with appropriate toolchains.

| Target | Status | Host OS | Toolchain | Notes |
|--------|--------|---------|-----------|-------|
| **x86_64-unknown-linux-gnu** | 📋 | Linux | gcc | Default Linux |
| **x86_64-unknown-linux-musl** | 📋 | Linux | musl-tools | Static linking |
| **x86_64-pc-windows-gnu** | 📋 | Linux | mingw-w64 | Cross to Windows |
| **x86_64-apple-darwin** | 📋 | macOS | Xcode | Intel Mac |
| **aarch64-apple-darwin** | 📋 | macOS | Xcode | Apple Silicon |
| **i686-pc-windows-gnu** | 📋 | Linux | mingw-w64 | 32-bit Windows |
| **i686-unknown-linux-gnu** | 📋 | Linux | gcc-multilib | 32-bit Linux |

---

## Tier 2: Container-Based

These targets require containers for cross-compilation (or are easier with containers).

| Target | Status | Container | Notes |
|--------|--------|-----------|-------|
| **aarch64-unknown-linux-gnu** | 📋 | Yes | ARM64 Linux |
| **aarch64-unknown-linux-musl** | 📋 | Yes | ARM64 static |
| **armv7-unknown-linux-gnueabihf** | 📋 | Yes | ARMv7 hard-float |
| **arm-unknown-linux-gnueabihf** | 📋 | Yes | ARMv6 hard-float |
| **x86_64-pc-windows-msvc** | 📋 | Yes | MSVC toolchain |
| **i686-pc-windows-msvc** | 📋 | Yes | MSVC 32-bit |
| **powerpc64le-unknown-linux-gnu** | ⏳ | Yes | POWER8+ |
| **s390x-unknown-linux-gnu** | ⏳ | Yes | IBM Z |

---

## Tier 3: Specialized Targets

### WebAssembly

| Target | Status | Notes |
|--------|--------|-------|
| **wasm32-unknown-unknown** | 📋 | Browser/WASI |
| **wasm32-wasi** | 📋 | WASI runtime |

### Android

| Target | Status | NDK | Notes |
|--------|--------|-----|-------|
| **aarch64-linux-android** | 📋 | r25+ | ARM64 |
| **armv7-linux-androideabi** | 📋 | r25+ | ARMv7 |
| **x86_64-linux-android** | 📋 | r25+ | Emulator |
| **i686-linux-android** | 📋 | r25+ | 32-bit emulator |

### iOS

| Target | Status | Xcode | Notes |
|--------|--------|-------|-------|
| **aarch64-apple-ios** | 📋 | 14+ | iPhone/iPad |
| **aarch64-apple-ios-sim** | 📋 | 14+ | Simulator |
| **x86_64-apple-ios** | 📋 | 14+ | Old simulator |

### Embedded (ARM)

| Target | Status | Notes |
|--------|--------|-------|
| **thumbv7em-none-eabihf** | ⏳ | Cortex-M4F/M7F |
| **thumbv7m-none-eabi** | ⏳ | Cortex-M3 |
| **thumbv6m-none-eabi** | ⏳ | Cortex-M0 |
| **riscv32imac-unknown-none-elf** | ⏳ | RISC-V 32-bit |

---

## Platform Support Matrix

### Linux → Other

| From Linux | To Windows | To macOS | To ARM | To Mobile |
|------------|------------|----------|--------|-----------|
| Native | ✅ mingw | ❌ * | ✅ Container | ✅ NDK/SDK |

\* *macOS cross-compilation from Linux requires osxcross (complex setup)*

### macOS → Other

| From macOS | To Linux | To Windows | To ARM | To iOS |
|------------|----------|------------|--------|--------|
| Universal | ✅ Container | ✅ Container | ✅ Container | ✅ Native |

### Windows → Other

| From Windows | To Linux | To macOS | To ARM |
|--------------|----------|----------|--------|
| WSL2 | ✅ Native | ❌ | ✅ Container |

---

## Target Aliases

apex provides convenient aliases for common targets:
```bash
# Platform aliases
apex build --target linux       # → x86_64-unknown-linux-gnu
apex build --target windows     # → x86_64-pc-windows-gnu
apex build --target macos       # → x86_64-apple-darwin (or aarch64 on M1/M2)

# Architecture variants
apex build --target linux-arm64 # → aarch64-unknown-linux-gnu
apex build --target linux-musl  # → x86_64-unknown-linux-musl

# Multiple targets
apex build --target linux,windows,macos
```

---

## Adding New Targets

To request a new target:

1. Check if rustup supports it: `rustup target list`
2. Open an issue with:
   - Target triple
   - Use case
   - Host OS you're building from
   - Toolchain information

---

## Implementation Status Legend

- ✅ Implemented
- 🚧 In Progress
- 📋 Planned
- ⏳ Backlog
- ❌ Not Supported

---

**Last Updated:** 2025-11-18  
**Targets Implemented:** 0 / 40+  
**Next Target:** x86_64-unknown-linux-gnu, x86_64-pc-windows-gnu
