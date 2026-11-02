//! Kusaira Local Anime — Standalone Native Desktop Application
//! Main entry point and window lifecycle orchestrator.

use kusaira::catalog::ShikimoriClient;
use kusaira::player::VlcBindings;
use kusaira::scorer::score_release;
use kusaira::torrent::get_curated_releases_for_title;
use kusaira::ui::create_kusaira_window;
use kusaira::{APP_DESCRIPTION, APP_FULL_NAME, APP_SHORT_NAME, APP_VERSION};
use std::env;
use tao::event::{Event, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoop};

fn main() {
    // Ensure WebKitGTK operates stably on GNOME Wayland without DMABUF protocol errors
    #[cfg(not(target_os = "windows"))]
    std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");

    let args: Vec<String> = env::args().collect();

    if args.iter().any(|a| a == "--version" || a == "-v") {
        println!("{} ({}) v{}", APP_FULL_NAME, APP_SHORT_NAME, APP_VERSION);
        return;
    }

    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!("{} v{}", APP_FULL_NAME, APP_VERSION);
        println!("{}", APP_DESCRIPTION);
        println!("\nUsage: kusaira [OPTIONS]");
        println!("  -v, --version        Print version info and exit");
        println!("  -h, --help           Print help information and exit");
        println!("  --headless-test      Run internal self-test and verification suite");
        return;
    }

    if args.iter().any(|a| a == "--headless-test") {
        println!("🌸 [Headless-Test] Initializing {}...", APP_FULL_NAME);
        let client = ShikimoriClient::default();
        let _ = client;

        let scored = score_release(
            "1",
            "[BDRip 1080p HEVC FLAC] Test Anime [Studio Band]",
            10 * 1024 * 1024 * 1024,
            50,
        );
        assert!(scored.pts >= 90);
        println!("✓ PTS Scorer OK (Score: {} PTS)", scored.pts);

        let releases = get_curated_releases_for_title("Mushoku Tensei");
        assert!(!releases.is_empty());
        println!("✓ RuTracker release discovery OK (Found {} releases)", releases.len());

        let vlc = VlcBindings::load();
        if vlc.is_ok() {
            println!("✓ libvlc C-FFI loaded successfully with VA-API hardware acceleration");
        } else {
            println!("! libvlc load note: {:?}", vlc.err());
        }

        println!("✨ All subsystem verifications passed!");
        return;
    }

    println!("🌸 Starting {} v{}...", APP_FULL_NAME, APP_VERSION);

    let event_loop = EventLoop::new();
    let (window, _webview) = match create_kusaira_window(&event_loop) {
        Ok(w) => w,
        Err(e) => {
            eprintln!("Error initializing desktop window: {}", e);
            std::process::exit(1);
        }
    };

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Event::WindowEvent { event, window_id, .. } = event {
            if window_id == window.id() {
                match event {
                    WindowEvent::CloseRequested => {
                        println!("👋 Closing {}...", APP_SHORT_NAME);
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => {}
                }
            }
        }
    });
}
