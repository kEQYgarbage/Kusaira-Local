//! Tests for Discord Rich Presence integration.

use kusaira::discord::DiscordRpc;

#[test]
fn test_discord_rpc_creation_and_graceful_fallback() {
    let rpc = DiscordRpc::default();
    // If Discord is not running in test env, it returns Err gracefully without panicking
    let res = rpc.set_activity("Sousou no Frieren", "Серия 1");
    // Either OK or gracefully Err, no panic
    let _ = res;
}
