use crate::core::request::MediaTypeHint;
use crate::core::types::*;
use crate::error::{Result, SourisError};
use crate::extractors::spotify::SpotifyExtractor;
use crate::extractors::youtube::YouTubeExtractor;

#[derive(Debug, Clone)]
pub enum Platform {
    YouTube,
    Spotify,
    Unknown,
}

#[derive(Debug, Clone)]
pub enum ResourceType {
    Video,
    Playlist,
    Track,
    Album,
    Search,
}

pub struct Resolver {
    youtube: YouTubeExtractor,
    spotify: Option<SpotifyExtractor>,
}

impl Resolver {
    pub async fn new(
        spotify_client_id: Option<String>,
        spotify_client_secret: Option<String>,
    ) -> Result<Self> {
        let youtube = YouTubeExtractor::new().await?;
        let spotify = if spotify_client_id.is_some() && spotify_client_secret.is_some() {
            Some(SpotifyExtractor::new(
                spotify_client_id,
                spotify_client_secret,
            ))
        } else {
            None
        };

        Ok(Self { youtube, spotify })
    }

    pub fn detect_platform(&self, url: &str) -> Platform {
        let url_lower = url.to_lowercase();

        if url_lower.contains("youtube.com") || url_lower.contains("youtu.be") {
            Platform::YouTube
        } else if url_lower.contains("spotify.com") {
            Platform::Spotify
        } else {
            Platform::Unknown
        }
    }

    pub fn detect_resource_type(&self, url: &str) -> ResourceType {
        let url_lower = url.to_lowercase();

        if url_lower.contains("/playlist") || url_lower.contains("list=") {
            ResourceType::Playlist
        } else if url_lower.contains("/track/") {
            ResourceType::Track
        } else if url_lower.contains("/album/") {
            ResourceType::Album
        } else {
            ResourceType::Video
        }
    }

    pub async fn resolve_info(&self, url: &str) -> Result<MediaInfo> {
        let platform = self.detect_platform(url);
        let resource_type = self.detect_resource_type(url);

        match platform {
            Platform::YouTube => match resource_type {
                ResourceType::Playlist => {
                    let items = self.youtube.extract_playlist_info(url).await?;
                    if let Some(first) = items.into_iter().next() {
                        Ok(first)
                    } else {
                        Err(SourisError::DownloadFailed {
                            reason: "Playlist is empty".into(),
                        })
                    }
                }
                _ => self.youtube.extract_info(url).await,
            },
            Platform::Spotify => {
                let spotify = self.spotify.as_ref().ok_or_else(|| {
                    SourisError::ConfigError(
                        "Spotify credentials not configured. Set spotify.client_id and spotify.client_secret in config.".into()
                    )
                })?;

                match resource_type {
                    ResourceType::Track => {
                        let track = spotify.extract_track_info(url).await?;
                        let search_query = track.to_search_query();
                        let search_results = self.youtube.search(&search_query, 1).await?;

                        if let Some(result) = search_results.into_iter().next() {
                            let mut info = self.youtube.extract_info(&result.url).await?;
                            info.platform = format!("Spotify -> YouTube ({})", info.platform);
                            Ok(info)
                        } else {
                            Err(SourisError::DownloadFailed {
                                reason: format!("No YouTube results for: {}", search_query),
                            })
                        }
                    }
                    ResourceType::Playlist => {
                        let tracks = spotify.extract_playlist_info(url).await?;
                        if let Some(first_track) = tracks.first() {
                            let search_query = first_track.to_search_query();
                            let search_results = self.youtube.search(&search_query, 1).await?;

                            if let Some(result) = search_results.into_iter().next() {
                                let mut info = self.youtube.extract_info(&result.url).await?;
                                info.platform = format!("Spotify -> YouTube ({})", info.platform);
                                info.playlist = Some(PlaylistInfo {
                                    id: url.to_string(),
                                    title: format!("Spotify Playlist ({} tracks)", tracks.len()),
                                    count: tracks.len(),
                                });
                                Ok(info)
                            } else {
                                Err(SourisError::DownloadFailed {
                                    reason: "No YouTube results found".into(),
                                })
                            }
                        } else {
                            Err(SourisError::DownloadFailed {
                                reason: "Spotify playlist is empty".into(),
                            })
                        }
                    }
                    _ => Err(SourisError::UnsupportedPlatform {
                        platform: "Spotify (only tracks and playlists supported)".into(),
                    }),
                }
            }
            Platform::Unknown => self.youtube.extract_info(url).await,
        }
    }

    pub async fn resolve_search(&self, query: &str, limit: usize) -> Result<Vec<SearchItem>> {
        self.youtube.search(query, limit).await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn resolve_download(
        &self,
        url: &str,
        format: Option<&Format>,
        quality: Option<&Quality>,
        output_dir: &str,
        embed_metadata: bool,
        embed_thumbnail: bool,
        embed_subtitles: bool,
        media_type: Option<&MediaTypeHint>,
    ) -> Result<DownloadResult> {
        let platform = self.detect_platform(url);
        let resource_type = self.detect_resource_type(url);

        match platform {
            Platform::YouTube => match resource_type {
                ResourceType::Playlist => {
                    let results = self
                        .youtube
                        .download_playlist(
                            url,
                            format,
                            quality,
                            output_dir,
                            embed_metadata,
                            embed_thumbnail,
                            embed_subtitles,
                            4,
                        )
                        .await?;

                    let success = results.iter().all(|r| r.success);
                    Ok(DownloadResult {
                        success,
                        path: None,
                        size: None,
                        error: if success {
                            None
                        } else {
                            Some("Some downloads failed".into())
                        },
                        elapsed: None,
                    })
                }
                _ => {
                    self.youtube
                        .download(
                            url,
                            format,
                            quality,
                            output_dir,
                            embed_metadata,
                            embed_thumbnail,
                            embed_subtitles,
                            media_type,
                        )
                        .await
                }
            },
            Platform::Spotify => {
                let spotify = self.spotify.as_ref().ok_or_else(|| {
                    SourisError::ConfigError("Spotify credentials not configured".into())
                })?;

                match resource_type {
                    ResourceType::Track => {
                        let track = spotify.extract_track_info(url).await?;
                        let search_query = track.to_search_query();
                        let search_results = self.youtube.search(&search_query, 1).await?;

                        if let Some(result) = search_results.into_iter().next() {
                            let mut download_result = self
                                .youtube
                                .download(
                                    &result.url,
                                    format,
                                    quality,
                                    output_dir,
                                    embed_metadata,
                                    embed_thumbnail,
                                    embed_subtitles,
                                    media_type,
                                )
                                .await?;

                            download_result.path = download_result
                                .path
                                .map(|p| p.replace(&result.title, &track.name));

                            Ok(download_result)
                        } else {
                            Err(SourisError::DownloadFailed {
                                reason: format!("No YouTube results for: {}", search_query),
                            })
                        }
                    }
                    ResourceType::Playlist => {
                        let tracks = spotify.extract_playlist_info(url).await?;
                        let mut results = Vec::new();

                        for track in tracks {
                            let search_query = track.to_search_query();
                            let search_results = self.youtube.search(&search_query, 1).await?;

                            if let Some(result) = search_results.into_iter().next() {
                                let download_result = self
                                    .youtube
                                    .download(
                                        &result.url,
                                        format,
                                        quality,
                                        output_dir,
                                        embed_metadata,
                                        embed_thumbnail,
                                        embed_subtitles,
                                        media_type,
                                    )
                                    .await;

                                match download_result {
                                    Ok(r) => results.push(r),
                                    Err(e) => results.push(DownloadResult {
                                        success: false,
                                        path: None,
                                        size: None,
                                        error: Some(e.to_string()),
                                        elapsed: None,
                                    }),
                                }
                            }
                        }

                        let success = results.iter().all(|r| r.success);
                        Ok(DownloadResult {
                            success,
                            path: None,
                            size: None,
                            error: if success {
                                None
                            } else {
                                Some("Some downloads failed".into())
                            },
                            elapsed: None,
                        })
                    }
                    _ => Err(SourisError::UnsupportedPlatform {
                        platform: "Spotify (only tracks and playlists supported)".into(),
                    }),
                }
            }
            Platform::Unknown => {
                self.youtube
                    .download(
                        url,
                        format,
                        quality,
                        output_dir,
                        embed_metadata,
                        embed_thumbnail,
                        embed_subtitles,
                        media_type,
                    )
                    .await
            }
        }
    }
}
