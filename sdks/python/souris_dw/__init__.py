"""
SourisDW - Cross-platform music & video downloader for YouTube and Spotify.

Usage:
    from souris_dw import SourisDW

    dw = SourisDW.builder() \\
        .format("mp4") \\
        .quality("1080p") \\
        .output("./downloads") \\
        .build()

    dw.download("https://youtube.com/watch?v=xxx").run()
"""

from .builder import SourisDWBuilder
from .request import DownloadRequest
from .types import MediaType, Format, Quality, ProgressEvent
from .exceptions import SourisError, DependencyError, DownloadError

__version__ = "0.1.0"
__all__ = [
    "SourisDW",
    "SourisDWBuilder",
    "DownloadRequest",
    "MediaType",
    "Format",
    "Quality",
    "ProgressEvent",
    "SourisError",
    "DependencyError",
    "DownloadError",
]


class SourisDW:
    """Main entry point for SourisDW."""

    def __init__(self, config: dict):
        self._config = config

    @staticmethod
    def builder() -> "SourisDWBuilder":
        """Create a new builder."""
        return SourisDWBuilder()

    def download(self, url: str) -> "DownloadRequest":
        """Download a URL (auto-detect audio/video)."""
        return DownloadRequest(self, url)

    def download_audio(self, url: str) -> "DownloadRequest":
        """Download audio only."""
        return DownloadRequest(self, url, media_type="audio")

    def download_video(self, url: str) -> "DownloadRequest":
        """Download video only."""
        return DownloadRequest(self, url, media_type="video")

    def download_playlist(self, url: str) -> "DownloadRequest":
        """Download a playlist."""
        return DownloadRequest(self, url, media_type="playlist")

    def info(self, url: str) -> dict:
        """Get media info without downloading."""
        import subprocess
        import json

        cmd = ["souris-dw", "info", url, "--json"]
        result = subprocess.run(cmd, capture_output=True, text=True)
        if result.returncode != 0:
            raise DownloadError(result.stderr.strip())
        return json.loads(result.stdout)

    def search(self, query: str, limit: int = 10) -> list:
        """Search for media."""
        import subprocess
        import json

        cmd = ["souris-dw", "search", query, "--json", "--limit", str(limit)]
        result = subprocess.run(cmd, capture_output=True, text=True)
        if result.returncode != 0:
            raise DownloadError(result.stderr.strip())
        return json.loads(result.stdout)

    def update(self) -> dict:
        """Update dependencies."""
        import subprocess
        import json

        cmd = ["souris-dw", "update", "--json"]
        result = subprocess.run(cmd, capture_output=True, text=True)
        if result.returncode != 0:
            raise DependencyError(result.stderr.strip())
        return json.loads(result.stdout)

    def update_check(self) -> list:
        """Check for updates."""
        import subprocess
        import json

        cmd = ["souris-dw", "update", "--json", "--check"]
        result = subprocess.run(cmd, capture_output=True, text=True)
        if result.returncode != 0:
            raise DependencyError(result.stderr.strip())
        return json.loads(result.stdout)

    @property
    def config(self) -> dict:
        """Get the configuration."""
        return self._config.copy()


def main():
    """CLI entry point."""
    import subprocess
    import sys

    result = subprocess.run(["souris-dw"] + sys.argv[1:])
    sys.exit(result.returncode)
