"""Binge-watching sequential download pipeline and episode buffer manager."""

from enum import Enum
import re
from typing import Callable

from anime_scout.torrent.downloader import BaseTorrentDownloader, FileStatus, TorrentStatus


class EpisodeBufferState(str, Enum):
    PENDING = "pending"
    BUFFERING = "buffering"
    READY = "ready"
    PLAYING = "playing"


def parse_episode_number(filename: str) -> int | None:
    """Extract episode number from filename (e.g. 'Mushoku_S2_01.mkv' -> 1)."""
    # 1. Pattern like - 01 or _01 or E01 or [01]
    patterns = [
        r"[sS]\d+[eE](\d{1,3})",
        r"[eE][pP]?\s*(\d{1,3})",
        r"[\[\(](\d{1,3})[\]\)]",
        r"[-_\s](\d{1,3})\s*(?:\[|\.mkv|\.mp4)",
        r"[-_\s](\d{1,3})[-_\s]",
    ]
    for pat in patterns:
        m = re.search(pat, filename)
        if m:
            num = int(m.group(1))
            if num < 1900:  # avoid years
                return num
    return None


class TorrentBingePipeline:
    """Orchestrates sequential piece prioritization and background pre-fetching."""

    def __init__(self, downloader: BaseTorrentDownloader):
        self.downloader = downloader
        self.current_torrent_id: str | None = None
        self.active_episode: int | None = None
        self.prefetched_episode: int | None = None
        self.episode_files: dict[int, FileStatus] = {}
        self.episode_states: dict[int, EpisodeBufferState] = {}
        self.on_ready_callback: Callable[[int, str], None] | None = None

    def attach_torrent(self, torrent_id: str):
        """Bind pipeline to an active torrent and map files to episodes."""
        self.current_torrent_id = torrent_id
        self._map_episodes()

    def _map_episodes(self):
        if not self.current_torrent_id:
            return
        status = self.downloader.get_status(self.current_torrent_id)
        if not status or not status.files:
            return

        ep_map: dict[int, FileStatus] = {}
        for f in status.files:
            ep_num = parse_episode_number(f.name)
            if ep_num is not None:
                ep_map[ep_num] = f
                self.episode_states[ep_num] = EpisodeBufferState.PENDING

        self.episode_files = ep_map

    def get_episode_files(self) -> dict[int, FileStatus]:
        """Return mapped episodes dictionary {ep_num: FileStatus}."""
        if not self.episode_files:
            self._map_episodes()
        return self.episode_files

    def start_episode(self, ep_num: int):
        """Set episode as active and prioritize its pieces for immediate streaming."""
        if not self.current_torrent_id:
            return
        self.active_episode = ep_num
        self.episode_states[ep_num] = EpisodeBufferState.BUFFERING

        ep_file = self.episode_files.get(ep_num)
        if ep_file:
            self.downloader.set_file_priority(self.current_torrent_id, ep_file.index, "highest")

    def on_playback_progress(self, ep_num: int, current_sec: float, duration_sec: float):
        """Check playback milestone and trigger background pre-fetch for next episode."""
        if not self.current_torrent_id or duration_sec <= 0:
            return

        ratio = current_sec / duration_sec
        # Trigger prefetch when user passes 40% of current episode
        if ratio >= 0.40:
            next_ep = ep_num + 1
            if next_ep in self.episode_files and self.prefetched_episode != next_ep:
                self.prefetched_episode = next_ep
                self.episode_states[next_ep] = EpisodeBufferState.BUFFERING
                next_file = self.episode_files[next_ep]
                self.downloader.set_file_priority(self.current_torrent_id, next_file.index, "high")

    def on_playback_finished(self, ep_num: int):
        """Advance pipeline to next episode upon completion."""
        next_ep = ep_num + 1
        if next_ep in self.episode_files:
            self.start_episode(next_ep)
