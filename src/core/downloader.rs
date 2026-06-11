use crate::core::progress::ProgressSender;
use crate::core::request::DownloadRequestBuilder;
use crate::core::types::*;
use crate::deps::DepManager;
use crate::error::Result;
use crate::extractors::resolver::Resolver;
use std::path::PathBuf;
use std::sync::Arc;

pub struct SourisDWBuilder {
    auto_update: bool,
    yt_dlp_channel: String,
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
    spotify_client_id: Option<String>,
    spotify_client_secret: Option<String>,
    cookies_file: Option<String>,
    cookies_from_browser: Option<String>,
}

impl Default for SourisDWBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl SourisDWBuilder {
    pub fn new() -> Self {
        Self {
            auto_update: true,
            yt_dlp_channel: "stable".to_string(),
            format: Some(Format::Video(VideoFormat::Mp4)),
            quality: Some(Quality::Video(VideoQuality::P1080)),
            output: None,
            parallel: 4,
            embed_metadata: true,
            embed_thumbnail: true,
            embed_subtitles: false,
            timeout: 300,
            max_retries: 3,
            on_progress: None,
            spotify_client_id: None,
            spotify_client_secret: None,
            cookies_file: None,
            cookies_from_browser: None,
        }
    }

    pub fn auto_update(mut self, enabled: bool) -> Self {
        self.auto_update = enabled;
        self
    }

    pub fn yt_dlp_channel(mut self, channel: impl Into<String>) -> Self {
        self.yt_dlp_channel = channel.into();
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

    pub fn spotify_credentials(mut self, client_id: String, client_secret: String) -> Self {
        self.spotify_client_id = Some(client_id);
        self.spotify_client_secret = Some(client_secret);
        self
    }

    pub fn cookies_file(mut self, path: impl Into<String>) -> Self {
        self.cookies_file = Some(path.into());
        self
    }

    pub fn cookies_from_browser(mut self, browser: impl Into<String>) -> Self {
        self.cookies_from_browser = Some(browser.into());
        self
    }

    pub async fn build(self) -> SourisDW {
        let deps = DepManager::new(self.auto_update, &self.yt_dlp_channel).await;

        let resolver = Resolver::new(
            self.spotify_client_id.clone(),
            self.spotify_client_secret.clone(),
        )
        .await;

        SourisDW {
            deps: Arc::new(deps),
            resolver: Arc::new(resolver),
            default_format: self.format,
            default_quality: self.quality,
            default_output: self
                .output
                .unwrap_or_else(|| dirs_or_default().join("downloads")),
            parallel: self.parallel,
            embed_metadata: self.embed_metadata,
            embed_thumbnail: self.embed_thumbnail,
            embed_subtitles: self.embed_subtitles,
            timeout: self.timeout,
            max_retries: self.max_retries,
            on_progress: self.on_progress,
            spotify_client_id: self.spotify_client_id,
            spotify_client_secret: self.spotify_client_secret,
            cookies_file: self.cookies_file,
            cookies_from_browser: self.cookies_from_browser,
        }
    }
}

pub struct SourisDW {
    deps: Arc<DepManager>,
    resolver: Arc<Resolver>,
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
    #[allow(dead_code)]
    spotify_client_id: Option<String>,
    #[allow(dead_code)]
    spotify_client_secret: Option<String>,
    cookies_file: Option<String>,
    cookies_from_browser: Option<String>,
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
        self.resolver.resolve_info(url).await
    }

    pub async fn search(&self, query: &str) -> Result<Vec<SearchItem>> {
        self.resolver.resolve_search(query, 10).await
    }

    pub async fn update(&self) -> Result<Vec<crate::deps::DepStatus>> {
        Ok(self.deps.update_all().await)
    }

    pub async fn update_check(&self) -> Result<Vec<crate::deps::DepStatus>> {
        Ok(self.deps.status())
    }

    pub async fn execute_request(&self, req: DownloadRequestBuilder) -> Result<DownloadResult> {
        let format = req.format.as_ref();
        let quality = req.quality.as_ref();
        let output_dir = req
            .output
            .as_ref()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| self.default_output.display().to_string());

        let ffmpeg_path = self.deps.ffmpeg().binary_path().to_path_buf();
        let cookies_file = req
            .cookies_file
            .clone()
            .or_else(|| self.cookies_file.clone());
        let cookies_from_browser = req
            .cookies_from_browser
            .clone()
            .or_else(|| self.cookies_from_browser.clone());

        self.resolver
            .resolve_download(
                &req.url,
                format,
                quality,
                &output_dir,
                req.embed_metadata.unwrap_or(self.embed_metadata),
                req.embed_thumbnail.unwrap_or(self.embed_thumbnail),
                req.embed_subtitles.unwrap_or(self.embed_subtitles),
                req.media_type.as_ref(),
                Some(&ffmpeg_path),
                cookies_file.as_deref(),
                cookies_from_browser.as_deref(),
            )
            .await
    }
}

fn dirs_or_default() -> PathBuf {
    crate::deps::platform::data_dir().unwrap_or_else(|| PathBuf::from("."))
}
