use crate::deps::platform;
use crate::error::Result;
use crate::utils::fs;
use std::path::{Path, PathBuf};

const FFMPEG_EMBEDDED: &[u8] = include_bytes!(env!("FFMPEG_PATH"));

pub struct FFmpeg {
    binary_path: PathBuf,
    version: Option<String>,
}

impl FFmpeg {
    pub fn binary_path(&self) -> &Path {
        &self.binary_path
    }

    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }

    pub fn is_installed(&self) -> bool {
        self.binary_path.exists()
    }

    pub fn unavailable() -> Self {
        Self {
            binary_path: PathBuf::new(),
            version: None,
        }
    }

    pub async fn ensure_installed() -> Self {
        let bin_dir = match platform::bin_dir() {
            Some(d) => d,
            None => {
                tracing::warn!("Cannot determine bin directory");
                return Self::unavailable();
            }
        };
        let binary_name = platform::ffmpeg_binary_name();
        let binary_path = bin_dir.join(&binary_name);

        if binary_path.exists() {
            let version = Self::get_version(&binary_path).await.ok();
            return Self {
                binary_path,
                version,
            };
        }

        if !FFMPEG_EMBEDDED.is_empty() {
            if let Err(e) = Self::extract_embedded(&bin_dir, &binary_path) {
                tracing::warn!("Failed to extract embedded ffmpeg: {}", e);
                return Self::unavailable();
            }
            let version = Self::get_version(&binary_path).await.ok();
            return Self {
                binary_path,
                version,
            };
        }

        tracing::warn!(
            "Embedded ffmpeg is empty (build.rs may have failed to download it). \
             Falling back to system ffmpeg..."
        );

        if let Some(system_ffmpeg) = fs::which("ffmpeg") {
            let version = Self::get_version(&system_ffmpeg).await.ok();
            return Self {
                binary_path: system_ffmpeg,
                version,
            };
        }

        tracing::warn!("ffmpeg not found via any method");
        Self::unavailable()
    }

    fn extract_embedded(bin_dir: &Path, binary_path: &Path) -> Result<()> {
        fs::ensure_dir(bin_dir)?;
        fs_err::write(binary_path, FFMPEG_EMBEDDED).map_err(|e| {
            tracing::error!("Failed to write ffmpeg: {}", e);
            crate::error::SourisError::io(binary_path, e)
        })?;
        fs::set_executable(binary_path)?;
        Ok(())
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

    pub async fn update(&self) {
        let bin_dir = match platform::bin_dir() {
            Some(d) => d,
            None => {
                tracing::warn!("Cannot determine bin directory for ffmpeg update");
                return;
            }
        };
        let binary_name = platform::ffmpeg_binary_name();
        let binary_path = bin_dir.join(&binary_name);

        if !FFMPEG_EMBEDDED.is_empty() {
            if let Err(e) = Self::extract_embedded(&bin_dir, &binary_path) {
                tracing::warn!("Failed to update ffmpeg: {}", e);
            }
        }
    }
}
