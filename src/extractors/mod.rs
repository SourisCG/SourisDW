pub mod resolver;
pub mod spotify;
pub mod youtube;

use crate::core::request::MediaTypeHint;
use crate::core::types::*;
use crate::error::Result;
use async_trait::async_trait;
use std::path::Path;

#[async_trait]
pub trait Extractor: Send + Sync {
    fn platform(&self) -> &str;
    fn can_handle(&self, url: &str) -> bool;

    async fn extract_info(&self, url: &str) -> Result<MediaInfo>;
    async fn extract_playlist_info(&self, url: &str) -> Result<Vec<MediaInfo>>;
    async fn search(&self, query: &str, limit: usize) -> Result<Vec<SearchItem>>;

    #[allow(clippy::too_many_arguments)]
    async fn download(
        &self,
        url: &str,
        format: Option<&Format>,
        quality: Option<&Quality>,
        output_dir: &str,
        embed_metadata: bool,
        embed_thumbnail: bool,
        embed_subtitles: bool,
        media_type: Option<&MediaTypeHint>,
        ffmpeg_path: Option<&Path>,
        cookies_file: Option<&str>,
        cookies_from_browser: Option<&str>,
    ) -> Result<DownloadResult>;

    #[allow(clippy::too_many_arguments)]
    async fn download_playlist(
        &self,
        url: &str,
        format: Option<&Format>,
        quality: Option<&Quality>,
        output_dir: &str,
        embed_metadata: bool,
        embed_thumbnail: bool,
        embed_subtitles: bool,
        parallel: usize,
        ffmpeg_path: Option<&Path>,
        cookies_file: Option<&str>,
        cookies_from_browser: Option<&str>,
    ) -> Result<Vec<DownloadResult>>;
}
