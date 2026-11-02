//! Torrent models, episode state machine, and filename parsers.

use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EpisodeBufferState {
    Pending,
    Buffering { progress: f32 },
    Ready { stream_url: String },
    Playing,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TorrentFile {
    pub index: usize,
    pub filename: String,
    pub size_bytes: u64,
    pub episode: Option<u32>,
    pub downloaded_bytes: u64,
}

/// Extract episode number from media filename (e.g. 'Mushoku_S2_01.mkv' -> 1).
pub fn parse_episode_number(filename: &str) -> Option<u32> {
    let patterns = [
        r"(?i)[sS]\d+[eE](\d{1,3})",
        r"(?i)[eE][pP]?\s*(\d{1,3})",
        r"\[(\d{1,3})\]",
        r"[-_\s](\d{1,3})\s*(?:\[|\.mkv|\.mp4|\.avi)",
        r"[-_\s](\d{1,3})[-_\s]",
    ];

    for pat in patterns {
        if let Ok(re) = Regex::new(pat) {
            if let Some(cap) = re.captures(filename) {
                if let Ok(num) = cap[1].parse::<u32>() {
                    if num < 1900 {
                        // Ignore years like 1080 or 2024
                        return Some(num);
                    }
                }
            }
        }
    }

    None
}
