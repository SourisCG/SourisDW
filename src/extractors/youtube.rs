use crate::core::types::*;
use crate::deps::yt_dlp::YtDlp;
use crate::error::{Result, SourisError};
use serde_json::Value;

pub struct YouTubeExtractor {
    yt_dlp: YtDlp,
}

impl YouTubeExtractor {
    pub async fn new() -> Result<Self> {
        let yt_dlp = YtDlp::ensure_installed().await?;
        Ok(Self { yt_dlp })
    }

    pub async fn extract_info(&self, url: &str) -> Result<MediaInfo> {
        let output = tokio::process::Command::new(self.yt_dlp.binary_path())
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

        let info: Value = serde_json::from_slice(&output.stdout)?;
        self.parse_media_info(&info)
    }

    pub async fn extract_playlist_info(&self, url: &str) -> Result<Vec<MediaInfo>> {
        let output = tokio::process::Command::new(self.yt_dlp.binary_path())
            .args(["--dump-json", "--flat-playlist", "--no-download", url])
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
            if let Ok(info) = serde_json::from_str::<Value>(line) {
                if let Ok(media_info) = self.parse_media_info(&info) {
                    items.push(media_info);
                }
            }
        }

        Ok(items)
    }

    pub async fn search(&self, query: &str, limit: usize) -> Result<Vec<SearchItem>> {
        let search_query = format!("ytsearch{}:{}", limit, query);
        let output = tokio::process::Command::new(self.yt_dlp.binary_path())
            .args([
                "--dump-json",
                "--flat-playlist",
                "--no-download",
                &search_query,
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
            if let Ok(info) = serde_json::from_str::<Value>(line) {
                let id = info["id"].as_str().unwrap_or("").to_string();
                let title = info["title"].as_str().unwrap_or("Unknown").to_string();
                let url = info["url"]
                    .as_str()
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| format!("https://youtube.com/watch?v={}", id));
                let platform = info["extractor_key"]
                    .as_str()
                    .unwrap_or("YouTube")
                    .to_string();
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

    #[allow(clippy::too_many_arguments)]
    pub async fn download(
        &self,
        url: &str,
        format: Option<&Format>,
        quality: Option<&Quality>,
        output_dir: &str,
        embed_metadata: bool,
        embed_thumbnail: bool,
        embed_subtitles: bool,
    ) -> Result<DownloadResult> {
        let mut cmd = tokio::process::Command::new(self.yt_dlp.binary_path());

        cmd.arg("--newline");
        cmd.arg("--no-color");

        if let Some(f) = format {
            let format_str = match f {
                Format::Audio(_) => {
                    let abr = quality.and_then(|q| match q {
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
                    let height = quality.and_then(|q| match q {
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
            };
            cmd.args(["-f", &format_str]);

            let is_audio = matches!(f, Format::Audio(_));
            if is_audio {
                cmd.args(["-x", "--audio-format", &f.to_string()]);
            }
        }

        let output_template = format!("{}/%(title)s.%(ext)s", output_dir);
        cmd.args(["-o", &output_template]);

        if embed_metadata {
            cmd.args(["--embed-metadata"]);
        }

        if embed_thumbnail {
            cmd.args(["--embed-thumbnail"]);
        }

        if embed_subtitles {
            cmd.args(["--write-sub", "--write-auto-sub", "--sub-format", "vtt"]);
        }

        cmd.arg(url);

        let output = cmd
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

        let stdout = String::from_utf8_lossy(&output.stdout);
        let downloaded_path = self.extract_downloaded_path(&stdout, output_dir);

        Ok(DownloadResult {
            success: true,
            path: Some(downloaded_path),
            size: None,
            error: None,
            elapsed: None,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn download_playlist(
        &self,
        url: &str,
        format: Option<&Format>,
        quality: Option<&Quality>,
        output_dir: &str,
        embed_metadata: bool,
        embed_thumbnail: bool,
        embed_subtitles: bool,
        parallel: usize,
    ) -> Result<Vec<DownloadResult>> {
        let items = self.extract_playlist_info(url).await?;
        let mut results = Vec::new();

        let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(parallel));
        let mut handles = Vec::new();

        for item in items {
            let permit = semaphore.clone().acquire_owned().await.unwrap();
            let yt_dlp = self.yt_dlp.binary_path().to_path_buf();
            let item_url = format!("https://youtube.com/watch?v={}", item.id);
            let format = format.cloned();
            let quality = quality.cloned();
            let output_dir = output_dir.to_string();

            handles.push(tokio::spawn(async move {
                let result = Self::download_single(
                    &yt_dlp,
                    &item_url,
                    format.as_ref(),
                    quality.as_ref(),
                    &output_dir,
                    embed_metadata,
                    embed_thumbnail,
                    embed_subtitles,
                )
                .await;
                drop(permit);
                result
            }));
        }

        for handle in handles {
            match handle.await {
                Ok(result) => results.push(result?),
                Err(e) => results.push(DownloadResult {
                    success: false,
                    path: None,
                    size: None,
                    error: Some(e.to_string()),
                    elapsed: None,
                }),
            }
        }

        Ok(results)
    }

    #[allow(clippy::too_many_arguments)]
    async fn download_single(
        yt_dlp_path: &std::path::Path,
        url: &str,
        format: Option<&Format>,
        _quality: Option<&Quality>,
        output_dir: &str,
        embed_metadata: bool,
        embed_thumbnail: bool,
        embed_subtitles: bool,
    ) -> Result<DownloadResult> {
        let mut cmd = tokio::process::Command::new(yt_dlp_path);

        cmd.arg("--newline");
        cmd.arg("--no-color");

        if let Some(f) = format {
            let format_str = match f {
                Format::Audio(_) => "-f ba".to_string(),
                Format::Video(_) => "-f bv+ba/b".to_string(),
            };
            cmd.args(["-f", &format_str]);

            let is_audio = matches!(f, Format::Audio(_));
            if is_audio {
                cmd.args(["-x", "--audio-format", &f.to_string()]);
            }
        }

        let output_template = format!("{}/%(title)s.%(ext)s", output_dir);
        cmd.args(["-o", &output_template]);

        if embed_metadata {
            cmd.args(["--embed-metadata"]);
        }

        if embed_thumbnail {
            cmd.args(["--embed-thumbnail"]);
        }

        if embed_subtitles {
            cmd.args(["--write-sub", "--write-auto-sub", "--sub-format", "vtt"]);
        }

        cmd.arg(url);

        let output = cmd
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

        Ok(DownloadResult {
            success: true,
            path: None,
            size: None,
            error: None,
            elapsed: None,
        })
    }

    fn parse_media_info(&self, info: &Value) -> Result<MediaInfo> {
        let id = info["id"].as_str().unwrap_or("").to_string();
        let title = info["title"].as_str().unwrap_or("Unknown").to_string();
        let platform = info["extractor_key"]
            .as_str()
            .unwrap_or("YouTube")
            .to_string();
        let duration = info["duration"].as_u64();
        let uploader = info["uploader"].as_str().map(|s| s.to_string());
        let thumbnail = info["thumbnail"].as_str().map(|s| s.to_string());

        let media_type = if info["_type"].as_str() == Some("playlist") {
            MediaType::Playlist
        } else if info["vcodec"]
            .as_str()
            .map(|s| s != "none")
            .unwrap_or(false)
        {
            MediaType::Video
        } else {
            MediaType::Audio
        };

        let mut formats = Vec::new();
        if let Some(fmts) = info["formats"].as_array() {
            for fmt in fmts {
                let format_id = fmt["format_id"].as_str().unwrap_or("").to_string();
                let ext = fmt["ext"].as_str().unwrap_or("").to_string();
                let acodec = fmt["acodec"].as_str().map(|s| s.to_string());
                let vcodec = fmt["vcodec"].as_str().map(|s| s.to_string());
                let abr = fmt["abr"].as_f64().map(|v| v as u32);
                let resolution = fmt["resolution"].as_str().map(|s| s.to_string());
                let filesize = fmt["filesize"].as_u64();

                let fmt_media_type = if vcodec.as_deref() == Some("none") || vcodec.is_none() {
                    MediaType::Audio
                } else {
                    MediaType::Video
                };

                formats.push(FormatInfo {
                    format_id,
                    ext,
                    media_type: fmt_media_type,
                    acodec,
                    vcodec,
                    abr,
                    resolution,
                    filesize,
                });
            }
        }

        let mut subtitles = std::collections::HashMap::new();
        if let Some(subs) = info["subtitles"].as_object() {
            for (lang, sub_list) in subs {
                if let Some(sub_array) = sub_list.as_array() {
                    let sub_infos: Vec<SubtitleInfo> = sub_array
                        .iter()
                        .filter_map(|s| {
                            Some(SubtitleInfo {
                                ext: s["ext"].as_str()?.to_string(),
                                url: s["url"].as_str().map(|u| u.to_string()),
                            })
                        })
                        .collect();
                    subtitles.insert(lang.clone(), sub_infos);
                }
            }
        }

        Ok(MediaInfo {
            id,
            title,
            platform,
            media_type,
            duration,
            uploader,
            thumbnail,
            formats,
            subtitles,
            playlist: None,
        })
    }

    fn extract_downloaded_path(&self, stdout: &str, output_dir: &str) -> String {
        for line in stdout.lines() {
            if line.starts_with("[download]") && line.contains("Destination:") {
                if let Some(path) = line.split("Destination:").nth(1) {
                    return path.trim().to_string();
                }
            }
            if line.starts_with("[download]") && line.contains("has already been downloaded") {
                if let Some(path) = line.split("has already been downloaded").next() {
                    let path = path.replace("[download]", "").trim().to_string();
                    return path;
                }
            }
        }
        format!("{}/unknown", output_dir)
    }
}
