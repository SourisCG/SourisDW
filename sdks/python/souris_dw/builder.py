"""Builder for SourisDW configuration."""

from typing import Optional, Callable
from .types import Format, Quality


class SourisDWBuilder:
    """Builder for configuring SourisDW defaults."""

    def __init__(self):
        self._config = {
            "auto_update": True,
            "format": "mp4",
            "quality": "1080p",
            "output": "./downloads",
            "parallel": 4,
            "embed_metadata": True,
            "embed_thumbnail": True,
            "embed_subtitles": False,
            "timeout": 300,
            "max_retries": 3,
            "spotify_client_id": None,
            "spotify_client_secret": None,
        }

    def auto_update(self, enabled: bool) -> "SourisDWBuilder":
        """Enable/disable auto-update of yt-dlp/ffmpeg."""
        self._config["auto_update"] = enabled
        return self

    def format(self, fmt: str) -> "SourisDWBuilder":
        """Set default output format."""
        self._config["format"] = fmt
        return self

    def quality(self, q: str) -> "SourisDWBuilder":
        """Set default quality."""
        self._config["quality"] = q
        return self

    def output(self, path: str) -> "SourisDWBuilder":
        """Set default output directory."""
        self._config["output"] = path
        return self

    def parallel(self, n: int) -> "SourisDWBuilder":
        """Set number of parallel downloads."""
        self._config["parallel"] = n
        return self

    def embed_metadata(self, enabled: bool) -> "SourisDWBuilder":
        """Enable/disable metadata embedding."""
        self._config["embed_metadata"] = enabled
        return self

    def embed_thumbnail(self, enabled: bool) -> "SourisDWBuilder":
        """Enable/disable thumbnail embedding."""
        self._config["embed_thumbnail"] = enabled
        return self

    def embed_subtitles(self, enabled: bool) -> "SourisDWBuilder":
        """Enable/disable subtitle embedding."""
        self._config["embed_subtitles"] = enabled
        return self

    def timeout(self, seconds: int) -> "SourisDWBuilder":
        """Set download timeout."""
        self._config["timeout"] = seconds
        return self

    def max_retries(self, n: int) -> "SourisDWBuilder":
        """Set max retries on failure."""
        self._config["max_retries"] = n
        return self

    def spotify_credentials(self, client_id: str, client_secret: str) -> "SourisDWBuilder":
        """Set Spotify API credentials."""
        self._config["spotify_client_id"] = client_id
        self._config["spotify_client_secret"] = client_secret
        return self

    def build(self) -> "SourisDW":
        """Build the SourisDW instance."""
        from . import SourisDW
        return SourisDW(self._config)
