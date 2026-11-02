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

#[test]
fn test_rqbit_binary_discovery() {
    use kusaira::torrent::find_rqbit_binary;
    let bin = find_rqbit_binary();
    assert!(bin.is_some(), "rqbit binary must be discoverable on local system");
    assert!(bin.unwrap().exists());
}

#[test]
fn test_rqbit_file_mapping() {
    use kusaira::torrent::rqbit::{find_episode_file_index, RqbitFile};

    let files = vec![
        RqbitFile { name: "poster.jpg".to_string(), length: 50_000 },
        RqbitFile { name: "Anime_S01E01.mkv".to_string(), length: 1_200_000_000 },
        RqbitFile { name: "Anime_S01E02.mkv".to_string(), length: 1_180_000_000 },
        RqbitFile { name: "Anime_S01E03.mkv".to_string(), length: 1_250_000_000 },
        RqbitFile { name: "subtitles.srt".to_string(), length: 25_000 },
    ];

    assert_eq!(find_episode_file_index(&files, 1), 1);
    assert_eq!(find_episode_file_index(&files, 2), 2);
    assert_eq!(find_episode_file_index(&files, 3), 3);
}

#[test]
fn test_rqbit_stream_url_and_magnet() {
    use kusaira::torrent::{get_magnet_for_topic, get_stream_url};

    let stream_url = get_stream_url(3030, 0, 2);
    assert_eq!(stream_url, "http://127.0.0.1:3030/torrents/0/stream/2");

    let magnet = get_magnet_for_topic("6581290", "Frieren");
    assert!(magnet.starts_with("magnet:?xt=urn:btih:"));
    assert!(magnet.contains("tracker.opentrackr.org"));
}
