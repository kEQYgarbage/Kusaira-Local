//! Catalog and recommendations module.

pub mod models;
pub mod presets;
pub mod shikimori;

pub use models::{AnimeCard, ShikimoriImage};
pub use presets::PresetAnime;
pub use shikimori::ShikimoriClient;
