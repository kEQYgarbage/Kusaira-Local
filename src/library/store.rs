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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchHistoryEntry {
    pub anime_id: u64,
    pub episode: u32,
    pub title: String,
    pub name: String,
    pub poster: String,
    pub position_sec: f64,
    pub duration_sec: f64,
    pub watched_at: i64,
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
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS watch_history (
                anime_id INTEGER NOT NULL,
                episode INTEGER NOT NULL,
                title TEXT NOT NULL,
                name TEXT NOT NULL,
                poster TEXT NOT NULL,
                position_sec REAL NOT NULL DEFAULT 0.0,
                duration_sec REAL NOT NULL DEFAULT 0.0,
                watched_at INTEGER NOT NULL,
                PRIMARY KEY (anime_id, episode)
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

    pub fn record_watch_progress(&self, entry: &WatchHistoryEntry) -> SqlResult<()> {
        self.conn.execute(
            "INSERT INTO watch_history (
                anime_id, episode, title, name, poster, position_sec, duration_sec, watched_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
            ON CONFLICT(anime_id, episode) DO UPDATE SET
                position_sec = excluded.position_sec,
                duration_sec = excluded.duration_sec,
                watched_at = excluded.watched_at;",
            params![
                entry.anime_id as i64,
                entry.episode,
                entry.title,
                entry.name,
                entry.poster,
                entry.position_sec,
                entry.duration_sec,
                entry.watched_at,
            ],
        )?;

        // Auto-update or insert into user_library as watching with latest episode
        let existing = self.get_entry(entry.anime_id)?;
        let status = existing.as_ref().map(|e| e.status.clone()).unwrap_or_else(|| "watching".to_string());
        let watched_eps = existing.as_ref().map(|e| e.watched_episodes.max(entry.episode)).unwrap_or(entry.episode);
        let episodes = existing.as_ref().map(|e| e.episodes).unwrap_or(12);
        let score = existing.as_ref().map(|e| e.score.clone()).unwrap_or_else(|| "8.5".to_string());
        let pts = existing.as_ref().map(|e| e.pts).unwrap_or(95);

        let lib_entry = LibraryEntry {
            anime_id: entry.anime_id,
            title: entry.title.clone(),
            name: entry.name.clone(),
            poster: entry.poster.clone(),
            status,
            score,
            pts,
            episodes,
            watched_episodes: watched_eps,
            updated_at: entry.watched_at,
        };
        self.save_entry(&lib_entry)?;

        Ok(())
    }

    pub fn list_history(&self, limit: usize) -> SqlResult<Vec<WatchHistoryEntry>> {
        let mut stmt = self.conn.prepare(
            "SELECT anime_id, episode, title, name, poster, position_sec, duration_sec, watched_at
             FROM watch_history ORDER BY watched_at DESC LIMIT ?1;",
        )?;
        let rows = stmt.query(params![limit as i64])?;
        let mut list = Vec::new();
        let mut rows = rows;
        while let Some(row) = rows.next()? {
            list.push(WatchHistoryEntry {
                anime_id: row.get::<_, i64>(0)? as u64,
                episode: row.get(1)?,
                title: row.get(2)?,
                name: row.get(3)?,
                poster: row.get(4)?,
                position_sec: row.get(5)?,
                duration_sec: row.get(6)?,
                watched_at: row.get(7)?,
            });
        }
        Ok(list)
    }

    pub fn get_last_watched(&self, anime_id: u64) -> SqlResult<Option<WatchHistoryEntry>> {
        let mut stmt = self.conn.prepare(
            "SELECT anime_id, episode, title, name, poster, position_sec, duration_sec, watched_at
             FROM watch_history WHERE anime_id = ?1 ORDER BY watched_at DESC LIMIT 1;",
        )?;
        let mut rows = stmt.query(params![anime_id as i64])?;
        if let Some(row) = rows.next()? {
            Ok(Some(WatchHistoryEntry {
                anime_id: row.get::<_, i64>(0)? as u64,
                episode: row.get(1)?,
                title: row.get(2)?,
                name: row.get(3)?,
                poster: row.get(4)?,
                position_sec: row.get(5)?,
                duration_sec: row.get(6)?,
                watched_at: row.get(7)?,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn get_watched_episodes(&self, anime_id: u64) -> SqlResult<Vec<u32>> {
        let mut stmt = self.conn.prepare(
            "SELECT episode FROM watch_history WHERE anime_id = ?1 ORDER BY episode ASC;",
        )?;
        let rows = stmt.query(params![anime_id as i64])?;
        let mut eps = Vec::new();
        let mut rows = rows;
        while let Some(row) = rows.next()? {
            eps.push(row.get(0)?);
        }
        Ok(eps)
    }

    pub fn clear_history(&self) -> SqlResult<()> {
        self.conn.execute("DELETE FROM watch_history;", [])?;
        Ok(())
    }
}
