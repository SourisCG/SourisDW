use crate::deps::download;
use crate::deps::platform;
use crate::deps::resolve;
use crate::deps::versions;
use crate::error::Result;
use std::path::{Path, PathBuf};

pub struct FFmpeg {
    binary_path: PathBuf,
    ffprobe_binary_path: PathBuf,
    version: Option<String>,
}

impl FFmpeg {
    pub fn binary_path(&self) -> &Path {
        &self.binary_path
    }

    pub fn ffprobe_path(&self) -> &Path {
        &self.ffprobe_binary_path
    }

    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }

    pub fn is_installed(&self) -> bool {
        self.binary_path.exists()
    }

    pub fn ffprobe_is_installed(&self) -> bool {
        self.ffprobe_binary_path.exists()
    }

    pub fn unavailable() -> Self {
        Self {
            binary_path: PathBuf::new(),
            ffprobe_binary_path: PathBuf::new(),
            version: None,
        }
    }

    /// Download or verify both ffmpeg and ffprobe.
    /// When `force` is true, re-downloads ffmpeg even if it exists (version check).
    async fn ensure_both(bin_dir: &Path, force: bool, quiet: bool) -> Self {
        let ffmpeg_name = platform::ffmpeg_binary_name();
        let ffprobe_name = platform::ffprobe_binary_name();
        let ffmpeg_path = bin_dir.join(&ffmpeg_name);
        let ffprobe_path = bin_dir.join(&ffprobe_name);

        // ffmpeg
        let should_dl = if ffmpeg_path.exists() {
            match Self::get_version_blocking(&ffmpeg_path) {
                Ok(v) => {
                    if force && v != resolve::default_ffmpeg_version() {
                        tracing::info!(
                            "New ffmpeg version available ({}), re-downloading",
                            resolve::default_ffmpeg_version()
                        );
                        let _ = fs_err::remove_file(&ffmpeg_path);
                        true
                    } else {
                        false
                    }
                }
                Err(_) => {
                    tracing::warn!("Existing ffmpeg binary is corrupt, re-downloading");
                    let _ = fs_err::remove_file(&ffmpeg_path);
                    true
                }
            }
        } else {
            true
        };

        if should_dl {
            let version = resolve::default_ffmpeg_version();
            let url = resolve::ffmpeg_download_url(&version);
            if let Err(e) =
                download::download_and_decompress_gz(&url, &ffmpeg_path, "ffmpeg", quiet).await
            {
                tracing::warn!("Failed to download ffmpeg: {}", e);
                return Self::unavailable();
            }
        }

        // ffprobe
        if !ffprobe_path.exists() || force {
            if ffprobe_path.exists() {
                let _ = fs_err::remove_file(&ffprobe_path);
            }
            let version = resolve::default_ffmpeg_version();
            let url = resolve::ffprobe_download_url(&version);
            if let Err(e) =
                download::download_and_decompress_gz(&url, &ffprobe_path, "ffprobe", quiet).await
            {
                tracing::warn!("Failed to download ffprobe: {}", e);
                // non-fatal: proceed without ffprobe
            }
        }

        let version = Self::get_version_blocking(&ffmpeg_path).ok();
        Self {
            binary_path: ffmpeg_path,
            ffprobe_binary_path: ffprobe_path,
            version,
        }
    }

    pub async fn ensure_installed_blocking(quiet: bool) -> Self {
        let bin_dir = match platform::bin_dir() {
            Some(d) => d,
            None => {
                tracing::warn!("Cannot determine bin directory");
                return Self::unavailable();
            }
        };
        Self::ensure_both(&bin_dir, true, quiet).await
    }

    pub async fn ensure_installed() -> Self {
        let bin_dir = match platform::bin_dir() {
            Some(d) => d,
            None => {
                tracing::warn!("Cannot determine bin directory");
                return Self::unavailable();
            }
        };
        Self::ensure_both(&bin_dir, false, false).await
    }

    fn get_version_blocking(binary_path: &Path) -> Result<String> {
        let output = std::process::Command::new(binary_path)
            .arg("-version")
            .output()
            .map_err(|e| crate::error::SourisError::io(binary_path, e))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let version = stdout
            .lines()
            .next()
            .and_then(|line| line.split_whitespace().nth(2))
            .unwrap_or("unknown")
            .to_string();

        Ok(version)
    }

    async fn get_version(binary_path: &Path) -> Result<String> {
        let output = tokio::process::Command::new(binary_path)
            .arg("-version")
            .output()
            .await
            .map_err(|e| crate::error::SourisError::io(binary_path, e))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let version = stdout
            .lines()
            .next()
            .and_then(|line| line.split_whitespace().nth(2))
            .unwrap_or("unknown")
            .to_string();

        Ok(version)
    }

    pub async fn update(&self) -> Option<Self> {
        if !self.is_installed() {
            return Some(Self::ensure_installed().await);
        }

        let latest = match versions::fetch_latest_ffmpeg_version().await {
            Ok(v) => v,
            Err(e) => {
                tracing::warn!("Failed to check latest ffmpeg version: {}", e);
                return None;
            }
        };

        let current = self.version.as_deref().unwrap_or("");
        if current == latest {
            return None;
        }

        tracing::info!("Updating ffmpeg: {} -> {}", current, latest);
        let bin_dir = self.binary_path.parent().unwrap();

        // Update ffmpeg
        let ffmpeg_name = platform::ffmpeg_binary_name();
        let ffmpeg_path = bin_dir.join(&ffmpeg_name);
        let ffmpeg_url = resolve::ffmpeg_download_url(&latest);
        if let Err(e) =
            download::download_and_decompress_gz(&ffmpeg_url, &ffmpeg_path, "ffmpeg", false).await
        {
            tracing::warn!("Failed to update ffmpeg: {}", e);
            return None;
        }

        // Update ffprobe alongside
        let ffprobe_name = platform::ffprobe_binary_name();
        let ffprobe_path = bin_dir.join(&ffprobe_name);
        let ffprobe_url = resolve::ffprobe_download_url(&latest);
        if let Err(e) =
            download::download_and_decompress_gz(&ffprobe_url, &ffprobe_path, "ffprobe", false)
                .await
        {
            tracing::warn!("Failed to update ffprobe: {}", e);
            // non-fatal
        }

        let version = Self::get_version(&ffmpeg_path).await.ok();
        Some(Self {
            binary_path: ffmpeg_path,
            ffprobe_binary_path: ffprobe_path,
            version,
        })
    }
}
