//! Embedded video player module using libvlc C-FFI.

pub mod bindings;
pub mod vlc;

pub use bindings::VlcBindings;
pub use vlc::{PlaybackState, PlayerEngine};
