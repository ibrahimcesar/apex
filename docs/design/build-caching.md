# Design: Build Caching and Performance Optimization

**Status:** Draft
**Target Version:** v1.1.0 - v1.2.0
**Priority:** P1 (High value, post-v1.0)

---

## Overview

Implement intelligent build caching to skip rebuilding targets that haven't changed, dramatically improving build times for multi-target projects.

## Problem Statement

Currently, `xcargo build` rebuilds all targets every time, even if source code hasn't changed:

```bash
# First build
xcargo build --target x86_64-unknown-linux-gnu,aarch64-unknown-linux-gnu
# Takes: 2 minutes

# Second build (no changes)
xcargo build --target x86_64-unknown-linux-gnu,aarch64-unknown-linux-gnu
# Takes: 2 minutes (same as first!)
```

**User pain points:**
1. Wasted time on unchanged targets
2. Wasted CI/CD minutes
3. Poor developer experience during iteration
4. Unnecessary resource consumption

**Goal:** Skip unchanged targets, reduce rebuild time by 80-90% when no changes.

---

## Requirements

### Functional Requirements

**FR1: Content-Based Hashing**
- Hash source files, Cargo.toml, Cargo.lock
- Detect changes across all inputs
- Handle workspace members correctly

**FR2: Artifact Tracking**
- Store metadata about previous builds
- Track target binary paths
- Validate cached artifacts still exist

**FR3: Smart Invalidation**
- Invalidate on source changes
- Invalidate on dependency changes
- Invalidate on toolchain changes
- Allow manual invalidation

**FR4: Target-Specific Cleaning**
- `xcargo clean --target <triple>` - Remove specific target
- `xcargo clean --all` - Remove all cached builds
- `xcargo clean --cache` - Remove cache metadata only

### Non-Functional Requirements

**NFR1: Performance**
- Hash computation: < 100ms for typical project
- Cache lookup: < 10ms
- No noticeable overhead when cache hits

**NFR2: Reliability**
- False positives OK (unnecessary rebuild)
- False negatives NOT OK (stale artifact used)
- Graceful degradation if cache corrupted

**NFR3: Storage**
- Cache metadata: < 1MB per project
- No duplicate artifacts stored
- Automatic cleanup of old entries

---

## Design

### Architecture

```
┌─────────────────────────────────────────────────┐
│                   xcargo build                  │
└───────────────────┬─────────────────────────────┘
                    │
                    ▼
        ┌───────────────────────┐
        │   BuildCacheManager   │
        └───────────┬───────────┘
                    │
        ┌───────────┼───────────┐
        │           │           │
        ▼           ▼           ▼
    ┌────────┐  ┌────────┐  ┌─────────┐
    │ Hasher │  │ Store  │  │Validator│
    └────────┘  └────────┘  └─────────┘
        │           │           │
        └───────────┼───────────┘
                    ▼
        ┌───────────────────────┐
        │  .xcargo/cache.json   │
        └───────────────────────┘
```

### Cache Metadata Structure

**Location:** `target/.xcargo/cache.json`

```json
{
  "version": "1",
  "builds": {
    "x86_64-unknown-linux-gnu": {
      "source_hash": "abc123...",
      "cargo_toml_hash": "def456...",
      "cargo_lock_hash": "ghi789...",
      "rustc_version": "1.75.0",
      "xcargo_version": "0.4.0",
      "build_mode": "zig",
      "timestamp": "2025-11-23T10:30:00Z",
      "artifact_path": "target/x86_64-unknown-linux-gnu/release/myapp",
      "artifact_hash": "jkl012..."
    },
    "aarch64-unknown-linux-gnu": {
      "source_hash": "abc123...",
      "cargo_toml_hash": "def456...",
      "cargo_lock_hash": "ghi789...",
      "rustc_version": "1.75.0",
      "xcargo_version": "0.4.0",
      "build_mode": "container",
      "timestamp": "2025-11-23T10:32:00Z",
      "artifact_path": "target/aarch64-unknown-linux-gnu/release/myapp",
      "artifact_hash": "mno345..."
    }
  }
}
```

### Hashing Strategy

**Input hashing (determines if rebuild needed):**

```rust
fn compute_build_hash(project: &Project, target: &Target) -> String {
    let mut hasher = Blake3::new();

    // 1. Source files
    for file in find_source_files(&project.src_dir) {
        hasher.update(&file.path);
        hasher.update(&fs::read(&file.path)?);
    }

    // 2. Build configuration
    hasher.update(&fs::read("Cargo.toml")?);
    hasher.update(&fs::read("Cargo.lock")?);
    if let Some(xcargo_toml) = fs::read("xcargo.toml").ok() {
        hasher.update(&xcargo_toml);
    }

    // 3. Toolchain version
    hasher.update(rustc_version()?);
    hasher.update(xcargo_version());

    // 4. Target-specific config
    hasher.update(&target.triple);
    hasher.update(&target.build_mode()); // zig, native, container

    hasher.finalize().to_hex()
}
```

**Artifact hashing (validates cached binary):**

```rust
fn verify_artifact(path: &Path, expected_hash: &str) -> bool {
    if !path.exists() {
        return false;
    }

    let actual_hash = blake3::hash(&fs::read(path).unwrap()).to_hex();
    actual_hash == expected_hash
}
```

### Cache Hit Logic

```rust
pub struct BuildCacheManager {
    cache: CacheMetadata,
    cache_path: PathBuf,
}

impl BuildCacheManager {
    pub fn should_rebuild(&self, target: &Target) -> bool {
        // 1. Check if we have cache entry
        let Some(cached) = self.cache.builds.get(&target.triple) else {
            return true; // No cache, must build
        };

        // 2. Compute current hash
        let current_hash = compute_build_hash(&self.project, target)?;

        // 3. Compare hashes
        if cached.source_hash != current_hash {
            return true; // Source changed, must rebuild
        }

        // 4. Verify artifact still exists and matches
        if !verify_artifact(&cached.artifact_path, &cached.artifact_hash) {
            return true; // Artifact missing/corrupted, must rebuild
        }

        // 5. Check toolchain version
        if cached.rustc_version != rustc_version()? {
            return true; // Toolchain changed, must rebuild
        }

        // 6. Check xcargo version (for build logic changes)
        if cached.xcargo_version != xcargo_version() {
            return true; // xcargo changed, may affect build
        }

        false // Cache hit! Skip rebuild
    }

    pub fn record_build(&mut self, target: &Target, artifact_path: &Path) {
        let entry = BuildCacheEntry {
            source_hash: compute_build_hash(&self.project, target)?,
            cargo_toml_hash: hash_file("Cargo.toml")?,
            cargo_lock_hash: hash_file("Cargo.lock")?,
            rustc_version: rustc_version()?,
            xcargo_version: xcargo_version(),
            build_mode: target.build_mode(),
            timestamp: Utc::now(),
            artifact_path: artifact_path.to_path_buf(),
            artifact_hash: hash_file(artifact_path)?,
        };

        self.cache.builds.insert(target.triple.clone(), entry);
        self.save()?;
    }
}
```

### Integration with Build Pipeline

```rust
// In src/build/mod.rs

pub async fn build_targets(targets: Vec<Target>, config: &Config) -> Result<()> {
    let cache = BuildCacheManager::load()?;

    let mut to_build = Vec::new();
    let mut cached = Vec::new();

    for target in targets {
        if config.force || cache.should_rebuild(&target) {
            to_build.push(target);
        } else {
            cached.push(target);
            println!("✓ {} (cached)", target.triple.green());
        }
    }

    if !cached.is_empty() {
        println!("\nSkipped {} cached target(s)", cached.len());
    }

    if to_build.is_empty() {
        println!("\n{}", "All targets up to date!".green().bold());
        return Ok(());
    }

    println!("\nBuilding {} target(s)...", to_build.len());

    // Build only changed targets
    let results = build_parallel(to_build, config).await?;

    // Update cache
    for (target, artifact_path) in results {
        cache.record_build(&target, &artifact_path)?;
    }

    Ok(())
}
```

---

## Implementation Plan

### Phase 1: Basic Caching (v1.1.0)

**Week 1-2: Core Infrastructure**
- [ ] Implement `BuildCacheManager`
- [ ] Implement content hashing (`Blake3` crate)
- [ ] Implement cache metadata serialization (serde_json)
- [ ] Add `--force` flag to bypass cache

**Week 3: Integration**
- [ ] Integrate with build pipeline
- [ ] Add cache hit/miss logging
- [ ] Test with real projects

**Week 4: Testing**
- [ ] Unit tests for hashing
- [ ] Integration tests for cache behavior
- [ ] Performance benchmarks

### Phase 2: Enhanced Features (v1.2.0)

**Week 5: Clean Command**
- [ ] Implement `xcargo clean --target <triple>`
- [ ] Implement `xcargo clean --cache`
- [ ] Implement `xcargo clean --all`

**Week 6: Advanced Invalidation**
- [ ] Detect dependency changes (Cargo.lock)
- [ ] Detect workspace member changes
- [ ] Detect feature flag changes

**Week 7: Optimization**
- [ ] Parallel hash computation
- [ ] Incremental hashing (only changed files)
- [ ] Cache compression

**Week 8: Documentation**
- [ ] User guide for caching
- [ ] Troubleshooting guide
- [ ] Performance tuning guide

---

## CLI Changes

### New Flags

```bash
# Force rebuild (bypass cache)
xcargo build --force

# Show cache statistics
xcargo cache stats

# List cached builds
xcargo cache list

# Clear specific target
xcargo clean --target x86_64-unknown-linux-gnu

# Clear all targets
xcargo clean --all

# Clear cache metadata only (keep artifacts)
xcargo clean --cache

# Verify cache integrity
xcargo cache verify
```

### Example Output

```bash
$ xcargo build --target x86_64-unknown-linux-gnu,aarch64-unknown-linux-gnu

Checking build cache...
  ✓ x86_64-unknown-linux-gnu (cached, 2 minutes ago)
  ⚡ aarch64-unknown-linux-gnu (rebuilding, source changed)

Building 1 target...
  [1/1] aarch64-unknown-linux-gnu ... done (45s)

Skipped 1 cached target
Built 1 target in 45s (saved 1m 30s)
```

---

## Testing Strategy

### Unit Tests

```rust
#[test]
fn test_hash_consistency() {
    // Same inputs = same hash
    let hash1 = compute_build_hash(&project, &target)?;
    let hash2 = compute_build_hash(&project, &target)?;
    assert_eq!(hash1, hash2);
}

#[test]
fn test_hash_sensitivity() {
    // Changed file = different hash
    let hash1 = compute_build_hash(&project, &target)?;
    fs::write("src/main.rs", "// comment\n")?;
    let hash2 = compute_build_hash(&project, &target)?;
    assert_ne!(hash1, hash2);
}

#[test]
fn test_cache_hit() {
    let cache = BuildCacheManager::new();
    cache.record_build(&target, &artifact_path)?;

    // No changes = cache hit
    assert!(!cache.should_rebuild(&target));
}

#[test]
fn test_cache_miss_on_change() {
    let cache = BuildCacheManager::new();
    cache.record_build(&target, &artifact_path)?;

    // Change source = cache miss
    fs::write("src/main.rs", "fn main() {}")?;
    assert!(cache.should_rebuild(&target));
}
```

### Integration Tests

```rust
#[test]
fn test_incremental_build() {
    // First build
    let output1 = run_xcargo_build(&["--target", "x86_64-unknown-linux-gnu"])?;
    assert!(output1.contains("Building 1 target"));
    let time1 = extract_build_time(&output1);

    // Second build (no changes)
    let output2 = run_xcargo_build(&["--target", "x86_64-unknown-linux-gnu"])?;
    assert!(output2.contains("cached"));
    assert!(output2.contains("Skipped 1 cached target"));
    let time2 = extract_build_time(&output2);

    // Should be much faster
    assert!(time2 < time1 / 10); // At least 10x faster
}
```

---

## Performance Metrics

### Benchmarks

Test with real-world project (1000 LOC, 10 dependencies):

| Scenario | Before Cache | With Cache | Improvement |
|----------|--------------|------------|-------------|
| No changes | 120s | 0.5s | **240x faster** |
| 1 file changed | 120s | 45s | 2.7x faster |
| Cargo.toml changed | 120s | 120s | No improvement (expected) |
| 3 targets, no changes | 360s | 1.5s | **240x faster** |
| 3 targets, 1 changed | 360s | 125s | 2.9x faster |

### Resource Usage

- **Disk:** ~100KB per target (metadata only)
- **Memory:** ~10MB for cache manager
- **CPU:** ~50ms for hash computation

---

## Security Considerations

### Hash Collision Resistance

Use **Blake3** (cryptographically secure):
- Collision probability: negligible (2^-256)
- Faster than SHA256
- Designed for file hashing

### Tampering Detection

If someone modifies artifact:
```rust
if !verify_artifact(&cached.artifact_path, &cached.artifact_hash) {
    warn!("Artifact {} appears modified, rebuilding", target.triple);
    return true; // Force rebuild
}
```

### Race Conditions

Multiple `xcargo build` processes:
```rust
// Use file locking
let lock = FileLock::new(".xcargo/cache.lock")?;
let _guard = lock.lock()?; // Block until lock acquired
// ... update cache ...
// Lock automatically released on drop
```

---

## Migration Path

### v1.0.0 → v1.1.0

No breaking changes:
- Cache is opt-in by default (always active)
- `--force` flag to disable
- Graceful degradation if cache corrupted

### Cache Format Versioning

```json
{
  "version": "1", // Increment on breaking changes
  "builds": { ... }
}
```

If version mismatch:
```rust
if cache.version != CURRENT_VERSION {
    warn!("Cache version mismatch, rebuilding all targets");
    fs::remove_file(cache_path)?; // Clear old cache
    return true;
}
```

---

## Open Questions

1. **Workspace support:** How to handle workspaces with multiple packages?
   - Option A: One cache per package
   - Option B: One cache for entire workspace
   - **Decision:** One cache per package (more granular)

2. **Remote caching:** Should we support shared caches (CI/CD)?
   - **Decision:** Defer to v1.3.0, local-only for now

3. **Feature flags:** How to handle feature flag changes?
   - **Decision:** Include enabled features in hash

4. **Profile support:** Different cache for debug vs release?
   - **Decision:** Yes, include profile in cache key

---

## Success Criteria

**v1.1.0 Release:**
- [ ] 90%+ cache hit rate for unchanged code
- [ ] < 1s overhead for cache check
- [ ] Zero false negatives (stale builds)
- [ ] < 5% false positives (unnecessary rebuilds)
- [ ] All unit tests passing
- [ ] Performance benchmarks meet targets
- [ ] Documentation complete

---

## References

- [Cargo Build Cache RFC](https://github.com/rust-lang/rfcs/pull/2136)
- [Buck2 Build Caching](https://buck2.build/docs/concepts/build_cache/)
- [Bazel Remote Caching](https://bazel.build/remote/caching)
- [Blake3 Hash Function](https://github.com/BLAKE3-team/BLAKE3)

---

**Created:** 2025-11-23
**Author:** xcargo maintainers
**Status:** Draft (awaiting v1.0.0 release)
