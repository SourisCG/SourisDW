"""Download request with fluent API."""

import subprocess
import json
from typing import Optional, Callable, Any


class ProgressEvent:
    """Represents a download progress event."""

    def __init__(self, data: dict):
        self._data = data

    @property
    def type(self) -> str:
        return self._data.get("type", "")

    @property
    def percent(self) -> float:
        return self._data.get("percent", 0.0)

    @property
    def speed(self) -> str:
        return self._data.get("speed", "")

    @property
    def eta(self) -> str:
        return self._data.get("eta", "")

    @property
    def path(self) -> Optional[str]:
        return self._data.get("path")

    @property
    def size(self) -> Optional[int]:
        return self._data.get("size")

    @property
    def message(self) -> str:
        return self._data.get("message", "")

    def __repr__(self):
        return f"ProgressEvent(type={self.type!r}, percent={self.percent})"


class DownloadRequest:
    """Fluent download request."""

    def __init__(self, dw: "SourisDW", url: str, media_type: Optional[str] = None):
        self._dw = dw
        self._url = url
        self._media_type = media_type
        self._overrides = {}
        self._on_progress = None
        self._on_complete = None
        self._on_error = None

    def format(self, fmt: str) -> "DownloadRequest":
        """Override format for this download."""
        self._overrides["format"] = fmt
        return self

    def quality(self, q: str) -> "DownloadRequest":
        """Override quality for this download."""
        self._overrides["quality"] = q
        return self

    def output(self, path: str) -> "DownloadRequest":
        """Override output directory for this download."""
        self._overrides["output"] = path
        return self

    def parallel(self, n: int) -> "DownloadRequest":
        """Override parallel downloads for this download."""
        self._overrides["parallel"] = n
        return self

    def embed_metadata(self, enabled: bool) -> "DownloadRequest":
        """Override metadata embedding for this download."""
        self._overrides["embed_metadata"] = enabled
        return self

    def embed_thumbnail(self, enabled: bool) -> "DownloadRequest":
        """Override thumbnail embedding for this download."""
        self._overrides["embed_thumbnail"] = enabled
        return self

    def embed_subtitles(self, enabled: bool) -> "DownloadRequest":
        """Override subtitle embedding for this download."""
        self._overrides["embed_subtitles"] = enabled
        return self

    def on_progress(self, callback: Callable[[ProgressEvent], None]) -> "DownloadRequest":
        """Set progress callback."""
        self._on_progress = callback
        return self

    def on_complete(self, callback: Callable[[dict], None]) -> "DownloadRequest":
        """Set completion callback."""
        self._on_complete = callback
        return self

    def on_error(self, callback: Callable[[str], None]) -> "DownloadRequest":
        """Set error callback."""
        self._on_error = callback
        return self

    def run(self) -> dict:
        """Execute the download."""
        config = {**self._dw.config, **self._overrides}

        cmd = ["souris-dw", "download", self._url, "--json"]

        if "format" in config:
            cmd.extend(["--format", config["format"]])
        if "quality" in config:
            cmd.extend(["--quality", config["quality"]])
        if "output" in config:
            cmd.extend(["--output", config["output"]])
        if "parallel" in config:
            cmd.extend(["--parallel", str(config["parallel"])])
        if config.get("embed_metadata"):
            cmd.append("--embed-metadata")
        if config.get("embed_thumbnail"):
            cmd.append("--embed-thumbnail")
        if config.get("embed_subtitles"):
            cmd.append("--embed-subtitles")
        if self._media_type == "audio":
            cmd.append("--audio-only")
        elif self._media_type == "video":
            cmd.append("--video-only")

        proc = subprocess.Popen(
            cmd,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
        )

        result = {}
        for line in proc.stdout:
            line = line.strip()
            if not line:
                continue

            try:
                event = json.loads(line)
            except json.JSONDecodeError:
                continue

            progress = ProgressEvent(event)

            if event.get("type") == "progress" and self._on_progress:
                self._on_progress(progress)
            elif event.get("type") == "complete":
                result = event
                if self._on_complete:
                    self._on_complete(event)
            elif event.get("type") == "error":
                error_msg = event.get("message", "Unknown error")
                if self._on_error:
                    self._on_error(error_msg)
                raise Exception(error_msg)

        proc.wait()

        if proc.returncode != 0 and not result:
            stderr = proc.stderr.read()
            raise Exception(f"Download failed: {stderr}")

        return result
