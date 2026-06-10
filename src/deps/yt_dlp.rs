use std::path::{Path, PathBuf};
use crate::error::{Result, SourisError};
use crate::deps::platform;
use crate::utils::fs;

pub struct YtDlp {
    binary_path: PathBuf,
    version: Option<String>,
}

impl YtDlp {
    pub fn binary_path(&self) -> &Path {
        &self.binary_path
    }

    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }

    pub fn is_installed(&self) -> bool {
        self.binary_path.exists()
    }

    pub async fn ensure_installed() -> Result<Self> {
        let bin_dir = platform::bin_dir()
            .ok_or_else(|| SourisError::ConfigError("Cannot determine bin directory".into()))?;
        let binary_name = platform::yt_dlp_binary_name();
        let binary_path = bin_dir.join(&binary_name);

        if binary_path.exists() {
            let version = Self::get_version(&binary_path).await.ok();
            return Ok(Self { binary_path, version });
        }

        Self::download(&bin_dir).await
    }

    pub async fn download(bin_dir: &Path) -> Result<Self> {
        fs::ensure_dir(bin_dir)?;

        let binary_name = platform::yt_dlp_binary_name();
        let binary_path = bin_dir.join(&binary_name);
        let url = platform::yt_dlp_download_url("latest");

        tracing::info!("Downloading yt-dlp from: {}", url);

        let response = reqwest::get(&url)
            .await
            .map_err(|e| SourisError::DependencyDownloadFailed {
                name: "yt-dlp".into(),
                reason: e.to_string(),
            })?;

        if !response.status().is_success() {
            return Err(SourisError::DependencyDownloadFailed {
                name: "yt-dlp".into(),
                reason: format!("HTTP {}", response.status()),
            });
        }

        let bytes = response.bytes().await.map_err(|e| SourisError::DependencyDownloadFailed {
            name: "yt-dlp".into(),
            reason: e.to_string(),
        })?;

        fs_err::write(&binary_path, &bytes).map_err(|e| SourisError::io(&binary_path, e))?;
        fs::set_executable(&binary_path)?;

        let version = Self::get_version(&binary_path).await.ok();

        tracing::info!("yt-dlp installed at: {}", binary_path.display());

        Ok(Self { binary_path, version })
    }

    pub async fn update_if_needed(&self) -> Result<Option<Self>> {
        if !self.is_installed() {
            return Ok(Some(Self::ensure_installed().await?));
        }

        match self.version.as_deref() {
            Some(_) => {
                let latest = Self::get_latest_version().await?;
                let needs_update = match &self.version {
                    Some(current) => current != &latest,
                    None => true,
                };

                if needs_update {
                    tracing::info!("Updating yt-dlp: {:?} -> {}", self.version, latest);
                    let bin_dir = self.binary_path.parent().unwrap();
                    Ok(Some(Self::download(bin_dir).await?))
                } else {
                    Ok(None)
                }
            }
            None => {
                let bin_dir = self.binary_path.parent().unwrap();
                Ok(Some(Self::download(bin_dir).await?))
            }
        }
    }

    async fn get_version(binary_path: &Path) -> Result<String> {
        let output = tokio::process::Command::new(binary_path)
            .arg("--version")
            .output()
            .await
            .map_err(|e| SourisError::io(binary_path, e))?;

        let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if version.is_empty() {
            return Err(SourisError::DependencyNotFound {
                name: "yt-dlp".into(),
            });
        }
        Ok(version)
    }

    async fn get_latest_version() -> Result<String> {
        let url = "https://api.github.com/repos/yt-dlp/yt-dlp/releases/latest";
        let client = reqwest::Client::new();
        let response = client
            .get(url)
            .header("User-Agent", "souris-dw")
            .send()
            .await
            .map_err(|e| SourisError::DependencyUpdateFailed {
                name: "yt-dlp".into(),
                reason: e.to_string(),
            })?;

        let json: serde_json::Value = response.json().await.map_err(|e| {
            SourisError::DependencyUpdateFailed {
                name: "yt-dlp".into(),
                reason: e.to_string(),
            }
        })?;

        json.get("tag_name")
            .and_then(|v| v.as_str())
            .map(|s| s.trim_start_matches('v').to_string())
            .ok_or_else(|| SourisError::DependencyUpdateFailed {
                name: "yt-dlp".into(),
                reason: "No tag_name in response".into(),
            })
    }
}
