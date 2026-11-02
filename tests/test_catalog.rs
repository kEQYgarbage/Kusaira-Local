//! Unit and integration tests for Shikimori catalog and models.

use kusaira::catalog::{AnimeCard, ShikimoriClient, ShikimoriImage};

#[test]
fn test_shikimori_client_creation() {
    let client = ShikimoriClient::new("https://shikimori.one");
    let _ = client;
}

#[test]
fn test_anime_card_display_title_prefers_russian() {
    let card = AnimeCard {
        id: 5114,
        name: "Fullmetal Alchemist: Brotherhood".to_string(),
        russian: Some("Стальной алхимик: Братство".to_string()),
        image: None,
        score: Some("9.10".to_string()),
        status: Some("released".to_string()),
        episodes: Some(64),
        episodes_aired: Some(64),
        kind: Some("tv".to_string()),
    };

    assert_eq!(card.display_title(), "Стальной алхимик: Братство");
    assert_eq!(card.score_f32(), 9.10);
}

#[test]
fn test_anime_card_fallback_to_english_name() {
    let card = AnimeCard {
        id: 9999,
        name: "Solo Leveling".to_string(),
        russian: None,
        image: None,
        score: Some("8.45".to_string()),
        status: Some("ongoing".to_string()),
        episodes: Some(12),
        episodes_aired: Some(8),
        kind: Some("tv".to_string()),
    };

    assert_eq!(card.display_title(), "Solo Leveling");
    assert_eq!(card.score_f32(), 8.45);
}

#[test]
fn test_anime_card_poster_url_construction() {
    let card = AnimeCard {
        id: 21,
        name: "One Piece".to_string(),
        russian: Some("Ван-Пис".to_string()),
        image: Some(ShikimoriImage {
            original: Some("/system/animes/original/21.jpg".to_string()),
            preview: Some("/system/animes/preview/21.jpg".to_string()),
            x96: None,
        }),
        score: Some("8.72".to_string()),
        status: Some("ongoing".to_string()),
        episodes: Some(0),
        episodes_aired: Some(1100),
        kind: Some("tv".to_string()),
    };

    assert_eq!(card.poster_url(), "https://shikimori.one/system/animes/original/21.jpg");
}

#[test]
fn test_deserialize_shikimori_fixture() {
    let json_data = r#"[
        {
            "id": 55888,
            "name": "Mushoku Tensei II: Isekai Ittara Honki Dasu Part 2",
            "russian": "Реинкарнация безработного 2. Часть 2",
            "image": {
                "original": "/system/animes/original/55888.jpg",
                "preview": "/system/animes/preview/55888.jpg",
                "x96": "/system/animes/x96/55888.jpg"
            },
            "url": "/animes/55888",
            "kind": "tv",
            "score": "8.54",
            "status": "released",
            "episodes": 12,
            "episodes_aired": 12
        }
    ]"#;

    let cards: Vec<AnimeCard> = serde_json::from_str(json_data).expect("Should deserialize JSON");
    assert_eq!(cards.len(), 1);
    assert_eq!(cards[0].id, 55888);
    assert_eq!(cards[0].display_title(), "Реинкарнация безработного 2. Часть 2");
    assert_eq!(cards[0].score_f32(), 8.54);
    assert_eq!(cards[0].poster_url(), "https://shikimori.one/system/animes/original/55888.jpg");
}
