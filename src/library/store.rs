//! Local SQLite database store for user's anime library (watching, planned, completed, favorite).

use rusqlite::{params, Connection, Result as SqlResult};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryEntry {
    pub anime_id: u64,
    pub title: String,
    pub name: String,
    pub poster: String,
    pub status: String, // "watching", "planned", "completed", "favorite"
    pub score: String,
    pub pts: u32,
    pub episodes: u32,
    pub watched_episodes: u32,
    pub updated_at: i64,
}

pub struct LibraryStore {
    conn: Connection,
}

impl LibraryStore {
    pub fn open_default() -> SqlResult<Self> {
        let db_path = Self::default_db_path();
        if let Some(parent) = db_path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        Self::open(&db_path)
    }

    pub fn open(path: &PathBuf) -> SqlResult<Self> {
        let conn = Connection::open(path)?;
        let store = Self { conn };
        store.init_schema()?;
        Ok(store)
    }

    pub fn open_in_memory() -> SqlResult<Self> {
        let conn = Connection::open_in_memory()?;
        let store = Self { conn };
        store.init_schema()?;
        Ok(store)
    }

    fn default_db_path() -> PathBuf {
        if let Ok(home) = std::env::var("HOME") {
            PathBuf::from(home)
                .join(".local")
                .join("share")
                .join("kusaira")
                .join("library.sqlite")
        } else {
            PathBuf::from("kusaira_library.sqlite")
        }
    }

    fn init_schema(&self) -> SqlResult<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS user_library (
                anime_id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                name TEXT NOT NULL,
                poster TEXT NOT NULL,
                status TEXT NOT NULL,
                score TEXT NOT NULL,
                pts INTEGER NOT NULL,
                episodes INTEGER NOT NULL,
                watched_episodes INTEGER NOT NULL DEFAULT 0,
                updated_at INTEGER NOT NULL
            );",
            [],
        )?;
        Ok(())
    }

    pub fn save_entry(&self, entry: &LibraryEntry) -> SqlResult<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO user_library (
                anime_id, title, name, poster, status, score, pts, episodes, watched_episodes, updated_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10);",
            params![
                entry.anime_id as i64,
                entry.title,
                entry.name,
                entry.poster,
                entry.status,
                entry.score,
                entry.pts,
                entry.episodes,
                entry.watched_episodes,
                entry.updated_at,
            ],
        )?;
        Ok(())
    }

    pub fn remove_entry(&self, anime_id: u64) -> SqlResult<bool> {
        let rows = self.conn.execute(
            "DELETE FROM user_library WHERE anime_id = ?1;",
            params![anime_id as i64],
        )?;
        Ok(rows > 0)
    }

    pub fn get_entry(&self, anime_id: u64) -> SqlResult<Option<LibraryEntry>> {
        let mut stmt = self.conn.prepare(
            "SELECT anime_id, title, name, poster, status, score, pts, episodes, watched_episodes, updated_at
             FROM user_library WHERE anime_id = ?1;",
        )?;

        let mut rows = stmt.query(params![anime_id as i64])?;
        if let Some(row) = rows.next()? {
            Ok(Some(LibraryEntry {
                anime_id: row.get::<_, i64>(0)? as u64,
                title: row.get(1)?,
                name: row.get(2)?,
                poster: row.get(3)?,
                status: row.get(4)?,
                score: row.get(5)?,
                pts: row.get(6)?,
                episodes: row.get(7)?,
                watched_episodes: row.get(8)?,
                updated_at: row.get(9)?,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn list_entries(&self, status_filter: Option<&str>) -> SqlResult<Vec<LibraryEntry>> {
        let (sql, filter_val) = match status_filter {
            Some(s) if !s.is_empty() && s != "all" => (
                "SELECT anime_id, title, name, poster, status, score, pts, episodes, watched_episodes, updated_at
                 FROM user_library WHERE status = ?1 ORDER BY updated_at DESC;",
                Some(s.to_string()),
            ),
            _ => (
                "SELECT anime_id, title, name, poster, status, score, pts, episodes, watched_episodes, updated_at
                 FROM user_library ORDER BY updated_at DESC;",
                None,
            ),
        };

        let mut stmt = self.conn.prepare(sql)?;
        let rows = if let Some(ref val) = filter_val {
            stmt.query(params![val])?
        } else {
            stmt.query([])?
        };

        let mut entries = Vec::new();
        let mut rows = rows;
        while let Some(row) = rows.next()? {
            entries.push(LibraryEntry {
                anime_id: row.get::<_, i64>(0)? as u64,
                title: row.get(1)?,
                name: row.get(2)?,
                poster: row.get(3)?,
                status: row.get(4)?,
                score: row.get(5)?,
                pts: row.get(6)?,
                episodes: row.get(7)?,
                watched_episodes: row.get(8)?,
                updated_at: row.get(9)?,
            });
        }

        Ok(entries)
    }
}
