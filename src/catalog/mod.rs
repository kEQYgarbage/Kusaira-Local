//! Catalog and recommendations module.

pub mod models;
pub mod shikimori;

pub use models::{AnimeCard, ShikimoriImage};
pub use shikimori::ShikimoriClient;
