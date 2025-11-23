# Release Process

This document describes how to create and publish a new release of xcargo.

## Prerequisites

- Write access to `ibrahimcesar/xcargo` repository
- Write access to `ibrahimcesar/homebrew-tap` repository
- GitHub token configured as `HOMEBREW_TAP_TOKEN` secret
- Local `dist` CLI installed: `cargo install cargo-dist`

## Release Checklist

### Pre-Release

- [ ] All tests passing: `cargo test`
- [ ] Code coverage at target level (80%+): `cargo tarpaulin`
- [ ] CI/CD workflows passing on main branch
- [ ] Documentation up to date
- [ ] CHANGELOG.md updated with version changes
- [ ] Version bumped in `Cargo.toml`
- [ ] No TODO or FIXME comments in critical code paths

### Version Bumping

1. **Update version in Cargo.toml:**
   ```bash
   # Example: 0.3.0 → 0.4.0
   sed -i '' 's/version = "0.3.0"/version = "0.4.0"/' Cargo.toml
   ```

2. **Update version in documentation:**
   ```bash
   # Update docs/installation.md expected version
   sed -i '' 's/xcargo 0.3.0/xcargo 0.4.0/' docs/installation.md
   ```

3. **Update CHANGELOG.md:**
   ```markdown
   ## [0.4.0] - 2025-XX-XX

   ### Added
   - Feature 1
   - Feature 2

   ### Changed
   - Change 1

   ### Fixed
   - Bug fix 1
   ```

4. **Commit version bump:**
   ```bash
   git add Cargo.toml docs/installation.md CHANGELOG.md
   git commit -m "chore: bump version to 0.4.0"
   git push
   ```

### Creating the Release

1. **Verify cargo-dist plan:**
   ```bash
   dist plan
   ```

   Expected output should show:
   - All 5 target platforms
   - Installers: shell, powershell, homebrew
   - Artifacts for each platform

2. **Create and push tag:**
   ```bash
   # Create annotated tag
   git tag -a v0.4.0 -m "Release v0.4.0"

   # Push tag to trigger release workflow
   git push origin v0.4.0
   ```

3. **Monitor GitHub Actions:**
   - Go to: https://github.com/ibrahimcesar/xcargo/actions
   - Watch the `Release` workflow
   - Expected jobs:
     - `plan` - Generate release plan
     - `build-*` - Build binaries for each target
     - `host` - Create GitHub release
     - `publish-homebrew-formula` - Update Homebrew tap
     - `announce` - Finalize release

4. **Verify artifacts:**
   - Check https://github.com/ibrahimcesar/xcargo/releases/latest
   - Verify all binaries are present:
     - `xcargo-aarch64-apple-darwin.tar.xz`
     - `xcargo-x86_64-apple-darwin.tar.xz`
     - `xcargo-x86_64-unknown-linux-gnu.tar.xz`
     - `xcargo-x86_64-unknown-linux-musl.tar.xz`
     - `xcargo-x86_64-pc-windows-msvc.zip`
   - Verify installers:
     - `xcargo-installer.sh`
     - `xcargo-installer.ps1`
     - `xcargo.rb` (Homebrew formula)
   - Verify checksums:
     - Each binary should have a `.sha256` file

### Post-Release Verification

1. **Test shell installer (Linux/macOS):**
   ```bash
   curl --proto '=https' --tlsv1.2 -LsSf \
     https://github.com/ibrahimcesar/xcargo/releases/download/v0.4.0/xcargo-installer.sh | sh

   xcargo --version
   # Should output: xcargo 0.4.0
   ```

2. **Test Homebrew installation:**
   ```bash
   brew update
   brew install ibrahimcesar/tap/xcargo
   xcargo --version
   ```

3. **Test PowerShell installer (Windows):**
   ```powershell
   irm https://github.com/ibrahimcesar/xcargo/releases/download/v0.4.0/xcargo-installer.ps1 | iex
   xcargo --version
   ```

4. **Test prebuilt binary download:**
   ```bash
   # Example for Linux
   curl -LO https://github.com/ibrahimcesar/xcargo/releases/download/v0.4.0/xcargo-x86_64-unknown-linux-gnu.tar.xz
   curl -LO https://github.com/ibrahimcesar/xcargo/releases/download/v0.4.0/xcargo-x86_64-unknown-linux-gnu.tar.xz.sha256

   # Verify checksum
   sha256sum -c xcargo-x86_64-unknown-linux-gnu.tar.xz.sha256

   # Extract and test
   tar -xf xcargo-x86_64-unknown-linux-gnu.tar.xz
   ./xcargo --version
   ```

5. **Verify Homebrew tap updated:**
   ```bash
   # Check the homebrew-tap repository
   curl https://raw.githubusercontent.com/ibrahimcesar/homebrew-tap/main/Formula/xcargo.rb

   # Should show version 0.4.0 in the formula
   ```

### Announcing the Release

1. **Update project website:**
   - Update https://ibrahimcesar.github.io/xcargo with new version
   - Update installation instructions if needed

2. **Social media announcement (optional):**
   - Twitter/X
   - Reddit (r/rust)
   - Rust Users Forum

3. **crates.io publication (when ready):**
   ```bash
   cargo publish --dry-run
   cargo publish
   ```

## Troubleshooting

### Release workflow fails

**Check build logs:**
```bash
gh run list --workflow=release.yml
gh run view <run-id>
```

**Common issues:**
- Missing `HOMEBREW_TAP_TOKEN` secret
- Cargo.toml version mismatch
- Build failures on specific platforms
- Network timeouts during uploads

**Solution:**
- Fix the issue
- Delete the tag: `git tag -d v0.4.0 && git push origin :refs/tags/v0.4.0`
- Create a new patch version: `v0.4.1`
- Re-release

### Homebrew formula not updated

**Check:**
1. Verify `HOMEBREW_TAP_TOKEN` has write access
2. Check workflow logs for `publish-homebrew-formula` job
3. Manually check homebrew-tap repository for commits

**Manual fix:**
```bash
# Clone the tap
git clone https://github.com/ibrahimcesar/homebrew-tap
cd homebrew-tap

# Download the formula from release
curl -L https://github.com/ibrahimcesar/xcargo/releases/download/v0.4.0/xcargo.rb -o Formula/xcargo.rb

# Commit and push
git add Formula/xcargo.rb
git commit -m "xcargo 0.4.0"
git push
```

### Installer verification fails

**Issue:** Users report SHA256 mismatch

**Solution:**
1. Re-download the artifact from GitHub releases
2. Verify it locally: `sha256sum xcargo-*`
3. If mismatch, rebuild the release (may indicate corruption)

### Binary doesn't run on target platform

**Issue:** "Illegal instruction" or "GLIBC version" errors

**Solution:**
- Check target triple in cargo-dist config
- Verify musl vs glibc builds
- Consider providing both static and dynamic builds

## Rollback Procedure

If a release has critical bugs:

1. **Mark release as pre-release:**
   - Go to GitHub releases
   - Edit the release
   - Check "This is a pre-release"
   - Add warning to release notes

2. **Create hotfix release:**
   ```bash
   # Fix the bug
   git checkout -b hotfix/0.4.1
   # Make fixes
   git commit -m "fix: critical bug"
   git push

   # Bump to patch version
   # Update Cargo.toml: 0.4.0 → 0.4.1
   git tag -a v0.4.1 -m "Hotfix v0.4.1"
   git push origin v0.4.1
   ```

3. **Update Homebrew tap:**
   - New release will auto-update tap
   - Or manually update if needed

4. **Notify users:**
   - Update release notes for v0.4.0 with deprecation notice
   - Announce v0.4.1 as recommended version

## Release Cadence

**Recommended schedule:**
- **Patch releases (0.x.Y):** Bug fixes, as needed
- **Minor releases (0.X.0):** New features, monthly
- **Major releases (X.0.0):** Breaking changes, quarterly or less

**v1.0.0 criteria:**
- 80%+ test coverage ✓
- All core features complete ✓
- Documentation complete ✓
- Professional distribution ✓
- No known critical bugs
- Stable API surface
- Published to crates.io

## Security Considerations

### Checksum Verification

All binaries include SHA256 checksums:
```bash
# Verify before installation
sha256sum -c xcargo-x86_64-unknown-linux-gnu.tar.xz.sha256
```

### Supply Chain Security

- All builds happen in GitHub Actions (auditable)
- No external build servers
- Reproducible builds (same commit → same binary)
- cargo-dist uses official Rust toolchains

### Vulnerability Response

If a security vulnerability is discovered:

1. **Do not** create a public issue
2. Email: security@ibrahimcesar.com (or create private security advisory)
3. Create patch in private
4. Coordinate disclosure timeline
5. Release patched version
6. Publish security advisory

## Metrics to Track

After each release, track:
- Download counts per platform (GitHub Insights)
- Installation success rate (if telemetry enabled)
- Issue reports within 48 hours
- Homebrew analytics (if enabled)
- crates.io downloads

---

**Last Updated:** 2025-11-23
**Maintained By:** xcargo maintainers
**Next Review:** Before v1.0.0 release
