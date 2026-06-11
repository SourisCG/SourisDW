use crate::deps::download;
use crate::deps::platform;
use std::path::{Path, PathBuf};

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";

#[derive(Clone)]
pub struct YtDlp {
    binary_path: PathBuf,
    version: Option<String>,
    channel: String,
    deno_path: Option<PathBuf>,
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

    pub fn deno_path(&self) -> Option<&Path> {
        self.deno_path.as_deref()
    }

    pub fn command(&self) -> tokio::process::Command {
        Self::command_with(&self.binary_path, self.deno_path.as_deref())
    }

    pub fn command_with(path: &Path, deno: Option<&Path>) -> tokio::process::Command {
        let mut cmd = tokio::process::Command::new(path);
        if let Some(deno) = deno {
            cmd.arg("--js-runtimes");
            cmd.arg(format!("deno:{}", deno.display()));
        }
        cmd
    }

    pub fn unavailable(channel: &str) -> Self {
        Self {
            binary_path: PathBuf::new(),
            version: None,
            channel: channel.to_string(),
            deno_path: None,
        }
    }

    pub async fn ensure_installed_blocking(channel: &str, quiet: bool) -> Self {
        let binary_name = platform::yt_dlp_binary_name();
        let deno_name = platform::deno_binary_name();

        let bin_dir = platform::bin_dir()
            .unwrap_or_else(|| std::env::temp_dir().join("souris-dw").join("bin"));
        let binary_path = bin_dir.join(&binary_name);
        let deno_path = bin_dir.join(&deno_name);

        // Always re-download fresh copy (deps install = refresh)
        if binary_path.exists() {
            let _ = fs_err::remove_file(&binary_path);
        }

        let url = platform::yt_dlp_download_url(channel);
        if download::download_binary(&url, &binary_path, "yt-dlp", quiet)
            .await
            .is_err()
        {
            return Self::unavailable(channel);
        }

        let version = Self::get_version_blocking(&binary_path).ok();
        if version.is_none() {
            let _ = fs_err::remove_file(&binary_path);
            return Self::unavailable(channel);
        }

        let dp = if deno_path.exists() {
            Some(deno_path)
        } else {
            which::which(&deno_name).ok()
        };

        Self {
            binary_path,
            version,
            channel: channel.to_string(),
            deno_path: dp,
        }
    }

    pub async fn ensure_installed(channel: &str) -> Self {
        let binary_name = platform::yt_dlp_binary_name();
        let deno_name = platform::deno_binary_name();

        let (binary_path, deno_path) = if let Ok(system_path) = which::which(&binary_name) {
            let system_deno = which::which(&deno_name).ok();
            (system_path, system_deno)
        } else {
            let bin_dir = platform::bin_dir()
                .unwrap_or_else(|| std::env::temp_dir().join("souris-dw").join("bin"));
            let bp = bin_dir.join(&binary_name);
            let dp = bin_dir.join(&deno_name);
            let system_deno = which::which(&deno_name).ok();
            (bp, if dp.exists() { Some(dp) } else { system_deno })
        };

        if binary_path.exists() {
            let version = Self::get_version(&binary_path).await.ok();
            return Self {
                binary_path,
                version,
                channel: channel.to_string(),
                deno_path,
            };
        }

        Self::download(binary_path.parent().unwrap_or(Path::new(".")), channel).await
    }

    pub async fn download(bin_dir: &Path, channel: &str) -> Self {
        let binary_name = platform::yt_dlp_binary_name();
        let deno_name = platform::deno_binary_name();
        let binary_path = bin_dir.join(&binary_name);
        let deno_path = bin_dir.join(&deno_name);
        let url = platform::yt_dlp_download_url(channel);

        match download::download_binary(&url, &binary_path, "yt-dlp", false).await {
            Ok(_) => {}
            Err(e) => {
                tracing::warn!("Failed to download yt-dlp: {}", e);
                return Self::unavailable(channel);
            }
        }

        let version = match Self::get_version(&binary_path).await {
            Ok(v) => Some(v),
            Err(_) => {
                let _ = fs_err::remove_file(&binary_path);
                tracing::warn!("Downloaded yt-dlp binary is corrupted or invalid");
                return Self::unavailable(channel);
            }
        };

        let dp = if deno_path.exists() {
            Some(deno_path)
        } else {
            which::which(&deno_name).ok()
        };

        tracing::info!("yt-dlp installed at: {}", binary_path.display());

        Self {
            binary_path,
            version,
            channel: channel.to_string(),
            deno_path: dp,
        }
    }

    pub async fn update_if_needed(&self) -> Option<Self> {
        if !self.is_installed() {
            return Some(Self::ensure_installed(&self.channel).await);
        }

        let latest = match Self::get_latest_version().await {
            Ok(v) => v,
            Err(e) => {
                tracing::warn!("Failed to check latest yt-dlp version: {}", e);
                return None;
            }
        };

        let needs_update = match &self.version {
            Some(current) => current.trim_start_matches('v') != latest.trim_start_matches('v'),
            None => true,
        };

        if !needs_update {
            return None;
        }

        tracing::info!("Updating yt-dlp: {:?} -> {}", self.version, latest);
        let bin_dir = self.binary_path.parent().unwrap();
        let updated = Self::download(bin_dir, &self.channel).await;
        if updated.is_installed() {
            Some(updated)
        } else {
            tracing::warn!("Failed to update yt-dlp");
            None
        }
    }

    fn get_version_blocking(binary_path: &Path) -> crate::error::Result<String> {
        let output = std::process::Command::new(binary_path)
            .arg("--version")
            .output()
            .map_err(|e| crate::error::SourisError::io(binary_path, e))?;

        let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if version.is_empty() {
            return Err(crate::error::SourisError::DependencyNotFound {
                name: "yt-dlp".into(),
            });
        }
        Ok(version)
    }

    async fn get_version(binary_path: &Path) -> crate::error::Result<String> {
        let output = tokio::process::Command::new(binary_path)
            .arg("--version")
            .output()
            .await
            .map_err(|e| crate::error::SourisError::io(binary_path, e))?;

        let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if version.is_empty() {
            return Err(crate::error::SourisError::DependencyNotFound {
                name: "yt-dlp".into(),
            });
        }
        Ok(version)
    }

    async fn get_latest_version() -> crate::error::Result<String> {
        let cache_dir = platform::cache_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
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
            .user_agent(USER_AGENT)
            .build()
            .map_err(|e| crate::error::SourisError::DependencyUpdateFailed {
                name: "yt-dlp".into(),
                reason: e.to_string(),
            })?;
        let response = client.get(url).send().await.map_err(|e| {
            crate::error::SourisError::DependencyUpdateFailed {
                name: "yt-dlp".into(),
                reason: e.to_string(),
            }
        })?;

        let json: serde_json::Value = response.json().await.map_err(|e| {
            crate::error::SourisError::DependencyUpdateFailed {
                name: "yt-dlp".into(),
                reason: e.to_string(),
            }
        })?;

        let version = json
            .get("tag_name")
            .and_then(|v| v.as_str())
            .map(|s| s.trim_start_matches('v').to_string())
            .ok_or_else(|| crate::error::SourisError::DependencyUpdateFailed {
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
