//! Tests for SQLite-backed watchlist library store.

use kusaira::library::{LibraryEntry, LibraryStore};

#[test]
fn test_library_crud_in_memory() {
    let store = LibraryStore::open_in_memory().expect("Should open in-memory sqlite store");

    let entry = LibraryEntry {
        anime_id: 52991,
        title: "Провожающая в последний путь Фрирен".to_string(),
        name: "Sousou no Frieren".to_string(),
        poster: "https://shikimori.io/system/animes/original/52991.jpg".to_string(),
        status: "watching".to_string(),
        score: "9.31".to_string(),
        pts: 99,
        episodes: 28,
        watched_episodes: 14,
        updated_at: 1730500000,
    };

    store.save_entry(&entry).expect("Should save entry");

    let loaded = store.get_entry(52991).expect("Query failed");
    assert!(loaded.is_some());
    let loaded = loaded.unwrap();
    assert_eq!(loaded.title, "Провожающая в последний путь Фрирен");
    assert_eq!(loaded.status, "watching");
    assert_eq!(loaded.watched_episodes, 14);

    let watching_list = store.list_entries(Some("watching")).expect("List failed");
    assert_eq!(watching_list.len(), 1);

    let planned_list = store.list_entries(Some("planned")).expect("List failed");
    assert_eq!(planned_list.len(), 0);

    let all_list = store.list_entries(None).expect("List failed");
    assert_eq!(all_list.len(), 1);

    let removed = store.remove_entry(52991).expect("Remove failed");
    assert!(removed);

    let check = store.get_entry(52991).expect("Query failed");
    assert!(check.is_none());
}
