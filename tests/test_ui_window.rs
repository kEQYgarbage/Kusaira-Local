//! Unit tests for UI HTML bundling and IPC dispatcher.

use kusaira::ui::{bundle_ui_html, handle_ipc_command};

#[test]
fn test_bundle_ui_html_inlines_assets() {
    let html = bundle_ui_html();
    assert!(html.contains("Kusaira Local Anime"));
    assert!(html.contains("--bg-base: #0D0F12")); // from style.css
    assert!(html.contains("function callRust(")); // from app.js
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
fn test_ui_no_title_button_and_clean_brand() {
    let html = bundle_ui_html();
    assert!(!html.contains("id=\"nav-btn-det\""));
    assert!(!html.contains("Тайтл</button>"));
    assert!(html.contains("<div class=\"header-brand\" onclick=\"navTo('catalog')\">\n        <span>Kusaira</span>"));
}
