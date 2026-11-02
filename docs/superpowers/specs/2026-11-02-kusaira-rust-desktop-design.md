# Architecture & Design Spec: Kusaira Native Rust Desktop (From Scratch)

**Date**: 2026-11-02  
**Status**: Draft for Builder Check & User Review  
**Target Platform**: Linux (NixOS / GNOME Wayland & X11)  
**Binary Name**: `kusaira`  

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

### 4.5 Torrent Streaming Pipeline (`src/torrent/engine.rs`)
- Torrent download manager with sequential chunk priority.
- Pipeline states: `Idle`, `Buffering(progress_percent)`, `Ready(file_path)`, `Completed`.
- Sequential pre-buffer strategy:
  1. Episode $N$: prioritize first 5% and last 2% of file chunks to parse container header.
  2. Signal readiness to player as soon as buffer duration threshold is met (typically 1–2 minutes of bandwidth).
  3. When playback reaches 40% of Episode $N$, trigger background download of Episode $N+1$.

### 4.6 Native `libvlc` Player Engine (`src/player/vlc.rs`)
- C-FFI to system `libvlc.so` (`/nix/store/...-vlc-3.0.24/lib/libvlc.so`).
- Options: `--avcodec-hw=vaapi`, `--no-video-title-show`, `--no-osd`.
- Exposes controls: `play(path)`, `pause()`, `seek(time_ms)`, `set_audio_track(id)`, `set_spu(subtitle_id)`.
- Playback event loop emits state updates (`time`, `duration`, `state`) to the frontend.

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
