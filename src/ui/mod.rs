//! Native UI and window management module.

pub mod ipc;
pub mod window;

pub use ipc::{handle_ipc_command, CatalogPayload, ClientCard, IpcMessage};
pub use window::{bundle_ui_html, create_kusaira_window};
