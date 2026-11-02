import unittest

from anime_scout.player.vlc_bindings import find_libvlc, LibVLC
from anime_scout.player.vlc_engine import VlcEngine, PlaybackState


class TestVlcBindings(unittest.TestCase):
    def test_find_libvlc(self):
        vlc_lib_path = find_libvlc()
        self.assertIsNotNone(vlc_lib_path)
        self.assertIn("libvlc.so", vlc_lib_path)

    def test_libvlc_version(self):
        vlc = LibVLC()
        version = vlc.get_version()
        self.assertIn("3.", version, "Should report VLC 3.x version")


class TestVlcEngine(unittest.TestCase):
    def setUp(self):
        self.engine = VlcEngine()

    def tearDown(self):
        self.engine.release()

    def test_engine_initialization(self):
        self.assertFalse(self.engine.is_playing())
        self.assertEqual(self.engine.state, PlaybackState.STOPPED)

    def test_state_transitions(self):
        # Test pausing when no media
        self.engine.pause()
        self.assertEqual(self.engine.state, PlaybackState.STOPPED)

    def test_audio_tracks_empty_when_no_media(self):
        tracks = self.engine.get_audio_tracks()
        self.assertEqual(tracks, [])


if __name__ == "__main__":
    unittest.main()
