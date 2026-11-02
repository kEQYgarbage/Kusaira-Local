//! High-level video player engine controlling embedded libvlc with VA-API.

use crate::player::bindings::VlcBindings;
use std::ffi::{c_char, CString};
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaybackState {
    Stopped,
    Playing,
    Paused,
}

pub struct PlayerEngine {
    bindings: Arc<VlcBindings>,
    instance: *mut std::ffi::c_void,
    player: *mut std::ffi::c_void,
    current_media: *mut std::ffi::c_void,
    state: PlaybackState,
}

// PlayerEngine manages raw pointers through C-FFI
unsafe impl Send for PlayerEngine {}
unsafe impl Sync for PlayerEngine {}

impl PlayerEngine {
    pub fn new() -> Result<Self, String> {
        let bindings = Arc::new(VlcBindings::load()?);

        // Hardware-accelerated decoding arguments (VA-API on Linux, D3D11 on Windows)
        let raw_args = [
            "--no-video-title-show",
            "--quiet",
            #[cfg(target_os = "windows")]
            "--avcodec-hw=d3d11va",
            #[cfg(not(target_os = "windows"))]
            "--avcodec-hw=vaapi",
        ];

        let c_strings: Vec<CString> = raw_args.iter().map(|&s| CString::new(s).unwrap()).collect();
        let c_ptrs: Vec<*const c_char> = c_strings.iter().map(|cs| cs.as_ptr()).collect();

        unsafe {
            let instance = (bindings.libvlc_new)(c_ptrs.len() as i32, c_ptrs.as_ptr());
            if instance.is_null() {
                return Err("libvlc_new returned NULL".to_string());
            }

            let player = (bindings.libvlc_media_player_new)(instance);
            if player.is_null() {
                (bindings.libvlc_release)(instance);
                return Err("libvlc_media_player_new returned NULL".to_string());
            }

            Ok(Self {
                bindings,
                instance,
                player,
                current_media: std::ptr::null_mut(),
                state: PlaybackState::Stopped,
            })
        }
    }

    /// Load and start playback of a local file or loopback HTTP stream.
    pub fn play(&mut self, path_or_url: &str) -> Result<(), String> {
        unsafe {
            if !self.current_media.is_null() {
                (self.bindings.libvlc_media_release)(self.current_media);
                self.current_media = std::ptr::null_mut();
            }

            let c_target = CString::new(path_or_url).map_err(|e| e.to_string())?;
            let media = if path_or_url.contains("://") {
                (self.bindings.libvlc_media_new_location)(self.instance, c_target.as_ptr())
            } else {
                (self.bindings.libvlc_media_new_path)(self.instance, c_target.as_ptr())
            };

            if media.is_null() {
                return Err("Failed to create libvlc media".to_string());
            }

            self.current_media = media;
            (self.bindings.libvlc_media_player_set_media)(self.player, media);
            (self.bindings.libvlc_media_player_play)(self.player);
            self.state = PlaybackState::Playing;
            Ok(())
        }
    }

    pub fn pause(&mut self) {
        unsafe {
            (self.bindings.libvlc_media_player_pause)(self.player);
            self.state = PlaybackState::Paused;
        }
    }

    pub fn toggle_pause(&mut self) {
        match self.state {
            PlaybackState::Playing => self.pause(),
            PlaybackState::Paused => {
                unsafe {
                    (self.bindings.libvlc_media_player_play)(self.player);
                    self.state = PlaybackState::Playing;
                }
            }
            PlaybackState::Stopped => {}
        }
    }

    pub fn stop(&mut self) {
        unsafe {
            (self.bindings.libvlc_media_player_stop)(self.player);
            self.state = PlaybackState::Stopped;
        }
    }

    pub fn seek(&mut self, time_ms: i64) {
        unsafe {
            (self.bindings.libvlc_media_player_set_time)(self.player, time_ms.max(0));
        }
    }

    pub fn get_time(&self) -> i64 {
        unsafe { (self.bindings.libvlc_media_player_get_time)(self.player) }
    }

    pub fn get_duration(&self) -> i64 {
        unsafe { (self.bindings.libvlc_media_player_get_length)(self.player) }
    }

    /// Set audio track and seamlessly preserve playback timestamp to avoid resetting to 0.
    pub fn set_audio_track(&mut self, track_id: i32) {
        let current_pos = self.get_time();
        unsafe {
            (self.bindings.libvlc_audio_set_track)(self.player, track_id);
            if current_pos > 0 {
                (self.bindings.libvlc_media_player_set_time)(self.player, current_pos);
            }
        }
    }

    pub fn set_subtitle_track(&mut self, spu_id: i32) {
        unsafe {
            (self.bindings.libvlc_video_set_spu)(self.player, spu_id);
        }
    }

    pub fn state(&self) -> PlaybackState {
        self.state
    }
}

impl Drop for PlayerEngine {
    fn drop(&mut self) {
        unsafe {
            if !self.current_media.is_null() {
                (self.bindings.libvlc_media_release)(self.current_media);
            }
            if !self.player.is_null() {
                (self.bindings.libvlc_media_player_stop)(self.player);
                (self.bindings.libvlc_media_player_release)(self.player);
            }
            if !self.instance.is_null() {
                (self.bindings.libvlc_release)(self.instance);
            }
        }
    }
}
