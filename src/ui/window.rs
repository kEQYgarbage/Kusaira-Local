//! Native desktop window builder using tao and wry WebKitGTK.

use crate::ui::ipc::handle_ipc_command;
use tao::event_loop::EventLoop;
use tao::window::{Window, WindowBuilder};
use wry::WebView;

pub const RAW_HTML: &str = include_str!("assets/index.html");
pub const RAW_CSS: &str = include_str!("assets/style.css");
pub const RAW_JS: &str = include_str!("assets/app.js");
pub const RAW_PLAYER_JS: &str = include_str!("assets/player.js");

/// Produce composite HTML with bundled inlined stylesheets and scripts.
pub fn bundle_ui_html() -> String {
    let combined_js = format!("{}\n{}", RAW_JS, RAW_PLAYER_JS);
    RAW_HTML
        .replace("/*__STYLE_CSS__*/", RAW_CSS)
        .replace("/*__APP_JS__*/", &combined_js)
}

use std::cell::RefCell;
use std::rc::Rc;

/// Create native window with WebKitGTK webview and bidirectional IPC.
pub fn create_kusaira_window(
    event_loop: &EventLoop<()>,
) -> Result<(Window, Rc<RefCell<Option<WebView>>>), String> {
    let window = WindowBuilder::new()
        .with_title("Kusaira Local Anime")
        .with_inner_size(tao::dpi::LogicalSize::new(1280.0, 820.0))
        .build(event_loop)
        .map_err(|e| format!("Failed to create native desktop window: {}", e))?;

    let html = bundle_ui_html();

    let webview_holder: Rc<RefCell<Option<WebView>>> = Rc::new(RefCell::new(None));
    let webview_ref = Rc::clone(&webview_holder);

    let builder = wry::WebViewBuilder::new()
        .with_html(html)
        .with_ipc_handler(move |req: wry::http::Request<String>| {
            if let Some(js_code) = handle_ipc_command(req.body()) {
                if let Ok(guard) = webview_ref.try_borrow() {
                    if let Some(ref wv) = *guard {
                        let _ = wv.evaluate_script(&js_code);
                    }
                }
            }
        });

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    let webview = {
        use tao::platform::unix::WindowExtUnix;
        use wry::WebViewBuilderExtUnix;
        let vbox = window
            .default_vbox()
            .ok_or_else(|| "Failed to get window default vbox".to_string())?;
        builder
            .build_gtk(vbox)
            .map_err(|e| format!("Failed to create WebKitGTK webview: {}", e))?
    };

    #[cfg(any(target_os = "windows", target_os = "macos"))]
    let webview = builder
        .build(&window)
        .map_err(|e| format!("Failed to create webview: {}", e))?;

    *webview_holder.borrow_mut() = Some(webview);

    Ok((window, webview_holder))
}
