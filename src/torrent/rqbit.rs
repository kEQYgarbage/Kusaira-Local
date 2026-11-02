//! Native local BitTorrent streaming engine via rqbit daemon.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::Mutex;
use std::time::{Duration, Instant};

pub const DEFAULT_RQBIT_PORT: u16 = 3030;

static GLOBAL_RQBIT_PROCESS: Mutex<Option<Child>> = Mutex::new(None);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RqbitFile {
    pub name: String,
    pub length: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RqbitTorrentDetails {
    pub id: usize,
    pub name: String,
    pub files: Vec<RqbitFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RqbitAddResponse {
    pub id: usize,
    pub details: RqbitTorrentDetails,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RqbitTorrentsList {
    pub torrents: Vec<RqbitTorrentDetails>,
}

/// Locate rqbit executable on the local system.
pub fn find_rqbit_binary() -> Option<PathBuf> {
    // 1. Next to current binary
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let candidate = dir.join("rqbit");
            if candidate.is_file() {
                return Some(candidate);
            }
        }
    }

    // 2. Local bin/rqbit
    let local_bin = PathBuf::from("bin/rqbit");
    if local_bin.is_file() {
        return Some(local_bin);
    }

    // 3. System PATH
    if let Ok(path_var) = std::env::var("PATH") {
        for dir in std::env::split_paths(&path_var) {
            let p = dir.join("rqbit");
            if p.is_file() {
                return Some(p);
            }
        }
    }

    // 4. Nix store fallback
    let nix_path = PathBuf::from("/nix/store/53fb6qlypglqzqlavsb7xrq1s5c17v1x-rqbit-9.0.1/bin/rqbit");
    if nix_path.is_file() {
        return Some(nix_path);
    }

    None
}

/// Ensure rqbit streaming server is running on the specified port.
pub fn ensure_rqbit_server(port: u16, cache_dir: &Path) -> Result<String, String> {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_millis(600))
        .build()
        .map_err(|e| e.to_string())?;

    let health_url = format!("http://127.0.0.1:{}/torrents", port);
    if let Ok(resp) = client.get(&health_url).send() {
        if resp.status().is_success() {
            return Ok(format!("http://127.0.0.1:{}", port));
        }
    }

    let binary = find_rqbit_binary().ok_or_else(|| "rqbit binary not found on system".to_string())?;
    std::fs::create_dir_all(cache_dir).map_err(|e| e.to_string())?;

    let child = Command::new(&binary)
        .arg("--http-api-listen-addr")
        .arg(format!("127.0.0.1:{}", port))
        .arg("server")
        .arg("start")
        .arg(cache_dir)
        .arg("--disable-persistence")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| format!("Failed to spawn rqbit daemon: {}", e))?;

    let mut guard = GLOBAL_RQBIT_PROCESS.lock().unwrap();
    *guard = Some(child);

    // Wait until HTTP API is ready (up to 3 seconds)
    let start = Instant::now();
    while start.elapsed() < Duration::from_secs(3) {
        std::thread::sleep(Duration::from_millis(100));
        if let Ok(resp) = client.get(&health_url).send() {
            if resp.status().is_success() {
                return Ok(format!("http://127.0.0.1:{}", port));
            }
        }
    }

    Ok(format!("http://127.0.0.1:{}", port))
}

/// Add magnet or torrent to rqbit and return details.
pub fn add_torrent(port: u16, magnet: &str) -> Result<RqbitAddResponse, String> {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(|e| e.to_string())?;

    let url = format!("http://127.0.0.1:{}/torrents", port);
    let resp = client
        .post(&url)
        .body(magnet.to_string())
        .send()
        .map_err(|e| format!("rqbit add torrent failed: {}", e))?;

    if !resp.status().is_success() {
        let err_text = resp.text().unwrap_or_default();
        return Err(format!("rqbit error ({}): {}", url, err_text));
    }

    let add_resp: RqbitAddResponse = resp
        .json()
        .map_err(|e| format!("Failed to parse rqbit add response: {}", e))?;

    Ok(add_resp)
}

/// Find matching video file for a specific episode.
pub fn find_episode_file_index(files: &[RqbitFile], ep: u32) -> usize {
    let video_exts = [".mkv", ".mp4", ".avi", ".webm", ".m4v", ".ts"];

    // 1. Try matching episode number in filename
    for (idx, f) in files.iter().enumerate() {
        let lower = f.name.to_lowercase();
        if video_exts.iter().any(|ext| lower.ends_with(ext)) {
            if let Some(parsed_ep) = crate::torrent::parse_episode_number(&f.name) {
                if parsed_ep == ep {
                    return idx;
                }
            }
        }
    }

    // 2. Try index-based if count of video files matches
    let video_files: Vec<(usize, &RqbitFile)> = files
        .iter()
        .enumerate()
        .filter(|(_, f)| {
            let lower = f.name.to_lowercase();
            video_exts.iter().any(|ext| lower.ends_with(ext))
        })
        .collect();

    if let Some(&(idx, _)) = video_files.get((ep.saturating_sub(1)) as usize) {
        return idx;
    }

    // 3. Fallback to largest video file
    video_files
        .into_iter()
        .max_by_key(|(_, f)| f.length)
        .map(|(idx, _)| idx)
        .unwrap_or(0)
}

/// Construct direct streaming URL for a specific torrent and file index.
pub fn get_stream_url(port: u16, torrent_id: usize, file_idx: usize) -> String {
    format!("http://127.0.0.1:{}/torrents/{}/stream/{}", port, torrent_id, file_idx)
}

/// Stop the global rqbit daemon if running.
pub fn stop_rqbit_server() {
    let mut guard = GLOBAL_RQBIT_PROCESS.lock().unwrap();
    if let Some(mut child) = guard.take() {
        let _ = child.kill();
        let _ = child.wait();
    }
}
