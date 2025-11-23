# Package Manager Templates

This directory contains templates for community-maintained package managers.

## Available Templates

### Scoop (Windows)

**Status:** Template ready, awaiting submission to [scoop-main](https://github.com/ScoopInstaller/Main)

**File:** `scoop-manifest.json`

**How to use:**
1. Update version number and SHA256 hash for new releases
2. Test locally:
   ```powershell
   scoop install path/to/scoop-manifest.json
   ```
3. Submit to Scoop bucket

**Requirements:**
- Windows 10+
- Scoop installed

**Installation (once published):**
```powershell
scoop install xcargo
```

### AUR (Arch Linux)

**Status:** Template ready, awaiting AUR submission

**File:** `PKGBUILD`

**How to use:**
1. Update `pkgver` and SHA256 checksum
2. Build and test:
   ```bash
   makepkg -si
   ```
3. Submit to AUR: https://aur.archlinux.org/submit/

**Requirements:**
- Arch Linux or Arch-based distro
- `base-devel` package group

**Installation (once published):**
```bash
yay -S xcargo-bin
# or
paru -S xcargo-bin
```

## Submitting to Package Managers

### Before Submission

1. **Test the package:**
   - Install in clean environment
   - Verify `xcargo --version`
   - Test cross-compilation: `xcargo build --target x86_64-unknown-linux-gnu`
   - Verify `xcargo doctor` works

2. **Update checksums:**
   ```bash
   # Get SHA256 from GitHub release
   curl -L https://github.com/ibrahimcesar/xcargo/releases/download/v0.3.0/xcargo-x86_64-unknown-linux-gnu.tar.xz.sha256
   ```

3. **Update version numbers:**
   - Package version should match xcargo release version
   - Update download URLs

### Scoop Submission

**Repository:** https://github.com/ScoopInstaller/Main

**Process:**
1. Fork the repository
2. Add `xcargo.json` to `bucket/` directory
3. Run validation: `.\bin\checkver.ps1 xcargo -u`
4. Create pull request
5. Wait for maintainer review

**Guidelines:**
- Follow [Scoop contribution guidelines](https://github.com/ScoopInstaller/Scoop/wiki/Contributing)
- Ensure autoupdate works correctly
- Include helpful notes for users

### AUR Submission

**Repository:** https://aur.archlinux.org

**Process:**
1. Create AUR account
2. Add SSH key
3. Clone package repository:
   ```bash
   git clone ssh://aur@aur.archlinux.org/xcargo-bin.git
   ```
4. Add PKGBUILD and .SRCINFO:
   ```bash
   makepkg --printsrcinfo > .SRCINFO
   ```
5. Commit and push:
   ```bash
   git add PKGBUILD .SRCINFO
   git commit -m "Initial commit: xcargo-bin 0.3.0"
   git push
   ```

**Guidelines:**
- Follow [AUR submission guidelines](https://wiki.archlinux.org/title/AUR_submission_guidelines)
- Use `-bin` suffix for binary packages
- Maintain package regularly (update for new releases)
- Respond to user comments

## Future Package Managers

### APT (Debian/Ubuntu) - Planned for v1.2.0

**Options:**
1. **PPA (Personal Package Archive):**
   - Host on Launchpad
   - Easy for users: `add-apt-repository ppa:ibrahimcesar/xcargo`

2. **Self-hosted repository:**
   - Host `.deb` files on GitHub Pages
   - Add to sources.list

3. **Repology:**
   - Submit to existing repository

**Resources:**
- [Debian packaging guide](https://www.debian.org/doc/manuals/maint-guide/)
- [Ubuntu PPA guide](https://help.launchpad.net/Packaging/PPA)

### Nix/NixOS - Community Request

**Template location:** https://github.com/NixOS/nixpkgs

**Process:**
- Add package to `nixpkgs/pkgs/development/tools/rust/xcargo/`
- Submit PR to nixpkgs

### Conda - For Python/Scientific Users

**Template location:** https://github.com/conda-forge

**Process:**
- Create feedstock repository
- Submit to conda-forge

## Maintaining Packages

### Automated Updates

For automated package maintenance:

1. **Watch GitHub releases:**
   - Enable notifications for xcargo releases
   - Or use GitHub Actions to detect new releases

2. **Update checksums automatically:**
   ```bash
   # Example script for AUR
   new_version="0.4.0"
   url="https://github.com/ibrahimcesar/xcargo/releases/download/v${new_version}/xcargo-x86_64-unknown-linux-gnu.tar.xz"
   checksum=$(curl -sL "${url}.sha256" | awk '{print $1}')

   sed -i "s/pkgver=.*/pkgver=${new_version}/" PKGBUILD
   sed -i "s/sha256sums_x86_64=.*/sha256sums_x86_64=('${checksum}')/" PKGBUILD
   ```

3. **Test and publish:**
   - Always test before publishing
   - Follow package manager guidelines

### Version Numbering

- Match xcargo release versions exactly
- Use package revision for packaging-only changes:
  - xcargo 0.3.0, first package: `0.3.0-1`
  - xcargo 0.3.0, packaging fix: `0.3.0-2`
  - xcargo 0.4.0, first package: `0.4.0-1`

## Contact

For questions about package management:
- Open issue: https://github.com/ibrahimcesar/xcargo/issues
- Email: email@ibrahimcesar.com

## License

Package manager manifests in this directory are licensed under MIT, same as xcargo.
