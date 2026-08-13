pub mod deno;
pub mod download;
pub mod ffmpeg;
pub mod path;
pub mod platform;
pub mod resolve;
pub mod versions;
pub mod yt_dlp;

use crate::deps::deno::Deno;
use crate::deps::ffmpeg::FFmpeg;
use crate::deps::yt_dlp::YtDlp;

pub struct DepManager {
    yt_dlp: YtDlp,
    ffmpeg: FFmpeg,
    deno: Deno,
    #[allow(dead_code)]
    auto_update: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DepStatus {
    pub name: String,
    pub installed: bool,
    pub version: Option<String>,
    pub path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest: Option<String>,
    #[serde(default)]
    pub update_available: bool,
}

impl DepManager {
    /// Creates a DepManager by downloading any missing dependencies (with progress).
    pub async fn setup(auto_update: bool, channel: &str) -> Self {
        let yt_dlp = YtDlp::ensure_installed(channel).await;
        let ffmpeg = FFmpeg::ensure_installed().await;
        let deno = Deno::ensure_installed().await;

        let manager = Self {
            yt_dlp,
            ffmpeg,
            deno,
            auto_update,
        };

        if auto_update {
            if let Some(updated) = manager.yt_dlp.update_if_needed().await {
                return Self {
                    yt_dlp: updated,
                    ffmpeg: manager.ffmpeg,
                    deno: manager.deno,
                    auto_update,
                };
            }
        }

        manager
    }

    /// Creates a DepManager using existing dependencies (no download).
    /// If a dependency is missing, it will be auto-downloaded.
    pub async fn build(auto_update: bool, channel: &str) -> Self {
        Self::build_with_quiet(auto_update, channel, false).await
    }

    pub async fn build_with_quiet(auto_update: bool, channel: &str, quiet: bool) -> Self {
        let yt_dlp = YtDlp::ensure_installed_with_quiet(channel, quiet).await;
        let ffmpeg = FFmpeg::ensure_installed_with_quiet(quiet).await;
        let deno = Deno::ensure_installed_with_quiet(quiet).await;

        let manager = Self {
            yt_dlp,
            ffmpeg,
            deno,
            auto_update,
        };

        if auto_update {
            if let Some(updated) = manager.yt_dlp.update_if_needed().await {
                return Self {
                    yt_dlp: updated,
                    ffmpeg: manager.ffmpeg,
                    deno: manager.deno,
                    auto_update,
                };
            }
        }

        manager
    }

    /// Downloads dependencies with blocking IO helpers (used during CLI setup).
    /// Must be called from within a tokio runtime.
    pub async fn setup_blocking(auto_update: bool, channel: &str, quiet: bool) -> Self {
        let yt_dlp = YtDlp::ensure_installed_blocking(channel, quiet).await;
        let ffmpeg = FFmpeg::ensure_installed_blocking(quiet).await;
        let deno = Deno::ensure_installed_blocking(quiet).await;

        Self {
            yt_dlp,
            ffmpeg,
            deno,
            auto_update,
        }
    }

    pub fn yt_dlp(&self) -> &YtDlp {
        &self.yt_dlp
    }

    pub fn ffmpeg(&self) -> &FFmpeg {
        &self.ffmpeg
    }

    pub fn deno(&self) -> &Deno {
        &self.deno
    }

    pub fn status(&self) -> Vec<DepStatus> {
        vec![
            DepStatus {
                name: "yt-dlp".into(),
                installed: self.yt_dlp.is_installed(),
                version: self.yt_dlp.version().map(|s| s.to_string()),
                path: self.yt_dlp.binary_path().display().to_string(),
                latest: None,
                update_available: false,
            },
            DepStatus {
                name: "ffmpeg".into(),
                installed: self.ffmpeg.is_installed(),
                version: self.ffmpeg.version().map(|s| s.to_string()),
                path: self.ffmpeg.binary_path().display().to_string(),
                latest: None,
                update_available: false,
            },
            DepStatus {
                name: "deno".into(),
                installed: self.deno.is_installed(),
                version: self.deno.version().map(|s| s.to_string()),
                path: self.deno.binary_path().display().to_string(),
                latest: None,
                update_available: false,
            },
        ]
    }

    pub async fn update_all(&self) -> Vec<DepStatus> {
        self.update_specific(true, true, true).await
    }

    /// Updates the requested dependencies and reports their status.
    pub async fn update_specific(&self, yt_dlp: bool, ffmpeg: bool, deno: bool) -> Vec<DepStatus> {
        let mut results = Vec::new();

        if yt_dlp {
            if let Some(updated) = self.yt_dlp.update_if_needed().await {
                results.push(DepStatus {
                    name: "yt-dlp".into(),
                    installed: true,
                    version: updated.version().map(|s| s.to_string()),
                    path: updated.binary_path().display().to_string(),
                    latest: None,
                    update_available: false,
                });
            } else {
                results.push(DepStatus {
                    name: "yt-dlp".into(),
                    installed: true,
                    version: self.yt_dlp.version().map(|s| s.to_string()),
                    path: self.yt_dlp.binary_path().display().to_string(),
                    latest: None,
                    update_available: false,
                });
            }
        }

        if ffmpeg {
            if let Some(updated) = self.ffmpeg.update().await {
                results.push(DepStatus {
                    name: "ffmpeg".into(),
                    installed: true,
                    version: updated.version().map(|s| s.to_string()),
                    path: updated.binary_path().display().to_string(),
                    latest: None,
                    update_available: false,
                });
            } else {
                results.push(DepStatus {
                    name: "ffmpeg".into(),
                    installed: self.ffmpeg.is_installed(),
                    version: self.ffmpeg.version().map(|s| s.to_string()),
                    path: self.ffmpeg.binary_path().display().to_string(),
                    latest: None,
                    update_available: false,
                });
            }
        }

        if deno {
            if let Some(updated) = self.deno.update().await {
                results.push(DepStatus {
                    name: "deno".into(),
                    installed: true,
                    version: updated.version().map(|s| s.to_string()),
                    path: updated.binary_path().display().to_string(),
                    latest: None,
                    update_available: false,
                });
            } else {
                results.push(DepStatus {
                    name: "deno".into(),
                    installed: self.deno.is_installed(),
                    version: self.deno.version().map(|s| s.to_string()),
                    path: self.deno.binary_path().display().to_string(),
                    latest: None,
                    update_available: false,
                });
            }
        }

        results
    }

    /// Checks whether updates are available without installing them.
    /// Fills `latest` and `update_available` for each dependency.
    pub async fn check_updates(&self) -> Vec<DepStatus> {
        let mut status = self.status();

        for dep in &mut status {
            let latest = match dep.name.as_str() {
                "yt-dlp" => crate::deps::versions::fetch_latest_yt_dlp_version()
                    .await
                    .ok(),
                "ffmpeg" => crate::deps::versions::fetch_latest_ffmpeg_version()
                    .await
                    .ok(),
                "deno" => crate::deps::versions::fetch_latest_deno_version()
                    .await
                    .ok(),
                _ => None,
            };
            if let Some(latest) = latest {
                let current = dep.version.as_deref().unwrap_or("");
                dep.latest = Some(latest.clone());
                dep.update_available =
                    !current.is_empty() && version_outdated(current.trim(), latest.trim());
            }
        }

        status
    }
}

fn norm_version(s: &str) -> &str {
    s.trim_start_matches("deno ")
        .trim_start_matches('v')
        .split_whitespace()
        .next()
        .unwrap_or(s)
}

/// Compares a local version string against the latest release tag, tolerating
/// prefixes (e.g. "deno 2.9.5", "v1.2.3") and unknown formats (e.g. "b6.1.1").
/// Returns true when the local version is older than the latest.
fn version_outdated(current: &str, latest: &str) -> bool {
    let current = norm_version(current);
    let latest = norm_version(latest);

    if current == latest {
        return false;
    }

    // Compare numeric segments when both are pure semver-like versions.
    let segments = |s: &str| -> Option<Vec<u64>> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.is_empty() || parts.len() > 4 {
            return None;
        }
        let mut nums = Vec::with_capacity(parts.len());
        for part in parts {
            let cleaned: String = part
                .chars()
                .skip_while(|c| !c.is_ascii_digit())
                .take_while(|c| c.is_ascii_digit())
                .collect();
            if cleaned.is_empty() {
                return None;
            }
            nums.push(cleaned.parse().ok()?);
        }
        Some(nums)
    };

    match (segments(current), segments(latest)) {
        (Some(cur), Some(latest)) => {
            let max_len = cur.len().max(latest.len());
            for i in 0..max_len {
                let a = cur.get(i).copied().unwrap_or(0);
                let b = latest.get(i).copied().unwrap_or(0);
                if a != b {
                    return a < b;
                }
            }
            false
        }
        // If either side isn't comparable, only report an update when the
        // strings differ exactly (avoids false positives like "7.0.2-static" vs "b6.1.1").
        _ => current != latest,
    }
}
