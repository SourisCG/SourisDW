use crate::deps::download;
use crate::deps::platform;
use crate::deps::resolve;
use crate::deps::versions;
use crate::error::Result;
use std::path::{Path, PathBuf};

pub struct Deno {
    binary_path: PathBuf,
    version: Option<String>,
}

impl Deno {
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

    pub async fn ensure_installed_blocking(quiet: bool) -> Self {
        if !resolve::deno_supported() {
            tracing::warn!("deno auto-install is not available for this platform");
            return Self::unavailable();
        }

        let bin_dir = match platform::bin_dir() {
            Some(d) => d,
            None => {
                tracing::warn!("Cannot determine bin directory");
                return Self::unavailable();
            }
        };
        let binary_name = platform::deno_binary_name();
        let binary_path = bin_dir.join(&binary_name);

        // Always re-download fresh copy (deps install = refresh)
        if binary_path.exists() {
            let _ = fs_err::remove_file(&binary_path);
        }

        let version = resolve::default_deno_version();
        let url = match resolve::try_deno_download_url(&version) {
            Ok(url) => url,
            Err(e) => {
                tracing::warn!("{}", e);
                return Self::unavailable();
            }
        };

        match download::download_and_extract_zip(&url, &binary_path, &binary_name, "deno", quiet)
            .await
        {
            Ok(_) => {
                let version = Self::get_version_blocking(&binary_path).ok();
                Self {
                    binary_path,
                    version,
                }
            }
            Err(e) => {
                tracing::warn!("Failed to download deno: {}", e);
                Self::unavailable()
            }
        }
    }

    pub async fn ensure_installed() -> Self {
        Self::ensure_installed_with_quiet(false).await
    }

    pub async fn ensure_installed_with_quiet(quiet: bool) -> Self {
        if !resolve::deno_supported() {
            tracing::warn!("deno auto-install is not available for this platform");
            return Self::unavailable();
        }

        let bin_dir = match platform::bin_dir() {
            Some(d) => d,
            None => {
                tracing::warn!("Cannot determine bin directory");
                return Self::unavailable();
            }
        };
        let binary_name = platform::deno_binary_name();
        let binary_path = bin_dir.join(&binary_name);

        if binary_path.exists() {
            let version = Self::get_version(&binary_path).await.ok();
            return Self {
                binary_path,
                version,
            };
        }

        let version = resolve::default_deno_version();
        let url = match resolve::try_deno_download_url(&version) {
            Ok(url) => url,
            Err(e) => {
                tracing::warn!("{}", e);
                return Self::unavailable();
            }
        };

        match download::download_and_extract_zip(&url, &binary_path, &binary_name, "deno", quiet)
            .await
        {
            Ok(_) => {
                let version = Self::get_version(&binary_path).await.ok();
                Self {
                    binary_path,
                    version,
                }
            }
            Err(e) => {
                tracing::warn!("Failed to download deno: {}", e);
                Self::unavailable()
            }
        }
    }

    fn get_version_blocking(binary_path: &Path) -> Result<String> {
        let output = std::process::Command::new(binary_path)
            .arg("--version")
            .output()
            .map_err(|e| crate::error::SourisError::io(binary_path, e))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let version = stdout
            .lines()
            .next()
            .map(|l| l.to_string())
            .unwrap_or_else(|| "unknown".to_string());

        Ok(version)
    }

    async fn get_version(binary_path: &Path) -> Result<String> {
        let output = tokio::process::Command::new(binary_path)
            .arg("--version")
            .output()
            .await
            .map_err(|e| crate::error::SourisError::io(binary_path, e))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let version = stdout
            .lines()
            .next()
            .map(|l| l.to_string())
            .unwrap_or_else(|| "unknown".to_string());

        Ok(version)
    }

    pub async fn update(&self) -> Option<Self> {
        if !self.is_installed() {
            return Some(Self::ensure_installed().await);
        }

        let latest = match versions::fetch_latest_deno_version().await {
            Ok(v) => v,
            Err(e) => {
                tracing::warn!("Failed to check latest deno version: {}", e);
                return None;
            }
        };

        let current = self.version.as_deref().unwrap_or("");
        if current.trim_start_matches('v') == latest.trim_start_matches('v') {
            return None;
        }

        tracing::info!("Updating deno: {:?} -> {}", self.version, latest);
        let bin_dir = self.binary_path.parent().unwrap();
        let url = match resolve::try_deno_download_url(&latest) {
            Ok(url) => url,
            Err(e) => {
                tracing::warn!("{}", e);
                return None;
            }
        };
        let binary_name = platform::deno_binary_name();
        let binary_path = bin_dir.join(&binary_name);

        match download::download_and_extract_zip(&url, &binary_path, &binary_name, "deno", false)
            .await
        {
            Ok(_) => {
                let version = Self::get_version(&binary_path).await.ok();
                Some(Self {
                    binary_path,
                    version,
                })
            }
            Err(e) => {
                tracing::warn!("Failed to update deno: {}", e);
                None
            }
        }
    }
}
