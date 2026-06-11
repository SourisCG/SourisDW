use souris_dw::SourisError;

#[test]
fn test_error_display_dependency_not_found() {
    let err = SourisError::DependencyNotFound {
        name: "deno".into(),
    };
    assert_eq!(err.to_string(), "Dependency not found: deno");
    assert_eq!(err.exit_code(), 2);
}

#[test]
fn test_error_display_download_failed() {
    let err = SourisError::DownloadFailed {
        reason: "Network error".into(),
    };
    assert_eq!(err.to_string(), "Download failed: Network error");
    assert_eq!(err.exit_code(), 1);
}

#[test]
fn test_error_display_invalid_url() {
    let err = SourisError::InvalidUrl {
        url: "not-a-url".into(),
    };
    assert_eq!(err.to_string(), "Invalid URL: not-a-url");
}

#[test]
fn test_error_display_unsupported_format() {
    let err = SourisError::UnsupportedFormat {
        format: "xyz".into(),
    };
    assert_eq!(err.to_string(), "Unsupported format: xyz");
}

#[test]
fn test_error_display_config_error() {
    let err = SourisError::ConfigError("bad config".into());
    assert!(err.to_string().contains("bad config"));
}

#[test]
fn test_error_exit_code_timeout() {
    let err = SourisError::Timeout { seconds: 30 };
    assert_eq!(err.exit_code(), 3);
}

#[test]
fn test_error_exit_code_cancelled() {
    let err = SourisError::Cancelled;
    assert_eq!(err.exit_code(), 0);
}
