//! Async client for Shikimori API (ongoing, ranked, similar recommendations, search).

use crate::catalog::models::AnimeCard;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct ShikimoriClient {
    client: reqwest::Client,
    base_url: String,
}

impl Default for ShikimoriClient {
    fn default() -> Self {
        Self::new("https://shikimori.io")
    }
}

impl ShikimoriClient {
    pub fn new(base_url: &str) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .user_agent("KusairaLocalAnime/0.1.0")
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());

        Self {
            client,
            base_url: base_url.trim_end_matches('/').to_string(),
        }
    }

    /// Fetch currently airing popular ongoing anime for the seasonal trending shelf.
    pub async fn fetch_ongoing(&self, limit: usize) -> Result<Vec<AnimeCard>, String> {
        let url = format!(
            "{}/api/animes?status=ongoing&order=popularity&limit={}",
            self.base_url, limit
        );
        self.request_cards(&url).await
    }

    /// Fetch highest rated anime classics for the Gold Collection shelf.
    pub async fn fetch_ranked(&self, limit: usize) -> Result<Vec<AnimeCard>, String> {
        let url = format!(
            "{}/api/animes?order=ranked&limit={}",
            self.base_url, limit
        );
        self.request_cards(&url).await
    }

    /// Fetch smart recommendations similar to a given anime ID.
    pub async fn fetch_similar(&self, anime_id: u64, limit: usize) -> Result<Vec<AnimeCard>, String> {
        let url = format!(
            "{}/api/animes/{}/similar?limit={}",
            self.base_url, anime_id, limit
        );
        self.request_cards(&url).await
    }

    /// Fetch related franchise anime (sequels, prequels, spin-offs) for an anime ID.
    pub async fn fetch_related(&self, anime_id: u64) -> Result<Vec<crate::catalog::models::RelatedItem>, String> {
        let url = format!("{}/api/animes/{}/related", self.base_url, anime_id);
        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Network request failed: {}", e))?;

        if !resp.status().is_success() {
            return Err(format!("Shikimori API returned status {}", resp.status()));
        }

        let items = resp
            .json::<Vec<crate::catalog::models::RelatedItem>>()
            .await
            .map_err(|e| format!("Failed to parse Shikimori related JSON: {}", e))?;

        Ok(items)
    }

    /// Search anime by title in Russian or English.
    pub async fn search_anime(&self, query: &str, limit: usize) -> Result<Vec<AnimeCard>, String> {
        let encoded = urlencoding::encode(query);
        let url = format!(
            "{}/api/animes?search={}&limit={}",
            self.base_url, encoded, limit
        );
        self.request_cards(&url).await
    }

    async fn request_cards(&self, url: &str) -> Result<Vec<AnimeCard>, String> {
        let resp = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|e| format!("Network request failed: {}", e))?;

        if !resp.status().is_success() {
            return Err(format!("Shikimori API returned status {}", resp.status()));
        }

        let cards = resp
            .json::<Vec<AnimeCard>>()
            .await
            .map_err(|e| format!("Failed to parse Shikimori JSON: {}", e))?;

        Ok(cards)
    }
}

mod urlencoding {
    pub fn encode(s: &str) -> String {
        let mut result = String::with_capacity(s.len());
        for b in s.bytes() {
            match b {
                b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                    result.push(b as char);
                }
                _ => {
                    result.push_str(&format!("%{:02X}", b));
                }
            }
        }
        result
    }
}
