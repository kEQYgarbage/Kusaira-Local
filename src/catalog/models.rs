//! Shikimori API data models and serialization.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ShikimoriImage {
    pub original: Option<String>,
    pub preview: Option<String>,
    pub x96: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AnimeCard {
    pub id: u64,
    pub name: String,
    pub russian: Option<String>,
    pub image: Option<ShikimoriImage>,
    pub score: Option<String>,
    pub status: Option<String>,
    pub episodes: Option<u32>,
    pub episodes_aired: Option<u32>,
    pub kind: Option<String>,
}

impl AnimeCard {
    /// Return best human-readable Russian display title, falling back to original Romaji.
    pub fn display_title(&self) -> &str {
        if let Some(ref r) = self.russian {
            if !r.trim().is_empty() {
                return r.as_str();
            }
        }
        &self.name
    }

    /// Return absolute HTTPS URL for the high-resolution poster.
    pub fn poster_url(&self) -> String {
        let base = "https://shikimori.one";
        if let Some(ref img) = self.image {
            if let Some(ref orig) = img.original {
                if orig.starts_with("http") {
                    return orig.clone();
                }
                return format!("{}{}", base, orig);
            }
            if let Some(ref prev) = img.preview {
                if prev.starts_with("http") {
                    return prev.clone();
                }
                return format!("{}{}", base, prev);
            }
        }
        format!("https://placehold.co/300x420/1f232b/fff?text={}", urlencoding(&self.name))
    }

    /// Return parsed score as f32, default 0.0.
    pub fn score_f32(&self) -> f32 {
        self.score
            .as_ref()
            .and_then(|s| s.parse::<f32>().ok())
            .unwrap_or(0.0)
    }
}

fn urlencoding(s: &str) -> String {
    s.chars()
        .map(|c| if c.is_alphanumeric() { c.to_string() } else { "+".to_string() })
        .collect()
}
