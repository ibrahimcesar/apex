//! File hashing utilities for cache invalidation

use crate::error::{Error, Result};
use std::fs;
use std::path::Path;
use std::time::SystemTime;

/// Compute a hash of a file's metadata and content
///
/// This creates a simple hash based on:
/// - File size
/// - Last modified time
/// - File path
///
/// For a more robust solution, we could use SHA256 or similar,
/// but this is sufficient for cache invalidation purposes.
#[must_use]
pub fn hash_file(path: &Path) -> Option<u64> {
    let metadata = fs::metadata(path).ok()?;

    let size = metadata.len();
    let modified = metadata
        .modified()
        .ok()?
        .duration_since(SystemTime::UNIX_EPOCH)
        .ok()?
        .as_secs();

    // Simple hash combining file size, modified time, and path
    let path_hash = hash_str(path.to_str()?);

    Some(hash_combine(&[size, modified, path_hash]))
}

/// Hash a string to u64
#[must_use]
fn hash_str(s: &str) -> u64 {
    // Simple DJB2 hash algorithm
    let mut hash: u64 = 5381;
    for byte in s.bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(u64::from(byte));
    }
    hash
}

/// Combine multiple u64 values into a single hash
#[must_use]
fn hash_combine(values: &[u64]) -> u64 {
    let mut hash: u64 = 0;
    for &value in values {
        hash ^= value.wrapping_add(0x9e3779b9)
            .wrapping_add(hash << 6)
            .wrapping_add(hash >> 2);
    }
    hash
}

/// Compute hash of multiple files
#[must_use]
pub fn hash_files(paths: &[&Path]) -> Option<u64> {
    let hashes: Vec<u64> = paths.iter().filter_map(|p| hash_file(p)).collect();

    if hashes.len() != paths.len() {
        // Some files couldn't be hashed
        return None;
    }

    Some(hash_combine(&hashes))
}

/// Check if a file has changed by comparing hashes
pub fn has_file_changed(path: &Path, previous_hash: u64) -> Result<bool> {
    let current_hash = hash_file(path)
        .ok_or_else(|| Error::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Failed to hash file: {}", path.display()),
        )))?;

    Ok(current_hash != previous_hash)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_hash_str() {
        let hash1 = hash_str("hello");
        let hash2 = hash_str("hello");
        let hash3 = hash_str("world");

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_hash_combine() {
        let hash1 = hash_combine(&[1, 2, 3]);
        let hash2 = hash_combine(&[1, 2, 3]);
        let hash3 = hash_combine(&[3, 2, 1]);

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3); // Order matters
    }

    #[test]
    fn test_hash_file() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "test content").unwrap();
        temp_file.flush().unwrap();

        let hash1 = hash_file(temp_file.path());
        assert!(hash1.is_some());

        // Hash should be consistent for same file
        let hash2 = hash_file(temp_file.path());
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_hash_file_changes() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "initial content").unwrap();
        temp_file.flush().unwrap();

        let hash1 = hash_file(temp_file.path()).unwrap();

        // Modify file
        std::thread::sleep(std::time::Duration::from_millis(10));
        writeln!(temp_file, "more content").unwrap();
        temp_file.flush().unwrap();

        let hash2 = hash_file(temp_file.path()).unwrap();

        // Hash should change after modification
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_hash_files() {
        let mut temp1 = NamedTempFile::new().unwrap();
        let mut temp2 = NamedTempFile::new().unwrap();

        writeln!(temp1, "file1").unwrap();
        writeln!(temp2, "file2").unwrap();
        temp1.flush().unwrap();
        temp2.flush().unwrap();

        let hash = hash_files(&[temp1.path(), temp2.path()]);
        assert!(hash.is_some());

        // Should be consistent
        let hash2 = hash_files(&[temp1.path(), temp2.path()]);
        assert_eq!(hash, hash2);
    }

    #[test]
    fn test_has_file_changed() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "content").unwrap();
        temp_file.flush().unwrap();

        let hash = hash_file(temp_file.path()).unwrap();

        // File hasn't changed
        let changed = has_file_changed(temp_file.path(), hash).unwrap();
        assert!(!changed);

        // Modify file
        std::thread::sleep(std::time::Duration::from_millis(10));
        writeln!(temp_file, "new content").unwrap();
        temp_file.flush().unwrap();

        // File has changed
        let changed = has_file_changed(temp_file.path(), hash).unwrap();
        assert!(changed);
    }
}
