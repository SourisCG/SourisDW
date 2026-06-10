use crate::error::{Result, SourisError};
use crate::deps::DepManager;
use crate::core::types::*;
use crate::core::request::DownloadRequestBuilder;
use crate::core::progress::ProgressSender;
use std::path::PathBuf;
use std::sync::Arc;

pub struct SourisDWBuilder {
    auto_update: bool,
    format: Option<Format>,
    quality: Option<Quality>,
    output: Option<PathBuf>,
    parallel: usize,
    embed_metadata: bool,
    embed_thumbnail: bool,
    embed_subtitles: bool,
    timeout: u64,
    max_retries: u32,
    on_progress: Option<ProgressSender>,
}

impl SourisDWBuilder {
    pub fn new() -> Self {
        Self {
            auto_update: true,
            format: None,
            quality: None,
            output: None,
            parallel: 4,
            embed_metadata: true,
            embed_thumbnail: true,
            embed_subtitles: false,
            timeout: 300,
            max_retries: 3,
            on_progress: None,
        }
    }

    pub fn auto_update(mut self, enabled: bool) -> Self {
        self.auto_update = enabled;
        self
    }

    pub fn format(mut self, format: impl Into<Format>) -> Self {
        self.format = Some(format.into());
        self
    }

    pub fn format_str(mut self, s: &str) -> Result<Self> {
        self.format = Some(s.parse()?);
        Ok(self)
    }

    pub fn quality(mut self, quality: impl Into<Quality>) -> Self {
        self.quality = Some(quality.into());
        self
    }

    pub fn quality_str(mut self, s: &str) -> Result<Self> {
        self.quality = Some(s.parse()?);
        Ok(self)
    }

    pub fn output(mut self, path: impl Into<PathBuf>) -> Self {
        self.output = Some(path.into());
        self
    }

    pub fn parallel(mut self, n: usize) -> Self {
        self.parallel = n;
        self
    }

    pub fn embed_metadata(mut self, enabled: bool) -> Self {
        self.embed_metadata = enabled;
        self
    }

    pub fn embed_thumbnail(mut self, enabled: bool) -> Self {
        self.embed_thumbnail = enabled;
        self
    }

    pub fn embed_subtitles(mut self, enabled: bool) -> Self {
        self.embed_subtitles = enabled;
        self
    }

    pub fn timeout(mut self, seconds: u64) -> Self {
        self.timeout = seconds;
        self
    }

    pub fn max_retries(mut self, n: u32) -> Self {
        self.max_retries = n;
        self
    }

    pub fn on_progress(mut self, sender: ProgressSender) -> Self {
        self.on_progress = Some(sender);
        self
    }

    pub async fn build(self) -> Result<SourisDW> {
        let deps = DepManager::new(self.auto_update).await?;

        Ok(SourisDW {
            deps: Arc::new(deps),
            default_format: self.format,
            default_quality: self.quality,
            default_output: self.output.unwrap_or_else(|| {
                dirs_or_default().join("downloads")
            }),
            parallel: self.parallel,
            embed_metadata: self.embed_metadata,
            embed_thumbnail: self.embed_thumbnail,
            embed_subtitles: self.embed_subtitles,
            timeout: self.timeout,
            max_retries: self.max_retries,
            on_progress: self.on_progress,
        })
    }
}

pub struct SourisDW {
    deps: Arc<DepManager>,
    default_format: Option<Format>,
    default_quality: Option<Quality>,
    default_output: PathBuf,
    parallel: usize,
    embed_metadata: bool,
    embed_thumbnail: bool,
    embed_subtitles: bool,
    timeout: u64,
    max_retries: u32,
    on_progress: Option<ProgressSender>,
}

impl SourisDW {
    pub fn builder() -> SourisDWBuilder {
        SourisDWBuilder::new()
    }

    pub fn download(&self, url: &str) -> DownloadRequestBuilder {
        let mut req = DownloadRequestBuilder::new(url);
        if let Some(ref f) = self.default_format {
            req = req.format(f.clone());
        }
        if let Some(ref q) = self.default_quality {
            req = req.quality(q.clone());
        }
        req = req.output(&self.default_output);
        req = req.parallel(self.parallel);
        req = req.embed_metadata(self.embed_metadata);
        req = req.embed_thumbnail(self.embed_thumbnail);
        req = req.embed_subtitles(self.embed_subtitles);
        req = req.timeout(self.timeout);
        req = req.max_retries(self.max_retries);
        if let Some(ref sender) = self.on_progress {
            req = req.on_progress(sender.clone());
        }
        req
    }

    pub fn download_audio(&self, url: &str) -> DownloadRequestBuilder {
        self.download(url)
            .media_type(crate::core::request::MediaTypeHint::Audio)
    }

    pub fn download_video(&self, url: &str) -> DownloadRequestBuilder {
        self.download(url)
            .media_type(crate::core::request::MediaTypeHint::Video)
    }

    pub fn download_playlist(&self, url: &str) -> DownloadRequestBuilder {
        self.download(url)
            .media_type(crate::core::request::MediaTypeHint::Playlist)
    }

    pub async fn info(&self, url: &str) -> Result<MediaInfo> {
        let yt_dlp = self.deps.yt_dlp();
        let output = tokio::process::Command::new(yt_dlp.binary_path())
            .args(["--dump-json", "--no-download", url])
            .output()
            .await
            .map_err(|e| SourisError::DownloadFailed {
                reason: e.to_string(),
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(SourisError::DownloadFailed {
                reason: stderr.to_string(),
            });
        }

        let info: serde_json::Value = serde_json::from_slice(&output.stdout)?;
        let title = info["title"].as_str().unwrap_or("Unknown").to_string();
        let id = info["id"].as_str().unwrap_or("").to_string();
        let platform = info["extractor_key"].as_str().unwrap_or("Unknown").to_string();
        let duration = info["duration"].as_u64();
        let uploader = info["uploader"].as_str().map(|s| s.to_string());
        let thumbnail = info["thumbnail"].as_str().map(|s| s.to_string());

        Ok(MediaInfo {
            id,
            title,
            platform,
            media_type: MediaType::Video,
            duration,
            uploader,
            thumbnail,
            formats: vec![],
            subtitles: std::collections::HashMap::new(),
            playlist: None,
        })
    }

    pub async fn search(&self, query: &str) -> Result<Vec<SearchItem>> {
        let yt_dlp = self.deps.yt_dlp();
        let output = tokio::process::Command::new(yt_dlp.binary_path())
            .args([
                "--dump-json",
                "--flat-playlist",
                "--no-download",
                &format!("ytsearch10:{}", query),
            ])
            .output()
            .await
            .map_err(|e| SourisError::DownloadFailed {
                reason: e.to_string(),
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(SourisError::DownloadFailed {
                reason: stderr.to_string(),
            });
        }

        let mut items = Vec::new();
        for line in String::from_utf8_lossy(&output.stdout).lines() {
            if let Ok(info) = serde_json::from_str::<serde_json::Value>(line) {
                let id = info["id"].as_str().unwrap_or("").to_string();
                let title = info["title"].as_str().unwrap_or("Unknown").to_string();
                let url = info["url"].as_str().unwrap_or(&format!("https://youtube.com/watch?v={}", id)).to_string();
                let platform = info["extractor_key"].as_str().unwrap_or("YouTube").to_string();
                let thumbnail = info["thumbnail"].as_str().map(|s| s.to_string());
                let duration = info["duration"].as_u64();
                let uploader = info["uploader"].as_str().map(|s| s.to_string());

                items.push(SearchItem {
                    id,
                    title,
                    platform,
                    url,
                    thumbnail,
                    duration,
                    uploader,
                });
            }
        }

        Ok(items)
    }

    pub async fn update(&self) -> Result<Vec<crate::deps::DepStatus>> {
        self.deps.update_all().await
    }

    pub async fn update_check(&self) -> Result<Vec<crate::deps::DepStatus>> {
        Ok(self.deps.status())
    }

    pub async fn execute_request(&self, req: DownloadRequestBuilder) -> Result<DownloadResult> {
        let yt_dlp = self.deps.yt_dlp();
        let mut cmd = tokio::process::Command::new(yt_dlp.binary_path());

        cmd.arg("--newline");
        cmd.arg("--no-color");

        let format_str = req.format.as_ref().map(|f| match f {
            Format::Audio(_) => {
                let abr = req.quality.as_ref().and_then(|q| match q {
                    Quality::Audio(a) => match a {
                        AudioQuality::Kbps128 => Some("128"),
                        AudioQuality::Kbps192 => Some("192"),
                        AudioQuality::Kbps256 => Some("256"),
                        AudioQuality::Kbps320 => Some("320"),
                        AudioQuality::Lossless => Some("0"),
                    },
                    _ => None,
                });
                if let Some(abr) = abr {
                    format!("-f ba[abr<={}]/ba", abr)
                } else {
                    "-f ba".to_string()
                }
            }
            Format::Video(_) => {
                let height = req.quality.as_ref().and_then(|q| match q {
                    Quality::Video(v) => match v {
                        VideoQuality::P360 => Some("360"),
                        VideoQuality::P480 => Some("480"),
                        VideoQuality::P720 => Some("720"),
                        VideoQuality::P1080 => Some("1080"),
                        VideoQuality::P1440 => Some("1440"),
                        VideoQuality::P4K => Some("2160"),
                        VideoQuality::P8K => Some("4320"),
                    },
                    _ => None,
                });
                if let Some(h) = height {
                    format!("-f bv[height<={}]+ba/b[height<={}]", h, h)
                } else {
                    "-f bv+ba/b".to_string()
                }
            }
        });

        if let Some(ref f) = format_str {
            cmd.args(["-f", f]);
        }

        let is_audio = req.format.as_ref().map(|f| matches!(f, Format::Audio(_))).unwrap_or(false);
        if is_audio {
            cmd.args(["-x", "--audio-format", &req.format.as_ref().unwrap().to_string()]);
        }

        let output_template = req.output.as_ref().map(|o| {
            let template = "%(title)s.%(ext)s";
            o.join(template).display().to_string()
        });

        if let Some(ref template) = output_template {
            cmd.args(["-o", template]);
        }

        cmd.arg(&req.url);

        let output = cmd.output().await.map_err(|e| SourisError::DownloadFailed {
            reason: e.to_string(),
        })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(SourisError::DownloadFailed {
                reason: stderr.to_string(),
            });
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let downloaded_path = stdout
            .lines()
            .filter(|l| l.starts_with("[download]") && l.contains("has already been downloaded"))
            .next()
            .or_else(|| {
                stdout
                    .lines()
                    .filter(|l| l.starts_with("[download]") && l.contains("Destination:"))
                    .next()
            })
            .and_then(|l| {
                l.split("Destination:").nth(1).or_else(|| l.split("has already been downloaded").next()).map(|s| s.trim().to_string())
            })
            .unwrap_or_else(|| "unknown".to_string());

        Ok(DownloadResult {
            success: true,
            path: Some(downloaded_path),
            size: None,
            error: None,
            elapsed: None,
        })
    }
}

fn dirs_or_default() -> PathBuf {
    crate::deps::platform::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
}
