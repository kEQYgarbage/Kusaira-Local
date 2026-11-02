//! Integration tests for embedded libvlc player and bindings.

use kusaira::player::{PlaybackState, PlayerEngine, VlcBindings};

#[test]
fn test_libvlc_bindings_load_successfully() {
    let bindings = VlcBindings::load();
    assert!(bindings.is_ok(), "Failed to load libvlc bindings: {:?}", bindings.err());
}

#[test]
fn test_player_engine_initialization_and_states() {
    let player_res = PlayerEngine::new();
    assert!(player_res.is_ok(), "Failed to initialize PlayerEngine: {:?}", player_res.err());

    let mut player = player_res.unwrap();
    assert_eq!(player.state(), PlaybackState::Stopped);
    assert_eq!(player.get_time(), -1); // No media loaded yet

    player.pause();
    assert_eq!(player.state(), PlaybackState::Paused);

    player.stop();
    assert_eq!(player.state(), PlaybackState::Stopped);
}
