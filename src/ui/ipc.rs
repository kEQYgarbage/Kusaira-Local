//! Bidirectional IPC dispatcher between WebKitGTK and Rust core.

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct IpcMessage {
    pub cmd: String,
    pub query: Option<String>,
    pub id: Option<serde_json::Value>,
    pub episode: Option<u32>,
    pub action: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CatalogPayload {
    pub ongoing: Vec<ClientCard>,
    pub similar: Vec<ClientCard>,
    pub ranked: Vec<ClientCard>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ClientCard {
    pub id: u64,
    pub title: String,
    pub name: String,
    pub poster: String,
    pub score: String,
    pub pts: u32,
    pub episodes: u32,
}

pub fn handle_ipc_command(json_str: &str) -> Option<String> {
    let msg: IpcMessage = match serde_json::from_str(json_str) {
        Ok(m) => m,
        Err(_) => return None,
    };

    match msg.cmd.as_str() {
        "get_catalog" => {
            let mock_ongoing = vec![
                ClientCard {
                    id: 55888,
                    title: "Реинкарнация безработного 2".to_string(),
                    name: "Mushoku Tensei II".to_string(),
                    poster: "https://shikimori.one/system/animes/original/55888.jpg".to_string(),
                    score: "8.54".to_string(),
                    pts: 96,
                    episodes: 12,
                },
                ClientCard {
                    id: 52991,
                    title: "Провожающая в последний путь Фрирен".to_string(),
                    name: "Sousou no Frieren".to_string(),
                    poster: "https://shikimori.one/system/animes/original/52991.jpg".to_string(),
                    score: "9.31".to_string(),
                    pts: 98,
                    episodes: 28,
                },
            ];

            let payload = CatalogPayload {
                ongoing: mock_ongoing.clone(),
                similar: mock_ongoing.clone(),
                ranked: mock_ongoing,
            };

            let json = serde_json::to_string(&payload).unwrap_or_default();
            Some(format!(
                "renderShelf('shelf-ongoing', {0}.ongoing); renderShelf('shelf-similar', {0}.similar); renderShelf('shelf-ranked', {0}.ranked);",
                json
            ))
        }
        "get_details" => {
            let card = ClientCard {
                id: 52991,
                title: "Провожающая в последний путь Фрирен".to_string(),
                name: "Sousou no Frieren".to_string(),
                poster: "https://shikimori.one/system/animes/original/52991.jpg".to_string(),
                score: "9.31".to_string(),
                pts: 98,
                episodes: 28,
            };
            let json = serde_json::to_string(&card).unwrap_or_default();
            Some(format!("setDetails({});", json))
        }
        _ => None,
    }
}
