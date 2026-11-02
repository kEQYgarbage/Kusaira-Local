//! PTS Quality Scoring Engine (0-100 metric for anime releases).

use crate::scorer::models::ScoredRelease;
use regex::Regex;

pub fn extract_resolution(title: &str) -> String {
    let lower = title.to_lowercase();
    if lower.contains("2160p") || lower.contains("4k") || lower.contains("uhd") {
        "4K".to_string()
    } else if lower.contains("1080p") || lower.contains("1080i") || lower.contains("full hd") {
        "1080p".to_string()
    } else if lower.contains("720p") {
        "720p".to_string()
    } else {
        "1080p".to_string()
    }
}

pub fn extract_codec(title: &str) -> String {
    let lower = title.to_lowercase();
    if lower.contains("hevc") || lower.contains("x265") || lower.contains("h.265") || lower.contains("10bit") {
        "HEVC (10-bit)".to_string()
    } else if lower.contains("av1") {
        "AV1".to_string()
    } else if lower.contains("x264") || lower.contains("h.264") || lower.contains("avc") {
        "AVC / x264".to_string()
    } else {
        "x264".to_string()
    }
}

pub fn extract_voiceovers(title: &str) -> Vec<String> {
    let mut studios = Vec::new();
    let lower = title.to_lowercase();

    if lower.contains("studio band") || lower.contains("студийная банда") {
        studios.push("Studio Band [DUB]".to_string());
    }
    if lower.contains("dream cast") || lower.contains("дрим каст") {
        studios.push("Dream Cast [DUB]".to_string());
    }
    if lower.contains("jam club") || lower.contains("jam") {
        studios.push("JAM Club [DUB]".to_string());
    }
    if lower.contains("anilibria") || lower.contains("анилибрия") {
        studios.push("AniLibria [MVO]".to_string());
    }
    if lower.contains("anidub") || lower.contains("анидаб") {
        studios.push("AniDUB [MVO]".to_string());
    }
    if lower.contains("shiza") || lower.contains("шиза") {
        studios.push("SHIZA Project [MVO]".to_string());
    }
    if lower.contains("original") || lower.contains("японский") || lower.contains("raw") {
        studios.push("Оригинал (Японский) [RAW]".to_string());
    }

    if studios.is_empty() {
        studios.push("Многоголосый [DUB]".to_string());
        studios.push("Оригинал (Японский) [RAW]".to_string());
    }

    studios
}

pub fn score_release(topic_id: &str, title: &str, size_bytes: u64, seeders: u32) -> ScoredRelease {
    let lower = title.to_lowercase();
    let res = extract_resolution(title);
    let codec = extract_codec(title);
    let voiceovers = extract_voiceovers(title);

    let is_bdrip = Regex::new(r"(?i)\b(bd-?rip|blu-?ray|remux)\b")
        .map(|r| r.is_match(title))
        .unwrap_or(false);
    let is_webdl = Regex::new(r"(?i)\b(web-?dl|web-?rip)\b")
        .map(|r| r.is_match(title))
        .unwrap_or(false);

    let mut pts: u32 = 40;
    let mut reasons = Vec::new();

    // 1. Resolution
    match res.as_str() {
        "4K" => {
            pts += 25;
            reasons.push("Сверхвысокое разрешение 4K Ultra HD (+25)".to_string());
        }
        "1080p" => {
            pts += 20;
            reasons.push("Full HD 1080p (+20)".to_string());
        }
        "720p" => {
            pts += 10;
            reasons.push("HD 720p (+10)".to_string());
        }
        _ => {}
    }

    // 2. Video Codec
    if codec.contains("HEVC") || lower.contains("10bit") {
        pts += 15;
        reasons.push("10-битный кодек HEVC без бандинга (+15)".to_string());
    } else {
        pts += 8;
        reasons.push("Стандартный кодек AVC/x264 (+8)".to_string());
    }

    // 3. Source
    if is_bdrip {
        pts += 15;
        reasons.push("Чистый Blu-Ray / BDRip исходник без логотипов ТВ (+15)".to_string());
    } else if is_webdl {
        pts += 10;
        reasons.push("WEB-DL исходник с официального стриминга (+10)".to_string());
    }

    // 4. Audio Quality
    if lower.contains("flac") || lower.contains("5.1") {
        pts += 5;
        reasons.push("Lossless FLAC / многоканальный звук 5.1 (+5)".to_string());
    }

    // 5. Multi-voiceover bonus
    if voiceovers.len() >= 3 {
        pts += 3;
        reasons.push("Мульти-озвучка: выбор из нескольких студий (+3)".to_string());
    }

    let final_pts = pts.clamp(60, 99);

    ScoredRelease::new(
        topic_id,
        title,
        final_pts,
        res,
        codec,
        voiceovers,
        size_bytes,
        seeders,
        is_bdrip,
        reasons,
    )
}

pub fn rank_releases(releases: &mut [ScoredRelease]) {
    releases.sort_by(|a, b| {
        b.pts
            .cmp(&a.pts)
            .then_with(|| b.seeders.cmp(&a.seeders))
            .then_with(|| b.size_bytes.cmp(&a.size_bytes))
    });
}
