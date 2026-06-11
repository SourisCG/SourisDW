use crate::deps::platform;
use crate::error::{Result, SourisError};
use crate::utils::fs;
use std::path::{Path, PathBuf};

const DENO_EMBEDDED: &[u8] = include_bytes!(env!("DENO_PATH"));

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

    pub async fn ensure_installed() -> Result<Self> {
        let bin_dir = platform::bin_dir()
            .ok_or_else(|| SourisError::ConfigError("Cannot determine bin directory".into()))?;
        let binary_name = platform::deno_binary_name();
        let binary_path = bin_dir.join(&binary_name);

        if binary_path.exists() {
            let version = Self::get_version(&binary_path).await.ok();
            return Ok(Self {
                binary_path,
                version,
            });
        }

        if !DENO_EMBEDDED.is_empty() {
            Self::extract_embedded(&bin_dir, &binary_path)?;
            let version = Self::get_version(&binary_path).await.ok();
            return Ok(Self {
                binary_path,
                version,
            });
        }

        tracing::warn!(
            "Embedded deno is empty (build.rs may have failed to download it). \
             Falling back to system deno..."
        );

        if let Some(system_deno) = fs::which("deno") {
            let version = Self::get_version(&system_deno).await.ok();
            return Ok(Self {
                binary_path: system_deno,
                version,
            });
        }

        Err(SourisError::DependencyNotFound {
            name: "deno".into(),
        })
    }

    fn extract_embedded(bin_dir: &Path, binary_path: &Path) -> Result<()> {
        fs::ensure_dir(bin_dir)?;
        fs_err::write(binary_path, DENO_EMBEDDED).map_err(|e| SourisError::io(binary_path, e))?;
        fs::set_executable(binary_path)?;
        Ok(())
    }

    async fn get_version(binary_path: &Path) -> Result<String> {
        let output = tokio::process::Command::new(binary_path)
            .arg("--version")
            .output()
            .await
            .map_err(|e| SourisError::io(binary_path, e))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let version = stdout
            .lines()
            .next()
            .map(|l| l.to_string())
            .unwrap_or_else(|| "unknown".to_string());

        Ok(version)
    }

    pub async fn update(&self) -> Result<()> {
        let bin_dir = platform::bin_dir()
            .ok_or_else(|| SourisError::ConfigError("Cannot determine bin directory".into()))?;
        let binary_name = platform::deno_binary_name();
        let binary_path = bin_dir.join(&binary_name);

        if !DENO_EMBEDDED.is_empty() {
            Self::extract_embedded(&bin_dir, &binary_path)?;
        }

        Ok(())
    }
}
