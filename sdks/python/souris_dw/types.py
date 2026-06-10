"""Type definitions for SourisDW."""

from enum import Enum
from dataclasses import dataclass
from typing import Optional


class MediaType(Enum):
    AUDIO = "audio"
    VIDEO = "video"
    PLAYLIST = "playlist"


class AudioFormat(Enum):
    MP3 = "mp3"
    FLAC = "flac"
    AAC = "aac"
    OGG = "ogg"
    M4A = "m4a"
    WAV = "wav"


class VideoFormat(Enum):
    MP4 = "mp4"
    MKV = "mkv"
    WEBM = "webm"
    AVI = "avi"
    MOV = "mov"


class AudioQuality(Enum):
    KBPS128 = "128"
    KBPS192 = "192"
    KBPS256 = "256"
    KBPS320 = "320"
    LOSSLESS = "lossless"


class VideoQuality(Enum):
    P360 = "360p"
    P480 = "480p"
    P720 = "720p"
    P1080 = "1080p"
    P1440 = "1440p"
    P4K = "4K"
    P8K = "8K"


@dataclass
class Format:
    """Output format."""
    value: str

    @staticmethod
    def audio(fmt: AudioFormat) -> "Format":
        return Format(fmt.value)

    @staticmethod
    def video(fmt: VideoFormat) -> "Format":
        return Format(fmt.value)


@dataclass
class Quality:
    """Output quality."""
    value: str

    @staticmethod
    def audio(q: AudioQuality) -> "Quality":
        return Quality(q.value)

    @staticmethod
    def video(q: VideoQuality) -> "Quality":
        return Quality(q.value)


@dataclass
class ProgressEvent:
    """Download progress event."""
    type: str
    percent: float = 0.0
    speed: str = ""
    eta: str = ""
    path: Optional[str] = None
    size: Optional[int] = None
    message: str = ""
