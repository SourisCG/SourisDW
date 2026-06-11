/// Contract tests: verify the public API surface matches expectations.
/// These tests ensure that breaking changes to public types are intentional.

#[test]
fn test_souris_error_is_public() {
    let _: souris_dw::SourisError;
}

#[test]
fn test_souris_dw_builder_is_public() {
    let _: souris_dw::SourisDWBuilder;
}

#[test]
fn test_dep_status_is_public() {
    let _: souris_dw::DepStatus;
}

#[test]
fn test_dep_manager_is_public() {
    let _: souris_dw::DepManager;
}

#[test]
fn test_format_types_are_public() {
    let _: souris_dw::Format;
    let _: souris_dw::AudioFormat;
    let _: souris_dw::VideoFormat;
    let _: souris_dw::AudioQuality;
    let _: souris_dw::VideoQuality;
}

#[test]
fn test_media_info_is_public() {
    let _: souris_dw::MediaInfo;
}

#[test]
fn test_download_result_is_public() {
    let _: souris_dw::DownloadResult;
}

#[test]
fn test_app_config_is_public() {
    let _: souris_dw::AppConfig;
}
