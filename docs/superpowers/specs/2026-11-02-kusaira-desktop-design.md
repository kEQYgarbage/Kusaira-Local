# Kusaira Desktop — Design Specification

**Date:** 2026-11-02  
**Status:** Approved  
**Target Platform:** Linux (NixOS x86_64, Wayland / GNOME desktop)  
**Author:** Antigravity & keqqyrr  

---

## 1. Intent & The Why

The user wants a dedicated Linux desktop application for watching anime locally from RuTracker torrents.
Instead of uploading files to remote servers or waiting for an entire 15–20 GB season to download before watching, the user wants a **seamless binge-watching workflow**:
1. Click "Watch Episode 1".
2. Playback starts in 2–3 minutes via sequential piece pre-buffering.
3. While Episode 1 plays, the background torrent engine pre-downloads Episode 2 to disk (`~/Videos/Anime/`).
4. When Episode 1 finishes, Episode 2 starts instantly with zero waiting time.
5. High quality releases are chosen automatically via RuTracker quality scoring (PTS).
6. The app features the authentic, polished dark visual design of **Kusaira** (`https://kusaira.xyz`), with video rendered directly through an embedded **`libvlc`** engine for native 10-bit HEVC, FLAC, and ASS subtitle support.

---

## 2. Goals, Non-Goals & Anti-Goals

### Goals
- **Authentic Kusaira UI**: Deep `#0D0F12` background, genuine Kusaira mascot logo, clean Apple TV / Netflix style typography, voiceover pills (`Studio Band`, `Dream Cast`, `JAM Club`, `RAW`), discreet release PTS badge (`94 PTS`).
- **Embedded `libvlc` Engine**: Use the system's native `/nix/store/...-vlc-3.0.24/lib/libvlc.so` directly inside the app window for full GPU hardware decoding (VA-API), 10-bit HEVC MKV playback, multi-track audio selection, and ASS subtitle rendering without CPU transcoding.
- **Smart Binge-Watching Pipeline**:
  - Episode $N$: Sequential download + playback.
  - Episode $N+1$: Background automatic pre-buffering while $N$ is being watched.
  - Instant transition to next episode upon completion.
- **RuTracker PTS Quality Scorer**: Uses the proven `scorer.py` algorithm to grade releases (BDRip > WEBRip > HDTV, HEVC > AVC, FLAC > AAC, Studio Band/Dream Cast detection). Top PTS release is preselected by default; discrete selector allows manual override.
- **GNOME / Linux Desktop Integration**: Single launch command and a `.desktop` file installed in `~/.local/share/applications/kusaira.desktop` with an icon.

### Non-Goals
- Remote cloud re-encoding or uploading to Telegram / remote VPS (all work is local on the user's PC).
- Complex tracker ratio manipulation or private tracker passkey sharing.

### Anti-Goals
- **No AI-slop / SaaS dashboard clichés**: No giant marketing banners, no "⭐ 94/100" billboard cards, no pulsing rainbow animations, no fake tech-demo boxes.
- **No browser-player transcoding**: Do NOT use Web HTML5 players (like ArtPlayer) that choke on 10-bit HEVC MKV and require CPU-burning ffmpeg transcoding. Video MUST be decoded by native `libvlc`.

---

## 3. Architecture & Components

```
+-------------------------------------------------------------------------+
|                           Kusaira Desktop Window                        |
|                                                                         |
|  +-------------------------------------------------------------------+  |
|  | Kusaira Header: Mascot Logo | Search Input | Buffer Status Pill   |  |
|  +-------------------------------------------------------------------+  |
|                                                                         |
|  +-----------------------------------+  +----------------------------+  |
|  | Catalog / Anime Detail View       |  | Embedded libvlc Surface    |  |
|  | - Poster + Metadata line          |  | - Hardware VA-API Decode   |  |
|  | - PTS Release Selector (94 PTS)   |  | - Multi-audio Track Switch |  |
|  | - Voiceover Pills (Studio Band)   |  | - ASS Subtitles Engine     |  |
|  | - Episode Grid [1] [2] [3]...     |  | - Direct Frame Output      |  |
|  +-----------------------------------+  +----------------------------+  |
+-------------------------------------------------------------------------+
                                    |
          +-------------------------+-------------------------+
          |                                                   |
+-------------------+                               +--------------------+
| RuTracker Scorer  |                               | Sequential Binge   |
| - Shikimori API   |                               | Torrent Worker     |
| - PTS Calculator  |                               | - Episode N stream |
| - Cookie Auth     |                               | - Ep N+1 pre-fetch |
+-------------------+                               +--------------------+
```

### Subsystems:
1. **Frontend / Shell**:
   - Modern, lightweight desktop interface styled 1:1 with Kusaira (`kusaira.xyz`).
   - Communicates with the local Python core via JSON IPC / local API.
2. **Video Playback Engine (`vlc_engine.py`)**:
   - Wraps `libvlc.so` (`3.0.24 Vetinari`) via `ctypes`.
   - Embeds native video output directly into the app window.
   - Handles track switching (`libvlc_audio_set_track`), subtitle selection, time tracking, and triggers next episode upon completion.
3. **Sequential Torrent Engine (`torrent_pipeline.py`)**:
   - Downloads torrent files via RuTracker client using local Firefox cookies.
   - Manages piece priority: first pieces of active episode downloaded first, remaining pieces downloaded sequentially.
   - Automatically kicks off pre-download of Episode $N+1$ when Episode $N$ reaches 40% playback progress.
4. **Scorer & Metadata Service (`scorer_service.py`)**:
   - Leverages existing `/home/keqqyrr/rutracker_parser/` modules (`scorer.py`, `html_parser.py`, `shikimori.py`).
   - Ranks releases by PTS points and extracts audio tracks.

---

## 4. UI / UX Design Specifications

### Colors & Design Tokens (Kusaira Authentic)
- Base background: `#0D0F12`
- Elevated surfaces: `#12151A` / `#1A1D24`
- Borders: `rgba(255, 255, 255, 0.08)`
- Primary Accent: `#2AABEE`
- Green Accent: `#30D158`
- Purple Accent: `#A855F7`
- Gold Accent: `#FFD60A`
- Text Primary: `#FFFFFF`
- Text Secondary: `rgba(255, 255, 255, 0.65)`

### Navigation & Views
- **Catalog View**: Search bar + grid of anime cards (`aspect-ratio: 3 / 4.2`) with poster, rating `★ 8.5`, and PTS tag `94 PTS`.
- **Detail View**:
  - `← Назад в каталог` back button.
  - Poster with gentle shadow + Title & Subtitle.
  - Metadata line: `★ 8.5 • 94 PTS • 2024 • 12 серий • Studio Bind • Завершён`.
  - Genre tags (`Фэнтези`, `Приключения`, etc.).
  - Big contrast "Смотреть 1 серию" button.
  - Discrete release dropdown with PTS: `94 PTS — BDRip 1080p • HEVC • FLAC Multi-audio (38 сидов, 14.2 ГБ)`.
  - Voiceover pills (`Studio Band`, `Dream Cast`, `JAM Club`, `RAW`).
  - Episode buttons (`1`, `2`, `3`...) with subtle status indicators (green dot = ready, blue dot = buffering).
- **Player View**:
  - Embedded `libvlc` surface.
  - Top overlay bar: `←` Close, Anime Title, Episode Number, Next Episode button.
  - Bottom bar: Timeline, Play/Pause, Timecode, Audio Track switcher, Fullscreen toggle.

---

## 5. Storage & Persistence
- Torrent data cache directory: `~/Videos/Anime/<Anime_Title>/`
- State & Cookies: `~/.config/kusaira-desktop/`
- Firefox cookies extracted automatically from `/home/keqqyrr/.config/mozilla/firefox/yhpdshyy.default/cookies.sqlite`.

---

## 6. Constraints & Coding Rules
- File size rule: Every file MUST be under 400–500 lines of code.
- Strict modularity: Split by domain (`core/`, `player/`, `torrent/`, `ui/`).
- Evidence before assertions: Every task verified with automated tests.
