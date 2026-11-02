//! Configuration models for Kusaira Local Anime.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppConfig {
    pub theme: String,             // "cyan", "neon", "emerald", "oled", "crimson"
    pub grid_density: String,      // "compact", "standard", "large"
    pub voiceover_priority: String,// "Studio Band", "AniLibria", etc.
    pub quality_preference: String,// "1080p", "4K", "720p"
    pub hw_accel: String,          // "vaapi", "auto", "none"
    pub opening_skip_mode: String, // "auto", "tab", "space", "off"
    pub skip_endings: bool,
    pub seek_step_sec: u32,
    pub autoplay_next: bool,
    pub resume_playback: bool,
    pub gc_max_disk_gb: u32,       // 5, 10, 20, 50, 0 (unlimited)
    pub gc_policy: String,         // "immediate", "keep_3", "ttl_3d", "manual"
    pub gc_on_exit: bool,
    pub discord_rpc_enabled: bool,
    pub proxy_enabled: bool,
    pub proxy_addr: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme: "cyan".to_string(),
            grid_density: "standard".to_string(),
            voiceover_priority: "Studio Band".to_string(),
            quality_preference: "1080p".to_string(),
            hw_accel: "vaapi".to_string(),
            opening_skip_mode: "tab".to_string(),
            skip_endings: false,
            seek_step_sec: 10,
            autoplay_next: true,
            resume_playback: true,
            gc_max_disk_gb: 20,
            gc_policy: "keep_3".to_string(),
            gc_on_exit: true,
            discord_rpc_enabled: true,
            proxy_enabled: false,
            proxy_addr: "".to_string(),
        }
    }
}
