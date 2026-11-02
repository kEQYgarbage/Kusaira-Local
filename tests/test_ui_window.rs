//! Unit tests for UI HTML bundling and IPC dispatcher.

use kusaira::ui::{bundle_ui_html, handle_ipc_command};

#[test]
fn test_bundle_ui_html_inlines_assets() {
    let html = bundle_ui_html();
    assert!(html.contains("Kusaira Local Anime"));
    assert!(html.contains("--bg-base: #0D0F12")); // from style.css
    assert!(html.contains("function callRust(")); // from app.js
    assert!(html.contains("function openPlayerModal(")); // from player.js
    assert!(html.contains("id=\"player-modal\""));
    assert!(html.contains("player-buffering-overlay"));
    assert!(html.contains("player-skip-prompt"));
    assert!(html.contains("player-prefetch-toast"));
    assert!(!html.contains("/*__STYLE_CSS__*/"));
    assert!(!html.contains("/*__APP_JS__*/"));
}

#[test]
fn test_handle_ipc_get_catalog() {
    let json_msg = r#"{"cmd": "get_catalog"}"#;
    let resp = handle_ipc_command(json_msg);
    assert!(resp.is_some());
    let js = resp.unwrap();
    assert!(js.contains("renderShelf('shelf-ongoing'"));
    assert!(js.contains("Mushoku Tensei"));
}

#[test]
fn test_handle_ipc_get_details() {
    let json_msg = r#"{"cmd": "get_details", "id": 52991}"#;
    let resp = handle_ipc_command(json_msg);
    assert!(resp.is_some());
    let js = resp.unwrap();
    assert!(js.contains("setDetails("));
    assert!(js.contains("Frieren"));
}

#[test]
fn test_handle_ipc_invalid_command() {
    let json_msg = r#"{"cmd": "unknown_action_xyz"}"#;
    let resp = handle_ipc_command(json_msg);
    assert!(resp.is_none());
}

#[test]
fn test_handle_ipc_search() {
    let json_msg = r#"{"cmd": "search", "query": "Дандадан"}"#;
    let resp = handle_ipc_command(json_msg);
    assert!(resp.is_some());
    let js = resp.unwrap();
    assert!(js.contains("renderSearchResults("));
    assert!(js.contains("Дандадан"));
}

#[test]
fn test_handle_ipc_get_details_dynamic() {
    let json_msg = r#"{"cmd": "get_details", "id": 54638}"#;
    let resp = handle_ipc_command(json_msg);
    assert!(resp.is_some());
    let js = resp.unwrap();
    assert!(js.contains("setDetails("));
    assert!(js.contains("Дандадан"));
}

#[test]
fn test_handle_ipc_get_details_non_preset_does_not_overwrite() {
    // 54492 is Kusuriya no Hitorigoto (Монолог фармацевта)
    let json_msg = r#"{"cmd": "get_details", "id": 54492}"#;
    let resp = handle_ipc_command(json_msg);
    assert!(resp.is_none());
}

#[test]
fn test_ui_no_title_button_and_clean_brand() {
    let html = bundle_ui_html();
    assert!(!html.contains("id=\"nav-btn-det\""));
    assert!(!html.contains("Тайтл</button>"));
    assert!(html.contains("<div class=\"header-brand\" onclick=\"navTo('catalog')\">\n        <span>Kusaira</span>"));
}

#[test]
fn test_handle_ipc_get_related() {
    let json_msg = r#"{"cmd": "get_related", "id": 52991}"#;
    let resp = handle_ipc_command(json_msg);
    assert!(resp.is_some());
    let js = resp.unwrap();
    assert!(js.contains("renderRelatedAnime("));
    assert!(js.contains("Продолжение"));
}

#[test]
fn test_handle_ipc_get_releases() {
    let json_msg = r#"{"cmd": "get_releases", "title": "Dandadan"}"#;
    let resp = handle_ipc_command(json_msg);
    assert!(resp.is_some());
    let js = resp.unwrap();
    assert!(js.contains("setReleases("));
    assert!(js.contains("\"pts\":"));
}

#[test]
fn test_ui_library_and_modals_present() {
    let html = bundle_ui_html();
    assert!(html.contains("id=\"nav-btn-lib\""));
    assert!(html.contains("id=\"view-library\""));
    assert!(html.contains("id=\"releases-modal\""));
    assert!(html.contains("id=\"settings-modal\""));
    assert!(html.contains("meta name=\"referrer\" content=\"no-referrer\""));
    assert!(!html.contains("⚙️"));
    assert!(html.contains("id=\"nav-btn-set\" onclick=\"openSettingsModal()\">Настройки</button>"));
}

#[test]
fn test_handle_ipc_config_and_gc() {
    let get_cfg_msg = r#"{"cmd": "get_config"}"#;
    let resp = handle_ipc_command(get_cfg_msg);
    assert!(resp.is_some());
    let js = resp.unwrap();
    assert!(js.contains("applyConfig("));
    assert!(js.contains("opening_skip_mode"));

    let gc_msg = r#"{"cmd": "run_gc", "gc_mode": "threshold"}"#;
    let resp = handle_ipc_command(gc_msg);
    assert!(resp.is_some());
    let js = resp.unwrap();
    assert!(js.contains("onGcComplete("));

    let gc_stats_msg = r#"{"cmd": "get_gc_stats"}"#;
    let resp = handle_ipc_command(gc_stats_msg);
    assert!(resp.is_some());
    let js = resp.unwrap();
    assert!(js.contains("setGcCacheBytes("));
}

#[test]
fn test_handle_ipc_play_episode_and_progress() {
    let play_msg = r#"{"cmd": "play_episode", "title": "Frieren", "episode": 2}"#;
    let resp = handle_ipc_command(play_msg);
    assert!(resp.is_some());
    let js = resp.unwrap();
    assert!(js.contains("onStreamReady("));
    assert!(js.contains("127.0.0.1:8766/stream/ep2"));
    assert!(js.contains("\"seeds\":142"));

    let prog_msg = r#"{"cmd": "playback_progress", "episode": 2, "progress": 0.45}"#;
    let resp = handle_ipc_command(prog_msg);
    assert!(resp.is_some());
    let js = resp.unwrap();
    assert!(js.contains("onPlaybackProgressSynced("));
    assert!(js.contains("\"prefetched\":3"));
}

