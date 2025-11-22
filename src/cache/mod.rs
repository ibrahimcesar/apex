//! Build cache management
//!
//! This module provides caching functionality to speed up repeated builds
//! by detecting when source files haven't changed.

mod hash;

pub use hash::{hash_file, hash_files, has_file_changed};

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Cache entry for a build target
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    /// Target triple
    pub target: String,
    /// Hash of source files
    pub source_hash: u64,
    /// Timestamp when build completed
    pub timestamp: u64,
    /// Build was successful
    pub success: bool,
}

/// Build cache manager
#[derive(Debug, Default)]
pub struct BuildCache {
    /// Cache directory path
    cache_dir: PathBuf,
    /// In-memory cache entries
    entries: HashMap<String, CacheEntry>,
}

impl BuildCache {
    /// Create a new build cache
    ///
    /// # Errors
    /// Returns error if cache directory cannot be created
    pub fn new() -> Result<Self> {
        let cache_dir = Self::default_cache_dir()?;
        fs::create_dir_all(&cache_dir)?;

        let mut cache = Self {
            cache_dir,
            entries: HashMap::new(),
        };

        cache.load()?;
        Ok(cache)
    }

    /// Create a cache with custom directory
    ///
    /// # Errors
    /// Returns error if cache directory cannot be created
    pub fn with_cache_dir(cache_dir: PathBuf) -> Result<Self> {
        fs::create_dir_all(&cache_dir)?;

        let mut cache = Self {
            cache_dir,
            entries: HashMap::new(),
        };

        cache.load()?;
        Ok(cache)
    }

    /// Get default cache directory
    ///
    /// # Errors
    /// Returns error if home directory cannot be determined
    fn default_cache_dir() -> Result<PathBuf> {
        let home = dirs::home_dir().ok_or_else(|| {
            Error::Config("Could not determine home directory".to_string())
        })?;

        Ok(home.join(".xcargo").join("cache"))
    }

    /// Get cache file path
    fn cache_file_path(&self) -> PathBuf {
        self.cache_dir.join("build-cache.json")
    }

    /// Load cache from disk
    ///
    /// # Errors
    /// Returns error if cache file cannot be read or parsed
    fn load(&mut self) -> Result<()> {
        let cache_file = self.cache_file_path();

        if !cache_file.exists() {
            return Ok(());
        }

        let contents = fs::read_to_string(&cache_file)?;
        self.entries = serde_json::from_str(&contents)
            .map_err(|e| Error::Config(format!("Failed to parse cache: {e}")))?;

        Ok(())
    }

    /// Save cache to disk
    ///
    /// # Errors
    /// Returns error if cache file cannot be written
    pub fn save(&self) -> Result<()> {
        let cache_file = self.cache_file_path();
        let contents = serde_json::to_string_pretty(&self.entries)
            .map_err(|e| Error::Config(format!("Failed to serialize cache: {e}")))?;

        fs::write(&cache_file, contents)?;
        Ok(())
    }

    /// Check if target needs rebuild
    ///
    /// Returns true if:
    /// - No cache entry exists
    /// - Previous build failed
    /// - Source files have changed
    #[must_use]
    pub fn needs_rebuild(&self, target: &str, source_hash: u64) -> bool {
        match self.entries.get(target) {
            None => true, // No cache entry
            Some(entry) => {
                !entry.success || entry.source_hash != source_hash
            }
        }
    }

    /// Get cache entry for target
    #[must_use]
    pub fn get(&self, target: &str) -> Option<&CacheEntry> {
        self.entries.get(target)
    }

    /// Update cache entry
    pub fn update(&mut self, target: String, source_hash: u64, success: bool) {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        self.entries.insert(
            target.clone(),
            CacheEntry {
                target,
                source_hash,
                timestamp,
                success,
            },
        );
    }

    /// Clear all cache entries
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Clear cache entry for specific target
    pub fn clear_target(&mut self, target: &str) {
        self.entries.remove(target);
    }

    /// Get cache statistics
    #[must_use]
    pub fn stats(&self) -> CacheStats {
        let total = self.entries.len();
        let successful = self.entries.values().filter(|e| e.success).count();
        let failed = total - successful;

        CacheStats {
            total_entries: total,
            successful_builds: successful,
            failed_builds: failed,
        }
    }
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    /// Total cache entries
    pub total_entries: usize,
    /// Successful builds cached
    pub successful_builds: usize,
    /// Failed builds cached
    pub failed_builds: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_build_cache_creation() {
        let temp_dir = TempDir::new().unwrap();
        let cache = BuildCache::with_cache_dir(temp_dir.path().to_path_buf()).unwrap();

        assert!(cache.entries.is_empty());
    }

    #[test]
    fn test_cache_update_and_get() {
        let temp_dir = TempDir::new().unwrap();
        let mut cache = BuildCache::with_cache_dir(temp_dir.path().to_path_buf()).unwrap();

        cache.update("x86_64-unknown-linux-gnu".to_string(), 12345, true);

        let entry = cache.get("x86_64-unknown-linux-gnu").unwrap();
        assert_eq!(entry.target, "x86_64-unknown-linux-gnu");
        assert_eq!(entry.source_hash, 12345);
        assert!(entry.success);
    }

    #[test]
    fn test_needs_rebuild() {
        let temp_dir = TempDir::new().unwrap();
        let mut cache = BuildCache::with_cache_dir(temp_dir.path().to_path_buf()).unwrap();

        // No cache entry - needs rebuild
        assert!(cache.needs_rebuild("x86_64-unknown-linux-gnu", 12345));

        // Add successful build
        cache.update("x86_64-unknown-linux-gnu".to_string(), 12345, true);

        // Same hash - no rebuild needed
        assert!(!cache.needs_rebuild("x86_64-unknown-linux-gnu", 12345));

        // Different hash - needs rebuild
        assert!(cache.needs_rebuild("x86_64-unknown-linux-gnu", 67890));

        // Failed build - needs rebuild
        cache.update("x86_64-pc-windows-gnu".to_string(), 11111, false);
        assert!(cache.needs_rebuild("x86_64-pc-windows-gnu", 11111));
    }

    #[test]
    fn test_cache_save_and_load() {
        let temp_dir = TempDir::new().unwrap();

        {
            let mut cache = BuildCache::with_cache_dir(temp_dir.path().to_path_buf()).unwrap();
            cache.update("x86_64-unknown-linux-gnu".to_string(), 12345, true);
            cache.save().unwrap();
        }

        // Load cache in new instance
        let cache = BuildCache::with_cache_dir(temp_dir.path().to_path_buf()).unwrap();
        let entry = cache.get("x86_64-unknown-linux-gnu").unwrap();
        assert_eq!(entry.source_hash, 12345);
    }

    #[test]
    fn test_cache_clear() {
        let temp_dir = TempDir::new().unwrap();
        let mut cache = BuildCache::with_cache_dir(temp_dir.path().to_path_buf()).unwrap();

        cache.update("target1".to_string(), 111, true);
        cache.update("target2".to_string(), 222, true);

        assert_eq!(cache.entries.len(), 2);

        cache.clear();
        assert!(cache.entries.is_empty());
    }

    #[test]
    fn test_cache_clear_target() {
        let temp_dir = TempDir::new().unwrap();
        let mut cache = BuildCache::with_cache_dir(temp_dir.path().to_path_buf()).unwrap();

        cache.update("target1".to_string(), 111, true);
        cache.update("target2".to_string(), 222, true);

        cache.clear_target("target1");

        assert!(cache.get("target1").is_none());
        assert!(cache.get("target2").is_some());
    }

    #[test]
    fn test_cache_stats() {
        let temp_dir = TempDir::new().unwrap();
        let mut cache = BuildCache::with_cache_dir(temp_dir.path().to_path_buf()).unwrap();

        cache.update("target1".to_string(), 111, true);
        cache.update("target2".to_string(), 222, true);
        cache.update("target3".to_string(), 333, false);

        let stats = cache.stats();
        assert_eq!(stats.total_entries, 3);
        assert_eq!(stats.successful_builds, 2);
        assert_eq!(stats.failed_builds, 1);
    }
}
