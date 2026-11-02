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
        _ => None,
    }
}
