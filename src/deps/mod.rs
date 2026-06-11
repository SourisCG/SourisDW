pub mod deno;
pub mod ffmpeg;
pub mod platform;
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
}

impl DepManager {
    pub async fn new(auto_update: bool, channel: &str) -> Self {
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
            },
            DepStatus {
                name: "ffmpeg".into(),
                installed: self.ffmpeg.is_installed(),
                version: self.ffmpeg.version().map(|s| s.to_string()),
                path: self.ffmpeg.binary_path().display().to_string(),
            },
            DepStatus {
                name: "deno".into(),
                installed: self.deno.is_installed(),
                version: self.deno.version().map(|s| s.to_string()),
                path: self.deno.binary_path().display().to_string(),
            },
        ]
    }

    pub async fn update_all(&self) -> Vec<DepStatus> {
        let mut results = Vec::new();

        if let Some(updated) = self.yt_dlp.update_if_needed().await {
            results.push(DepStatus {
                name: "yt-dlp".into(),
                installed: true,
                version: updated.version().map(|s| s.to_string()),
                path: updated.binary_path().display().to_string(),
            });
        } else {
            results.push(DepStatus {
                name: "yt-dlp".into(),
                installed: true,
                version: self.yt_dlp.version().map(|s| s.to_string()),
                path: self.yt_dlp.binary_path().display().to_string(),
            });
        }

        self.ffmpeg.update().await;
        results.push(DepStatus {
            name: "ffmpeg".into(),
            installed: self.ffmpeg.is_installed(),
            version: self.ffmpeg.version().map(|s| s.to_string()),
            path: self.ffmpeg.binary_path().display().to_string(),
        });

        self.deno.update().await;
        results.push(DepStatus {
            name: "deno".into(),
            installed: self.deno.is_installed(),
            version: self.deno.version().map(|s| s.to_string()),
            path: self.deno.binary_path().display().to_string(),
        });

        results
    }
}
