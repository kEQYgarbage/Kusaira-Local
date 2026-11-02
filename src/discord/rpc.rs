//! Discord Rich Presence client over local Unix domain socket IPC.

use serde::Serialize;
use std::env;
use std::io::Write;
use std::os::unix::net::UnixStream;
use std::path::PathBuf;

pub struct DiscordRpc {
    client_id: String,
}

#[derive(Serialize)]
struct HandshakePayload<'a> {
    v: u32,
    client_id: &'a str,
}

#[derive(Serialize)]
struct SetActivityPayload<'a> {
    cmd: &'a str,
    args: ActivityArgs<'a>,
    nonce: &'a str,
}

#[derive(Serialize)]
struct ActivityArgs<'a> {
    pid: u32,
    activity: ActivityData<'a>,
}

#[derive(Serialize)]
struct ActivityData<'a> {
    details: &'a str,
    state: &'a str,
    assets: ActivityAssets<'a>,
}

#[derive(Serialize)]
struct ActivityAssets<'a> {
    large_image: &'a str,
    large_text: &'a str,
    small_image: &'a str,
    small_text: &'a str,
}

impl Default for DiscordRpc {
    fn default() -> Self {
        Self {
            client_id: "1302354890203099146".to_string(), // Kusaira Local Anime App ID
        }
    }
}

impl DiscordRpc {
    pub fn new(client_id: impl Into<String>) -> Self {
        Self {
            client_id: client_id.into(),
        }
    }

    fn socket_path() -> Option<PathBuf> {
        let runtime_dir = env::var("XDG_RUNTIME_DIR").unwrap_or_else(|_| "/tmp".to_string());
        for i in 0..10 {
            let p = PathBuf::from(&runtime_dir).join(format!("discord-ipc-{}", i));
            if p.exists() {
                return Some(p);
            }
        }
        let fallback = PathBuf::from("/tmp/discord-ipc-0");
        if fallback.exists() {
            Some(fallback)
        } else {
            None
        }
    }

    /// Update presence if Discord is running, non-blocking / silent fallback.
    pub fn set_activity(&self, title: &str, episode_info: &str) -> Result<(), String> {
        let sock_path = Self::socket_path().ok_or_else(|| "Discord IPC socket not found".to_string())?;
        let mut stream = UnixStream::connect(sock_path)
            .map_err(|e| format!("Failed to connect to Discord IPC: {}", e))?;

        // 1. Handshake (Opcode 0)
        let handshake = serde_json::to_string(&HandshakePayload {
            v: 1,
            client_id: &self.client_id,
        }).map_err(|e| format!("Serialization error: {}", e))?;

        Self::send_packet(&mut stream, 0, &handshake)?;

        // 2. Set Activity (Opcode 1)
        let activity = serde_json::to_string(&SetActivityPayload {
            cmd: "SET_ACTIVITY",
            args: ActivityArgs {
                pid: std::process::id(),
                activity: ActivityData {
                    details: title,
                    state: episode_info,
                    assets: ActivityAssets {
                        large_image: "kusaira_logo",
                        large_text: "Kusaira Local Anime",
                        small_image: "play",
                        small_text: "Смотрит",
                    },
                },
            },
            nonce: "1",
        }).map_err(|e| format!("Serialization error: {}", e))?;

        Self::send_packet(&mut stream, 1, &activity)?;
        Ok(())
    }

    fn send_packet(stream: &mut UnixStream, opcode: u32, json_str: &str) -> Result<(), String> {
        let bytes = json_str.as_bytes();
        let len = bytes.len() as u32;

        let mut header = [0u8; 8];
        header[0..4].copy_from_slice(&opcode.to_le_bytes());
        header[4..8].copy_from_slice(&len.to_le_bytes());

        stream.write_all(&header).map_err(|e| e.to_string())?;
        stream.write_all(bytes).map_err(|e| e.to_string())?;
        Ok(())
    }
}
