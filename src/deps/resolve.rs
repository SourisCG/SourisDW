use crate::deps::platform::{self, Arch, Os};
use crate::deps::versions;

pub fn ffmpeg_download_url(version: &str) -> String {
    let os = platform::current_os();
    let arch = platform::current_arch();
    let filename = match (os, arch) {
        (Os::Linux, Arch::X86_64) => "ffmpeg-linux-x64.gz",
        (Os::Linux, Arch::Aarch64) => "ffmpeg-linux-arm64.gz",
        (Os::Linux, Arch::Armv7l) => "ffmpeg-linux-arm64.gz",
        (Os::Macos, Arch::X86_64) => "ffmpeg-darwin-x64.gz",
        (Os::Macos, Arch::Aarch64) => "ffmpeg-darwin-arm64.gz",
        (Os::Windows, Arch::X86_64) => "ffmpeg-win32-x64.gz",
        (Os::Windows, Arch::Aarch64) => "ffmpeg-win32-arm64.gz",
        (Os::Windows, Arch::Armv7l) | (Os::Macos, Arch::Armv7l) => {
            panic!("No ffmpeg for this platform")
        }
    };
    let release_path = if version == "latest" {
        "latest/download".to_string()
    } else {
        format!("download/{}", version)
    };
    format!(
        "https://github.com/eugeneware/ffmpeg-static/releases/{}/{}",
        release_path, filename
    )
}

pub fn deno_download_url(version: &str) -> String {
    let os = platform::current_os();
    let arch = platform::current_arch();
    let target = match (os, arch) {
        (Os::Windows, Arch::X86_64) => "x86_64-pc-windows-msvc",
        (Os::Windows, Arch::Aarch64) => "aarch64-pc-windows-msvc",
        (Os::Linux, Arch::X86_64) => "x86_64-unknown-linux-gnu",
        (Os::Linux, Arch::Aarch64) => "aarch64-unknown-linux-gnu",
        (Os::Linux, Arch::Armv7l) => "aarch64-unknown-linux-gnu",
        (Os::Macos, Arch::Armv7l) | (Os::Windows, Arch::Armv7l) => {
            panic!("No deno for this platform")
        }
        (Os::Macos, Arch::X86_64) => "x86_64-apple-darwin",
        (Os::Macos, Arch::Aarch64) => "aarch64-apple-darwin",
    };
    let release_path = if version == "latest" {
        "latest/download".to_string()
    } else {
        format!("download/{}", version)
    };
    format!(
        "https://github.com/denoland/deno/releases/{}/deno-{}.zip",
        release_path, target
    )
}

pub fn ffprobe_download_url(version: &str) -> String {
    let os = platform::current_os();
    let arch = platform::current_arch();
    let filename = match (os, arch) {
        (Os::Linux, Arch::X86_64) => "ffprobe-linux-x64.gz",
        (Os::Linux, Arch::Aarch64) => "ffprobe-linux-arm64.gz",
        (Os::Linux, Arch::Armv7l) => "ffprobe-linux-arm64.gz",
        (Os::Macos, Arch::X86_64) => "ffprobe-darwin-x64.gz",
        (Os::Macos, Arch::Aarch64) => "ffprobe-darwin-arm64.gz",
        (Os::Windows, Arch::X86_64) => "ffprobe-win32-x64.gz",
        (Os::Windows, Arch::Aarch64) => "ffprobe-win32-arm64.gz",
        (Os::Windows, Arch::Armv7l) | (Os::Macos, Arch::Armv7l) => {
            panic!("No ffprobe for this platform")
        }
    };
    let release_path = if version == "latest" {
        "latest/download".to_string()
    } else {
        format!("download/{}", version)
    };
    format!(
        "https://github.com/eugeneware/ffmpeg-static/releases/{}/{}",
        release_path, filename
    )
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
