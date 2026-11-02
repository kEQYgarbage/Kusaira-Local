//! Persistent JSON configuration store.

use crate::config::models::AppConfig;
use std::fs;
use std::path::PathBuf;

pub struct ConfigStore;

impl ConfigStore {
    pub fn config_path() -> PathBuf {
        if let Ok(home) = std::env::var("HOME") {
            PathBuf::from(home)
                .join(".config")
                .join("kusaira")
                .join("config.json")
        } else {
            PathBuf::from("kusaira_config.json")
        }
    }

    pub fn load() -> AppConfig {
        let path = Self::config_path();
        if path.exists() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(cfg) = serde_json::from_str::<AppConfig>(&content) {
                    return cfg;
                }
            }
        }
        AppConfig::default()
    }

    pub fn save(cfg: &AppConfig) -> Result<(), String> {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let json = serde_json::to_string_pretty(cfg)
            .map_err(|e| format!("Serialization error: {}", e))?;
        fs::write(&path, json).map_err(|e| format!("Write failed: {}", e))?;
        Ok(())
    }
}
