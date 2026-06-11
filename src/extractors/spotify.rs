use crate::error::{Result, SourisError};
use serde_json::Value;
use std::sync::Mutex;
use std::time::{Duration, Instant};

const TOKEN_TTL: u64 = 3300;

pub struct SpotifyExtractor {
    client_id: Option<String>,
    client_secret: Option<String>,
    token_cache: Mutex<Option<(String, Instant)>>,
}

impl SpotifyExtractor {
    pub fn new(client_id: Option<String>, client_secret: Option<String>) -> Self {
        Self {
            client_id,
            client_secret,
            token_cache: Mutex::new(None),
        }
    }

    pub async fn extract_track_info(&self, url: &str) -> Result<SpotifyTrackInfo> {
        let track_id = self.extract_track_id(url)?;
        let access_token = self.get_access_token().await?;

        let api_url = format!("https://api.spotify.com/v1/tracks/{}", track_id);
        let client = reqwest::Client::new();

        let response = client
            .get(&api_url)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await
            .map_err(|e| SourisError::DownloadFailed {
                reason: e.to_string(),
            })?;

        if !response.status().is_success() {
            return Err(SourisError::DownloadFailed {
                reason: format!("Spotify API error: {}", response.status()),
            });
        }

        let data: Value = response
            .json()
            .await
            .map_err(|e| SourisError::DownloadFailed {
                reason: e.to_string(),
            })?;

        let name = data["name"].as_str().unwrap_or("Unknown").to_string();
        let artist = data["artists"]
            .as_array()
            .and_then(|a| a.first())
            .and_then(|a| a["name"].as_str())
            .unwrap_or("Unknown")
            .to_string();
        let album = data["album"]["name"]
            .as_str()
            .unwrap_or("Unknown")
            .to_string();
        let duration_ms = data["duration_ms"].as_u64().unwrap_or(0);
        let thumbnail = data["album"]["images"]
            .as_array()
            .and_then(|i| i.first())
            .and_then(|i| i["url"].as_str())
            .map(|s| s.to_string());

        Ok(SpotifyTrackInfo {
            id: track_id,
            name,
            artist,
            album,
            duration_ms,
            thumbnail,
        })
    }

    pub async fn extract_playlist_info(&self, url: &str) -> Result<Vec<SpotifyTrackInfo>> {
        let playlist_id = self.extract_playlist_id(url)?;
        let access_token = self.get_access_token().await?;

        let mut items = Vec::new();
        let mut next_url = Some(format!(
            "https://api.spotify.com/v1/playlists/{}/tracks?limit=100",
            playlist_id
        ));

        while let Some(api_url) = next_url {
            let client = reqwest::Client::new();
            let response = client
                .get(&api_url)
                .header("Authorization", format!("Bearer {}", access_token))
                .send()
                .await
                .map_err(|e| SourisError::DownloadFailed {
                    reason: e.to_string(),
                })?;

            if !response.status().is_success() {
                return Err(SourisError::DownloadFailed {
                    reason: format!("Spotify API error: {}", response.status()),
                });
            }

            let data: Value = response
                .json()
                .await
                .map_err(|e| SourisError::DownloadFailed {
                    reason: e.to_string(),
                })?;

            if let Some(tracks) = data["items"].as_array() {
                for item in tracks {
                    let track = &item["track"];
                    if track.is_null() {
                        continue;
                    }

                    let id = track["id"].as_str().unwrap_or("").to_string();
                    let name = track["name"].as_str().unwrap_or("Unknown").to_string();
                    let artist = track["artists"]
                        .as_array()
                        .and_then(|a| a.first())
                        .and_then(|a| a["name"].as_str())
                        .unwrap_or("Unknown")
                        .to_string();
                    let album = track["album"]["name"]
                        .as_str()
                        .unwrap_or("Unknown")
                        .to_string();
                    let duration_ms = track["duration_ms"].as_u64().unwrap_or(0);
                    let thumbnail = track["album"]["images"]
                        .as_array()
                        .and_then(|i| i.first())
                        .and_then(|i| i["url"].as_str())
                        .map(|s| s.to_string());

                    items.push(SpotifyTrackInfo {
                        id,
                        name,
                        artist,
                        album,
                        duration_ms,
                        thumbnail,
                    });
                }
            }

            next_url = data["next"].as_str().map(|s| s.to_string());
        }

        Ok(items)
    }

    fn extract_track_id(&self, url: &str) -> Result<String> {
        let url = url.trim_end_matches('/');
        let parts: Vec<&str> = url.split('/').collect();

        for (i, part) in parts.iter().enumerate() {
            if *part == "track" {
                if let Some(id) = parts.get(i + 1) {
                    let id = id.split('?').next().unwrap_or(id);
                    return Ok(id.to_string());
                }
            }
        }

        Err(SourisError::InvalidUrl {
            url: url.to_string(),
        })
    }

    fn extract_playlist_id(&self, url: &str) -> Result<String> {
        let url = url.trim_end_matches('/');
        let parts: Vec<&str> = url.split('/').collect();

        for (i, part) in parts.iter().enumerate() {
            if *part == "playlist" {
                if let Some(id) = parts.get(i + 1) {
                    let id = id.split('?').next().unwrap_or(id);
                    return Ok(id.to_string());
                }
            }
        }

        Err(SourisError::InvalidUrl {
            url: url.to_string(),
        })
    }

    async fn get_access_token(&self) -> Result<String> {
        if let Some((token, expires_at)) = self.token_cache.lock().unwrap().as_ref() {
            if Instant::now() < *expires_at {
                return Ok(token.clone());
            }
        }

        let client_id = self
            .client_id
            .as_deref()
            .ok_or_else(|| SourisError::ConfigError("Spotify client_id not configured".into()))?;
        let client_secret = self.client_secret.as_deref().ok_or_else(|| {
            SourisError::ConfigError("Spotify client_secret not configured".into())
        })?;

        let client = reqwest::Client::new();
        let params = [
            ("grant_type", "client_credentials"),
            ("client_id", client_id),
            ("client_secret", client_secret),
        ];

        let response = client
            .post("https://accounts.spotify.com/api/token")
            .form(&params)
            .send()
            .await
            .map_err(|e| SourisError::DownloadFailed {
                reason: e.to_string(),
            })?;

        if !response.status().is_success() {
            return Err(SourisError::DownloadFailed {
                reason: format!("Spotify auth error: {}", response.status()),
            });
        }

        let data: Value = response
            .json()
            .await
            .map_err(|e| SourisError::DownloadFailed {
                reason: e.to_string(),
            })?;

        let token = data["access_token"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| SourisError::DownloadFailed {
                reason: "No access_token in response".into(),
            })?;

        *self.token_cache.lock().unwrap() = Some((
            token.clone(),
            Instant::now() + Duration::from_secs(TOKEN_TTL),
        ));

        Ok(token)
    }
}

#[derive(Debug, Clone)]
pub struct SpotifyTrackInfo {
    pub id: String,
    pub name: String,
    pub artist: String,
    pub album: String,
    pub duration_ms: u64,
    pub thumbnail: Option<String>,
}

impl SpotifyTrackInfo {
    pub fn to_search_query(&self) -> String {
        format!("{} {} {}", self.name, self.artist, self.album)
    }
}
