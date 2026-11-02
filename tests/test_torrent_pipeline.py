import unittest
from unittest.mock import MagicMock

from anime_scout.torrent.downloader import FileStatus, TorrentStatus, BaseTorrentDownloader
from anime_scout.torrent.pipeline import TorrentBingePipeline, EpisodeBufferState


class MockDownloader(BaseTorrentDownloader):
    """Mock downloader for testing pipeline logic without network I/O."""

    def __init__(self):
        self.torrents = {}
        self.priorities = {}

    def add_torrent(self, torrent_data: bytes, save_path: str, name: str) -> str:
        tid = "mock_t1"
        self.torrents[tid] = TorrentStatus(
            torrent_id=tid,
            name=name,
            total_bytes=10 * 1024**3,
            downloaded_bytes=0,
            download_speed=15 * 1024**2,
            progress=0.0,
            files=[
                FileStatus(0, "Mushoku_Tensei_S2_01.mkv", 1024**3, 0, 0.0),
                FileStatus(1, "Mushoku_Tensei_S2_02.mkv", 1024**3, 0, 0.0),
                FileStatus(2, "Mushoku_Tensei_S2_03.mkv", 1024**3, 0, 0.0),
            ],
        )
        return tid

    def set_file_priority(self, torrent_id: str, file_index: int, priority: str):
        self.priorities[(torrent_id, file_index)] = priority

    def get_status(self, torrent_id: str) -> TorrentStatus | None:
        return self.torrents.get(torrent_id)


class TestTorrentPipeline(unittest.TestCase):
    def setUp(self):
        self.downloader = MockDownloader()
        self.pipeline = TorrentBingePipeline(self.downloader)
        self.tid = self.downloader.add_torrent(b"dummy_torrent", "/tmp/anime", "Mushoku Tensei II")
        self.pipeline.attach_torrent(self.tid)

    def test_episode_file_mapping(self):
        ep_map = self.pipeline.get_episode_files()
        self.assertEqual(len(ep_map), 3)
        self.assertEqual(ep_map[1].name, "Mushoku_Tensei_S2_01.mkv")
        self.assertEqual(ep_map[2].name, "Mushoku_Tensei_S2_02.mkv")

    def test_start_episode_buffering(self):
        # Select episode 1 to watch
        self.pipeline.start_episode(1)
        self.assertEqual(self.pipeline.active_episode, 1)
        # Episode 1 should be set to highest priority
        self.assertEqual(self.downloader.priorities.get((self.tid, 0)), "highest")

    def test_binge_prefetch_trigger_at_midpoint(self):
        self.pipeline.start_episode(1)
        # Simulate playback at 10% (should not trigger prefetch yet)
        self.pipeline.on_playback_progress(1, current_sec=120, duration_sec=1200)
        self.assertNotEqual(self.downloader.priorities.get((self.tid, 1)), "high")

        # Simulate playback reaching 45% (should trigger prefetch of Episode 2)
        self.pipeline.on_playback_progress(1, current_sec=540, duration_sec=1200)
        self.assertEqual(self.downloader.priorities.get((self.tid, 1)), "high")


if __name__ == "__main__":
    unittest.main()
