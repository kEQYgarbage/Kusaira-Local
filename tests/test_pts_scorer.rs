//! Unit tests for PTS release scoring engine.

use kusaira::scorer::{
    extract_codec, extract_resolution, extract_voiceovers, format_size, rank_releases, score_release,
};

#[test]
fn test_resolution_and_codec_extraction() {
    let title1 = "[BDRip 1080p HEVC 10bit] Mushoku Tensei S2 [Studio Band, Dream Cast]";
    assert_eq!(extract_resolution(title1), "1080p");
    assert_eq!(extract_codec(title1), "HEVC (10-bit)");

    let title2 = "Frieren 2160p 4K UHD WEB-DL x265";
    assert_eq!(extract_resolution(title2), "4K");
    assert_eq!(extract_codec(title2), "HEVC (10-bit)");

    let title3 = "Attack on Titan 720p HDTV x264";
    assert_eq!(extract_resolution(title3), "720p");
    assert_eq!(extract_codec(title3), "AVC / x264");
}

#[test]
fn test_voiceover_extraction() {
    let title = "DanDaDan [01-12] [BDRip 1080p] [MVO - Studio Band, Dream Cast, JAM Club, Raw]";
    let vos = extract_voiceovers(title);
    assert!(vos.iter().any(|v| v.contains("Studio Band")));
    assert!(vos.iter().any(|v| v.contains("Dream Cast")));
    assert!(vos.iter().any(|v| v.contains("JAM Club")));
}

#[test]
fn test_score_calculation_bdrip_flac() {
    let title = "[BDRip 1080p HEVC 10-bit FLAC] Chainsaw Man [Studio Band, Dream Cast, JAM]";
    let scored = score_release("12345", title, 14 * 1024 * 1024 * 1024, 45);

    // Resolution (20) + Codec (15) + BDRip (15) + FLAC (5) + Multi-vo (3) + Base (40) = 98 PTS
    assert!(scored.pts >= 95, "Expected PTS >= 95, got {}", scored.pts);
    assert!(scored.is_bdrip);
    assert_eq!(scored.resolution, "1080p");
    assert_eq!(scored.codec, "HEVC (10-bit)");
}

#[test]
fn test_rank_releases_ordering() {
    let mut releases = vec![
        score_release("1", "Low Quality 720p HDTV x264", 2 * 1024 * 1024 * 1024, 5),
        score_release("2", "Top BDRip 1080p HEVC 10bit FLAC [Studio Band]", 15 * 1024 * 1024 * 1024, 50),
        score_release("3", "Good 1080p WEB-DL x264 [AniLibria]", 8 * 1024 * 1024 * 1024, 20),
    ];

    rank_releases(&mut releases);
    assert_eq!(releases[0].topic_id, "2"); // Top BDRip first
    assert!(releases[0].pts > releases[1].pts);
    assert!(releases[1].pts > releases[2].pts);
}

#[test]
fn test_format_size() {
    assert_eq!(format_size(15 * 1024 * 1024 * 1024), "15.0 ГБ");
    assert_eq!(format_size(450 * 1024 * 1024), "450 МБ");
}
