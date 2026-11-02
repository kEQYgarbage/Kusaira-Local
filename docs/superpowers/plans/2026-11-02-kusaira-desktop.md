# Kusaira Desktop Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a standalone Linux desktop anime streaming application with authentic Kusaira UI, RuTracker PTS quality scoring, sequential torrent binge-buffering, and embedded libvlc video playback.

**Architecture:** Modular Python architecture composed of an embedded `libvlc` video engine (`vlc_engine.py`), a sequential torrent pre-buffer pipeline (`torrent_engine.py`), RuTracker scoring client (`scorer_service.py`), and a Kusaira-styled desktop GUI.

**Tech Stack:** Python 3 (standard library, ctypes), libvlc (`3.0.24 Vetinari`), HTML5/CSS3 Kusaira design tokens, qbittorrent / torrent-stream, Shikimori API, RuTracker parser.

**Spec:** `docs/superpowers/specs/2026-11-02-kusaira-desktop-design.md`

## Global Constraints
- Every file MUST be under 400–500 lines of code (Rule #1).
- Surgical edits only (Rule #3).
- Verification before assertions with automated commands and tests (Rule #4).
- Target platform: Linux NixOS x86_64, Wayland/X11 GNOME desktop.

## Review Focus
1. RuTracker cookie expiry: Gracefully detect invalid session and refresh from Firefox sqlite profile.
2. Torrent piece starvation: Prevent video freeze by checking buffer watermark before initiating playback.
3. Multi-audio container track mapping: Correctly identify Studio Band, Dream Cast, and Japanese RAW tracks.
4. libvlc window embedding on Wayland: Correctly pass the native window ID without crashing XWayland.
5. Disk space management: Ensure sequential cache directory in `~/Videos/Anime/` respects free space limits.

---

### Task 1: Scorer & RuTracker Metadata Service

**Files:**
- Create: `anime_scout/core/scorer_service.py`
- Create: `anime_scout/core/cookie_extractor.py`
- Test: `tests/test_scorer_service.py`

- [ ] **Step 1: Write failing test for Firefox cookie extraction**
  Extract `bb_data` cookie from `/home/keqqyrr/.config/mozilla/firefox/yhpdshyy.default/cookies.sqlite`.
- [ ] **Step 2: Implement `cookie_extractor.py`**
  Connect to sqlite in read-only immutable mode and return the valid `bb_data` string.
- [ ] **Step 3: Write failing test for `scorer_service.py`**
  Test querying Shikimori anime metadata and grading RuTracker releases with PTS points.
- [ ] **Step 4: Implement `scorer_service.py`**
  Integrate with `/home/keqqyrr/rutracker_parser` to return ranked releases, PTS scores, and audio tracks.
- [ ] **Step 5: Run tests and verify PASS**
  Run `python3 -m unittest tests/test_scorer_service.py`.

---

### Task 2: Embedded libvlc Video Playback Engine

**Files:**
- Create: `anime_scout/player/vlc_engine.py`
- Create: `anime_scout/player/vlc_bindings.py`
- Test: `tests/test_vlc_engine.py`

- [ ] **Step 1: Write failing test for libvlc binding**
  Test loading `/nix/store/...-vlc-3.0.24/lib/libvlc.so` and verifying version string (`3.0.24 Vetinari`).
- [ ] **Step 2: Implement minimal ctypes wrapper `vlc_bindings.py`**
  Wrap core libvlc methods (`libvlc_new`, `libvlc_media_player_new`, `libvlc_audio_set_track`, etc.).
- [ ] **Step 3: Write failing test for `vlc_engine.py` lifecycle**
  Test media loading, play, pause, seek, audio track enumeration, and time update callbacks.
- [ ] **Step 4: Implement `vlc_engine.py`**
  Support window embedding (`set_xwindow`), audio track switching, subtitle track switching, and end-of-media event.
- [ ] **Step 5: Run tests and verify PASS**
  Run `python3 -m unittest tests/test_vlc_engine.py`.

---

### Task 3: Sequential Torrent Binge Pipeline

**Files:**
- Create: `anime_scout/torrent/pipeline.py`
- Create: `anime_scout/torrent/downloader.py`
- Test: `tests/test_torrent_pipeline.py`

- [ ] **Step 1: Write failing test for sequential piece prioritization**
  Verify active episode head/tail pieces are prioritized and next episode is queued.
- [ ] **Step 2: Implement `downloader.py`**
  Manage local torrent downloads into `~/Videos/Anime/<Title>/` with sequential piece requests.
- [ ] **Step 3: Write failing test for `pipeline.py` binge triggers**
  Verify that when Episode $N$ reaches 40% progress, background pre-download for Episode $N+1$ begins.
- [ ] **Step 4: Implement `pipeline.py`**
  Orchestrate active streaming buffer and background pre-fetching.
- [ ] **Step 5: Run tests and verify PASS**
  Run `python3 -m unittest tests/test_torrent_pipeline.py`.

---

### Task 4: Kusaira Desktop UI & Application Shell

**Files:**
- Create: `anime_scout/ui/app.py`
- Create: `anime_scout/ui/kusaira_theme.py`
- Create: `anime_scout/ui/templates/index.html`
- Test: `tests/test_ui_endpoints.py`

- [ ] **Step 1: Write failing test for local API endpoints**
  Test search, release selection, episode list, and playback start endpoints.
- [ ] **Step 2: Implement `kusaira_theme.py` and `index.html`**
  Deploy the approved authentic Kusaira design (mascot logo, dark obsidian theme, PTS badges, voiceover pills, episode grid).
- [ ] **Step 3: Implement `app.py`**
  Desktop window hosting the Kusaira interface with the embedded `libvlc` player surface.
- [ ] **Step 4: Connect UI events to VLC engine and Torrent pipeline**
  Wire "Watch Episode 1" -> sequential buffer -> libvlc play -> background pre-buffer Episode 2.
- [ ] **Step 5: Run tests and verify PASS**
  Run `python3 -m unittest tests/test_ui_endpoints.py`.

---

### Task 5: Desktop Launcher & End-to-End Verification

**Files:**
- Create: `bin/kusaira-desktop`
- Create: `kusaira-desktop.desktop`
- Test: `tests/test_e2e_flow.py`

- [ ] **Step 1: Write runner script `bin/kusaira-desktop`**
  Set PYTHONPATH and launch `anime_scout.ui.app`.
- [ ] **Step 2: Create GNOME `.desktop` entry**
  Install `kusaira-desktop.desktop` into `~/.local/share/applications/` with icon.
- [ ] **Step 3: Write E2E integration test**
  Simulate searching "Mushoku Tensei II", selecting top PTS release (94 PTS), starting Episode 1, and verifying Episode 2 pre-fetch triggers.
- [ ] **Step 4: Run full verification suite**
  Run all unit and integration tests to ensure 100% pass rate with zero errors.
