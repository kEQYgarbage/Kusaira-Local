use kusaira::config::AppConfig;

#[test]
fn test_default_config_values() {
    let cfg = AppConfig::default();
    assert_eq!(cfg.theme, "cyan");
    assert_eq!(cfg.grid_density, "standard");
    assert_eq!(cfg.opening_skip_mode, "tab");
    assert_eq!(cfg.gc_max_disk_gb, 20);
    assert_eq!(cfg.gc_policy, "keep_3");
    assert!(cfg.discord_rpc_enabled);
}

#[test]
fn test_config_serialization_roundtrip() {
    let mut cfg = AppConfig::default();
    cfg.theme = "neon".to_string();
    cfg.opening_skip_mode = "space".to_string();

    let json = serde_json::to_string(&cfg).expect("Serialization failed");
    let loaded: AppConfig = serde_json::from_str(&json).expect("Deserialization failed");
    assert_eq!(loaded.theme, "neon");
    assert_eq!(loaded.opening_skip_mode, "space");
}
