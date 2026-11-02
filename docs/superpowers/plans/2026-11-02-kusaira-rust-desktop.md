# Kusaira Local Anime Native Rust Desktop Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build Kusaira Local Anime from scratch as a standalone Linux desktop application in Rust featuring Shikimori anime recommendations, PTS release scoring, sequential torrent streaming, and embedded libvlc video playback.

**Architecture:** Native GTK3/WebKit window shell (`tao` + `wry`) communicating with a high-performance asynchronous Rust core (`tokio`, `librqbit`, `rusqlite`, `libvlc` FFI). The app renders an authentic dark Kusaira UI, pulls live catalog recommendations from Shikimori, automatically extracts RuTracker Firefox cookies for release discovery, prioritizes torrent chunks sequentially, and plays video with hardware acceleration (VA-API) directly inside the desktop window.

**Tech Stack:** Rust 2021/2024, `tokio`, `tao`, `wry` (WebKitGTK 4.1), `librqbit`, `rusqlite`, `libvlc` C-FFI, `reqwest`, `serde`, NixOS (`shell.nix`).

**Spec:** `docs/superpowers/specs/2026-11-02-kusaira-rust-desktop-design.md`

## Global Constraints
- **Application Full Name**: Kusaira Local Anime
- **Application Short Name**: Kusaira
- **Binary Name**: `kusaira`
- **Description**: Kusaira Local Anime — нативное десктопное приложение для просмотра аниме в наивысшем качестве. Умный каталог с онгоингами и рекомендациями, автоскоринг раздач с RuTracker (PTS), мгновенный стриминг торрентов без ожидания загрузки и встроенный плеер libvlc с аппаратным ускорением, 10-bit HEVC и выбором озвучек на лету.
- **Rule #1**: No source code file may exceed 400–500 lines of code. Orchestrators must stay between 100–200 lines.
- **Rule #3**: Surgical edits only when modifying existing files.
- **Rule #4**: Evidence before assertions: run `cargo test` and verify clean compile and passing tests before marking tasks done.
- **Platform**: Linux (NixOS / GNOME Wayland & X11) with cross-platform architecture ready for Windows (WebView2 + libvlc.dll).

## Review Focus
1. **Network offline / Rate-limit handling**: If Shikimori API is unreachable or rate-limited, catalog must gracefully fallback to cached SQLite records or built-in fallback items without crashing.
2. **Missing Firefox cookies**: If Firefox is not installed or has no RuTracker cookies, the app must not panic, but return friendly UI status and support public indexers or manual input.
3. **Sequential stream seeking**: When user seeks forward in player before chunks are downloaded, stream server must prioritize new seek position pieces and pause socket cleanly rather than crashing VLC.
4. **Audio track preservation**: Switching voiceovers on the fly must keep the exact current playback timestamp ($T$) and resume seamlessly.
5. **Clean window exit**: Closing the window (via GNOME [X] or Super+Q) must abort torrent background worker threads and exit the process immediately without zombie processes.

---

## Tasks

### Task 1: NixOS Environment & Cargo Scaffold
**Files:**
- Create: `shell.nix`
- Create: `Cargo.toml`
- Create: `src/main.rs`
- Test: `tests/test_scaffold.rs`

- [ ] Create `shell.nix` with NixOS dependencies: `pkg-config`, `cargo`, `rustc`, `gtk3`, `webkitgtk_4_1`, `glib`, `cairo`, `pango`, `vlc`, `openssl`, `sqlite`.
- [ ] Create `Cargo.toml` defining binary `kusaira` with dependencies: `tokio`, `serde`, `serde_json`, `reqwest`, `rusqlite`, `regex`, `libloading`, `tao`, `wry`.
- [ ] Create minimal `src/main.rs` displaying application banner and confirming environment initialization.
- [ ] Verify `nix-shell --run "cargo test"` runs and passes.
- [ ] Commit: `chore(scaffold): initialize Rust workspace and NixOS build shell`

---

### Task 2: Shikimori Catalog & Recommendations Engine
**Files:**
- Create: `src/catalog/models.rs`
- Create: `src/catalog/shikimori.rs`
- Create: `src/catalog/mod.rs`
- Test: `tests/test_catalog.rs`

- [ ] Write tests in `tests/test_catalog.rs` for deserializing Shikimori anime objects and generating API query URLs (`ongoing`, `ranked`, `similar`, `search`).
- [ ] Implement `src/catalog/models.rs` with `AnimeCard`, `AnimeDetails`, `Genre`, `ImageLinks` struct definitions.
- [ ] Implement `src/catalog/shikimori.rs` async client fetching ongoing anime, top-ranked anime, similar recommendations, and keyword search.
- [ ] Run `nix-shell --run "cargo test --test test_catalog"` and ensure all tests pass.
- [ ] Commit: `feat(catalog): implement Shikimori API client and recommendation models`

---

### Task 3: PTS Quality Scoring Engine in Rust
**Files:**
- Create: `src/scorer/models.rs`
- Create: `src/scorer/pts.rs`
- Create: `src/scorer/mod.rs`
- Test: `tests/test_pts_scorer.rs`

- [ ] Write unit tests in `tests/test_pts_scorer.rs` testing PTS score calculation for 4K/1080p, 10-bit HEVC, FLAC audio, and Russian voiceover groups (Studio Band, Dream Cast, Jam Club).
- [ ] Implement `src/scorer/models.rs` defining `ScoredRelease`, `ParsedMediaInfo`, and `QualityBreakdown`.
- [ ] Implement `src/scorer/pts.rs` with regex parsers for resolution, codec, audio channels, and the 0–100 PTS formula.
- [ ] Run `nix-shell --run "cargo test --test test_pts_scorer"` and ensure 100% pass.
- [ ] Commit: `feat(scorer): implement 0-100 PTS anime release quality scoring engine`

---

### Task 4: RuTracker Cookie Extractor & Topic Discovery
**Files:**
- Create: `src/torrent/cookies.rs`
- Create: `src/torrent/rutracker.rs`
- Create: `src/torrent/mod.rs`
- Test: `tests/test_rutracker.rs`

- [ ] Write unit tests in `tests/test_rutracker.rs` with mock SQLite database and sample RuTracker search HTML.
- [ ] Implement `src/torrent/cookies.rs` locating Firefox `cookies.sqlite` in `~/.mozilla/firefox/*.default*/` and extracting `bb_session` and `bb_data`.
- [ ] Implement `src/torrent/rutracker.rs` querying RuTracker Category 33 (Anime), parsing HTML topics (title, size, seeds, topic_id), and grading them via PTS scorer.
- [ ] Run `nix-shell --run "cargo test --test test_rutracker"` and verify all tests pass.
- [ ] Commit: `feat(torrent): implement RuTracker cookie extraction and release parsing`

---

### Task 5: Sequential Torrent Streaming Pipeline
**Files:**
- Create: `src/torrent/models.rs`
- Create: `src/torrent/pipeline.rs`
- Create: `src/torrent/stream_server.rs`
- Test: `tests/test_torrent_pipeline.rs`

- [ ] Write unit tests in `tests/test_torrent_pipeline.rs` testing episode filename parsing, sequential chunk priority ordering, and pre-buffer threshold math.
- [ ] Implement `src/torrent/models.rs` defining `EpisodeBufferStatus`, `StreamProgress`, and `FilePieceRange`.
- [ ] Implement `src/torrent/pipeline.rs` tracking active episode buffer percentage and triggering Episode $N+1$ prefetch at 40% playback.
- [ ] Implement `src/torrent/stream_server.rs` lightweight loopback HTTP range streamer serving video stream safely to libvlc.
- [ ] Run `nix-shell --run "cargo test --test test_torrent_pipeline"` and confirm all tests pass.
- [ ] Commit: `feat(torrent): implement sequential streaming pipeline and loopback stream server`

---

### Task 6: Embedded libvlc Player Engine
**Files:**
- Create: `src/player/bindings.rs`
- Create: `src/player/vlc.rs`
- Create: `src/player/mod.rs`
- Test: `tests/test_vlc_player.rs`

- [ ] Write unit tests in `tests/test_vlc_player.rs` testing state machine (`Playing`, `Paused`, `Stopped`), seek time clamping, and track description parsing.
- [ ] Implement `src/player/bindings.rs` dynamically loading `/nix/store/...-vlc-3.0.24/lib/libvlc.so` via `libloading` with hardware acceleration arguments (`--avcodec-hw=vaapi`, `--quiet`).
- [ ] Implement `src/player/vlc.rs` wrapping player instance, media controls, audio track switching, subtitle track switching, and playback event notifications.
- [ ] Run `nix-shell --run "cargo test --test test_vlc_player"` and verify all tests pass.
- [ ] Commit: `feat(player): implement libvlc C-FFI player engine with VA-API hardware acceleration`

---

### Task 7: Native Desktop UI & Window Shell (Kusaira Theme + IPC)
**Files:**
- Create: `src/ui/assets/index.html`
- Create: `src/ui/assets/style.css`
- Create: `src/ui/assets/app.js`
- Create: `src/ui/ipc.rs`
- Create: `src/ui/window.rs`
- Create: `src/ui/mod.rs`

- [ ] Create `src/ui/assets/style.css` and `index.html` implementing Kusaira dark theme, 4 recommendation shelves, PTS release cards, and video player overlay.
- [ ] Implement `src/ui/ipc.rs` handling bidirectional messages between WebKitGTK and Rust core (`get_catalog`, `search_anime`, `get_details`, `play_episode`, `player_action`).
- [ ] Implement `src/ui/window.rs` using `tao` + `wry` WebKitGTK to create native window titled `Kusaira Local Anime`, with size 1280x820, dark background, and embedded assets.
- [ ] Test window launching under `nix-shell --run "cargo run -- --headless-test"` or integration test.
- [ ] Commit: `feat(ui): implement Kusaira native desktop window shell and IPC dispatcher`

---

### Task 8: End-to-End Integration, Desktop Entry & Launcher
**Files:**
- Create: `bin/kusaira`
- Modify: `kusaira.desktop`
- Modify: `src/main.rs`
- Test: `tests/test_e2e_integration.rs`

- [ ] Wire all components into `src/main.rs` (runtime startup, catalog service, IPC handler, window loop).
- [ ] Create executable runner script `bin/kusaira` invoking the compiled Rust binary.
- [ ] Update `kusaira.desktop` with `Exec=/home/keqqyrr/anime-scout/bin/kusaira` and `StartupWMClass=kusaira`.
- [ ] Install desktop file to `~/.local/share/applications/kusaira.desktop`.
- [ ] Write integration test in `tests/test_e2e_integration.rs` validating full pipeline from catalog fetch to player readiness.
- [ ] Run full test suite: `nix-shell --run "cargo test"` (all tests passing).
- [ ] Commit: `feat: finalize Kusaira Local Anime native Rust desktop application and launcher`
