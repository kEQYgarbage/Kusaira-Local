//! Integration tests for basic application scaffold and metadata.

#[test]
fn test_app_constants() {
    assert_eq!(kusaira::APP_SHORT_NAME, "Kusaira");
    assert_eq!(kusaira::APP_FULL_NAME, "Kusaira Local Anime");
}
