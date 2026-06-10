use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SourisError {
    #[error("Dependency not found: {name}")]
    DependencyNotFound { name: String },

    #[error("Dependency download failed: {name} - {reason}")]
    DependencyDownloadFailed { name: String, reason: String },

    #[error("Dependency update failed: {name} - {reason}")]
    DependencyUpdateFailed { name: String, reason: String },

    #[error("Invalid URL: {url}")]
    InvalidUrl { url: String },

    #[error("Unsupported platform: {platform}")]
    UnsupportedPlatform { platform: String },

    #[error("Unsupported format: {format}")]
    UnsupportedFormat { format: String },

    #[error("Unsupported quality: {quality}")]
    UnsupportedQuality { quality: String },

    #[error("Download failed: {reason}")]
    DownloadFailed { reason: String },

    #[error("Post-processing failed: {stage} - {reason}")]
    PostProcessFailed { stage: String, reason: String },

    #[error("IO error at {path}: {source}")]
    IoError {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("JSON parse error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Config error: {0}")]
    ConfigError(String),

    #[error("FFmpeg error: {0}")]
    FFmpegError(String),

    #[error("Metadata error: {0}")]
    MetadataError(String),

    #[error("Unicode error: {0}")]
    UnicodeError(String),

    #[error("Timeout after {seconds}s")]
    Timeout { seconds: u64 },

    #[error("Cancelled by user")]
    Cancelled,

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl SourisError {
    pub fn io(path: impl Into<PathBuf>, source: std::io::Error) -> Self {
        SourisError::IoError {
            path: path.into(),
            source,
        }
    }

    pub fn exit_code(&self) -> i32 {
        match self {
            SourisError::DependencyNotFound { .. } => 2,
            SourisError::DependencyDownloadFailed { .. } => 2,
            SourisError::DependencyUpdateFailed { .. } => 2,
            SourisError::DownloadFailed { .. } => 1,
            SourisError::HttpError(_) => 3,
            SourisError::Timeout { .. } => 3,
            SourisError::Cancelled => 0,
            _ => 1,
        }
    }
}

pub type Result<T> = std::result::Result<T, SourisError>;
