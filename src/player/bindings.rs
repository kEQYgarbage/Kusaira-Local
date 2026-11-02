//! Dynamic C-FFI loader for libvlc with hardware-acceleration parameters.

use libloading::Library;
use std::ffi::{c_char, c_int, c_void};
use std::sync::Arc;

pub struct VlcBindings {
    _lib: Arc<Library>,
    pub libvlc_new: unsafe extern "C" fn(argc: c_int, argv: *const *const c_char) -> *mut c_void,
    pub libvlc_release: unsafe extern "C" fn(p_instance: *mut c_void),
    pub libvlc_media_player_new: unsafe extern "C" fn(p_instance: *mut c_void) -> *mut c_void,
    pub libvlc_media_player_release: unsafe extern "C" fn(p_mi: *mut c_void),
    pub libvlc_media_new_path: unsafe extern "C" fn(p_instance: *mut c_void, path: *const c_char) -> *mut c_void,
    pub libvlc_media_new_location: unsafe extern "C" fn(p_instance: *mut c_void, psz_mrl: *const c_char) -> *mut c_void,
    pub libvlc_media_release: unsafe extern "C" fn(p_md: *mut c_void),
    pub libvlc_media_player_set_media: unsafe extern "C" fn(p_mi: *mut c_void, p_md: *mut c_void),
    pub libvlc_media_player_play: unsafe extern "C" fn(p_mi: *mut c_void) -> c_int,
    pub libvlc_media_player_pause: unsafe extern "C" fn(p_mi: *mut c_void),
    pub libvlc_media_player_stop: unsafe extern "C" fn(p_mi: *mut c_void),
    pub libvlc_media_player_is_playing: unsafe extern "C" fn(p_mi: *mut c_void) -> c_int,
    pub libvlc_media_player_get_time: unsafe extern "C" fn(p_mi: *mut c_void) -> i64,
    pub libvlc_media_player_set_time: unsafe extern "C" fn(p_mi: *mut c_void, i_time: i64),
    pub libvlc_media_player_get_length: unsafe extern "C" fn(p_mi: *mut c_void) -> i64,
    pub libvlc_audio_set_track: unsafe extern "C" fn(p_mi: *mut c_void, i_track: c_int) -> c_int,
    pub libvlc_audio_get_track: unsafe extern "C" fn(p_mi: *mut c_void) -> c_int,
    pub libvlc_video_set_spu: unsafe extern "C" fn(p_mi: *mut c_void, i_spu: c_int) -> c_int,
    pub libvlc_video_get_spu: unsafe extern "C" fn(p_mi: *mut c_void) -> c_int,
}

impl VlcBindings {
    /// Try loading libvlc from NixOS store path or standard system dynamic library paths.
    pub fn load() -> Result<Self, String> {
        let candidates = [
            "/nix/store/s5q6s4vj4cr9b0k0id189yay0ajhd4ia-vlc-3.0.24/lib/libvlc.so",
            "libvlc.so.5",
            "libvlc.so",
            "libvlc.dll",
        ];

        for &path in &candidates {
            unsafe {
                if let Ok(raw_lib) = Library::new(path) {
                    let lib = Arc::new(raw_lib);
                    let lib_hold = Arc::clone(&lib);
                    if let (
                        Ok(f_new),
                        Ok(f_rel),
                        Ok(f_mp_new),
                        Ok(f_mp_rel),
                        Ok(f_m_path),
                        Ok(f_m_loc),
                        Ok(f_m_rel),
                        Ok(f_mp_set_m),
                        Ok(f_play),
                        Ok(f_pause),
                        Ok(f_stop),
                        Ok(f_is_playing),
                        Ok(f_get_time),
                        Ok(f_set_time),
                        Ok(f_get_length),
                        Ok(f_set_track),
                        Ok(f_get_track),
                        Ok(f_set_spu),
                        Ok(f_get_spu),
                    ) = (
                        lib.get::<unsafe extern "C" fn(c_int, *const *const c_char) -> *mut c_void>(b"libvlc_new\0"),
                        lib.get::<unsafe extern "C" fn(*mut c_void)>(b"libvlc_release\0"),
                        lib.get::<unsafe extern "C" fn(*mut c_void) -> *mut c_void>(b"libvlc_media_player_new\0"),
                        lib.get::<unsafe extern "C" fn(*mut c_void)>(b"libvlc_media_player_release\0"),
                        lib.get::<unsafe extern "C" fn(*mut c_void, *const c_char) -> *mut c_void>(b"libvlc_media_new_path\0"),
                        lib.get::<unsafe extern "C" fn(*mut c_void, *const c_char) -> *mut c_void>(b"libvlc_media_new_location\0"),
                        lib.get::<unsafe extern "C" fn(*mut c_void)>(b"libvlc_media_release\0"),
                        lib.get::<unsafe extern "C" fn(*mut c_void, *mut c_void)>(b"libvlc_media_player_set_media\0"),
                        lib.get::<unsafe extern "C" fn(*mut c_void) -> c_int>(b"libvlc_media_player_play\0"),
                        lib.get::<unsafe extern "C" fn(*mut c_void)>(b"libvlc_media_player_pause\0"),
                        lib.get::<unsafe extern "C" fn(*mut c_void)>(b"libvlc_media_player_stop\0"),
                        lib.get::<unsafe extern "C" fn(*mut c_void) -> c_int>(b"libvlc_media_player_is_playing\0"),
                        lib.get::<unsafe extern "C" fn(*mut c_void) -> i64>(b"libvlc_media_player_get_time\0"),
                        lib.get::<unsafe extern "C" fn(*mut c_void, i64)>(b"libvlc_media_player_set_time\0"),
                        lib.get::<unsafe extern "C" fn(*mut c_void) -> i64>(b"libvlc_media_player_get_length\0"),
                        lib.get::<unsafe extern "C" fn(*mut c_void, c_int) -> c_int>(b"libvlc_audio_set_track\0"),
                        lib.get::<unsafe extern "C" fn(*mut c_void) -> c_int>(b"libvlc_audio_get_track\0"),
                        lib.get::<unsafe extern "C" fn(*mut c_void, c_int) -> c_int>(b"libvlc_video_set_spu\0"),
                        lib.get::<unsafe extern "C" fn(*mut c_void) -> c_int>(b"libvlc_video_get_spu\0"),
                    ) {
                        return Ok(Self {
                            _lib: lib_hold,
                            libvlc_new: *f_new,
                            libvlc_release: *f_rel,
                            libvlc_media_player_new: *f_mp_new,
                            libvlc_media_player_release: *f_mp_rel,
                            libvlc_media_new_path: *f_m_path,
                            libvlc_media_new_location: *f_m_loc,
                            libvlc_media_release: *f_m_rel,
                            libvlc_media_player_set_media: *f_mp_set_m,
                            libvlc_media_player_play: *f_play,
                            libvlc_media_player_pause: *f_pause,
                            libvlc_media_player_stop: *f_stop,
                            libvlc_media_player_is_playing: *f_is_playing,
                            libvlc_media_player_get_time: *f_get_time,
                            libvlc_media_player_set_time: *f_set_time,
                            libvlc_media_player_get_length: *f_get_length,
                            libvlc_audio_set_track: *f_set_track,
                            libvlc_audio_get_track: *f_get_track,
                            libvlc_video_set_spu: *f_set_spu,
                            libvlc_video_get_spu: *f_get_spu,
                        });
                    }
                }
            }
        }

        Err("Could not load libvlc dynamic library from candidate paths".to_string())
    }
}
