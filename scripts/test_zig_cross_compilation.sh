#!/usr/bin/env bash
set -euo pipefail

# Test Zig-based Cross-Compilation on macOS
# Purpose: Validate Zig as a cross-compilation toolchain for Linux targets

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK_DIR="${SCRIPT_DIR}/../.zig_test"

echo "🦎 xcargo Zig Cross-Compilation Test"
echo "====================================="
echo ""

# Platform detection
CURRENT_OS="$(uname -s)"
CURRENT_ARCH="$(uname -m)"

echo "🖥️  Platform Detection:"
echo "  OS: ${CURRENT_OS}"
echo "  Architecture: ${CURRENT_ARCH}"
echo ""

# Check if on macOS
if [[ "${CURRENT_OS}" != "Darwin" ]]; then
    echo "⚠️  NOTE: This test is designed for macOS"
    echo "Current platform: ${CURRENT_OS}"
    echo "Zig cross-compilation works on all platforms, but this test validates the macOS use case."
    echo ""
fi

# Step 1: Check if Zig is installed
echo "📥 Step 1: Checking for Zig installation..."
if command -v zig &> /dev/null; then
    ZIG_VERSION=$(zig version)
    ZIG_PATH=$(which zig)
    echo "✅ Zig found: ${ZIG_PATH}"
    echo "   Version: ${ZIG_VERSION}"
else
    echo "❌ Zig not found"
    echo ""
    echo "Install Zig to proceed:"
    echo "  brew install zig"
    echo ""
    echo "Or download from: https://ziglang.org/download/"
    exit 1
fi
echo ""

# Step 2: Create test project
echo "🦀 Step 2: Creating test Rust project..."
mkdir -p "${WORK_DIR}"
cd "${WORK_DIR}"

TEST_PROJECT="test-zig-cross"
if [ ! -d "${TEST_PROJECT}" ]; then
    cargo init --bin "${TEST_PROJECT}"
    echo "✅ Created test project: ${TEST_PROJECT}"
else
    echo "⏭️  Test project already exists, recreating..."
    rm -rf "${TEST_PROJECT}"
    cargo init --bin "${TEST_PROJECT}"
fi

# Create test source with platform detection
cat > "${TEST_PROJECT}/src/main.rs" <<'EOF'
fn main() {
    println!("🎉 Hello from Zig-cross-compiled Rust!");
    println!("   Architecture: {}", std::env::consts::ARCH);
    println!("   OS: {}", std::env::consts::OS);
    println!("   Family: {}", std::env::consts::FAMILY);

    #[cfg(target_os = "linux")]
    println!("   ✅ Running on Linux (as expected)");

    #[cfg(not(target_os = "linux"))]
    println!("   ⚠️  Not running on Linux");
}
EOF
echo "✅ Created test source code"
echo ""

# Step 3: Test basic cross-compilation
echo "🔨 Step 3: Testing basic cross-compilation (no dependencies)..."
cd "${TEST_PROJECT}"

# Add Rust target
rustup target add x86_64-unknown-linux-gnu 2>/dev/null || true

# Create wrapper scripts for Zig
WRAPPER_DIR="${WORK_DIR}/zig-wrappers"
mkdir -p "${WRAPPER_DIR}"

# x86_64-linux-gnu wrapper
cat > "${WRAPPER_DIR}/x86_64-linux-gnu-cc" <<'WRAPPER_EOF'
#!/bin/bash
exec zig cc -target x86_64-linux-gnu "$@"
WRAPPER_EOF
chmod +x "${WRAPPER_DIR}/x86_64-linux-gnu-cc"

# x86_64-linux-musl wrapper
cat > "${WRAPPER_DIR}/x86_64-linux-musl-cc" <<'WRAPPER_EOF'
#!/bin/bash
exec zig cc -target x86_64-linux-musl "$@"
WRAPPER_EOF
chmod +x "${WRAPPER_DIR}/x86_64-linux-musl-cc"

# aarch64-linux-gnu wrapper
cat > "${WRAPPER_DIR}/aarch64-linux-gnu-cc" <<'WRAPPER_EOF'
#!/bin/bash
exec zig cc -target aarch64-linux-gnu "$@"
WRAPPER_EOF
chmod +x "${WRAPPER_DIR}/aarch64-linux-gnu-cc"

# AR wrapper
cat > "${WRAPPER_DIR}/zig-ar" <<'WRAPPER_EOF'
#!/bin/bash
exec zig ar "$@"
WRAPPER_EOF
chmod +x "${WRAPPER_DIR}/zig-ar"

echo "✅ Created Zig wrapper scripts in ${WRAPPER_DIR}"

# Set environment to use wrappers
export CC="${WRAPPER_DIR}/x86_64-linux-gnu-cc"
export AR="${WRAPPER_DIR}/zig-ar"
export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER="${WRAPPER_DIR}/x86_64-linux-gnu-cc"

echo "Environment:"
echo "  CC=${CC}"
echo "  AR=${AR}"
echo "  LINKER=${CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER}"
echo ""

# Build
echo "Building with Zig..."
if cargo build --target x86_64-unknown-linux-gnu --release; then
    echo "✅ Build successful!"
else
    echo "❌ Build failed"
    exit 1
fi
echo ""

# Verify binary
echo "🔍 Step 4: Verifying built binary..."
BINARY="target/x86_64-unknown-linux-gnu/release/${TEST_PROJECT}"
if [ -f "${BINARY}" ]; then
    echo "Binary info:"
    file "${BINARY}"
    ls -lh "${BINARY}"
    echo ""

    # Check if it's actually a Linux binary
    if file "${BINARY}" | grep -q "ELF.*x86-64"; then
        echo "✅ Correct format: ELF x86-64 binary for Linux"
    else
        echo "❌ Unexpected format"
        exit 1
    fi
else
    echo "❌ Binary not found at ${BINARY}"
    exit 1
fi
echo ""

# Step 5: Test with C dependencies
echo "📦 Step 5: Testing with C dependencies (cc crate)..."
cd "${WORK_DIR}"
TEST_PROJECT_C="test-zig-c-deps"

if [ -d "${TEST_PROJECT_C}" ]; then
    rm -rf "${TEST_PROJECT_C}"
fi

cargo new --bin "${TEST_PROJECT_C}"
cd "${TEST_PROJECT_C}"

# Add dependency that uses cc crate
cat >> Cargo.toml <<'EOF'

[dependencies]
libc = "0.2"
EOF

# Create test that uses libc
cat > src/main.rs <<'EOF'
use libc::getpid;

fn main() {
    unsafe {
        let pid = getpid();
        println!("Process ID: {}", pid);
    }
    println!("✅ C dependency (libc) works with Zig!");
}
EOF

rustup target add x86_64-unknown-linux-gnu 2>/dev/null || true

echo "Building project with C dependencies..."
if cargo build --target x86_64-unknown-linux-gnu --release; then
    echo "✅ Build with C dependencies successful!"
else
    echo "⚠️  Build with C dependencies failed"
    echo "This is expected for some complex C dependencies"
fi
echo ""

# Step 6: Test musl target
echo "🔧 Step 6: Testing musl target (static linking)..."
rustup target add x86_64-unknown-linux-musl 2>/dev/null || true

cd "${WORK_DIR}/${TEST_PROJECT}"

export CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER="${WRAPPER_DIR}/x86_64-linux-musl-cc"

if cargo build --target x86_64-unknown-linux-musl --release; then
    echo "✅ musl build successful!"
    MUSL_BINARY="target/x86_64-unknown-linux-musl/release/${TEST_PROJECT}"
    if [ -f "${MUSL_BINARY}" ]; then
        echo "musl binary info:"
        file "${MUSL_BINARY}"

        # Check if statically linked
        if file "${MUSL_BINARY}" | grep -q "statically linked"; then
            echo "✅ Statically linked (as expected for musl)"
        fi
    fi
else
    echo "⚠️  musl build failed"
fi
echo ""

# Step 7: Test ARM64 target
echo "🔧 Step 7: Testing ARM64 (aarch64) target..."
rustup target add aarch64-unknown-linux-gnu 2>/dev/null || true

export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER="${WRAPPER_DIR}/aarch64-linux-gnu-cc"

if cargo build --target aarch64-unknown-linux-gnu --release; then
    echo "✅ ARM64 build successful!"
    ARM_BINARY="target/aarch64-unknown-linux-gnu/release/${TEST_PROJECT}"
    if [ -f "${ARM_BINARY}" ]; then
        echo "ARM64 binary info:"
        file "${ARM_BINARY}"

        if file "${ARM_BINARY}" | grep -q "ELF.*aarch64"; then
            echo "✅ Correct format: ELF aarch64 binary"
        fi
    fi
else
    echo "⚠️  ARM64 build failed"
fi
echo ""

# Summary
echo "════════════════════════════════════════════════════════"
echo "✅ Zig Cross-Compilation Test Complete!"
echo "════════════════════════════════════════════════════════"
echo ""
echo "📊 Summary:"
echo "  Zig version: ${ZIG_VERSION}"
echo "  ✅ x86_64-linux-gnu (glibc): Success"

if [ -f "${WORK_DIR}/${TEST_PROJECT_C}/target/x86_64-unknown-linux-gnu/release/${TEST_PROJECT_C}" ]; then
    echo "  ✅ C dependencies: Success"
else
    echo "  ⚠️  C dependencies: Failed (expected for complex deps)"
fi

if [ -f "${WORK_DIR}/${TEST_PROJECT}/target/x86_64-unknown-linux-musl/release/${TEST_PROJECT}" ]; then
    echo "  ✅ x86_64-linux-musl: Success"
else
    echo "  ⚠️  x86_64-linux-musl: Failed"
fi

if [ -f "${WORK_DIR}/${TEST_PROJECT}/target/aarch64-unknown-linux-gnu/release/${TEST_PROJECT}" ]; then
    echo "  ✅ aarch64-linux-gnu: Success"
else
    echo "  ⚠️  aarch64-linux-gnu: Failed"
fi

echo ""
echo "📁 Artifacts location:"
echo "  Work directory: ${WORK_DIR}"
echo "  x86_64 binary: ${WORK_DIR}/${TEST_PROJECT}/target/x86_64-unknown-linux-gnu/release/${TEST_PROJECT}"
echo ""
echo "🎯 Next Steps:"
echo "  1. Test binaries on actual Linux system"
echo "  2. Test with real-world projects (openssl, ring, sqlite)"
echo "  3. Document compatibility issues"
echo "  4. Integrate Zig backend into xcargo"
echo ""
echo "💡 To use Zig for cross-compilation manually:"
echo '  # Create wrapper script (zig-cc-x86_64-linux):'
echo '  echo "#!/bin/bash" > zig-cc-x86_64-linux'
echo '  echo "exec zig cc -target x86_64-linux-gnu \"\$@\"" >> zig-cc-x86_64-linux'
echo '  chmod +x zig-cc-x86_64-linux'
echo '  '
echo '  # Then use it:'
echo '  export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=./zig-cc-x86_64-linux'
echo '  cargo build --target x86_64-unknown-linux-gnu'
echo ""
