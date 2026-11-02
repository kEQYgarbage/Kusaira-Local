//! Firefox SQLite cookie extractor for zero-login RuTracker authentication.

use rusqlite::{Connection, OpenFlags};
use std::path::{Path, PathBuf};

/// Find the active Firefox cookies.sqlite database on Linux or Windows.
pub fn find_firefox_cookies_db() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    let appdata = std::env::var("APPDATA").ok()?;
    #[cfg(target_os = "windows")]
    let firefox_dir = Path::new(&appdata).join("Mozilla").join("Firefox").join("Profiles");

    #[cfg(not(target_os = "windows"))]
    let home = std::env::var("HOME").ok()?;
    #[cfg(not(target_os = "windows"))]
    let firefox_dir = Path::new(&home).join(".mozilla").join("firefox");

    if !firefox_dir.exists() {
        return None;
    }

    if let Ok(entries) = std::fs::read_dir(firefox_dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() {
                let cookie_file = p.join("cookies.sqlite");
                if cookie_file.exists() {
                    return Some(cookie_file);
                }
            }
        }
    }

    None
}

/// Extract RuTracker cookies (bb_session, bb_data) from the given SQLite file.
pub fn extract_rutracker_cookie_from_file(db_path: &Path) -> Result<Option<String>, String> {
    if !db_path.exists() {
        return Ok(None);
    }

    // Open read-only to avoid locking active Firefox instances
    let conn = Connection::open_with_flags(db_path, OpenFlags::SQLITE_OPEN_READ_ONLY)
        .map_err(|e| format!("Failed to open cookie db: {}", e))?;

    let mut stmt = conn
        .prepare("SELECT name, value FROM moz_cookies WHERE host LIKE '%rutracker%'")
        .map_err(|e| format!("Query prepare failed: {}", e))?;

    let rows = stmt
        .query_map([], |row| {
            let name: String = row.get(0)?;
            let value: String = row.get(1)?;
            Ok((name, value))
        })
        .map_err(|e| format!("Query failed: {}", e))?;

    let mut cookies = Vec::new();
    for row in rows.flatten() {
        cookies.push(format!("{}={}", row.0, row.1));
    }

    if cookies.is_empty() {
        Ok(None)
    } else {
        Ok(Some(cookies.join("; ")))
    }
}

/// Helper extracting cookies automatically from the default Firefox profile.
pub fn get_rutracker_cookie() -> Option<String> {
    let db = find_firefox_cookies_db()?;
    extract_rutracker_cookie_from_file(&db).ok().flatten()
}
