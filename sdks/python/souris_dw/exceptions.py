"""Exceptions for SourisDW."""


class SourisError(Exception):
    """Base exception for SourisDW."""
    pass


class DependencyError(SourisError):
    """Error with dependencies (yt-dlp, ffmpeg)."""
    pass


class DownloadError(SourisError):
    """Error during download."""
    pass


class ConfigError(SourisError):
    """Configuration error."""
    pass


class PlatformError(SourisError):
    """Unsupported platform error."""
    pass
