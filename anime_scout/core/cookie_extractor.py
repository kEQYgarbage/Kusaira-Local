"""Extract RuTracker session cookies from local Firefox profiles safely."""

import glob
import os
import shutil
import sqlite3
import tempfile


def get_firefox_rutracker_cookie() -> str | None:
    """Find and parse RuTracker cookies from Firefox sqlite database."""
    home = os.path.expanduser("~")
    patterns = [
        f"{home}/.config/mozilla/firefox/*.default*/cookies.sqlite",
        f"{home}/.mozilla/firefox/*.default*/cookies.sqlite",
        f"{home}/.var/app/org.mozilla.firefox/.mozilla/firefox/*.default*/cookies.sqlite",
    ]

    cookie_paths: list[str] = []
    for pat in patterns:
        cookie_paths.extend(glob.glob(pat))

    if not cookie_paths:
        return None

    for db_path in cookie_paths:
        src_dir = os.path.dirname(db_path)
        with tempfile.TemporaryDirectory() as tmp_dir:
            # Copy database + WAL/SHM to avoid locking Firefox
            for f in glob.glob(f"{src_dir}/cookies.sqlite*"):
                try:
                    shutil.copy2(f, os.path.join(tmp_dir, os.path.basename(f)))
                except OSError:
                    continue

            tmp_db = os.path.join(tmp_dir, "cookies.sqlite")
            if not os.path.exists(tmp_db):
                continue

            try:
                conn = sqlite3.connect(tmp_db)
                cursor = conn.cursor()
                cursor.execute(
                    "SELECT name, value FROM moz_cookies WHERE host LIKE '%rutracker%'"
                )
                rows = cursor.fetchall()
                conn.close()

                if rows:
                    cookie_dict = dict(rows)
                    if "bb_session" in cookie_dict or "bb_data" in cookie_dict:
                        return "; ".join([f"{k}={v}" for k, v in cookie_dict.items()])
            except Exception:
                continue

    return None
