"""Torrent downloading interfaces and status models."""

from abc import ABC, abstractmethod
from dataclasses import dataclass, field
import os


@dataclass
class FileStatus:
    """Status of an individual file within a torrent."""

    index: int
    name: str
    size: int
    downloaded: int = 0
    progress: float = 0.0

    @property
    def is_complete(self) -> bool:
        return self.progress >= 0.999


@dataclass
class TorrentStatus:
    """Live status of an active torrent download."""

    torrent_id: str
    name: str
    total_bytes: int
    downloaded_bytes: int = 0
    download_speed: int = 0
    progress: float = 0.0
    files: list[FileStatus] = field(default_factory=list)


class BaseTorrentDownloader(ABC):
    """Abstract interface for torrent download clients."""

    @abstractmethod
    def add_torrent(self, torrent_data: bytes, save_path: str, name: str) -> str:
        """Add torrent and return unique torrent_id."""
        pass

    @abstractmethod
    def set_file_priority(self, torrent_id: str, file_index: int, priority: str):
        """Set file priority ('highest', 'high', 'normal', 'off')."""
        pass

    @abstractmethod
    def get_status(self, torrent_id: str) -> TorrentStatus | None:
        """Fetch current torrent download progress and speeds."""
        pass


class LocalTorrentManager(BaseTorrentDownloader):
    """Local manager holding torrent download state and file mappings."""

    def __init__(self, default_save_dir: str | None = None):
        self.save_dir = default_save_dir or os.path.expanduser("~/Videos/Anime")
        os.makedirs(self.save_dir, exist_ok=True)
        self.active_torrents: dict[str, TorrentStatus] = {}
        self.file_priorities: dict[tuple[str, int], str] = {}

    def add_torrent(self, torrent_data: bytes, save_path: str, name: str) -> str:
        import hashlib
        tid = hashlib.sha1(torrent_data[:200] if torrent_data else name.encode()).hexdigest()[:12]
        
        status = TorrentStatus(
            torrent_id=tid,
            name=name,
            total_bytes=0,
            downloaded_bytes=0,
            download_speed=0,
            progress=0.0,
            files=[],
        )
        self.active_torrents[tid] = status
        return tid

    def set_file_priority(self, torrent_id: str, file_index: int, priority: str):
        self.file_priorities[(torrent_id, file_index)] = priority

    def get_status(self, torrent_id: str) -> TorrentStatus | None:
        return self.active_torrents.get(torrent_id)
