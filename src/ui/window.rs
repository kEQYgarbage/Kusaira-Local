//! Native desktop window builder using tao and wry WebKitGTK.

use crate::ui::ipc::handle_ipc_command;
use tao::event_loop::EventLoop;
use tao::window::{Window, WindowBuilder};
use wry::WebView;

pub const RAW_HTML: &str = include_str!("assets/index.html");
pub const RAW_CSS: &str = include_str!("assets/style.css");
pub const RAW_JS: &str = include_str!("assets/app.js");

/// Produce composite HTML with bundled inlined stylesheets and scripts.
pub fn bundle_ui_html() -> String {
    RAW_HTML
        .replace("/*__STYLE_CSS__*/", RAW_CSS)
        .replace("/*__APP_JS__*/", RAW_JS)
}

/// Create native window with WebKitGTK webview and bidirectional IPC.
pub fn create_kusaira_window(
    event_loop: &EventLoop<()>,
) -> Result<(Window, WebView), String> {
    let window = WindowBuilder::new()
        .with_title("Kusaira Local Anime")
        .with_inner_size(tao::dpi::LogicalSize::new(1280.0, 820.0))
        .build(event_loop)
        .map_err(|e| format!("Failed to create native desktop window: {}", e))?;

    let html = bundle_ui_html();

    let webview = wry::WebViewBuilder::new()
        .with_html(html)
        .with_ipc_handler(move |req: wry::http::Request<String>| {
            if let Some(js_code) = handle_ipc_command(req.body()) {
                let _ = js_code;
            }
        })
        .build(&window)
        .map_err(|e| format!("Failed to create WebKitGTK webview: {}", e))?;

    Ok((window, webview))
}
