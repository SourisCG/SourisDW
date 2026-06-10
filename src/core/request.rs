use crate::core::progress::ProgressSender;
use crate::core::types::{Format, Quality};
use std::path::PathBuf;

pub struct DownloadRequestBuilder {
    pub url: String,
    pub media_type: Option<MediaTypeHint>,
    pub format: Option<Format>,
    pub quality: Option<Quality>,
    pub output: Option<PathBuf>,
    pub parallel: Option<usize>,
    pub embed_metadata: Option<bool>,
    pub embed_thumbnail: Option<bool>,
    pub embed_subtitles: Option<bool>,
    pub on_progress: Option<ProgressSender>,
    pub timeout: Option<u64>,
    pub max_retries: Option<u32>,
    pub auto_update: Option<bool>,
}

#[derive(Debug, Clone)]
pub enum MediaTypeHint {
    Audio,
    Video,
    Playlist,
    Auto,
}

impl DownloadRequestBuilder {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            media_type: None,
            format: None,
            quality: None,
            output: None,
            parallel: None,
            embed_metadata: None,
            embed_thumbnail: None,
            embed_subtitles: None,
            on_progress: None,
            timeout: None,
            max_retries: None,
            auto_update: None,
        }
    }

    pub fn media_type(mut self, hint: MediaTypeHint) -> Self {
        self.media_type = Some(hint);
        self
    }

    pub fn format(mut self, format: impl Into<Format>) -> Self {
        self.format = Some(format.into());
        self
    }

    pub fn format_str(mut self, s: &str) -> crate::error::Result<Self> {
        self.format = Some(s.parse()?);
        Ok(self)
    }

    pub fn quality(mut self, quality: impl Into<Quality>) -> Self {
        self.quality = Some(quality.into());
        self
    }

    pub fn quality_str(mut self, s: &str) -> crate::error::Result<Self> {
        self.quality = Some(s.parse()?);
        Ok(self)
    }

    pub fn output(mut self, path: impl Into<PathBuf>) -> Self {
        self.output = Some(path.into());
        self
    }

    pub fn parallel(mut self, n: usize) -> Self {
        self.parallel = Some(n);
        self
    }

    pub fn embed_metadata(mut self, enabled: bool) -> Self {
        self.embed_metadata = Some(enabled);
        self
    }

    pub fn embed_thumbnail(mut self, enabled: bool) -> Self {
        self.embed_thumbnail = Some(enabled);
        self
    }

    pub fn embed_subtitles(mut self, enabled: bool) -> Self {
        self.embed_subtitles = Some(enabled);
        self
    }

    pub fn on_progress(mut self, sender: ProgressSender) -> Self {
        self.on_progress = Some(sender);
        self
    }

    pub fn timeout(mut self, seconds: u64) -> Self {
        self.timeout = Some(seconds);
        self
    }

    pub fn max_retries(mut self, n: u32) -> Self {
        self.max_retries = Some(n);
        self
    }

    pub fn auto_update(mut self, enabled: bool) -> Self {
        self.auto_update = Some(enabled);
        self
    }

    pub async fn run(self) -> crate::error::Result<crate::core::types::DownloadResult> {
        let downloader = crate::core::downloader::SourisDW::builder()
            .auto_update(self.auto_update.unwrap_or(true))
            .build()
            .await?;

        downloader.execute_request(self).await
    }
}

pub trait IntoFormat {
    fn into_format(self) -> Format;
}

impl IntoFormat for Format {
    fn into_format(self) -> Format {
        self
    }
}

impl IntoFormat for &str {
    fn into_format(self) -> Format {
        self.parse().unwrap()
    }
}

impl IntoFormat for String {
    fn into_format(self) -> Format {
        self.parse().unwrap()
    }
}

pub trait IntoQuality {
    fn into_quality(self) -> Quality;
}

impl IntoQuality for Quality {
    fn into_quality(self) -> Quality {
        self
    }
}

impl IntoQuality for &str {
    fn into_quality(self) -> Quality {
        self.parse().unwrap()
    }
}

impl IntoQuality for String {
    fn into_quality(self) -> Quality {
        self.parse().unwrap()
    }
}
