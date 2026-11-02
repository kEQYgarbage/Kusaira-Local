//! Torrent download, streaming pipeline, and RuTracker discovery modules.

pub mod cookies;
pub mod rutracker;

pub use cookies::{extract_rutracker_cookie_from_file, find_firefox_cookies_db, get_rutracker_cookie};
pub use rutracker::{get_curated_releases_for_title, parse_rutracker_html};
