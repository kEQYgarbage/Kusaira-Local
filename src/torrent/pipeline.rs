//! Binge-watching sequential download pipeline and background episode buffer manager.

use crate::torrent::models::{EpisodeBufferState, TorrentFile};
use std::collections::HashMap;

pub struct BingePipeline {
    pub active_episode: Option<u32>,
    pub prefetched_episode: Option<u32>,
    pub episode_states: HashMap<u32, EpisodeBufferState>,
    pub episode_files: HashMap<u32, TorrentFile>,
}

impl Default for BingePipeline {
    fn default() -> Self {
        Self::new()
    }
}

impl BingePipeline {
    pub fn new() -> Self {
        Self {
            active_episode: None,
            prefetched_episode: None,
            episode_states: HashMap::new(),
            episode_files: HashMap::new(),
        }
    }

    /// Register torrent files into episode mapping.
    pub fn register_file(&mut self, file: TorrentFile) {
        if let Some(ep) = file.episode {
            self.episode_files.insert(ep, file);
            self.episode_states.entry(ep).or_insert(EpisodeBufferState::Pending);
        }
    }

    /// Mark episode as ready to play with given stream URL.
    pub fn set_episode_ready(&mut self, episode: u32, stream_url: &str) {
        self.episode_states.insert(
            episode,
            EpisodeBufferState::Ready {
                stream_url: stream_url.to_string(),
            },
        );
    }

    /// Start watching an episode.
    pub fn play_episode(&mut self, episode: u32) {
        self.active_episode = Some(episode);
        self.episode_states.insert(episode, EpisodeBufferState::Playing);
    }

    /// Handle playback progress update.
    ///
    /// When the current episode reaches >= 40% playback, triggers background pre-buffering
    /// for Episode N+1 if not already triggered.
    pub fn on_playback_progress(&mut self, current_ep: u32, progress_ratio: f32) -> Option<u32> {
        if progress_ratio >= 0.40 {
            let next_ep = current_ep + 1;
            if self.prefetched_episode != Some(next_ep) {
                self.prefetched_episode = Some(next_ep);
                self.episode_states.entry(next_ep).or_insert(EpisodeBufferState::Buffering { progress: 0.05 });
                return Some(next_ep);
            }
        }
        None
    }

    /// Check if episode is ready for instant playback.
    pub fn is_episode_ready(&self, episode: u32) -> bool {
        matches!(
            self.episode_states.get(&episode),
            Some(EpisodeBufferState::Ready { .. }) | Some(EpisodeBufferState::Playing)
        )
    }
}
