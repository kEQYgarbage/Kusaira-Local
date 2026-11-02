"""Desktop window launcher and manager for Kusaira Desktop."""

import glob
import os
import shutil
import subprocess
import webbrowser
from typing import Any


def find_desktop_window_engine() -> str | None:
    """Find a browser executable capable of running in standalone --app mode."""
    # 1. Standard binaries in system PATH
    candidates = [
        "google-chrome",
        "google-chrome-stable",
        "chromium",
        "chromium-browser",
        "brave",
        "brave-browser",
        "microsoft-edge",
    ]
    for name in candidates:
        path = shutil.which(name)
        if path:
            return path

    # 2. NixOS /nix/store patterns
    nix_patterns = [
        "/nix/store/*google-chrome*/bin/google-chrome",
        "/nix/store/*google-chrome*/bin/google-chrome-stable",
        "/nix/store/*chromium*/bin/chromium",
        "/nix/store/*brave*/bin/brave",
    ]
    for pattern in nix_patterns:
        matches = glob.glob(pattern)
        if matches:
            for m in matches:
                if os.path.isfile(m) and os.access(m, os.X_OK):
                    return m

    return None


def get_default_profile_dir() -> str:
    """Return user data directory for isolated desktop window profile."""
    base = os.environ.get("XDG_DATA_HOME") or os.path.expanduser("~/.local/share")
    profile = os.path.join(base, "kusaira-desktop", "window-profile")
    os.makedirs(profile, exist_ok=True)
    return profile


def launch_desktop_window(
    url: str,
    profile_dir: str | None = None,
    app_id: str = "kusaira-desktop",
) -> subprocess.Popen[Any] | None:
    """Launch URL in standalone application window mode (no address bar, no browser tabs).

    Returns subprocess.Popen if standalone window process was launched, or None if fallback.
    """
    engine = find_desktop_window_engine()
    if engine:
        profile = profile_dir or get_default_profile_dir()
        args = [
            engine,
            f"--app={url}",
            f"--user-data-dir={profile}",
            f"--class={app_id}",
            f"--name={app_id}",
            "--disable-features=TranslateUI",
            "--no-first-run",
            "--no-default-browser-check",
            "--window-size=1280,800",
        ]
        try:
            return subprocess.Popen(
                args,
                stdout=subprocess.DEVNULL,
                stderr=subprocess.DEVNULL,
            )
        except Exception:
            pass

    # Fallback to default system browser if no standalone app runner is found
    webbrowser.open(url)
    return None
