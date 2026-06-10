pub mod ffmpeg;
pub mod platform;
pub mod yt_dlp;

use crate::deps::ffmpeg::FFmpeg;
use crate::deps::yt_dlp::YtDlp;
use crate::error::Result;

pub struct DepManager {
    yt_dlp: YtDlp,
    ffmpeg: FFmpeg,
    #[allow(dead_code)]
    auto_update: bool,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct DepStatus {
    pub name: String,
    pub installed: bool,
    pub version: Option<String>,
    pub path: String,
}

impl DepManager {
    pub async fn new(auto_update: bool) -> Result<Self> {
        let yt_dlp = YtDlp::ensure_installed().await?;
        let ffmpeg = FFmpeg::ensure_installed().await?;

        let manager = Self {
            yt_dlp,
            ffmpeg,
            auto_update,
        };

        if auto_update {
            if let Some(updated) = manager.yt_dlp.update_if_needed().await? {
                let mut m = manager;
                m.yt_dlp = updated;
                return Ok(m);
            }
        }

        Ok(manager)
    }

    pub fn yt_dlp(&self) -> &YtDlp {
        &self.yt_dlp
    }

    pub fn ffmpeg(&self) -> &FFmpeg {
        &self.ffmpeg
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
        ]
    }

    pub async fn update_all(&self) -> Result<Vec<DepStatus>> {
        let mut results = Vec::new();

        if let Some(updated) = self.yt_dlp.update_if_needed().await? {
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

        results.push(DepStatus {
            name: "ffmpeg".into(),
            installed: self.ffmpeg.is_installed(),
            version: self.ffmpeg.version().map(|s| s.to_string()),
            path: self.ffmpeg.binary_path().display().to_string(),
        });

        Ok(results)
    }
}
