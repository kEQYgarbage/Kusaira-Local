//! Bidirectional IPC dispatcher between WebKitGTK and Rust core.

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct IpcMessage {
    pub cmd: String,
    pub query: Option<String>,
    pub id: Option<serde_json::Value>,
    pub title: Option<String>,
    pub episode: Option<u32>,
    pub action: Option<String>,
    pub topic_id: Option<String>,
    pub status: Option<String>,
    pub entry: Option<crate::library::LibraryEntry>,
    pub settings: Option<serde_json::Value>,
    pub config: Option<crate::config::AppConfig>,
    pub gc_mode: Option<String>,
    pub progress: Option<f32>,
    pub history_entry: Option<crate::library::WatchHistoryEntry>,
}

#[derive(Debug, Serialize)]
pub struct CatalogPayload {
    pub ongoing: Vec<ClientCard>,
    pub similar: Vec<ClientCard>,
    pub ranked: Vec<ClientCard>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientCard {
    pub id: u64,
    pub title: String,
    pub name: String,
    pub poster: String,
    pub score: String,
    pub pts: u32,
    pub episodes: u32,
    pub description: String,
    pub release: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientRelatedItem {
    pub relation: String,
    pub relation_russian: String,
    pub anime: ClientRelatedAnime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientRelatedAnime {
    pub id: u64,
    pub name: String,
    pub russian: String,
    pub image: Option<crate::catalog::ShikimoriImage>,
}

impl From<crate::catalog::PresetAnime> for ClientCard {
    fn from(p: crate::catalog::PresetAnime) -> Self {
        Self {
            id: p.id,
            title: p.title.to_string(),
            name: p.name.to_string(),
            poster: p.poster.to_string(),
            score: p.score.to_string(),
            pts: p.pts,
            episodes: p.episodes,
            description: p.description.to_string(),
            release: p.release.to_string(),
        }
    }
}

pub fn handle_ipc_command(json_str: &str) -> Option<String> {
    let msg: IpcMessage = match serde_json::from_str(json_str) {
        Ok(m) => m,
        Err(_) => return None,
    };

    match msg.cmd.as_str() {
        "get_catalog" => {
            let ongoing: Vec<ClientCard> = crate::catalog::presets::get_preset_ongoing()
                .into_iter()
                .map(ClientCard::from)
                .collect();
            let similar: Vec<ClientCard> = crate::catalog::presets::get_preset_similar()
                .into_iter()
                .map(ClientCard::from)
                .collect();
            let ranked: Vec<ClientCard> = crate::catalog::presets::get_preset_ranked()
                .into_iter()
                .map(ClientCard::from)
                .collect();

            let payload = CatalogPayload {
                ongoing,
                similar,
                ranked,
            };

            let json = serde_json::to_string(&payload).unwrap_or_default();
            Some(format!(
                "renderShelf('shelf-ongoing', {0}.ongoing); renderShelf('shelf-similar', {0}.similar); renderShelf('shelf-ranked', {0}.ranked);",
                json
            ))
        }
        "get_details" => {
            let target_id = msg.id.and_then(|v| match v {
                serde_json::Value::Number(n) => n.as_u64(),
                serde_json::Value::String(s) => s.parse::<u64>().ok(),
                _ => None,
            })?;

            let card = crate::catalog::presets::find_preset_by_id(target_id).map(ClientCard::from)?;
            let json = serde_json::to_string(&card).unwrap_or_default();
            Some(format!("setDetails({});", json))
        }
        "search" => {
            let query = msg.query.unwrap_or_default();
            let results: Vec<ClientCard> = crate::catalog::presets::search_presets(&query)
                .into_iter()
                .map(ClientCard::from)
                .collect();

            let json = serde_json::to_string(&results).unwrap_or_default();
            Some(format!("renderSearchResults({});", json))
        }
        "get_related" => {
            let target_id = msg.id.and_then(|v| match v {
                serde_json::Value::Number(n) => n.as_u64(),
                serde_json::Value::String(s) => s.parse::<u64>().ok(),
                _ => None,
            }).unwrap_or(52991);

            let related_items = get_mock_or_preset_related(target_id);
            let json = serde_json::to_string(&related_items).unwrap_or_default();
            Some(format!("renderRelatedAnime({});", json))
        }
        "get_releases" => {
            let title = msg.title.unwrap_or_else(|| "Anime".to_string());
            let releases = crate::torrent::get_curated_releases_for_title(&title);
            let json = serde_json::to_string(&releases).unwrap_or_default();
            Some(format!("setReleases({});", json))
        }
        "select_release" => {
            // Acknowledge release selection
            None
        }
        "library_add" => {
            if let Some(entry) = msg.entry {
                if let Ok(store) = crate::library::LibraryStore::open_default() {
                    let _ = store.save_entry(&entry);
                }
            }
            None
        }
        "library_remove" => {
            if let Some(v) = msg.id {
                let id = match v {
                    serde_json::Value::Number(n) => n.as_u64(),
                    serde_json::Value::String(s) => s.parse::<u64>().ok(),
                    _ => None,
                };
                if let Some(id_num) = id {
                    if let Ok(store) = crate::library::LibraryStore::open_default() {
                        let _ = store.remove_entry(id_num);
                    }
                }
            }
            None
        }
        "library_list" => {
            if let Ok(store) = crate::library::LibraryStore::open_default() {
                if let Ok(entries) = store.list_entries(msg.status.as_deref()) {
                    let json = serde_json::to_string(&entries).unwrap_or_default();
                    return Some(format!(
                        "{}.forEach(e => userLibrary.set(Number(e.anime_id), e)); renderLibraryGrid();",
                        json
                    ));
                }
            }
            None
        }
        "record_history" => {
            if let Some(entry) = msg.history_entry {
                if let Ok(store) = crate::library::LibraryStore::open_default() {
                    let _ = store.record_watch_progress(&entry);
                }
            }
            None
        }
        "get_history" => {
            if let Ok(store) = crate::library::LibraryStore::open_default() {
                if let Ok(history) = store.list_history(30) {
                    let json = serde_json::to_string(&history).unwrap_or_default();
                    return Some(format!("renderWatchHistory({});", json));
                }
            }
            None
        }
        "get_anime_history" => {
            let id = msg.id.and_then(|v| match v {
                serde_json::Value::Number(n) => n.as_u64(),
                serde_json::Value::String(s) => s.parse::<u64>().ok(),
                _ => None,
            });
            if let Some(id_num) = id {
                if let Ok(store) = crate::library::LibraryStore::open_default() {
                    let eps = store.get_watched_episodes(id_num).unwrap_or_default();
                    let last = store.get_last_watched(id_num).ok().flatten();
                    let payload = serde_json::json!({
                        "anime_id": id_num,
                        "watched_episodes": eps,
                        "last_watched": last,
                    });
                    return Some(format!("setAnimeWatchedHistory({});", serde_json::to_string(&payload).unwrap_or_default()));
                }
            }
            None
        }
        "clear_history" => {
            if let Ok(store) = crate::library::LibraryStore::open_default() {
                let _ = store.clear_history();
            }
            Some("renderWatchHistory([]);".to_string())
        }
        "get_config" => {
            let cfg = crate::config::ConfigStore::load();
            let json = serde_json::to_string(&cfg).unwrap_or_default();
            Some(format!("applyConfig({});", json))
        }
        "save_config" => {
            if let Some(cfg) = msg.config {
                let _ = crate::config::ConfigStore::save(&cfg);
            }
            None
        }
        "run_gc" => {
            let cache_dir = crate::torrent::default_cache_dir();
            let stats = if msg.gc_mode.as_deref() == Some("all") {
                crate::torrent::clean_all_cache(&cache_dir)
            } else {
                let cfg = crate::config::ConfigStore::load();
                let max_bytes = (cfg.gc_max_disk_gb as u64) * 1024 * 1024 * 1024;
                crate::torrent::run_garbage_collection(&cache_dir, max_bytes, &cfg.gc_policy)
            };
            let json = serde_json::to_string(&stats).unwrap_or_default();
            Some(format!("onGcComplete({});", json))
        }
        "get_gc_stats" => {
            let cache_dir = crate::torrent::default_cache_dir();
            let total = crate::torrent::calculate_cache_size(&cache_dir);
            Some(format!("setGcCacheBytes({});", total))
        }
        "discord_update" => {
            let title = msg.title.unwrap_or_else(|| "Kusaira Local Anime".to_string());
            let episode = msg.status.unwrap_or_else(|| "Просмотр".to_string());
            let _ = crate::discord::DiscordRpc::default().set_activity(&title, &episode);
            None
        }
        "play_episode" => {
            let ep = msg.episode.unwrap_or(1);
            let title = msg.title.unwrap_or_else(|| "Anime".to_string());
            if let Some(entry) = msg.history_entry {
                if let Ok(store) = crate::library::LibraryStore::open_default() {
                    let _ = store.record_watch_progress(&entry);
                }
            }
            let _ = crate::discord::DiscordRpc::default().set_activity(&title, &format!("Эпизод {}", ep));
            let endpoint = crate::torrent::StreamEndpoint::default();
            let stream_url = endpoint.stream_url(ep);
            let payload = serde_json::json!({
                "episode": ep,
                "title": title,
                "stream_url": stream_url,
                "seeds": 142,
                "speed_mbps": 18.5,
                "status": "ready"
            });
            Some(format!("onStreamReady({});", serde_json::to_string(&payload).unwrap_or_default()))
        }
        "playback_progress" => {
            let ep = msg.episode.unwrap_or(1);
            let prog = msg.progress.unwrap_or(0.0);
            let mut pipeline = crate::torrent::BingePipeline::new();
            let prefetch_ep = pipeline.on_playback_progress(ep, prog);
            let payload = serde_json::json!({
                "episode": ep,
                "prefetched": prefetch_ep
            });
            Some(format!("onPlaybackProgressSynced({});", serde_json::to_string(&payload).unwrap_or_default()))
        }
        "open_vlc" => {
            let ep = msg.episode.unwrap_or(1);
            let stream_url = crate::torrent::StreamEndpoint::default().stream_url(ep);
            let target_url = msg.query.unwrap_or(stream_url);
            let _ = std::process::Command::new("vlc").arg(&target_url).spawn();
            None
        }
        "save_settings" => {
            // Backward-compatibility alias
            None
        }
        _ => None,
    }
}

fn get_mock_or_preset_related(anime_id: u64) -> Vec<ClientRelatedItem> {
    match anime_id {
        52991 => vec![
            ClientRelatedItem {
                relation: "Sequel".to_string(),
                relation_russian: "Продолжение".to_string(),
                anime: ClientRelatedAnime {
                    id: 59978,
                    name: "Sousou no Frieren 2nd Season".to_string(),
                    russian: "Провожающая в последний путь Фрирен 2".to_string(),
                    image: Some(crate::catalog::ShikimoriImage {
                        original: Some("/system/animes/original/52991.jpg".to_string()),
                        preview: None,
                        x96: None,
                    }),
                },
            },
            ClientRelatedItem {
                relation: "Side Story".to_string(),
                relation_russian: "Другая история".to_string(),
                anime: ClientRelatedAnime {
                    id: 56885,
                    name: "Sousou no Frieren: ●● no Mahou".to_string(),
                    russian: "Провожающая в последний путь Фрирен: Магия ●●".to_string(),
                    image: Some(crate::catalog::ShikimoriImage {
                        original: Some("/system/animes/original/56885.jpg".to_string()),
                        preview: None,
                        x96: None,
                    }),
                },
            },
        ],
        55888 => vec![
            ClientRelatedItem {
                relation: "Prequel".to_string(),
                relation_russian: "Предыстория".to_string(),
                anime: ClientRelatedAnime {
                    id: 39535,
                    name: "Mushoku Tensei: Isekai Ittara Honki Dasu".to_string(),
                    russian: "Реинкарнация безработного".to_string(),
                    image: Some(crate::catalog::ShikimoriImage {
                        original: Some("/system/animes/original/39535.jpg".to_string()),
                        preview: None,
                        x96: None,
                    }),
                },
            },
        ],
        5114 => vec![
            ClientRelatedItem {
                relation: "Alternative version".to_string(),
                relation_russian: "Альтернативная версия".to_string(),
                anime: ClientRelatedAnime {
                    id: 121,
                    name: "Fullmetal Alchemist".to_string(),
                    russian: "Стальной алхимик".to_string(),
                    image: Some(crate::catalog::ShikimoriImage {
                        original: Some("/system/animes/original/121.jpg".to_string()),
                        preview: None,
                        x96: None,
                    }),
                },
            },
        ],
        _ => vec![],
    }
}
