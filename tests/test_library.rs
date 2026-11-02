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

#[test]
fn test_watch_history() {
    use kusaira::library::WatchHistoryEntry;

    let store = LibraryStore::open_in_memory().expect("Should open in-memory sqlite store");

    let entry1 = WatchHistoryEntry {
        anime_id: 52991,
        episode: 1,
        title: "Фрирен".to_string(),
        name: "Sousou no Frieren".to_string(),
        poster: "https://shikimori.io/system/animes/original/52991.jpg".to_string(),
        position_sec: 1420.5,
        duration_sec: 1440.0,
        watched_at: 1000,
    };
    store.record_watch_progress(&entry1).expect("Record ep 1");

    let entry2 = WatchHistoryEntry {
        anime_id: 52991,
        episode: 2,
        title: "Фрирен".to_string(),
        name: "Sousou no Frieren".to_string(),
        poster: "https://shikimori.io/system/animes/original/52991.jpg".to_string(),
        position_sec: 320.0,
        duration_sec: 1440.0,
        watched_at: 2000,
    };
    store.record_watch_progress(&entry2).expect("Record ep 2");

    let history = store.list_history(10).expect("List history");
    assert_eq!(history.len(), 2);
    assert_eq!(history[0].episode, 2); // Ordered by watched_at desc

    let last = store.get_last_watched(52991).expect("Get last");
    assert!(last.is_some());
    let last = last.unwrap();
    assert_eq!(last.episode, 2);
    assert_eq!(last.position_sec, 320.0);

    let eps = store.get_watched_episodes(52991).expect("Get watched episodes");
    assert_eq!(eps, vec![1, 2]);

    // Check user_library was auto-updated to watching with episode 2
    let lib_entry = store.get_entry(52991).expect("Get lib entry").expect("Should exist");
    assert_eq!(lib_entry.status, "watching");
    assert_eq!(lib_entry.watched_episodes, 2);

    store.clear_history().expect("Clear history");
    let history_empty = store.list_history(10).expect("List history empty");
    assert!(history_empty.is_empty());
}
