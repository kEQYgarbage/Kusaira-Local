//! End-to-End integration tests validating the full native Rust pipeline.

use kusaira::catalog::AnimeCard;
use kusaira::player::VlcBindings;
use kusaira::scorer::rank_releases;
use kusaira::torrent::{get_curated_releases_for_title, BingePipeline, StreamEndpoint, TorrentFile};
use kusaira::ui::{bundle_ui_html, handle_ipc_command};
use kusaira::{APP_DESCRIPTION, APP_FULL_NAME, APP_SHORT_NAME};

#[test]
fn test_e2e_full_stack_pipeline() {
    // 1. Metadata check
    assert_eq!(APP_SHORT_NAME, "Kusaira");
    assert_eq!(APP_FULL_NAME, "Kusaira Local Anime");
    assert!(APP_DESCRIPTION.contains("нативное десктопное приложение"));

    // 2. Catalog mock item
    let anime = AnimeCard {
        id: 52991,
        name: "Sousou no Frieren".to_string(),
        russian: Some("Провожающая в последний путь Фрирен".to_string()),
        image: None,
        score: Some("9.31".to_string()),
        status: Some("released".to_string()),
        episodes: Some(28),
        episodes_aired: Some(28),
        kind: Some("tv".to_string()),
    };
    assert_eq!(anime.display_title(), "Провожающая в последний путь Фрирен");

    // 3. RuTracker release discovery & PTS ranking
    let mut releases = get_curated_releases_for_title(anime.display_title());
    rank_releases(&mut releases);
    assert!(!releases.is_empty());
    assert!(releases[0].pts >= 90);
    assert!(releases[0].voiceovers.iter().any(|v| v.contains("Studio Band")));

    // 4. Sequential Torrent Pipeline
    let mut pipeline = BingePipeline::new();
    pipeline.register_file(TorrentFile {
        index: 0,
        filename: "Frieren_01.mkv".to_string(),
        size_bytes: 800 * 1024 * 1024,
        episode: Some(1),
        downloaded_bytes: 0,
    });
    pipeline.register_file(TorrentFile {
        index: 1,
        filename: "Frieren_02.mkv".to_string(),
        size_bytes: 800 * 1024 * 1024,
        episode: Some(2),
        downloaded_bytes: 0,
    });

    let endpoint = StreamEndpoint::default();
    pipeline.set_episode_ready(1, &endpoint.stream_url(1));
    assert!(pipeline.is_episode_ready(1));

    pipeline.play_episode(1);
    let prefetch = pipeline.on_playback_progress(1, 0.40);
    assert_eq!(prefetch, Some(2));

    // 5. Video Player FFI bindings
    let bindings = VlcBindings::load();
    assert!(bindings.is_ok());

    // 6. UI & IPC
    let html = bundle_ui_html();
    assert!(html.contains("Kusaira Local Anime"));

    let ipc_catalog = handle_ipc_command(r#"{"cmd": "get_catalog"}"#);
    assert!(ipc_catalog.is_some());
    assert!(ipc_catalog.unwrap().contains("renderShelf"));
}
