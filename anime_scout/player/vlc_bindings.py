"""ctypes bindings to native libvlc on Linux."""

import ctypes
import ctypes.util
import glob
import os


class TrackDescription(ctypes.Structure):
    """Linked list of media tracks returned by libvlc."""

    pass


TrackDescription._fields_ = [
    ("i_id", ctypes.c_int),
    ("psz_name", ctypes.c_char_p),
    ("p_next", ctypes.POINTER(TrackDescription)),
]


def find_libvlc() -> str | None:
    """Find native libvlc.so on Linux/NixOS."""
    # 1. Search Nix store / system paths
    nix_candidates = glob.glob("/nix/store/*-vlc-*/lib/libvlc.so")
    if nix_candidates:
        return sorted(nix_candidates, reverse=True)[0]

    common_paths = [
        "/run/current-system/sw/lib/libvlc.so",
        "/usr/lib/libvlc.so",
        "/usr/lib64/libvlc.so",
        "/usr/lib/x86_64-linux-gnu/libvlc.so",
    ]
    for p in common_paths:
        if os.path.exists(p):
            return p

    # 2. find_library fallback
    lib = ctypes.util.find_library("vlc")
    return lib


class LibVLC:
    """Interface to native libvlc functions."""

    def __init__(self, lib_path: str | None = None):
        path = lib_path or find_libvlc()
        if not path:
            raise RuntimeError("libvlc.so not found on system. Please ensure VLC is installed.")
        self.lib_path = path
        self.dll = ctypes.CDLL(path)
        self._bind_functions()

    def _bind_functions(self):
        # Version
        self.dll.libvlc_get_version.restype = ctypes.c_char_p
        self.dll.libvlc_get_version.argtypes = []

        # Instance
        self.dll.libvlc_new.restype = ctypes.c_void_p
        self.dll.libvlc_new.argtypes = [ctypes.c_int, ctypes.POINTER(ctypes.c_char_p)]

        self.dll.libvlc_release.restype = None
        self.dll.libvlc_release.argtypes = [ctypes.c_void_p]

        # Media Player
        self.dll.libvlc_media_player_new.restype = ctypes.c_void_p
        self.dll.libvlc_media_player_new.argtypes = [ctypes.c_void_p]

        self.dll.libvlc_media_player_release.restype = None
        self.dll.libvlc_media_player_release.argtypes = [ctypes.c_void_p]

        # Media creation
        self.dll.libvlc_media_new_path.restype = ctypes.c_void_p
        self.dll.libvlc_media_new_path.argtypes = [ctypes.c_void_p, ctypes.c_char_p]

        self.dll.libvlc_media_new_location.restype = ctypes.c_void_p
        self.dll.libvlc_media_new_location.argtypes = [ctypes.c_void_p, ctypes.c_char_p]

        self.dll.libvlc_media_release.restype = None
        self.dll.libvlc_media_release.argtypes = [ctypes.c_void_p]

        self.dll.libvlc_media_player_set_media.restype = None
        self.dll.libvlc_media_player_set_media.argtypes = [ctypes.c_void_p, ctypes.c_void_p]

        # Playback controls
        self.dll.libvlc_media_player_play.restype = ctypes.c_int
        self.dll.libvlc_media_player_play.argtypes = [ctypes.c_void_p]

        self.dll.libvlc_media_player_pause.restype = None
        self.dll.libvlc_media_player_pause.argtypes = [ctypes.c_void_p]

        self.dll.libvlc_media_player_stop.restype = None
        self.dll.libvlc_media_player_stop.argtypes = [ctypes.c_void_p]

        self.dll.libvlc_media_player_is_playing.restype = ctypes.c_int
        self.dll.libvlc_media_player_is_playing.argtypes = [ctypes.c_void_p]

        # Time & Length
        self.dll.libvlc_media_player_get_time.restype = ctypes.c_int64
        self.dll.libvlc_media_player_get_time.argtypes = [ctypes.c_void_p]

        self.dll.libvlc_media_player_set_time.restype = None
        self.dll.libvlc_media_player_set_time.argtypes = [ctypes.c_void_p, ctypes.c_int64]

        self.dll.libvlc_media_player_get_length.restype = ctypes.c_int64
        self.dll.libvlc_media_player_get_length.argtypes = [ctypes.c_void_p]

        # Window integration
        self.dll.libvlc_media_player_set_xwindow.restype = None
        self.dll.libvlc_media_player_set_xwindow.argtypes = [ctypes.c_void_p, ctypes.c_uint32]

        # Audio tracks
        self.dll.libvlc_audio_get_track_count.restype = ctypes.c_int
        self.dll.libvlc_audio_get_track_count.argtypes = [ctypes.c_void_p]

        self.dll.libvlc_audio_get_track.restype = ctypes.c_int
        self.dll.libvlc_audio_get_track.argtypes = [ctypes.c_void_p]

        self.dll.libvlc_audio_set_track.restype = ctypes.c_int
        self.dll.libvlc_audio_set_track.argtypes = [ctypes.c_void_p, ctypes.c_int]

        self.dll.libvlc_audio_get_track_description.restype = ctypes.POINTER(TrackDescription)
        self.dll.libvlc_audio_get_track_description.argtypes = [ctypes.c_void_p]

        # Subtitle tracks
        self.dll.libvlc_video_get_spu_count.restype = ctypes.c_int
        self.dll.libvlc_video_get_spu_count.argtypes = [ctypes.c_void_p]

        self.dll.libvlc_video_get_spu.restype = ctypes.c_int
        self.dll.libvlc_video_get_spu.argtypes = [ctypes.c_void_p]

        self.dll.libvlc_video_set_spu.restype = ctypes.c_int
        self.dll.libvlc_video_set_spu.argtypes = [ctypes.c_void_p, ctypes.c_int]

        self.dll.libvlc_video_get_spu_description.restype = ctypes.POINTER(TrackDescription)
        self.dll.libvlc_video_get_spu_description.argtypes = [ctypes.c_void_p]

        self.dll.libvlc_track_description_list_release.restype = None
        self.dll.libvlc_track_description_list_release.argtypes = [ctypes.POINTER(TrackDescription)]

    def get_version(self) -> str:
        raw = self.dll.libvlc_get_version()
        return raw.decode("utf-8") if raw else ""
