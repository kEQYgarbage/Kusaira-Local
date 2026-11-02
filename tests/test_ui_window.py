"""Tests for desktop window launcher and manager."""

import os
import unittest
from unittest.mock import patch, MagicMock

from anime_scout.ui.window import (
    find_desktop_window_engine,
    get_default_profile_dir,
    launch_desktop_window,
)


class TestUIWindow(unittest.TestCase):
    """Test suite for desktop window management."""

    @patch("shutil.which")
    def test_find_engine_via_path(self, mock_which):
        mock_which.side_effect = lambda cmd: "/usr/bin/google-chrome" if cmd == "google-chrome" else None
        engine = find_desktop_window_engine()
        self.assertEqual(engine, "/usr/bin/google-chrome")

    @patch("shutil.which", return_value=None)
    @patch("glob.glob")
    @patch("os.path.isfile", return_value=True)
    @patch("os.access", return_value=True)
    def test_find_engine_via_nix_store(self, mock_access, mock_isfile, mock_glob, mock_which):
        mock_glob.side_effect = lambda pat: ["/nix/store/abc-google-chrome-155/bin/google-chrome"] if "google-chrome" in pat else []
        engine = find_desktop_window_engine()
        self.assertEqual(engine, "/nix/store/abc-google-chrome-155/bin/google-chrome")

    @patch("shutil.which", return_value=None)
    @patch("glob.glob", return_value=[])
    def test_find_engine_none_found(self, mock_glob, mock_which):
        engine = find_desktop_window_engine()
        self.assertIsNone(engine)

    def test_get_default_profile_dir(self):
        profile = get_default_profile_dir()
        self.assertTrue(os.path.exists(profile))
        self.assertIn("kusaira-desktop", profile)

    @patch("anime_scout.ui.window.find_desktop_window_engine", return_value="/usr/bin/google-chrome")
    @patch("subprocess.Popen")
    def test_launch_desktop_window_standalone(self, mock_popen, mock_find):
        fake_proc = MagicMock()
        mock_popen.return_value = fake_proc

        proc = launch_desktop_window("http://127.0.0.1:8765")
        self.assertEqual(proc, fake_proc)
        mock_popen.assert_called_once()
        args = mock_popen.call_args[0][0]
        self.assertEqual(args[0], "/usr/bin/google-chrome")
        self.assertIn("--app=http://127.0.0.1:8765", args)
        self.assertIn("--class=kusaira-desktop", args)

    @patch("anime_scout.ui.window.find_desktop_window_engine", return_value=None)
    @patch("webbrowser.open")
    def test_launch_desktop_window_fallback(self, mock_webopen, mock_find):
        proc = launch_desktop_window("http://127.0.0.1:8765")
        self.assertIsNone(proc)
        mock_webopen.assert_called_once_with("http://127.0.0.1:8765")


if __name__ == "__main__":
    unittest.main()
