import unittest
from unittest.mock import AsyncMock, patch

from anime_scout.core.cookie_extractor import get_firefox_rutracker_cookie
from anime_scout.core.scorer_service import AnimeReleaseService, ScoredRelease


class TestCookieExtractor(unittest.TestCase):
    def test_extract_cookie_from_firefox(self):
        cookie = get_firefox_rutracker_cookie()
        self.assertIsNotNone(cookie, "Firefox RuTracker cookie should be extractable")
        self.assertTrue("bb_session" in cookie or "bb_data" in cookie)


class TestScorerService(unittest.IsolatedAsyncioTestCase):
    def setUp(self):
        self.service = AnimeReleaseService()

    def test_score_calculation(self):
        # High quality multi-audio release
        release = self.service.grade_release(
            title="[BDRip 1080p] Реинкарнация безработного / Mushoku Tensei [HEVC 10-bit] [DUB (Studio Band, Dream Cast) + JAP]",
            seeders=38,
            size_bytes=14 * 1024 * 1024 * 1024,
            topic_id="654321",
        )
        self.assertIsInstance(release, ScoredRelease)
        self.assertGreaterEqual(release.pts, 85, "BDRip HEVC multi-audio should score high PTS")
        self.assertIn("Studio Band", " ".join(release.voiceovers))
        self.assertEqual(release.resolution, "1080p")

    def test_ranking_multiple_releases(self):
        releases = [
            self.service.grade_release("[WEBRip 720p] Mushoku [AVC] [Crunchyroll]", 10, 4 * 1024**3, "1"),
            self.service.grade_release("[BDRip 1080p] Mushoku [HEVC FLAC] [Studio Band + RAW]", 40, 14 * 1024**3, "2"),
            self.service.grade_release("[HDTV 1080p] Mushoku [AVC AAC] [AniLibria]", 25, 8 * 1024**3, "3"),
        ]
        ranked = self.service.rank_releases(releases)
        self.assertEqual(ranked[0].topic_id, "2", "Top release should be the 1080p HEVC BDRip")
        self.assertGreater(ranked[0].pts, ranked[1].pts)


if __name__ == "__main__":
    unittest.main()
