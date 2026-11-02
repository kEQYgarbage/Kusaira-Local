//! Kusaira Local Anime — Library core modules

pub const APP_FULL_NAME: &str = "Kusaira Local Anime";
pub const APP_SHORT_NAME: &str = "Kusaira";
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const APP_DESCRIPTION: &str = "Kusaira Local Anime — нативное десктопное приложение для просмотра аниме в наивысшем качестве. Умный каталог с онгоингами и рекомендациями, автоскоринг раздач с RuTracker (PTS), мгновенный стриминг торрентов без ожидания загрузки и встроенный плеер libvlc с аппаратным ускорением, 10-bit HEVC и выбором озвучек на лету.";

pub mod catalog;
pub mod config;
pub mod discord;
pub mod library;
pub mod player;
pub mod scorer;
pub mod torrent;
pub mod ui;
