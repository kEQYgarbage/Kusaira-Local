//! Release data models and score structures.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScoredRelease {
    pub topic_id: String,
    pub title: String,
    pub pts: u32,
    pub resolution: String,
    pub codec: String,
    pub voiceovers: Vec<String>,
    pub size_bytes: u64,
    pub size_human: String,
    pub seeders: u32,
    pub is_bdrip: bool,
    pub reasons: Vec<String>,
}

impl ScoredRelease {
    pub fn new(
        topic_id: impl Into<String>,
        title: impl Into<String>,
        pts: u32,
        resolution: impl Into<String>,
        codec: impl Into<String>,
        voiceovers: Vec<String>,
        size_bytes: u64,
        seeders: u32,
        is_bdrip: bool,
        reasons: Vec<String>,
    ) -> Self {
        let size_human = format_size(size_bytes);
        Self {
            topic_id: topic_id.into(),
            title: title.into(),
            pts: pts.clamp(0, 100),
            resolution: resolution.into(),
            codec: codec.into(),
            voiceovers,
            size_bytes,
            size_human,
            seeders,
            is_bdrip,
            reasons,
        }
    }
}

pub fn format_size(bytes: u64) -> String {
    const GIB: u64 = 1024 * 1024 * 1024;
    const MIB: u64 = 1024 * 1024;
    if bytes >= GIB {
        format!("{:.1} ГБ", bytes as f64 / GIB as f64)
    } else if bytes >= MIB {
        format!("{:.0} МБ", bytes as f64 / MIB as f64)
    } else {
        format!("{} Б", bytes)
    }
}
