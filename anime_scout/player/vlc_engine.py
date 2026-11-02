"""High-level video player engine controlling embedded libvlc."""

from enum import Enum
import os
from typing import Callable

from anime_scout.player.vlc_bindings import LibVLC, TrackDescription


class PlaybackState(str, Enum):
    STOPPED = "stopped"
    PLAYING = "playing"
    PAUSED = "paused"


class VlcEngine:
    """Wrapper managing native libvlc player lifecycle and controls."""

    def __init__(self, xwindow_id: int | None = None):
        self.vlc = LibVLC()
        # Initialize libvlc with hardware acceleration and quiet logging
        args = [
            b"--no-video-title-show",
            b"--quiet",
            b"--avcodec-hw=any",
        ]
        import ctypes
        c_args = (ctypes.c_char_p * len(args))(*args)
        self.instance = self.vlc.dll.libvlc_new(len(args), c_args)
        if not self.instance:
            raise RuntimeError("Failed to create libvlc instance")

        self.player = self.vlc.dll.libvlc_media_player_new(self.instance)
        if not self.player:
            raise RuntimeError("Failed to create libvlc media player")

        self.current_media = None
        self.state = PlaybackState.STOPPED
        self.on_eof_callback: Callable[[], None] | None = None

        if xwindow_id:
            self.set_window(xwindow_id)

    def set_window(self, xwindow_id: int):
        """Attach video output to a native X11 / Wayland window ID."""
        self.vlc.dll.libvlc_media_player_set_xwindow(self.player, xwindow_id)

    def load_media(self, path_or_url: str):
        """Prepare media file or HTTP stream for playback."""
        if self.current_media:
            self.vlc.dll.libvlc_media_release(self.current_media)
            self.current_media = None

        if "://" in path_or_url:
            self.current_media = self.vlc.dll.libvlc_media_new_location(
                self.instance, path_or_url.encode("utf-8")
            )
        else:
            abs_path = os.path.abspath(path_or_url)
            self.current_media = self.vlc.dll.libvlc_media_new_path(
                self.instance, abs_path.encode("utf-8")
            )

        if not self.current_media:
            raise RuntimeError(f"Failed to load media: {path_or_url}")

        self.vlc.dll.libvlc_media_player_set_media(self.player, self.current_media)
        self.state = PlaybackState.STOPPED

    def play(self):
        """Start or resume playback."""
        if not self.player:
            return
        res = self.vlc.dll.libvlc_media_player_play(self.player)
        if res == 0:
            self.state = PlaybackState.PLAYING

    def pause(self):
        """Toggle play/pause state."""
        if not self.player or not self.current_media:
            return
        self.vlc.dll.libvlc_media_player_pause(self.player)
        self.state = PlaybackState.PAUSED if self.is_playing() else PlaybackState.PLAYING

    def stop(self):
        """Stop playback and reset position."""
        if not self.player:
            return
        self.vlc.dll.libvlc_media_player_stop(self.player)
        self.state = PlaybackState.STOPPED

    def is_playing(self) -> bool:
        """Check if media is actively playing."""
        if not self.player:
            return False
        return bool(self.vlc.dll.libvlc_media_player_is_playing(self.player))

    def get_time(self) -> float:
        """Return current playback position in seconds."""
        if not self.player:
            return 0.0
        ms = self.vlc.dll.libvlc_media_player_get_time(self.player)
        return max(0.0, ms / 1000.0)

    def set_time(self, seconds: float):
        """Seek to specified playback position in seconds."""
        if not self.player:
            return
        ms = int(max(0.0, seconds) * 1000)
        self.vlc.dll.libvlc_media_player_set_time(self.player, ms)

    def get_duration(self) -> float:
        """Return total duration in seconds, or 0 if unknown."""
        if not self.player:
            return 0.0
        ms = self.vlc.dll.libvlc_media_player_get_length(self.player)
        return max(0.0, ms / 1000.0)

    def get_audio_tracks(self) -> list[tuple[int, str]]:
        """Return list of available audio tracks: [(id, name), ...]."""
        if not self.player:
            return []
        desc = self.vlc.dll.libvlc_audio_get_track_description(self.player)
        if not desc:
            return []

        tracks: list[tuple[int, str]] = []
        curr = desc
        while curr:
            t_id = curr.contents.i_id
            t_name = curr.contents.psz_name.decode("utf-8", errors="replace") if curr.contents.psz_name else f"Track {t_id}"
            if t_id != -1:  # -1 is 'Disable'
                tracks.append((t_id, t_name))
            curr = curr.contents.p_next

        self.vlc.dll.libvlc_track_description_list_release(desc)
        return tracks

    def set_audio_track(self, track_id: int):
        """Switch active audio track by ID."""
        if self.player:
            self.vlc.dll.libvlc_audio_set_track(self.player, track_id)

    def get_subtitle_tracks(self) -> list[tuple[int, str]]:
        """Return list of available subtitle tracks."""
        if not self.player:
            return []
        desc = self.vlc.dll.libvlc_video_get_spu_description(self.player)
        if not desc:
            return []

        subs: list[tuple[int, str]] = []
        curr = desc
        while curr:
            s_id = curr.contents.i_id
            s_name = curr.contents.psz_name.decode("utf-8", errors="replace") if curr.contents.psz_name else f"Sub {s_id}"
            if s_id != -1:
                subs.append((s_id, s_name))
            curr = curr.contents.p_next

        self.vlc.dll.libvlc_track_description_list_release(desc)
        return subs

    def set_subtitle_track(self, sub_id: int):
        """Switch active subtitle track by ID."""
        if self.player:
            self.vlc.dll.libvlc_video_set_spu(self.player, sub_id)

    def release(self):
        """Release all allocated native libvlc resources."""
        if self.current_media:
            self.vlc.dll.libvlc_media_release(self.current_media)
            self.current_media = None
        if self.player:
            self.vlc.dll.libvlc_media_player_release(self.player)
            self.player = None
        if self.instance:
            self.vlc.dll.libvlc_release(self.instance)
            self.instance = None
