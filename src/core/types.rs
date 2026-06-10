use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MediaType {
    Audio,
    Video,
    Playlist,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AudioFormat {
    Mp3,
    Flac,
    Aac,
    Ogg,
    M4a,
    Wav,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum VideoFormat {
    Mp4,
    Mkv,
    Webm,
    Avi,
    Mov,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Format {
    Audio(AudioFormat),
    Video(VideoFormat),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AudioQuality {
    Kbps128,
    Kbps192,
    Kbps256,
    Kbps320,
    Lossless,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum VideoQuality {
    P360,
    P480,
    P720,
    P1080,
    P1440,
    P4K,
    P8K,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Quality {
    Audio(AudioQuality),
    Video(VideoQuality),
}

impl std::fmt::Display for AudioFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AudioFormat::Mp3 => write!(f, "mp3"),
            AudioFormat::Flac => write!(f, "flac"),
            AudioFormat::Aac => write!(f, "aac"),
            AudioFormat::Ogg => write!(f, "ogg"),
            AudioFormat::M4a => write!(f, "m4a"),
            AudioFormat::Wav => write!(f, "wav"),
        }
    }
}

impl std::fmt::Display for VideoFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VideoFormat::Mp4 => write!(f, "mp4"),
            VideoFormat::Mkv => write!(f, "mkv"),
            VideoFormat::Webm => write!(f, "webm"),
            VideoFormat::Avi => write!(f, "avi"),
            VideoFormat::Mov => write!(f, "mov"),
        }
    }
}

impl std::fmt::Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Format::Audio(a) => write!(f, "{}", a),
            Format::Video(v) => write!(f, "{}", v),
        }
    }
}

impl std::fmt::Display for AudioQuality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AudioQuality::Kbps128 => write!(f, "128"),
            AudioQuality::Kbps192 => write!(f, "192"),
            AudioQuality::Kbps256 => write!(f, "256"),
            AudioQuality::Kbps320 => write!(f, "320"),
            AudioQuality::Lossless => write!(f, "lossless"),
        }
    }
}

impl std::fmt::Display for VideoQuality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VideoQuality::P360 => write!(f, "360p"),
            VideoQuality::P480 => write!(f, "480p"),
            VideoQuality::P720 => write!(f, "720p"),
            VideoQuality::P1080 => write!(f, "1080p"),
            VideoQuality::P1440 => write!(f, "1440p"),
            VideoQuality::P4K => write!(f, "4K"),
            VideoQuality::P8K => write!(f, "8K"),
        }
    }
}

impl std::fmt::Display for Quality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Quality::Audio(a) => write!(f, "{}", a),
            Quality::Video(v) => write!(f, "{}", v),
        }
    }
}

impl std::str::FromStr for AudioFormat {
    type Err = crate::error::SourisError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "mp3" => Ok(AudioFormat::Mp3),
            "flac" => Ok(AudioFormat::Flac),
            "aac" => Ok(AudioFormat::Aac),
            "ogg" | "vorbis" => Ok(AudioFormat::Ogg),
            "m4a" => Ok(AudioFormat::M4a),
            "wav" => Ok(AudioFormat::Wav),
            _ => Err(crate::error::SourisError::UnsupportedFormat {
                format: s.to_string(),
            }),
        }
    }
}

impl std::str::FromStr for VideoFormat {
    type Err = crate::error::SourisError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "mp4" => Ok(VideoFormat::Mp4),
            "mkv" => Ok(VideoFormat::Mkv),
            "webm" => Ok(VideoFormat::Webm),
            "avi" => Ok(VideoFormat::Avi),
            "mov" => Ok(VideoFormat::Mov),
            _ => Err(crate::error::SourisError::UnsupportedFormat {
                format: s.to_string(),
            }),
        }
    }
}

impl std::str::FromStr for Format {
    type Err = crate::error::SourisError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "mp3" => Ok(Format::Audio(AudioFormat::Mp3)),
            "flac" => Ok(Format::Audio(AudioFormat::Flac)),
            "aac" => Ok(Format::Audio(AudioFormat::Aac)),
            "ogg" | "vorbis" => Ok(Format::Audio(AudioFormat::Ogg)),
            "m4a" => Ok(Format::Audio(AudioFormat::M4a)),
            "wav" => Ok(Format::Audio(AudioFormat::Wav)),
            "mp4" => Ok(Format::Video(VideoFormat::Mp4)),
            "mkv" => Ok(Format::Video(VideoFormat::Mkv)),
            "webm" => Ok(Format::Video(VideoFormat::Webm)),
            "avi" => Ok(Format::Video(VideoFormat::Avi)),
            "mov" => Ok(Format::Video(VideoFormat::Mov)),
            _ => Err(crate::error::SourisError::UnsupportedFormat {
                format: s.to_string(),
            }),
        }
    }
}

impl std::str::FromStr for AudioQuality {
    type Err = crate::error::SourisError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "128" | "128kbps" => Ok(AudioQuality::Kbps128),
            "192" | "192kbps" => Ok(AudioQuality::Kbps192),
            "256" | "256kbps" => Ok(AudioQuality::Kbps256),
            "320" | "320kbps" => Ok(AudioQuality::Kbps320),
            "lossless" => Ok(AudioQuality::Lossless),
            _ => Err(crate::error::SourisError::UnsupportedQuality {
                quality: s.to_string(),
            }),
        }
    }
}

impl std::str::FromStr for VideoQuality {
    type Err = crate::error::SourisError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "360p" | "360" => Ok(VideoQuality::P360),
            "480p" | "480" => Ok(VideoQuality::P480),
            "720p" | "720" => Ok(VideoQuality::P720),
            "1080p" | "1080" | "fhd" => Ok(VideoQuality::P1080),
            "1440p" | "1440" | "2k" => Ok(VideoQuality::P1440),
            "4k" | "2160p" | "2160" => Ok(VideoQuality::P4K),
            "8k" | "4320p" | "4320" => Ok(VideoQuality::P8K),
            _ => Err(crate::error::SourisError::UnsupportedQuality {
                quality: s.to_string(),
            }),
        }
    }
}

impl std::str::FromStr for Quality {
    type Err = crate::error::SourisError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if let Ok(aq) = s.parse::<AudioQuality>() {
            return Ok(Quality::Audio(aq));
        }
        if let Ok(vq) = s.parse::<VideoQuality>() {
            return Ok(Quality::Video(vq));
        }
        Err(crate::error::SourisError::UnsupportedQuality {
            quality: s.to_string(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaInfo {
    pub id: String,
    pub title: String,
    pub platform: String,
    pub media_type: MediaType,
    pub duration: Option<u64>,
    pub uploader: Option<String>,
    pub thumbnail: Option<String>,
    pub formats: Vec<FormatInfo>,
    pub subtitles: std::collections::HashMap<String, Vec<SubtitleInfo>>,
    pub playlist: Option<PlaylistInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatInfo {
    pub format_id: String,
    pub ext: String,
    pub media_type: MediaType,
    pub acodec: Option<String>,
    pub vcodec: Option<String>,
    pub abr: Option<u32>,
    pub resolution: Option<String>,
    pub filesize: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubtitleInfo {
    pub ext: String,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistInfo {
    pub id: String,
    pub title: String,
    pub count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadResult {
    pub success: bool,
    pub path: Option<String>,
    pub size: Option<u64>,
    pub error: Option<String>,
    pub elapsed: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchItem {
    pub id: String,
    pub title: String,
    pub platform: String,
    pub url: String,
    pub thumbnail: Option<String>,
    pub duration: Option<u64>,
    pub uploader: Option<String>,
}
