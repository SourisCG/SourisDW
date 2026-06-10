use souris_dw::config::AppConfig;

#[test]
fn test_default_config() {
    let config = AppConfig::default();
    assert_eq!(config.download.default_format, "mp4");
    assert_eq!(config.download.default_quality, "1080p");
    assert_eq!(config.download.parallel, 4);
    assert!(config.download.embed_metadata);
    assert!(config.download.embed_thumbnail);
    assert!(!config.download.embed_subtitles);
    assert_eq!(config.download.timeout, 300);
    assert_eq!(config.download.max_retries, 3);
}

#[test]
fn test_config_get_known_keys() {
    let config = AppConfig::default();

    assert_eq!(
        config.get("download.default_format"),
        Some("mp4".to_string())
    );
    assert_eq!(
        config.get("download.default_quality"),
        Some("1080p".to_string())
    );
    assert_eq!(config.get("download.parallel"), Some("4".to_string()));
    assert_eq!(
        config.get("download.embed_metadata"),
        Some("true".to_string())
    );
    assert_eq!(config.get("yt_dlp.auto_update"), Some("true".to_string()));
    assert_eq!(config.get("yt_dlp.channel"), Some("nightly".to_string()));
}

#[test]
fn test_config_get_unknown_key() {
    let config = AppConfig::default();
    assert_eq!(config.get("nonexistent.key"), None);
}

#[test]
fn test_config_set_and_get() {
    let mut config = AppConfig::default();

    config.set("download.default_format", "mp3").unwrap();
    assert_eq!(
        config.get("download.default_format"),
        Some("mp3".to_string())
    );

    config.set("download.default_quality", "320").unwrap();
    assert_eq!(
        config.get("download.default_quality"),
        Some("320".to_string())
    );

    config.set("download.parallel", "8").unwrap();
    assert_eq!(config.get("download.parallel"), Some("8".to_string()));
}

#[test]
fn test_config_set_invalid_boolean() {
    let mut config = AppConfig::default();
    let result = config.set("download.embed_metadata", "notabool");
    assert!(result.is_err());
}

#[test]
fn test_config_set_invalid_number() {
    let mut config = AppConfig::default();
    let result = config.set("download.parallel", "notanumber");
    assert!(result.is_err());
}

#[test]
fn test_config_set_unknown_key() {
    let mut config = AppConfig::default();
    let result = config.set("unknown.key", "value");
    assert!(result.is_err());
}
