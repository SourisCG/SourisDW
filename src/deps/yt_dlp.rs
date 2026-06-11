use crate::deps::platform;
use crate::error::{Result, SourisError};
use crate::utils::fs;
use std::path::{Path, PathBuf};

pub struct YtDlp {
    binary_path: PathBuf,
    version: Option<String>,
    channel: String,
}

impl YtDlp {
    pub fn binary_path(&self) -> &Path {
        &self.binary_path
    }

    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }

    pub fn channel(&self) -> &str {
        &self.channel
    }

    pub fn is_installed(&self) -> bool {
        self.binary_path.exists()
    }

    pub async fn ensure_installed(channel: &str) -> Result<Self> {
        let bin_dir = platform::bin_dir()
            .ok_or_else(|| SourisError::ConfigError("Cannot determine bin directory".into()))?;
        let binary_name = platform::yt_dlp_binary_name();
        let binary_path = bin_dir.join(&binary_name);

        if binary_path.exists() {
            let version = Self::get_version(&binary_path).await.ok();
            return Ok(Self {
                binary_path,
                version,
                channel: channel.to_string(),
            });
        }

        Self::download(&bin_dir, channel).await
    }

    pub async fn download(bin_dir: &Path, channel: &str) -> Result<Self> {
        fs::ensure_dir(bin_dir)?;

        let binary_name = platform::yt_dlp_binary_name();
        let binary_path = bin_dir.join(&binary_name);
        let url = platform::yt_dlp_download_url(channel);

        tracing::info!("Downloading yt-dlp (channel: {}) from: {}", channel, url);

        let client = reqwest::Client::builder()
            .user_agent("SourisDW/0.2.0")
            .build()
            .map_err(|e| SourisError::DependencyDownloadFailed {
                name: "yt-dlp".into(),
                reason: e.to_string(),
            })?;
        let response = client
            .get(&url)
            .send()
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

        let bytes = response
            .bytes()
            .await
            .map_err(|e| SourisError::DependencyDownloadFailed {
                name: "yt-dlp".into(),
                reason: e.to_string(),
            })?;

        fs_err::write(&binary_path, &bytes).map_err(|e| SourisError::io(&binary_path, e))?;
        fs::set_executable(&binary_path)?;

        let version = Self::get_version(&binary_path).await.ok();

        tracing::info!("yt-dlp installed at: {}", binary_path.display());

        Ok(Self {
            binary_path,
            version,
            channel: channel.to_string(),
        })
    }

    pub async fn update_if_needed(&self) -> Result<Option<Self>> {
        if !self.is_installed() {
            return Ok(Some(Self::ensure_installed(&self.channel).await?));
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
                    Ok(Some(Self::download(bin_dir, &self.channel).await?))
                } else {
                    Ok(None)
                }
            }
            None => {
                let bin_dir = self.binary_path.parent().unwrap();
                Ok(Some(Self::download(bin_dir, &self.channel).await?))
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
        let cache_dir = platform::cache_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."));
        let cache_file = cache_dir.join("yt-dlp-version.txt");

        if let Ok(content) = fs_err::read_to_string(&cache_file) {
            if let Ok(metadata) = std::fs::metadata(&cache_file) {
                if let Ok(modified) = metadata.modified() {
                    if let Ok(elapsed) = std::time::SystemTime::now().duration_since(modified) {
                        if elapsed.as_secs() < 86400 {
                            let version = content.trim().to_string();
                            if !version.is_empty() {
                                return Ok(version);
                            }
                        }
                    }
                }
            }
        }

        let url = "https://api.github.com/repos/yt-dlp/yt-dlp/releases/latest";
        let client = reqwest::Client::builder()
            .user_agent("SourisDW/0.2.0")
            .build()
            .map_err(|e| SourisError::DependencyUpdateFailed {
                name: "yt-dlp".into(),
                reason: e.to_string(),
            })?;
        let response = client
            .get(url)
            .send()
            .await
            .map_err(|e| SourisError::DependencyUpdateFailed {
                name: "yt-dlp".into(),
                reason: e.to_string(),
            })?;

        let json: serde_json::Value =
            response
                .json()
                .await
                .map_err(|e| SourisError::DependencyUpdateFailed {
                    name: "yt-dlp".into(),
                    reason: e.to_string(),
                })?;

        let version = json
            .get("tag_name")
            .and_then(|v| v.as_str())
            .map(|s| s.trim_start_matches('v').to_string())
            .ok_or_else(|| SourisError::DependencyUpdateFailed {
                name: "yt-dlp".into(),
                reason: "No tag_name in response".into(),
            })?;

        if let Some(parent) = cache_file.parent() {
            let _ = fs_err::create_dir_all(parent);
            let _ = fs_err::write(&cache_file, &version);
        }

        Ok(version)
    }
}
