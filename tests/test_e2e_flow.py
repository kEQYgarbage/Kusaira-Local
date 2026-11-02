"""End-to-end integration test for the full Kusaira Desktop user workflow."""

import unittest

from anime_scout.core.scorer_service import AnimeReleaseService
from anime_scout.player.vlc_engine import VlcEngine, PlaybackState
from anime_scout.torrent.downloader import LocalTorrentManager
from anime_scout.torrent.pipeline import TorrentBingePipeline


class TestE2EFlow(unittest.TestCase):
    def setUp(self):
        self.scorer = AnimeReleaseService()
        self.downloader = LocalTorrentManager("/tmp/anime_e2e")
        self.pipeline = TorrentBingePipeline(self.downloader)
        self.vlc = VlcEngine()

    def tearDown(self):
        self.vlc.release()

    def test_full_search_score_buffer_play_flow(self):
        # 1. User searches anime and gets graded releases with PTS
        release = self.scorer.grade_release(
            title="[BDRip 1080p] Mushoku Tensei S2 [HEVC 10-bit FLAC] [Studio Band, Dream Cast, RAW]",
            seeders=38,
            size_bytes=14 * 1024**3,
            topic_id="55888",
        )
        self.assertGreaterEqual(release.pts, 85)
        self.assertIn("Studio Band", " ".join(release.voiceovers))

        # 2. Add torrent to download manager
        from anime_scout.torrent.downloader import FileStatus
        tid = self.downloader.add_torrent(b"mock_torrent_bytes", "/tmp/anime_e2e", "Mushoku S2")
        status = self.downloader.get_status(tid)
        status.files = [
            FileStatus(0, "Mushoku_Tensei_S2_01.mkv", 1024**3),
            FileStatus(1, "Mushoku_Tensei_S2_02.mkv", 1024**3),
        ]
        self.pipeline.attach_torrent(tid)

        # 3. User clicks "Watch Episode 1"
        self.pipeline.start_episode(1)
        self.assertEqual(self.pipeline.active_episode, 1)

        # 4. Simulate playback progress reaching 50%
        self.pipeline.on_playback_progress(1, current_sec=600, duration_sec=1200)

        # 5. Verify Episode 2 pre-fetching is triggered in background
        self.assertEqual(self.pipeline.prefetched_episode, 2)


if __name__ == "__main__":
    unittest.main()
