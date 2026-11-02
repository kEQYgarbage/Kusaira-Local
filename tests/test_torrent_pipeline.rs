//! Unit tests for torrent sequential pre-buffering pipeline and filename parsers.

use kusaira::torrent::{parse_episode_number, BingePipeline, EpisodeBufferState, StreamEndpoint, TorrentFile};

#[test]
fn test_parse_episode_numbers() {
    assert_eq!(parse_episode_number("Mushoku_Tensei_S2_01.mkv"), Some(1));
    assert_eq!(parse_episode_number("[Erai-raws] Frieren - 04 [1080p].mkv"), Some(4));
    assert_eq!(parse_episode_number("Dandadan E12 [WEBRip 1080p].mp4"), Some(12));
    assert_eq!(parse_episode_number("Chainsaw_Man_EP03.mkv"), Some(3));
}

#[test]
fn test_binge_pipeline_lifecycle_and_prefetch() {
    let mut pipeline = BingePipeline::new();

    pipeline.register_file(TorrentFile {
        index: 0,
        filename: "Anime_01.mkv".to_string(),
        size_bytes: 1024 * 1024 * 500,
        episode: Some(1),
        downloaded_bytes: 0,
    });

    pipeline.register_file(TorrentFile {
        index: 1,
        filename: "Anime_02.mkv".to_string(),
        size_bytes: 1024 * 1024 * 500,
        episode: Some(2),
        downloaded_bytes: 0,
    });

    assert!(!pipeline.is_episode_ready(1));

    let endpoint = StreamEndpoint::new("127.0.0.1", 8766);
    let stream_url = endpoint.stream_url(1);
    pipeline.set_episode_ready(1, &stream_url);

    assert!(pipeline.is_episode_ready(1));

    // Start playing Episode 1
    pipeline.play_episode(1);
    assert_eq!(pipeline.episode_states.get(&1), Some(&EpisodeBufferState::Playing));

    // Progress at 20%: no prefetch yet
    let trigger1 = pipeline.on_playback_progress(1, 0.20);
    assert_eq!(trigger1, None);
    assert_eq!(pipeline.prefetched_episode, None);

    // Progress reaches 42%: triggers Episode 2 prefetch!
    let trigger2 = pipeline.on_playback_progress(1, 0.42);
    assert_eq!(trigger2, Some(2));
    assert_eq!(pipeline.prefetched_episode, Some(2));

    // Later progress at 65%: does not re-trigger
    let trigger3 = pipeline.on_playback_progress(1, 0.65);
    assert_eq!(trigger3, None);
}

#[test]
fn test_stream_endpoint_url_formatting() {
    let endpoint = StreamEndpoint::new("127.0.0.1", 8999);
    assert_eq!(endpoint.stream_url(3), "http://127.0.0.1:8999/stream/ep3");
}
