use directories::ProjectDirs;
use std::path::PathBuf;

pub const APP_NAME: &str = "souris-dw";
pub const APP_ORG: &str = "souris";
pub const APP_QUALIFIER: &str = "io";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Os {
    Linux,
    Macos,
    Windows,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Arch {
    X86_64,
    Aarch64,
    Armv7l,
}

impl std::fmt::Display for Os {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Os::Linux => write!(f, "linux"),
            Os::Macos => write!(f, "macos"),
            Os::Windows => write!(f, "windows"),
        }
    }
}

impl std::fmt::Display for Arch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Arch::X86_64 => write!(f, "x86_64"),
            Arch::Aarch64 => write!(f, "aarch64"),
            Arch::Armv7l => write!(f, "armv7l"),
        }
    }
}

pub fn current_os() -> Os {
    match std::env::consts::OS {
        "linux" => Os::Linux,
        "macos" => Os::Macos,
        "windows" => Os::Windows,
        other => panic!("Unsupported OS: {}", other),
    }
}

pub fn try_current_os() -> Result<Os, String> {
    match std::env::consts::OS {
        "linux" => Ok(Os::Linux),
        "macos" => Ok(Os::Macos),
        "windows" => Ok(Os::Windows),
        other => Err(format!("Unsupported OS: {}", other)),
    }
}

pub fn current_arch() -> Arch {
    match std::env::consts::ARCH {
        "x86_64" => Arch::X86_64,
        "aarch64" | "arm64" => Arch::Aarch64,
        "armv7l" => Arch::Armv7l,
        other => panic!("Unsupported architecture: {}", other),
    }
}

pub fn try_current_arch() -> Result<Arch, String> {
    match std::env::consts::ARCH {
        "x86_64" => Ok(Arch::X86_64),
        "aarch64" | "arm64" => Ok(Arch::Aarch64),
        "armv7l" => Ok(Arch::Armv7l),
        other => Err(format!("Unsupported architecture: {}", other)),
    }
}

pub fn exe_extension() -> &'static str {
    std::env::consts::EXE_SUFFIX
}

pub fn project_dirs() -> Option<ProjectDirs> {
    ProjectDirs::from(APP_QUALIFIER, APP_ORG, APP_NAME)
}

pub fn config_dir() -> Option<PathBuf> {
    project_dirs().map(|d| d.config_dir().to_path_buf())
}

pub fn data_dir() -> Option<PathBuf> {
    project_dirs().map(|d| d.data_dir().to_path_buf())
}

pub fn cache_dir() -> Option<PathBuf> {
    project_dirs().map(|d| d.cache_dir().to_path_buf())
}

pub fn bin_dir() -> Option<PathBuf> {
    Some(
        data_dir()
            .map(|d| d.join("bin"))
            .unwrap_or_else(|| std::env::temp_dir().join("souris-dw").join("bin")),
    )
}

pub fn yt_dlp_binary_name() -> String {
    let os = current_os();
    let ext = exe_extension();
    match os {
        Os::Windows => format!("yt-dlp{}", ext),
        _ => "yt-dlp".to_string(),
    }
}

pub fn ffmpeg_binary_name() -> String {
    let os = current_os();
    let ext = exe_extension();
    match os {
        Os::Windows => format!("ffmpeg{}", ext),
        _ => "ffmpeg".to_string(),
    }
}

pub fn deno_binary_name() -> String {
    let os = current_os();
    let ext = exe_extension();
    match os {
        Os::Windows => format!("deno{}", ext),
        _ => "deno".to_string(),
    }
}

pub fn yt_dlp_release_filename() -> &'static str {
    let os = current_os();
    let arch = current_arch();
    match (os, arch) {
        (Os::Linux, Arch::X86_64) => "yt-dlp_linux",
        (Os::Linux, Arch::Aarch64) => "yt-dlp_linux_aarch64",
        (Os::Linux, Arch::Armv7l) => "yt-dlp_linux_armv7l",
        (Os::Macos, _) => "yt-dlp_macos",
        (Os::Windows, Arch::X86_64) => "yt-dlp.exe",
        (Os::Windows, Arch::Aarch64) => "yt-dlp_arm64.exe",
        (Os::Windows, Arch::Armv7l) => panic!("Windows ARMv7 not supported"),
    }
}

pub fn yt_dlp_download_url(version: &str) -> String {
    let release_path = if version == "latest" || version == "stable" {
        "latest/download".to_string()
    } else {
        format!("download/{}", version)
    };
    format!(
        "https://github.com/yt-dlp/yt-dlp/releases/{}/{}",
        release_path,
        yt_dlp_release_filename()
    )
}
