use crate::core::request::MediaTypeHint;
use crate::core::types::*;
use crate::deps::yt_dlp::YtDlp;
use crate::error::{Result, SourisError};
use serde_json::Value;
use std::path::Path;

pub struct YouTubeExtractor {
    yt_dlp: YtDlp,
}

impl YouTubeExtractor {
    pub fn new(yt_dlp: YtDlp) -> Self {
        Self { yt_dlp }
    }

    pub async fn extract_info(&self, url: &str) -> Result<MediaInfo> {
        let output = self
            .yt_dlp
            .command()
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
        let output = self
            .yt_dlp
            .command()
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
        let output = self
            .yt_dlp
            .command()
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
        media_type: Option<&MediaTypeHint>,
        ffmpeg_path: Option<&std::path::Path>,
        cookies_file: Option<&str>,
        cookies_from_browser: Option<&str>,
    ) -> Result<DownloadResult> {
        let mut cmd = self.yt_dlp.command();

        cmd.arg("--newline");
        cmd.arg("--no-color");

        if let Some(ffmpeg) = ffmpeg_path {
            cmd.arg("--ffmpeg-location");
            cmd.arg(ffmpeg);
        }

        if let Some(cf) = cookies_file {
            cmd.args(["--cookies", cf]);
        }
        if let Some(cb) = cookies_from_browser {
            cmd.args(["--cookies-from-browser", cb]);
        }

        let is_audio = matches!(media_type, Some(MediaTypeHint::Audio));

        // WAV 2-step deprecated: WAV can't embed thumbnails, so direct download is fine
        let wav_2step = false;

        // Helper for audio bitrate filter
        let abr_value = |q: &AudioQuality| -> Option<&str> {
            match q {
                AudioQuality::Kbps128 => Some("128"),
                AudioQuality::Kbps192 => Some("192"),
                AudioQuality::Kbps256 => Some("256"),
                AudioQuality::Kbps320 => Some("320"),
                AudioQuality::Lossless => None,
            }
        };

        // Helper for video height filter
        let height_value = |q: &VideoQuality| -> &str {
            match q {
                VideoQuality::P360 => "360",
                VideoQuality::P480 => "480",
                VideoQuality::P720 => "720",
                VideoQuality::P1080 => "1080",
                VideoQuality::P1440 => "1440",
                VideoQuality::P4K => "2160",
                VideoQuality::P8K => "4320",
            }
        };

        if is_audio {
            let abr = quality.and_then(|q| match q {
                Quality::Audio(a) => abr_value(a),
                _ => None,
            });
            let format_str = if let Some(abr) = abr {
                format!("bestaudio[abr<={}]/bestaudio", abr)
            } else {
                "bestaudio".to_string()
            };
            cmd.args(["-f", &format_str]);
            let audio_fmt = if wav_2step {
                "vorbis".to_string()
            } else {
                format
                    .and_then(|f| match f {
                        Format::Audio(af) => Some(af.yt_dlp_format().to_string()),
                        _ => None,
                    })
                    .unwrap_or_else(|| "mp3".to_string())
            };
            cmd.args(["-x", "--audio-format", &audio_fmt]);
        } else if let Some(f) = format {
            let format_str = match f {
                Format::Audio(_) => {
                    let abr = quality.and_then(|q| match q {
                        Quality::Audio(a) => abr_value(a),
                        _ => None,
                    });
                    if let Some(abr) = abr {
                        format!("bestaudio[abr<={}]/bestaudio", abr)
                    } else {
                        "bestaudio".to_string()
                    }
                }
                Format::Video(vf) => {
                    let h = quality
                        .and_then(|q| match q {
                            Quality::Video(v) => Some(height_value(v)),
                            _ => None,
                        })
                        .unwrap_or("1080");
                    if ffmpeg_path.is_some() {
                        match vf {
                            VideoFormat::Mov => {
                                format!("bestvideo[vcodec^=avc1][ext=mp4][height<={}]+bestaudio[ext=m4a]/best[height<={}]", h, h)
                            }
                            VideoFormat::Avi => {
                                format!("bestvideo[ext=mp4][height<={}]+bestaudio[ext=m4a]/best[height<={}]", h, h)
                            }
                            _ => {
                                format!("bestvideo[height<={}]+bestaudio/best[height<={}]", h, h)
                            }
                        }
                    } else {
                        format!("best[height<={}]", h)
                    }
                }
            };
            cmd.args(["-f", &format_str]);

            if let Format::Video(vf) = f {
                if ffmpeg_path.is_some() {
                    cmd.args(["--merge-output-format", &vf.to_string()]);
                }
            }
            if let Format::Audio(af) = f {
                let fmt = if wav_2step {
                    "vorbis"
                } else {
                    af.yt_dlp_format()
                };
                cmd.args(["-x", "--audio-format", fmt]);
            }
        } else {
            if let Some(_ff) = ffmpeg_path {
                cmd.args(["-f", "bestvideo[height<=1080]+bestaudio/best[height<=1080]"]);
            } else {
                cmd.args(["-f", "best[height<=1080]"]);
            }
        }

        cmd.args(["--socket-timeout", "30", "--retries", "10"]);

        let output_template = format!("{}/%(title)s.%(ext)s", output_dir);
        cmd.args(["-o", &output_template]);
        cmd.arg("--windows-filenames");
        cmd.args(["--replace-in-metadata", "title", "\\.+$", ""]);

        let thumbnail_ok = embed_thumbnail && format.is_none_or(|f| f.supports_thumbnail());

        if embed_metadata {
            cmd.args(["--embed-metadata"]);
        }

        if thumbnail_ok {
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

        let is_403 = |stderr: &str| stderr.contains("403") || stderr.contains("HTTP Error 403");

        let output = if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            if is_403(&stderr) {
                // Retry with android client to bypass YouTube restrictions
                let mut cmd2 = self.yt_dlp.command();
                cmd2.args(["--newline", "--no-color"]);
                cmd2.args(["--extractor-args", "youtube:player_client=android"]);

                if let Some(ffmpeg) = ffmpeg_path {
                    cmd2.args(["--ffmpeg-location", &ffmpeg.display().to_string()]);
                }

                if let Some(cf) = cookies_file {
                    cmd2.args(["--cookies", cf]);
                }
                if let Some(cb) = cookies_from_browser {
                    cmd2.args(["--cookies-from-browser", cb]);
                }

                // Re-apply format selection
                if is_audio {
                    let abr = quality.and_then(|q| match q {
                        Quality::Audio(a) => abr_value(a),
                        _ => None,
                    });
                    let format_str = if let Some(abr) = abr {
                        format!("bestaudio[abr<={}]/bestaudio", abr)
                    } else {
                        "bestaudio".to_string()
                    };
                    cmd2.args(["-f", &format_str]);
                    cmd2.args(["-x", "--audio-format", "mp3"]);
                } else if let Some(f) = format {
                    let format_str = match f {
                        Format::Audio(_) => {
                            let abr = quality.and_then(|q| match q {
                                Quality::Audio(a) => abr_value(a),
                                _ => None,
                            });
                            if let Some(abr) = abr {
                                format!("bestaudio[abr<={}]/bestaudio", abr)
                            } else {
                                "bestaudio".to_string()
                            }
                        }
                        Format::Video(_) => {
                            let h = quality
                                .and_then(|q| match q {
                                    Quality::Video(v) => Some(height_value(v)),
                                    _ => None,
                                })
                                .unwrap_or("1080");
                            if ffmpeg_path.is_some() {
                                format!("bestvideo[height<={}]+bestaudio/best[height<={}]", h, h)
                            } else {
                                format!("best[height<={}]", h)
                            }
                        }
                    };
                    cmd2.args(["-f", &format_str]);
                    if let Format::Video(vf) = f {
                        if ffmpeg_path.is_some() {
                            cmd2.args(["--merge-output-format", &vf.to_string()]);
                        }
                    }
                    if let Format::Audio(af) = f {
                        cmd2.args(["-x", "--audio-format", af.yt_dlp_format()]);
                    }
                } else {
                    let ff = if ffmpeg_path.is_some() {
                        "bestvideo[height<=1080]+bestaudio/best[height<=1080]"
                    } else {
                        "best[height<=1080]"
                    };
                    cmd2.args(["-f", ff]);
                }

                cmd2.args(["--socket-timeout", "30", "--retries", "10"]);
                cmd2.args(["-o", &output_template]);
                cmd2.arg("--windows-filenames");
                cmd2.args(["--replace-in-metadata", "title", "\\.+$", ""]);

                if embed_metadata {
                    cmd2.arg("--embed-metadata");
                }
                if thumbnail_ok {
                    cmd2.arg("--embed-thumbnail");
                }
                if embed_subtitles {
                    cmd2.args(["--write-sub", "--write-auto-sub", "--sub-format", "vtt"]);
                }

                cmd2.arg(url);

                cmd2.output()
                    .await
                    .map_err(|e| SourisError::DownloadFailed {
                        reason: e.to_string(),
                    })?
            } else {
                if thumbnail_ok {
                    Self::cleanup_webp(output_dir);
                }
                return Err(SourisError::DownloadFailed {
                    reason: stderr.to_string(),
                });
            }
        } else {
            output
        };

        let stdout = String::from_utf8_lossy(&output.stdout);
        let downloaded_path = Self::extract_downloaded_path_static(&stdout, output_dir);

        // WAV 2-step: convert from ogg (with thumbnail) to wav
        let final_path = if wav_2step {
            let ogg_path = Path::new(&downloaded_path);
            let wav_path = ogg_path.with_extension("wav");
            if let Some(ffmpeg) = ffmpeg_path {
                let convert = tokio::process::Command::new(ffmpeg)
                    .args([
                        "-i",
                        &ogg_path.display().to_string(),
                        "-y",
                        &wav_path.display().to_string(),
                    ])
                    .output()
                    .await;
                match convert {
                    Ok(status) if status.status.success() => {
                        let _ = fs_err::remove_file(ogg_path);
                        wav_path.display().to_string()
                    }
                    _ => {
                        tracing::warn!("WAV conversion failed, keeping ogg file");
                        downloaded_path
                    }
                }
            } else {
                downloaded_path
            }
        } else {
            downloaded_path
        };

        if thumbnail_ok {
            Self::cleanup_thumbnail_sidecars(&final_path);
        }

        Ok(DownloadResult {
            success: true,
            path: Some(final_path),
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
        ffmpeg_path: Option<&std::path::Path>,
        cookies_file: Option<&str>,
        cookies_from_browser: Option<&str>,
    ) -> Result<Vec<DownloadResult>> {
        let items = self.extract_playlist_info(url).await?;
        let mut results = Vec::new();

        let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(parallel));
        let mut handles = Vec::new();

        let cookies_file = cookies_file.map(|s| s.to_string());
        let cookies_from_browser = cookies_from_browser.map(|s| s.to_string());

        for item in items {
            let permit = semaphore.clone().acquire_owned().await.unwrap();
            let yt_dlp = self.yt_dlp.binary_path().to_path_buf();
            let deno = self.yt_dlp.deno_path().map(|p| p.to_path_buf());
            let item_url = format!("https://youtube.com/watch?v={}", item.id);
            let format = format.cloned();
            let quality = quality.cloned();
            let output_dir = output_dir.to_string();
            let ffmpeg = ffmpeg_path.map(|p| p.to_path_buf());
            let cf = cookies_file.clone();
            let cb = cookies_from_browser.clone();

            handles.push(tokio::spawn(async move {
                let result = Self::download_single(
                    &yt_dlp,
                    deno.as_deref(),
                    &item_url,
                    format.as_ref(),
                    quality.as_ref(),
                    &output_dir,
                    embed_metadata,
                    embed_thumbnail,
                    embed_subtitles,
                    ffmpeg.as_deref(),
                    cf.as_deref(),
                    cb.as_deref(),
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
        deno_path: Option<&std::path::Path>,
        url: &str,
        format: Option<&Format>,
        quality: Option<&Quality>,
        output_dir: &str,
        embed_metadata: bool,
        embed_thumbnail: bool,
        embed_subtitles: bool,
        ffmpeg_path: Option<&std::path::Path>,
        cookies_file: Option<&str>,
        cookies_from_browser: Option<&str>,
    ) -> Result<DownloadResult> {
        // WAV 2-step deprecated: WAV can't embed thumbnails, direct download is fine
        let wav_2step = false;

        let mut cmd = YtDlp::command_with(yt_dlp_path, deno_path);

        cmd.arg("--newline");
        cmd.arg("--no-color");

        if let Some(ffmpeg) = ffmpeg_path {
            cmd.arg("--ffmpeg-location");
            cmd.arg(ffmpeg);
        }

        if let Some(cf) = cookies_file {
            cmd.args(["--cookies", cf]);
        }
        if let Some(cb) = cookies_from_browser {
            cmd.args(["--cookies-from-browser", cb]);
        }

        let abr_value = |q: &AudioQuality| -> Option<&str> {
            match q {
                AudioQuality::Kbps128 => Some("128"),
                AudioQuality::Kbps192 => Some("192"),
                AudioQuality::Kbps256 => Some("256"),
                AudioQuality::Kbps320 => Some("320"),
                AudioQuality::Lossless => None,
            }
        };

        let height_value = |q: &VideoQuality| -> &str {
            match q {
                VideoQuality::P360 => "360",
                VideoQuality::P480 => "480",
                VideoQuality::P720 => "720",
                VideoQuality::P1080 => "1080",
                VideoQuality::P1440 => "1440",
                VideoQuality::P4K => "2160",
                VideoQuality::P8K => "4320",
            }
        };

        if let Some(f) = format {
            let format_str = match f {
                Format::Audio(_) => {
                    let abr = quality.and_then(|q| match q {
                        Quality::Audio(a) => abr_value(a),
                        _ => None,
                    });
                    if let Some(abr) = abr {
                        format!("bestaudio[abr<={}]/bestaudio", abr)
                    } else {
                        "bestaudio".to_string()
                    }
                }
                Format::Video(vf) => {
                    let h = quality
                        .and_then(|q| match q {
                            Quality::Video(v) => Some(height_value(v)),
                            _ => None,
                        })
                        .unwrap_or("1080");
                    if ffmpeg_path.is_some() {
                        match vf {
                            VideoFormat::Mov => {
                                format!("bestvideo[vcodec^=avc1][ext=mp4][height<={}]+bestaudio[ext=m4a]/best[height<={}]", h, h)
                            }
                            VideoFormat::Avi => {
                                format!("bestvideo[ext=mp4][height<={}]+bestaudio[ext=m4a]/best[height<={}]", h, h)
                            }
                            _ => {
                                format!("bestvideo[height<={}]+bestaudio/best[height<={}]", h, h)
                            }
                        }
                    } else {
                        format!("best[height<={}]", h)
                    }
                }
            };
            cmd.args(["-f", &format_str]);

            if let Format::Video(vf) = f {
                if ffmpeg_path.is_some() {
                    cmd.args(["--merge-output-format", &vf.to_string()]);
                }
            }
            if let Format::Audio(af) = f {
                let fmt = if wav_2step {
                    "vorbis"
                } else {
                    af.yt_dlp_format()
                };
                cmd.args(["-x", "--audio-format", fmt]);
            }
        } else {
            if ffmpeg_path.is_some() {
                cmd.args(["-f", "bestvideo[height<=1080]+bestaudio/best[height<=1080]"]);
            } else {
                cmd.args(["-f", "best[height<=1080]"]);
            }
        }

        cmd.args(["--socket-timeout", "30", "--retries", "10"]);

        let output_template = format!("{}/%(title)s.%(ext)s", output_dir);
        cmd.args(["-o", &output_template]);
        cmd.arg("--windows-filenames");
        cmd.args(["--replace-in-metadata", "title", "\\.+$", ""]);

        let thumbnail_ok = embed_thumbnail && format.is_none_or(|f| f.supports_thumbnail());

        if embed_metadata {
            cmd.args(["--embed-metadata"]);
        }

        if thumbnail_ok {
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

        let is_403 = |stderr: &str| stderr.contains("403") || stderr.contains("HTTP Error 403");

        let output = if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            if is_403(&stderr) {
                let mut cmd2 = YtDlp::command_with(yt_dlp_path, deno_path);
                cmd2.args(["--newline", "--no-color"]);
                cmd2.args(["--extractor-args", "youtube:player_client=android"]);

                if let Some(ffmpeg) = ffmpeg_path {
                    cmd2.args(["--ffmpeg-location", &ffmpeg.display().to_string()]);
                }

                if let Some(cf) = cookies_file {
                    cmd2.args(["--cookies", cf]);
                }
                if let Some(cb) = cookies_from_browser {
                    cmd2.args(["--cookies-from-browser", cb]);
                }

                if let Some(f) = format {
                    let format_str = match f {
                        Format::Audio(_) => {
                            let abr = quality.and_then(|q| match q {
                                Quality::Audio(a) => abr_value(a),
                                _ => None,
                            });
                            if let Some(abr) = abr {
                                format!("bestaudio[abr<={}]/bestaudio", abr)
                            } else {
                                "bestaudio".to_string()
                            }
                        }
                        Format::Video(_) => {
                            let h = quality
                                .and_then(|q| match q {
                                    Quality::Video(v) => Some(height_value(v)),
                                    _ => None,
                                })
                                .unwrap_or("1080");
                            if ffmpeg_path.is_some() {
                                format!("bestvideo[height<={}]+bestaudio/best[height<={}]", h, h)
                            } else {
                                format!("best[height<={}]", h)
                            }
                        }
                    };
                    cmd2.args(["-f", &format_str]);
                    if let Format::Video(vf) = f {
                        if ffmpeg_path.is_some() {
                            cmd2.args(["--merge-output-format", &vf.to_string()]);
                        }
                    }
                    if let Format::Audio(af) = f {
                        cmd2.args(["-x", "--audio-format", af.yt_dlp_format()]);
                    }
                } else {
                    let ff = if ffmpeg_path.is_some() {
                        "bestvideo[height<=1080]+bestaudio/best[height<=1080]"
                    } else {
                        "best[height<=1080]"
                    };
                    cmd2.args(["-f", ff]);
                }

                cmd2.args(["--socket-timeout", "30", "--retries", "10"]);
                cmd2.args(["-o", &output_template]);
                cmd2.arg("--windows-filenames");
                cmd2.args(["--replace-in-metadata", "title", "\\.+$", ""]);

                if embed_metadata {
                    cmd2.arg("--embed-metadata");
                }
                if thumbnail_ok {
                    cmd2.arg("--embed-thumbnail");
                }
                if embed_subtitles {
                    cmd2.args(["--write-sub", "--write-auto-sub", "--sub-format", "vtt"]);
                }

                cmd2.arg(url);

                cmd2.output()
                    .await
                    .map_err(|e| SourisError::DownloadFailed {
                        reason: e.to_string(),
                    })?
            } else {
                if thumbnail_ok {
                    YouTubeExtractor::cleanup_webp(output_dir);
                }
                return Err(SourisError::DownloadFailed {
                    reason: stderr.to_string(),
                });
            }
        } else {
            output
        };

        // WAV 2-step for playlist items
        if wav_2step {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let ogg_path_str =
                YouTubeExtractor::extract_downloaded_path_static(&stdout, output_dir);
            let ogg_path = Path::new(&ogg_path_str);
            if ogg_path.exists() {
                let wav_path = ogg_path.with_extension("wav");
                if let Some(ffmpeg) = ffmpeg_path {
                    let convert = tokio::process::Command::new(ffmpeg)
                        .args([
                            "-i",
                            &ogg_path.display().to_string(),
                            "-y",
                            &wav_path.display().to_string(),
                        ])
                        .output()
                        .await;
                    if let Ok(status) = convert {
                        if status.status.success() {
                            let _ = fs_err::remove_file(ogg_path);
                        }
                    }
                }
            }
        }

        if embed_thumbnail {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let downloaded_path =
                YouTubeExtractor::extract_downloaded_path_static(&stdout, output_dir);
            YouTubeExtractor::cleanup_thumbnail_sidecars(&downloaded_path);
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

    fn extract_downloaded_path_static(stdout: &str, output_dir: &str) -> String {
        let mut fallback = format!("{}/unknown", output_dir);
        for line in stdout.lines() {
            // Post-processing destination takes priority (final file after conversion)
            if line.contains("Destination:") {
                if let Some(path) = line.split("Destination:").nth(1) {
                    let p = path.trim().to_string();
                    if !p.ends_with(".webm") && !p.ends_with(".m4a") {
                        return p;
                    }
                    fallback = p;
                }
            }
            if line.starts_with("[download]") && line.contains("has already been downloaded") {
                if let Some(path) = line.split("has already been downloaded").next() {
                    return path.replace("[download]", "").trim().to_string();
                }
            }
        }
        fallback
    }

    /// Clean up orphan .webp files left by yt-dlp when thumbnail embedding fails.
    fn cleanup_webp(output_dir: &str) {
        if let Ok(entries) = std::fs::read_dir(output_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().is_some_and(|e| e == "webp") {
                    let _ = std::fs::remove_file(&path);
                }
            }
        }
    }

    fn cleanup_thumbnail_sidecars(downloaded_path: &str) {
        let path = Path::new(downloaded_path);
        let Some(parent) = path.parent() else {
            return;
        };
        let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
            return;
        };

        for ext in ["webp", "jpg", "jpeg", "png"] {
            let sidecar = parent.join(format!("{}.{}", stem, ext));
            if sidecar.exists() {
                let _ = fs_err::remove_file(sidecar);
            }
        }
    }
}
