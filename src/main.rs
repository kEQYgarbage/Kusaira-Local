//! Kusaira Local Anime — Standalone Desktop Application
//! Main entry point and orchestrator.

pub const APP_FULL_NAME: &str = "Kusaira Local Anime";
pub const APP_SHORT_NAME: &str = "Kusaira";
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {
    println!("🌸 Starting {} v{}...", APP_FULL_NAME, APP_VERSION);
}
