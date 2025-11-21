#!/usr/bin/env bash
set -euo pipefail

# Test Manual Cross-Compilation with Bootlin Toolchain
# Purpose: Validate minimal toolchain requirements for xcargo bundled toolchains

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK_DIR="${SCRIPT_DIR}/../.toolchain_test"
TOOLCHAIN_URL="https://toolchains.bootlin.com/downloads/releases/toolchains/x86-64/tarballs/x86-64--glibc--stable-2025.08-1.tar.xz"
TOOLCHAIN_NAME="x86-64--glibc--stable-2025.08-1"

echo "🔧 xcargo Toolchain Test - Manual Cross-Compilation"
echo "===================================================="
echo ""

# Platform detection
CURRENT_OS="$(uname -s)"
CURRENT_ARCH="$(uname -m)"

echo "🖥️  Platform Detection:"
echo "  OS: ${CURRENT_OS}"
echo "  Architecture: ${CURRENT_ARCH}"
echo ""

# Warning for non-Linux platforms
if [[ "${CURRENT_OS}" != "Linux" ]]; then
    echo "⚠️  WARNING: Bootlin toolchains require Linux host"
    echo ""
    echo "Current platform: ${CURRENT_OS}"
    echo "Bootlin toolchains: ELF binaries (Linux-only)"
    echo ""
    echo "This test will:"
    echo "  ✅ Download and extract the toolchain"
    echo "  ✅ Inspect toolchain structure"
    echo "  ❌ Fail at build step (cannot execute Linux binaries on ${CURRENT_OS})"
    echo ""
    echo "To run full test, use:"
    echo "  • Linux VM or WSL2"
    echo "  • GitHub Actions (ubuntu-latest)"
    echo "  • Docker container with Linux"
    echo ""
    read -p "Continue anyway to inspect toolchain? [y/N] " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Test cancelled."
        exit 0
    fi
    echo ""
fi

# Create work directory
mkdir -p "${WORK_DIR}"
cd "${WORK_DIR}"

# Step 1: Download toolchain
echo "📥 Step 1: Downloading Bootlin toolchain..."
echo "URL: ${TOOLCHAIN_URL}"
if [ ! -f "${TOOLCHAIN_NAME}.tar.xz" ]; then
    curl -L -o "${TOOLCHAIN_NAME}.tar.xz" "${TOOLCHAIN_URL}"
    echo "✅ Downloaded $(du -h "${TOOLCHAIN_NAME}.tar.xz" | cut -f1)"
else
    echo "⏭️  Toolchain already downloaded, skipping..."
fi
echo ""

# Step 2: Extract toolchain
echo "📦 Step 2: Extracting toolchain..."
if [ ! -d "${TOOLCHAIN_NAME}" ]; then
    tar xf "${TOOLCHAIN_NAME}.tar.xz"
    echo "✅ Extracted to ${TOOLCHAIN_NAME}/"
    echo "Directory size: $(du -sh "${TOOLCHAIN_NAME}" | cut -f1)"
else
    echo "⏭️  Toolchain already extracted, skipping..."
fi
echo ""

# Step 3: Inspect toolchain structure
echo "🔍 Step 3: Inspecting toolchain structure..."
echo "Contents:"
ls -lh "${TOOLCHAIN_NAME}/"
echo ""
echo "Binaries in bin/:"
ls -lh "${TOOLCHAIN_NAME}/bin/" | grep -E "(gcc|ld|ar|as|objcopy)" || true
echo ""

# Step 4: Create test Rust project
echo "🦀 Step 4: Creating test Rust project..."
TEST_PROJECT="test-hello"
if [ ! -d "${TEST_PROJECT}" ]; then
    cargo init --bin "${TEST_PROJECT}"
    echo "✅ Created test project: ${TEST_PROJECT}"
else
    echo "⏭️  Test project already exists, skipping..."
fi

# Create simple main.rs
cat > "${TEST_PROJECT}/src/main.rs" <<'EOF'
fn main() {
    println!("Hello from xcargo cross-compiled binary!");
    println!("Architecture: {}", std::env::consts::ARCH);
    println!("OS: {}", std::env::consts::OS);
}
EOF
echo "✅ Created test source code"
echo ""

# Step 5: Configure Cargo for cross-compilation
echo "⚙️  Step 5: Configuring Cargo for cross-compilation..."
TOOLCHAIN_ROOT="${WORK_DIR}/${TOOLCHAIN_NAME}"
TOOLCHAIN_BIN="${TOOLCHAIN_ROOT}/bin"
TOOLCHAIN_SYSROOT="${TOOLCHAIN_ROOT}/x86_64-buildroot-linux-gnu/sysroot"

# Find the actual gcc binary name
GCC_BINARY=$(ls "${TOOLCHAIN_BIN}/" | grep -E "gcc$" | head -1)
if [ -z "${GCC_BINARY}" ]; then
    echo "❌ Error: Could not find gcc binary in toolchain"
    exit 1
fi
echo "Found GCC binary: ${GCC_BINARY}"

# Create .cargo/config.toml
mkdir -p "${TEST_PROJECT}/.cargo"
cat > "${TEST_PROJECT}/.cargo/config.toml" <<EOF
[target.x86_64-unknown-linux-gnu]
linker = "${TOOLCHAIN_BIN}/${GCC_BINARY}"

[build]
target = "x86_64-unknown-linux-gnu"
EOF
echo "✅ Created .cargo/config.toml"
echo ""

# Step 6: Set environment variables
echo "🌍 Step 6: Setting environment variables..."
export CC="${TOOLCHAIN_BIN}/${GCC_BINARY}"
export LD="${TOOLCHAIN_BIN}/${GCC_BINARY/gcc/ld}"
export AR="${TOOLCHAIN_BIN}/${GCC_BINARY/gcc/ar}"
export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER="${CC}"

echo "CC=${CC}"
echo "LD=${LD}"
echo "AR=${AR}"
echo ""

# Step 7: Build the test project
echo "🔨 Step 7: Building test project with custom toolchain..."
cd "${TEST_PROJECT}"

# Add the Rust target if not already installed
rustup target add x86_64-unknown-linux-gnu 2>/dev/null || true

# Attempt to build
echo "Running: cargo build --release --target x86_64-unknown-linux-gnu"
if cargo build --release --target x86_64-unknown-linux-gnu; then
    echo "✅ Build successful!"
else
    echo "❌ Build failed. Checking what went wrong..."
    exit 1
fi
echo ""

# Step 8: Verify the binary
echo "🔍 Step 8: Verifying built binary..."
BINARY="target/x86_64-unknown-linux-gnu/release/${TEST_PROJECT}"
if [ -f "${BINARY}" ]; then
    echo "Binary info:"
    file "${BINARY}"
    ls -lh "${BINARY}"
    echo ""

    # Check if we can run it (only works on x86_64 Linux)
    if [[ "$(uname -m)" == "x86_64" ]] && [[ "$(uname -s)" == "Linux" ]]; then
        echo "Running binary:"
        ./"${BINARY}"
    else
        echo "⏭️  Skipping execution (not on x86_64 Linux)"
        echo "Binary is built for x86_64 Linux, current system: $(uname -m) $(uname -s)"
    fi
    echo ""
    echo "✅ Binary successfully created!"
else
    echo "❌ Binary not found at ${BINARY}"
    exit 1
fi
echo ""

# Step 9: Analyze toolchain usage
echo "📊 Step 9: Analyzing toolchain file usage..."
echo "This step identifies which toolchain files were actually used during build."
echo ""

# List all binaries
echo "Binaries in toolchain/bin/:"
ls -lh "${TOOLCHAIN_BIN}/" | awk '{print $9, $5}' | grep -v "^$"
echo ""

# Estimate minimal size
echo "Toolchain component sizes:"
echo "  bin/: $(du -sh "${TOOLCHAIN_BIN}" | cut -f1)"
if [ -d "${TOOLCHAIN_ROOT}/lib" ]; then
    echo "  lib/: $(du -sh "${TOOLCHAIN_ROOT}/lib" | cut -f1)"
fi
if [ -d "${TOOLCHAIN_ROOT}/libexec" ]; then
    echo "  libexec/: $(du -sh "${TOOLCHAIN_ROOT}/libexec" | cut -f1)"
fi
if [ -d "${TOOLCHAIN_SYSROOT}" ]; then
    echo "  sysroot/: $(du -sh "${TOOLCHAIN_SYSROOT}" | cut -f1)"
fi
echo ""

# Step 10: Create minimal toolchain package
echo "📦 Step 10: Creating minimal toolchain repackage..."
cd "${WORK_DIR}"
MINIMAL_DIR="xcargo-toolchain-x86_64-linux-gnu"
rm -rf "${MINIMAL_DIR}"
mkdir -p "${MINIMAL_DIR}/bin"
mkdir -p "${MINIMAL_DIR}/lib"
mkdir -p "${MINIMAL_DIR}/sysroot"

echo "Copying essential files..."
# Copy essential binaries
for binary in gcc ld ar as objcopy strip; do
    cp "${TOOLCHAIN_BIN}/"*-${binary} "${MINIMAL_DIR}/bin/" 2>/dev/null || true
done

# Copy GCC libraries
if [ -d "${TOOLCHAIN_ROOT}/lib/gcc" ]; then
    cp -r "${TOOLCHAIN_ROOT}/lib/gcc" "${MINIMAL_DIR}/lib/"
fi

# Copy sysroot
if [ -d "${TOOLCHAIN_SYSROOT}" ]; then
    cp -r "${TOOLCHAIN_SYSROOT}"/* "${MINIMAL_DIR}/sysroot/"
fi

echo "✅ Created minimal toolchain directory"
echo "Minimal toolchain size: $(du -sh "${MINIMAL_DIR}" | cut -f1)"
echo ""

# Create tarball
MINIMAL_TARBALL="xcargo-toolchain-x86_64-linux-gnu.tar.gz"
echo "Creating tarball: ${MINIMAL_TARBALL}"
tar czf "${MINIMAL_TARBALL}" "${MINIMAL_DIR}"
echo "✅ Created tarball"
echo "Tarball size: $(du -h "${MINIMAL_TARBALL}" | cut -f1)"
echo ""

# Summary
echo "════════════════════════════════════════════════════════"
echo "✅ Test Complete!"
echo "════════════════════════════════════════════════════════"
echo ""
echo "📊 Summary:"
echo "  Original toolchain: $(du -sh "${TOOLCHAIN_NAME}" | cut -f1)"
echo "  Original tarball: $(du -h "${TOOLCHAIN_NAME}.tar.xz" | cut -f1)"
echo "  Minimal toolchain: $(du -sh "${MINIMAL_DIR}" | cut -f1)"
echo "  Minimal tarball: $(du -h "${MINIMAL_TARBALL}" | cut -f1)"
echo ""
echo "📁 Artifacts location:"
echo "  Work directory: ${WORK_DIR}"
echo "  Minimal toolchain: ${WORK_DIR}/${MINIMAL_DIR}"
echo "  Minimal tarball: ${WORK_DIR}/${MINIMAL_TARBALL}"
echo "  Test binary: ${WORK_DIR}/${TEST_PROJECT}/target/x86_64-unknown-linux-gnu/release/${TEST_PROJECT}"
echo ""
echo "🎯 Next Steps:"
echo "  1. Verify the minimal tarball works for cross-compilation"
echo "  2. Test on clean environment"
echo "  3. Upload to GitHub releases for testing"
echo "  4. Implement src/toolchains/ module for automatic download/cache"
echo ""
