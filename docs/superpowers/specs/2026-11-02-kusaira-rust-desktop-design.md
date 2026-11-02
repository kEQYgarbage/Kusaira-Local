# Architecture & Design Spec: Kusaira Native Rust Desktop (From Scratch)

**Date**: 2026-11-02  
**Full Name**: Kusaira Local Anime  
**Short Name**: Kusaira  
**Binary Name**: `kusaira`  
**Description**: Kusaira Local Anime — нативное десктопное приложение для просмотра аниме в наивысшем качестве. Умный каталог с онгоингами и рекомендациями, автоскоринг раздач с RuTracker (PTS), мгновенный стриминг торрентов без ожидания загрузки и встроенный плеер libvlc с аппаратным ускорением, 10-bit HEVC и выбором озвучек на лету.  
**Target Platforms**: Linux (NixOS / GNOME Wayland & X11), Windows 10/11 (WebView2 + libvlc.dll)  

---

## 1. Intent & Motivation
The user requested a complete clean-slate rewrite of the application in **Rust** as a true native desktop application ("с нуля, не как сайт, а как аппка"). The app must not run as a local web server opened in a browser tab. Instead, it must launch as an isolated native desktop window, displaying a rich anime catalog with recommendations, scoring releases with PTS quality points, streaming torrents sequentially, and playing video seamlessly inside the app using an embedded `libvlc` player without bugs.

---

## 2. Goals, Non-Goals & Anti-Goals

### Goals
1. **True Desktop App**: Single compiled Rust binary (`kusaira`), launching an isolated desktop window (via `tao` + `wry` WebKitGTK engine), with zero browser tabs, zero address bar, native GNOME Wayland/X11 window decorations, and a memory footprint of ~25–40 MB.
2. **Catalog & Smart Recommendations**:
   - Four live shelves:
     1. *В тренде / Онгоинги* — currently broadcasting anime sorted by popularity from Shikimori API.
     2. *Продолжить просмотр* — locally saved history with last episode and progress timestamp.
     3. *Потому что вы смотрели [Название]* — automatic smart recommendations based on watched titles via Shikimori `similar` endpoint.
     4. *Золотая коллекция* — top-ranked titles (8.5+ score).
   - Fast instant search across Russian and English titles.
3. **PTS Release Scoring Engine**:
   - Calculates 0–100 quality score for RuTracker releases based on resolution, bitrate, 10-bit color, lossless/multichannel audio, and encode source (BDRip vs HDTV).
4. **Sequential Torrent Streaming Engine**:
   - Direct sequential chunk buffering of Episode 1 in 1–2 minutes.
   - Background pre-fetching of Episode 2 when Episode 1 reaches 40% playback.
5. **Embedded Bug-Free Video Player**:
   - Embedded directly in the application window with custom dark Kusaira controls.
   - Hardware-accelerated decoding (VA-API).
   - Switching audio tracks (voiceovers) on the fly without resetting playback position.
   - Full support for 10-bit HEVC and styled ASS subtitles.

### Non-Goals
- User cloud account registration or remote sync (all history and settings stored locally in SQLite).
- Server-side video transcoding (client plays native streams directly via libvlc).

### Anti-Goals
- Opening in default web browser tabs (`xdg-open` / Firefox / Chrome browser tabs).
- Heavy Electron or Chromium wrapper runtimes.
- Monolithic source files exceeding 400 lines (strictly obeying Rule #1).

---

## 3. High-Level Architecture

```
+-------------------------------------------------------------------+
|                        Native App Window                          |
|             (tao + wry / WebKitGTK / GNOME Wayland)               |
|                                                                   |
|  +--------------------+  +-------------------------------------+  |
|  |  Navigation & Bar  |  |            Main View                |  |
|  |  [Catalog]         |  | - Shelves: Ongoing, History, Similar|  |
|  |  [Search]          |  | - Anime Cards with PTS badges       |  |
|  |  [Settings]        |  | - Details: Episodes, Voiceovers     |  |
|  +--------------------+  +-------------------------------------+  |
|                                                                   |
|  +-------------------------------------------------------------+  |
|  |              Embedded Video Player Overlay                  |  |
|  |   [libvlc XWindow/Wayland sub-surface or WebKit Canvas]     |  |
|  |   - Custom Dark Timeline, Audio Track Switcher, ASS Subs    |  |
|  +-------------------------------------------------------------+  |
+-------------------------------------------------------------------+
                                  |
                                  | Rust IPC & Native Channels
                                  v
+-------------------------------------------------------------------+
|                        Rust Core Runtime                          |
|                                                                   |
|  +---------------------+  +-----------------+  +---------------+  |
|  |  Shikimori Client   |  |   PTS Scorer    |  | SQLite Storage|  |
|  | (Catalog & Recs)    |  | (Quality Metric)|  | (Local History|  |
|  +---------------------+  +-----------------+  +---------------+  |
|                                                                   |
|  +----------------------------------+  +-----------------------+  |
|  |     Torrent Streaming Engine     |  |    libvlc Engine      |  |
|  |   (Sequential Chunks Buffer)     |  | (Hardware VA-API FFI) |  |
|  +----------------------------------+  +-----------------------+  |
+-------------------------------------------------------------------+
```

---

## 4. Domain Subsystems

### 4.1 Native Desktop Shell (`src/ui/window.rs`)
- Uses `tao` (window management) + `wry` (WebKitGTK webview).
- Sets `title = "Kusaira"`, window dimensions `1280x820`, dark background `#0D0F12`.
- Embeds UI assets directly into binary via `include_str!` or compile-time bundling.
- Two-way IPC between JavaScript frontend and Rust core via `wry::WebViewBuilder::with_ipc_handler`.

### 4.2 Catalog & Recommendation Service (`src/catalog/shikimori.rs`)
- Asynchronous HTTP client using `reqwest` / `tokio`.
- Endpoints:
  - `GET https://shikimori.one/api/animes?status=ongoing&order=popularity&limit=16`
  - `GET https://shikimori.one/api/animes?order=ranked&limit=16`
  - `GET https://shikimori.one/api/animes/{id}/similar`
  - `GET https://shikimori.one/api/animes?search={query}&limit=20`
- Response caching to avoid redundant API hits and respect rate limits.

### 4.3 PTS Quality Scorer (`src/scorer/pts.rs`)
- Parses resolution, video codec, audio channels, and group metadata.
- Metric formula:
  - Base points: Resolution (1080p = 45, 720p = 30, 4K = 50).
  - Codec: HEVC/x265 10-bit = +20, AVC/x264 = +10.
  - Audio: FLAC / 5.1 = +15, AAC 2.0 = +10.
  - Source: BDRip / Remux = +15, WEB-DL = +10.
  - Bonus for popular voiceover groups (Studio Band, Dream Cast, Jam Club).
  - Total score clamped to 0–100.

### 4.4 Local History & State (`src/storage/db.rs`)
- Embedded SQLite (`rusqlite`) in `~/.local/share/kusaira/data.db`.
- Tables:
  - `history`: `anime_id`, `anime_title`, `poster_url`, `episode`, `timestamp_sec`, `duration_sec`, `updated_at`.
  - `favorites`: `anime_id`, `anime_title`, `poster_url`, `added_at`.

### 4.5 RuTracker Release Discovery & Cookie Extraction (`src/torrent/rutracker.rs`)
- Automatically reads Firefox SQLite database (`~/.mozilla/firefox/*.default*/cookies.sqlite`) in Rust.
- Extracts `bb_session` and `bb_data` cookies for zero-friction authenticated search on RuTracker.
- Performs HTML search query for the anime Russian and Japanese titles in category 33 (Аниме).
- Parses topics and feeds them into the PTS Quality Scorer.

### 4.6 Torrent Streaming Engine (`src/torrent/engine.rs`)
- Embeds `librqbit` session with sequential chunk prioritization.
- Exposes an internal, local loopback HTTP byte-range stream on `127.0.0.1:<dynamic_port>/stream/{episode_index}` strictly for player consumption (no external browser exposure).
- Handles byte-range requests and blocks socket reads on missing pieces, eliminating sparse-file zero-read crashes in VLC.
- Sequential pre-buffer strategy:
  1. Episode $N$: prioritize container header chunks and initial 2-minute buffer threshold.
  2. Signal readiness to player as soon as HTTP stream starts responding.
  3. When playback reaches 40% of Episode $N$, automatically trigger background sequential fetch of Episode $N+1$.

### 4.7 Native Video Player & Window View Switcher (`src/player/vlc.rs`, `src/ui/window.rs`)
- On Linux (GNOME Wayland/X11), the native window utilizes a GTK3 container holding two stacked views:
  1. **Catalog & Details View**: Powered by `wry` (WebKitGTK) for responsive, rich card layouts, PTS badges, and shelves.
  2. **Player View**: Powered by native GTK drawing widget bound directly to `libvlc` (`libvlc_media_player_set_xwindow` / Wayland sub-surface).
- Hardware acceleration: VA-API (`--avcodec-hw=vaapi`, `--quiet`).
- UI overlays: Custom dark header with "Назад в каталог", progress timeline, subtitle picker, and audio track switcher.
- Audio track switching on the fly via `libvlc_audio_set_track` without resetting playback time.

---

## 5. NixOS Build & Packaging (`shell.nix`)
To build and run without global dependencies:
```nix
{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    pkg-config
    cargo
    rustc
  ];
  buildInputs = with pkgs; [
    gtk3
    webkitgtk_4_1
    glib
    cairo
    pango
    vlc
    openssl
    sqlite
  ];
}
```

---

## 6. Verification Plan
1. **Unit tests in Rust** (`cargo test`):
   - PTS scoring parser tests with diverse release naming patterns.
   - Shikimori JSON deserialization tests with mock fixtures.
   - Sequential pre-buffer queue math tests.
2. **Integration verification**:
   - `cargo build` succeeds cleanly under `nix-shell`.
   - Binary executes and launches native GTK/WebKit window without errors.
   - Catalog shelves populate with real data from Shikimori API.
   - Memory usage stays below 50 MB on startup.
