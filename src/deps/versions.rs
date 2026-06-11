use crate::deps::platform;
use crate::error::{Result, SourisError};
use std::time::SystemTime;

pub fn default_ffmpeg_version() -> String {
    "latest".to_string()
}

pub fn default_deno_version() -> String {
    "latest".to_string()
}

pub fn default_yt_dlp_channel() -> String {
    "stable".to_string()
}

pub async fn fetch_latest_ffmpeg_version() -> Result<String> {
    fetch_latest_tag("eugeneware/ffmpeg-static").await
}

pub async fn fetch_latest_deno_version() -> Result<String> {
    fetch_latest_tag("denoland/deno").await
}

pub async fn fetch_latest_yt_dlp_version() -> Result<String> {
    fetch_latest_tag("yt-dlp/yt-dlp").await
}

async fn fetch_latest_tag(repo: &str) -> Result<String> {
    let cache_key = repo.replace('/', "-");
    let cache_dir = platform::cache_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
    let cache_file = cache_dir.join(format!("{}-version.txt", cache_key));

    if let Ok(content) = fs_err::read_to_string(&cache_file) {
        if let Ok(metadata) = std::fs::metadata(&cache_file) {
            if let Ok(modified) = metadata.modified() {
                if let Ok(elapsed) = SystemTime::now().duration_since(modified) {
                    if elapsed.as_secs() < 86400 {
                        let version = content.trim().to_string();
                        if !version.is_empty() {
                            return Ok(version);
                        }
                    }
                }
            }
        }
    }

    let url = format!("https://api.github.com/repos/{}/releases/latest", repo);
    let client = reqwest::Client::builder()
        .user_agent("SourisDW/0.4.0")
        .build()
        .map_err(|e| SourisError::DependencyUpdateFailed {
            name: repo.into(),
            reason: e.to_string(),
        })?;

    let response =
        client
            .get(&url)
            .send()
            .await
            .map_err(|e| SourisError::DependencyUpdateFailed {
                name: repo.into(),
                reason: e.to_string(),
            })?;

    let json: serde_json::Value =
        response
            .json()
            .await
            .map_err(|e| SourisError::DependencyUpdateFailed {
                name: repo.into(),
                reason: e.to_string(),
            })?;

    let version = json
        .get("tag_name")
        .and_then(|v| v.as_str())
        .map(|s| s.trim_start_matches('v').to_string())
        .ok_or_else(|| SourisError::DependencyUpdateFailed {
            name: repo.into(),
            reason: "No tag_name in response".into(),
        })?;

    if let Some(parent) = cache_file.parent() {
        let _ = fs_err::create_dir_all(parent);
        let _ = fs_err::write(&cache_file, &version);
    }

    Ok(version)
}
