//! Tests for RuTracker parsing and Firefox cookie extraction.

use kusaira::torrent::{extract_rutracker_cookie_from_file, get_curated_releases_for_title, parse_rutracker_html};
use rusqlite::Connection;
use tempfile::NamedTempFile;

#[test]
fn test_cookie_extraction_from_sqlite() {
    let temp_db = NamedTempFile::new().expect("Create temp db");
    let conn = Connection::open(temp_db.path()).expect("Open temp db");

    conn.execute(
        "CREATE TABLE moz_cookies (id INTEGER PRIMARY KEY, host TEXT, name TEXT, value TEXT)",
        [],
    ).expect("Create table");

    conn.execute(
        "INSERT INTO moz_cookies (host, name, value) VALUES ('.rutracker.org', 'bb_session', 'test_session_123')",
        [],
    ).expect("Insert session");
    conn.execute(
        "INSERT INTO moz_cookies (host, name, value) VALUES ('.rutracker.org', 'bb_data', 'test_data_456')",
        [],
    ).expect("Insert data");

    let result = extract_rutracker_cookie_from_file(temp_db.path())
        .expect("Extraction should succeed")
        .expect("Cookie string should exist");

    assert!(result.contains("bb_session=test_session_123"));
    assert!(result.contains("bb_data=test_data_456"));
}

#[test]
fn test_parse_rutracker_html_table() {
    let html = r#"
        <tr class="tCenter hl-tr">
            <td class="t-title">
                <a href="viewtopic.php?t=5588801"><b>[BDRip 1080p HEVC FLAC] Sousou no Frieren [Studio Band]</b></a>
            </td>
        </tr>
        <tr class="tCenter hl-tr">
            <td class="t-title">
                <a href="viewtopic.php?t=5588802">Sousou no Frieren 720p HDTV [AniDUB]</a>
            </td>
        </tr>
    "#;

    let releases = parse_rutracker_html(html);
    assert_eq!(releases.len(), 2);
    // Highest PTS (BDRip 1080p HEVC) should be ranked first
    assert_eq!(releases[0].topic_id, "5588801");
    assert!(releases[0].pts > releases[1].pts);
}

#[test]
fn test_curated_releases_for_title() {
    let list = get_curated_releases_for_title("Mushoku Tensei");
    assert_eq!(list.len(), 3);
    assert!(list[0].pts >= 90);
    assert!(list[0].title.contains("Mushoku Tensei"));
}
