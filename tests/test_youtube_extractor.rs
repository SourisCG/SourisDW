/// Static analysis test: verify youtube.rs contains no --extractor-args flags.
/// This ensures we never accidentally re-add extractor args that could
/// cause issues like JS runtime requirements or poToken requirements.
#[test]
fn test_no_extractor_args_in_youtube_rs() {
    let source = include_str!("../src/extractors/youtube.rs");
    assert!(
        !source.contains("--extractor-args"),
        "youtube.rs should NOT contain --extractor-args. Found: extractor-args"
    );
    assert!(
        !source.contains("player_js_version"),
        "youtube.rs should NOT contain player_js_version"
    );
    assert!(
        !source.contains("player_client"),
        "youtube.rs should NOT contain player_client"
    );
}

/// Verify that the youtube extractor uses YtDlp::command() instead of raw Command
#[test]
fn test_youtube_uses_ytdlp_command() {
    let source = include_str!("../src/extractors/youtube.rs");
    assert!(
        source.contains("yt_dlp.command()") || source.contains("YtDlp::command_with"),
        "youtube.rs should use YtDlp::command() or command_with() for --js-runtimes"
    );
    assert!(
        !source.contains("tokio::process::Command::new"),
        "youtube.rs should NOT use tokio::process::Command::new directly"
    );
}
