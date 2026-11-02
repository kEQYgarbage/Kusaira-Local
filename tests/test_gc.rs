//! Tests for torrent cache Garbage Collector.

use kusaira::torrent::gc::{calculate_cache_size, clean_all_cache, run_garbage_collection};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_gc_cache_size_and_cleanup() {
    let tmp = tempdir().expect("Failed to create tempdir");
    let path = tmp.path();

    let f1 = path.join("ep1.mkv");
    let f2 = path.join("ep2.mkv");
    let f3 = path.join("ep3.mkv");

    fs::write(&f1, vec![0u8; 1024 * 1024]).expect("Write f1"); // 1MB
    fs::write(&f2, vec![0u8; 2 * 1024 * 1024]).expect("Write f2"); // 2MB
    fs::write(&f3, vec![0u8; 3 * 1024 * 1024]).expect("Write f3"); // 3MB

    let total = calculate_cache_size(path);
    assert_eq!(total, 6 * 1024 * 1024);

    // Run GC with max 3MB limit -> should delete oldest files
    let stats = run_garbage_collection(path, 3 * 1024 * 1024, "keep_3");
    assert!(stats.freed_bytes > 0);
    assert!(stats.remaining_bytes <= 3 * 1024 * 1024);

    // Clean all
    let clean_stats = clean_all_cache(path);
    assert_eq!(clean_stats.remaining_bytes, 0);
    assert_eq!(calculate_cache_size(path), 0);
}
