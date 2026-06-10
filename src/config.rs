use crate::deps::platform;
use crate::error::{Result, SourisError};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub yt_dlp: YtDlpConfig,
    pub ffmpeg: FFmpegConfig,
    pub download: DownloadConfig,
    pub spotify: Option<SpotifyConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YtDlpConfig {
    pub auto_update: bool,
    pub channel: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FFmpegConfig {
    pub auto_update: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadConfig {
    pub default_format: String,
    pub default_quality: String,
    pub output_dir: PathBuf,
    pub parallel: usize,
    pub embed_metadata: bool,
    pub embed_thumbnail: bool,
    pub embed_subtitles: bool,
    pub timeout: u64,
    pub max_retries: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpotifyConfig {
    pub client_id: String,
    pub client_secret: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            yt_dlp: YtDlpConfig {
                auto_update: true,
                channel: "nightly".to_string(),
            },
            ffmpeg: FFmpegConfig { auto_update: true },
            download: DownloadConfig {
                default_format: "mp4".to_string(),
                default_quality: "1080p".to_string(),
                output_dir: default_output_dir(),
                parallel: 4,
                embed_metadata: true,
                embed_thumbnail: true,
                embed_subtitles: false,
                timeout: 300,
                max_retries: 3,
            },
            spotify: None,
        }
    }
}

fn default_output_dir() -> PathBuf {
    platform::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("downloads")
}

impl AppConfig {
    pub fn config_path() -> Option<PathBuf> {
        platform::config_dir().map(|d| d.join("config.toml"))
    }

    pub fn load() -> Result<Self> {
        let path = Self::config_path()
            .ok_or_else(|| SourisError::ConfigError("Cannot determine config directory".into()))?;

        if !path.exists() {
            let config = Self::default();
            config.save()?;
            return Ok(config);
        }

        let contents = fs_err::read_to_string(&path).map_err(|e| SourisError::io(&path, e))?;

        let config: AppConfig =
            toml::from_str(&contents).map_err(|e| SourisError::ConfigError(e.to_string()))?;

        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::config_path()
            .ok_or_else(|| SourisError::ConfigError("Cannot determine config directory".into()))?;

        if let Some(parent) = path.parent() {
            crate::utils::fs::ensure_dir(parent)?;
        }

        let contents =
            toml::to_string_pretty(self).map_err(|e| SourisError::ConfigError(e.to_string()))?;

        fs_err::write(&path, contents).map_err(|e| SourisError::io(&path, e))?;

        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<String> {
        match key {
            "yt_dlp.auto_update" => Some(self.yt_dlp.auto_update.to_string()),
            "yt_dlp.channel" => Some(self.yt_dlp.channel.clone()),
            "ffmpeg.auto_update" => Some(self.ffmpeg.auto_update.to_string()),
            "download.default_format" => Some(self.download.default_format.clone()),
            "download.default_quality" => Some(self.download.default_quality.clone()),
            "download.output_dir" => Some(self.download.output_dir.display().to_string()),
            "download.parallel" => Some(self.download.parallel.to_string()),
            "download.embed_metadata" => Some(self.download.embed_metadata.to_string()),
            "download.embed_thumbnail" => Some(self.download.embed_thumbnail.to_string()),
            "download.embed_subtitles" => Some(self.download.embed_subtitles.to_string()),
            "download.timeout" => Some(self.download.timeout.to_string()),
            "download.max_retries" => Some(self.download.max_retries.to_string()),
            _ => None,
        }
    }

    pub fn set(&mut self, key: &str, value: &str) -> Result<()> {
        match key {
            "yt_dlp.auto_update" => {
                self.yt_dlp.auto_update = value
                    .parse()
                    .map_err(|_| SourisError::ConfigError(format!("Invalid boolean: {}", value)))?;
            }
            "yt_dlp.channel" => {
                self.yt_dlp.channel = value.to_string();
            }
            "ffmpeg.auto_update" => {
                self.ffmpeg.auto_update = value
                    .parse()
                    .map_err(|_| SourisError::ConfigError(format!("Invalid boolean: {}", value)))?;
            }
            "download.default_format" => {
                self.download.default_format = value.to_string();
            }
            "download.default_quality" => {
                self.download.default_quality = value.to_string();
            }
            "download.output_dir" => {
                self.download.output_dir = PathBuf::from(value);
            }
            "download.parallel" => {
                self.download.parallel = value
                    .parse()
                    .map_err(|_| SourisError::ConfigError(format!("Invalid number: {}", value)))?;
            }
            "download.embed_metadata" => {
                self.download.embed_metadata = value
                    .parse()
                    .map_err(|_| SourisError::ConfigError(format!("Invalid boolean: {}", value)))?;
            }
            "download.embed_thumbnail" => {
                self.download.embed_thumbnail = value
                    .parse()
                    .map_err(|_| SourisError::ConfigError(format!("Invalid boolean: {}", value)))?;
            }
            "download.embed_subtitles" => {
                self.download.embed_subtitles = value
                    .parse()
                    .map_err(|_| SourisError::ConfigError(format!("Invalid boolean: {}", value)))?;
            }
            "download.timeout" => {
                self.download.timeout = value
                    .parse()
                    .map_err(|_| SourisError::ConfigError(format!("Invalid number: {}", value)))?;
            }
            "download.max_retries" => {
                self.download.max_retries = value
                    .parse()
                    .map_err(|_| SourisError::ConfigError(format!("Invalid number: {}", value)))?;
            }
            _ => return Err(SourisError::ConfigError(format!("Unknown key: {}", key))),
        }
        self.save()
    }
}
