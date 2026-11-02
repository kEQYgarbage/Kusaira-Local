//! Garbage Collector for torrent download cache, temporary chunks, and video buffers.

use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Default, PartialEq)]
pub struct GcStats {
    pub total_cache_bytes: u64,
    pub freed_bytes: u64,
    pub deleted_files: usize,
    pub remaining_bytes: u64,
}

pub fn default_cache_dir() -> PathBuf {
    if let Ok(home) = std::env::var("HOME") {
        PathBuf::from(home)
            .join(".cache")
            .join("kusaira")
            .join("torrents")
    } else {
        PathBuf::from(".cache/torrents")
    }
}

pub fn calculate_cache_size(dir: &Path) -> u64 {
    if !dir.exists() {
        return 0;
    }
    let mut total = 0;
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            if let Ok(meta) = entry.metadata() {
                if meta.is_file() {
                    total += meta.len();
                } else if meta.is_dir() {
                    total += calculate_cache_size(&entry.path());
                }
            }
        }
    }
    total
}

/// Run GC on the cache directory adhering to max_disk_bytes threshold and retention policy.
pub fn run_garbage_collection(dir: &Path, max_bytes: u64, _policy: &str) -> GcStats {
    let initial_size = calculate_cache_size(dir);
    if !dir.exists() || (max_bytes > 0 && initial_size <= max_bytes) {
        return GcStats {
            total_cache_bytes: initial_size,
            freed_bytes: 0,
            deleted_files: 0,
            remaining_bytes: initial_size,
        };
    }

    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            if let Ok(meta) = entry.metadata() {
                if meta.is_file() {
                    let mtime = meta.modified().unwrap_or(std::time::SystemTime::UNIX_EPOCH);
                    files.push((entry.path(), meta.len(), mtime));
                }
            }
        }
    }

    // Sort oldest files first
    files.sort_by_key(|f| f.2);

    let mut current_size = initial_size;
    let mut freed = 0;
    let mut deleted = 0;

    for (path, len, _) in files {
        if max_bytes > 0 && current_size <= max_bytes {
            break;
        }
        if fs::remove_file(&path).is_ok() {
            current_size = current_size.saturating_sub(len);
            freed += len;
            deleted += 1;
        }
    }

    GcStats {
        total_cache_bytes: initial_size,
        freed_bytes: freed,
        deleted_files: deleted,
        remaining_bytes: current_size,
    }
}

/// Force cleanup of all cached files in directory.
pub fn clean_all_cache(dir: &Path) -> GcStats {
    let initial_size = calculate_cache_size(dir);
    let mut deleted = 0;

    if dir.exists() {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                if let Ok(meta) = entry.metadata() {
                    if meta.is_file() {
                        if fs::remove_file(entry.path()).is_ok() {
                            deleted += 1;
                        }
                    }
                }
            }
        }
    }

    let remaining = calculate_cache_size(dir);
    GcStats {
        total_cache_bytes: initial_size,
        freed_bytes: initial_size.saturating_sub(remaining),
        deleted_files: deleted,
        remaining_bytes: remaining,
    }
}
