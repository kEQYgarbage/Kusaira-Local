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
            }).unwrap_or(52991);

            let card = crate::catalog::presets::find_preset_by_id(target_id)
                .map(ClientCard::from)
                .unwrap_or_else(|| {
                    ClientCard::from(crate::catalog::presets::get_preset_ongoing()[0].clone())
                });

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
        "save_settings" => {
            // Acknowledge settings saved
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
