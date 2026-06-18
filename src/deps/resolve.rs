use crate::deps::platform::{self, Arch, Os};
use crate::deps::versions;

pub fn ffmpeg_asset_filename(os: Os, arch: Arch) -> Option<&'static str> {
    match (os, arch) {
        (Os::Linux, Arch::X86_64) => Some("ffmpeg-linux-x64.gz"),
        (Os::Linux, Arch::Aarch64) => Some("ffmpeg-linux-arm64.gz"),
        (Os::Linux, Arch::Armv7l) => Some("ffmpeg-linux-arm.gz"),
        (Os::Macos, Arch::X86_64) => Some("ffmpeg-darwin-x64.gz"),
        (Os::Macos, Arch::Aarch64) => Some("ffmpeg-darwin-arm64.gz"),
        (Os::Windows, Arch::X86_64) => Some("ffmpeg-win32-x64.gz"),
        (Os::Windows, Arch::Aarch64) => None,
        (Os::Windows, Arch::Armv7l) | (Os::Macos, Arch::Armv7l) => None,
    }
}

pub fn ffprobe_asset_filename(os: Os, arch: Arch) -> Option<&'static str> {
    match (os, arch) {
        (Os::Linux, Arch::X86_64) => Some("ffprobe-linux-x64.gz"),
        (Os::Linux, Arch::Aarch64) => Some("ffprobe-linux-arm64.gz"),
        (Os::Linux, Arch::Armv7l) => Some("ffprobe-linux-arm.gz"),
        (Os::Macos, Arch::X86_64) => Some("ffprobe-darwin-x64.gz"),
        (Os::Macos, Arch::Aarch64) => Some("ffprobe-darwin-arm64.gz"),
        (Os::Windows, Arch::X86_64) => Some("ffprobe-win32-x64.gz"),
        (Os::Windows, Arch::Aarch64) => None,
        (Os::Windows, Arch::Armv7l) | (Os::Macos, Arch::Armv7l) => None,
    }
}

pub fn deno_target(os: Os, arch: Arch) -> Option<&'static str> {
    match (os, arch) {
        (Os::Windows, Arch::X86_64) => Some("x86_64-pc-windows-msvc"),
        (Os::Windows, Arch::Aarch64) => Some("aarch64-pc-windows-msvc"),
        (Os::Linux, Arch::X86_64) => Some("x86_64-unknown-linux-gnu"),
        (Os::Linux, Arch::Aarch64) => Some("aarch64-unknown-linux-gnu"),
        (Os::Macos, Arch::X86_64) => Some("x86_64-apple-darwin"),
        (Os::Macos, Arch::Aarch64) => Some("aarch64-apple-darwin"),
        (Os::Linux, Arch::Armv7l) | (Os::Macos, Arch::Armv7l) | (Os::Windows, Arch::Armv7l) => None,
    }
}

fn release_path(version: &str) -> String {
    if version == "latest" {
        "latest/download".to_string()
    } else {
        format!("download/{}", version)
    }
}

pub fn ffmpeg_supported() -> bool {
    ffmpeg_asset_filename(platform::current_os(), platform::current_arch()).is_some()
}

pub fn deno_supported() -> bool {
    deno_target(platform::current_os(), platform::current_arch()).is_some()
}

pub fn try_ffmpeg_download_url(version: &str) -> std::result::Result<String, String> {
    let os = platform::current_os();
    let arch = platform::current_arch();
    let filename = ffmpeg_asset_filename(os, arch)
        .ok_or_else(|| format!("No ffmpeg binary for {} {}", os, arch))?;
    Ok(format!(
        "https://github.com/eugeneware/ffmpeg-static/releases/{}/{}",
        release_path(version),
        filename
    ))
}

pub fn ffmpeg_download_url(version: &str) -> String {
    try_ffmpeg_download_url(version).expect("No ffmpeg for this platform")
}

pub fn try_deno_download_url(version: &str) -> std::result::Result<String, String> {
    let os = platform::current_os();
    let arch = platform::current_arch();
    let target =
        deno_target(os, arch).ok_or_else(|| format!("No deno binary for {} {}", os, arch))?;
    Ok(format!(
        "https://github.com/denoland/deno/releases/{}/deno-{}.zip",
        release_path(version),
        target
    ))
}

pub fn deno_download_url(version: &str) -> String {
    try_deno_download_url(version).expect("No deno for this platform")
}

pub fn try_ffprobe_download_url(version: &str) -> std::result::Result<String, String> {
    let os = platform::current_os();
    let arch = platform::current_arch();
    let filename = ffprobe_asset_filename(os, arch)
        .ok_or_else(|| format!("No ffprobe binary for {} {}", os, arch))?;
    Ok(format!(
        "https://github.com/eugeneware/ffmpeg-static/releases/{}/{}",
        release_path(version),
        filename
    ))
}

pub fn ffprobe_download_url(version: &str) -> String {
    try_ffprobe_download_url(version).expect("No ffprobe for this platform")
}

pub fn default_ffmpeg_version() -> String {
    versions::default_ffmpeg_version()
}

pub fn default_deno_version() -> String {
    versions::default_deno_version()
}

pub fn default_yt_dlp_channel() -> String {
    versions::default_yt_dlp_channel()
}
