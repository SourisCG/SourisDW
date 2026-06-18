use souris_dw::deps::platform::{self, Arch, Os};
use souris_dw::deps::resolve;

#[test]
fn test_ffmpeg_assets_for_supported_platforms() {
    assert_eq!(
        resolve::ffmpeg_asset_filename(Os::Linux, Arch::X86_64),
        Some("ffmpeg-linux-x64.gz")
    );
    assert_eq!(
        resolve::ffmpeg_asset_filename(Os::Linux, Arch::Aarch64),
        Some("ffmpeg-linux-arm64.gz")
    );
    assert_eq!(
        resolve::ffmpeg_asset_filename(Os::Macos, Arch::X86_64),
        Some("ffmpeg-darwin-x64.gz")
    );
    assert_eq!(
        resolve::ffmpeg_asset_filename(Os::Macos, Arch::Aarch64),
        Some("ffmpeg-darwin-arm64.gz")
    );
    assert_eq!(
        resolve::ffmpeg_asset_filename(Os::Windows, Arch::X86_64),
        Some("ffmpeg-win32-x64.gz")
    );
}

#[test]
fn test_windows_arm64_ffmpeg_is_not_claimed() {
    assert_eq!(
        resolve::ffmpeg_asset_filename(Os::Windows, Arch::Aarch64),
        None
    );
    assert_eq!(
        resolve::ffprobe_asset_filename(Os::Windows, Arch::Aarch64),
        None
    );
}

#[test]
fn test_deno_targets_for_release_platforms() {
    assert_eq!(
        resolve::deno_target(Os::Linux, Arch::X86_64),
        Some("x86_64-unknown-linux-gnu")
    );
    assert_eq!(
        resolve::deno_target(Os::Linux, Arch::Aarch64),
        Some("aarch64-unknown-linux-gnu")
    );
    assert_eq!(
        resolve::deno_target(Os::Macos, Arch::X86_64),
        Some("x86_64-apple-darwin")
    );
    assert_eq!(
        resolve::deno_target(Os::Macos, Arch::Aarch64),
        Some("aarch64-apple-darwin")
    );
    assert_eq!(
        resolve::deno_target(Os::Windows, Arch::X86_64),
        Some("x86_64-pc-windows-msvc")
    );
    assert_eq!(
        resolve::deno_target(Os::Windows, Arch::Aarch64),
        Some("aarch64-pc-windows-msvc")
    );
}

#[test]
fn test_deno_armv7_is_not_mapped_to_aarch64() {
    assert_eq!(resolve::deno_target(Os::Linux, Arch::Armv7l), None);
}

#[test]
fn test_yt_dlp_asset_names() {
    assert_eq!(
        platform::yt_dlp_release_filename_for(Os::Linux, Arch::X86_64),
        "yt-dlp_linux"
    );
    assert_eq!(
        platform::yt_dlp_release_filename_for(Os::Linux, Arch::Aarch64),
        "yt-dlp_linux_aarch64"
    );
    assert_eq!(
        platform::yt_dlp_release_filename_for(Os::Macos, Arch::Aarch64),
        "yt-dlp_macos"
    );
    assert_eq!(
        platform::yt_dlp_release_filename_for(Os::Windows, Arch::X86_64),
        "yt-dlp.exe"
    );
    assert_eq!(
        platform::yt_dlp_release_filename_for(Os::Windows, Arch::Aarch64),
        "yt-dlp_arm64.exe"
    );
}
