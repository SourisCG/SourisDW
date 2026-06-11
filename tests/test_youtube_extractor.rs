/// Static analysis test: verify youtube.rs uses --extractor-args with android
/// player client to avoid 403 errors and JS runtime/poToken requirements.
#[test]
fn test_extractor_args_uses_android_client() {
    let source = include_str!("../src/extractors/youtube.rs");
    assert!(
        source.contains("--extractor-args"),
        "youtube.rs should contain --extractor-args"
    );
    assert!(
        source.contains("player_client=android"),
        "youtube.rs should use player_client=android to avoid 403 errors"
    );
    assert!(
        !source.contains("player_js_version"),
        "youtube.rs should NOT contain player_js_version"
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
    // Allow tokio::process::Command::new only for ffmpeg post-processing (WAV conversion)
    let lines_using_raw_command: Vec<&str> = source
        .lines()
        .filter(|l| l.contains("tokio::process::Command::new"))
        .collect();
    let cmd_lines: Vec<&&str> = lines_using_raw_command
        .iter()
        .filter(|l| !l.contains("ffmpeg"))
        .collect();
    assert!(
        cmd_lines.is_empty(),
        "youtube.rs should NOT use tokio::process::Command::new directly (except for ffmpeg). Found: {:?}",
        cmd_lines
    );
}
