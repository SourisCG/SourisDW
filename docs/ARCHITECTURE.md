# Architecture

## Overview

SourisDW is a cross-platform music & video downloader built in Rust. It uses yt-dlp as its download engine and supports YouTube and Spotify. All external dependencies (yt-dlp, ffmpeg, ffprobe, deno) are downloaded at runtime, not embedded at compile time.

## Module Structure

```
souris-dw/
├── build.rs                 # No-op (deps managed at runtime via DepManager)
├── src/
│   ├── lib.rs              # Public API exports
│   ├── bin/souris-dw/      # CLI binary
│   │   └── main.rs         # Entry point (CLI + TUI, 1059 lines)
│   ├── core/               # Core business logic
│   │   ├── downloader.rs   # SourisDW struct + builder (fluent API)
│   │   ├── request.rs      # DownloadRequestBuilder (chainable overrides)
│   │   ├── types.rs        # MediaType, Format, Quality, MediaInfo, etc.
│   │   ├── progress.rs     # ProgressEvent enum (JSON streaming)
│   │   └── queue.rs        # Parallel download queue via semaphore
│   ├── deps/               # Runtime dependency management
│   │   ├── mod.rs          # DepManager (unified API)
│   │   ├── platform.rs     # OS/arch detection + paths
│   │   ├── path.rs         # Platform-specific paths
│   │   ├── download.rs     # HTTP download with progress bars
│   │   ├── resolve.rs      # Download URL resolution per platform
│   │   ├── versions.rs     # Version tracking + caching
│   │   ├── yt_dlp.rs       # yt-dlp download, update, version check
│   │   ├── ffmpeg.rs       # ffmpeg + ffprobe download, update
│   │   └── deno.rs         # Deno runtime download, update
│   ├── extractors/         # Platform extractors
│   │   ├── youtube.rs      # YouTube via yt-dlp (format selection, thumbnail, metadata)
│   │   ├── spotify.rs      # Spotify Web API metadata lookup
│   │   └── resolver.rs     # URL detection, platform routing
│   ├── postprocess/        # Post-processing (currently unused/empty)
│   │   └── mod.rs          # Reserved for future use
│   ├── config.rs           # Configuration (TOML file management)
│   ├── error.rs            # Error types (SourisError enum)
│   ├── tui/                # Terminal UI (ratatui + crossterm)
│   │   ├── app.rs          # Application state
│   │   ├── ui.rs           # Rendering
│   │   ├── events.rs       # Keyboard event handling
│   │   ├── theme.rs        # Color palette
│   │   └── views/          # Screen views
│   │       ├── download.rs
│   │       ├── help.rs
│   │       ├── queue.rs
│   │       ├── search.rs
│   │       └── settings.rs
│   └── utils/              # Utilities
│       ├── fs.rs           # Filesystem helpers (ensure_dir, set_executable)
│       ├── unicode.rs      # Unicode normalization
│       └── mod.rs
```

## Data Flow

```
User Input (URL)
      |
      v
+--------------+
|   Resolver   |  Detects platform (YouTube/Spotify) and resource type
|  (resolver)  |  (video/playlist/track/album)
+------+-------+
       |
       v
+--------------+     +--------------+
|  Extractor   |---->|  yt-dlp      |  YouTube: direct extraction via yt-dlp subprocess
|  (youtube)   |     |  (subprocess)|
+------+-------+     +--------------+
       |
       |  Spotify flow: metadata lookup -> YouTube search -> download matched video
       v
+--------------+
|  Downloader  |  Builds yt-dlp command with format selection, quality filters,
|  (youtube)   |  codec preferences, cookie support, and retry logic.
|              |  Handles:
|              |   - Format string per codec (AVI: ext=mp4, MOV: vcodec^=avc1)
|              |   - --embed-metadata, --embed-thumbnail (throttled per format)
|              |   - --windows-filenames, --replace-in-metadata
|              |   - HTTP 403 retry with android player client fallback
|              |   - wav_2step (deprecated, always false)
+------+-------+
       |
       v
+--------------+
|  Output File |  Final path determined by yt-dlp output, parsed from stdout
+--------------+
```

## Format Selection Strategy

Each video format uses a specific yt-dlp format string:

| Format | Format String | Notes |
|--------|---------------|-------|
| MP4 | `bestvideo[height<=h]+bestaudio/best[height<=h]` | Default |
| MKV | `bestvideo[height<=h]+bestaudio/best[height<=h]` | Same as MP4 |
| WebM | `bestvideo[height<=h]+bestaudio/best[height<=h]` | Same as MP4 |
| AVI | `bestvideo[ext=mp4][height<=h]+bestaudio[ext=m4a]/best[height<=h]` | Forces mp4/m4a codecs |
| MOV | `bestvideo[vcodec^=avc1][ext=mp4][height<=h]+bestaudio[ext=m4a]/best[height<=h]` | Forces H.264 + `--merge-output-format mov` |

On HTTP 403, the retry format string falls back to generic `bestvideo[height<=h]+bestaudio/best[height<=h]` (codec filters are removed for compatibility).

### Thumbnail Embedding

| Format | Thumbnail | Method |
|--------|-----------|--------|
| MP3 | Yes | yt-dlp `--embed-thumbnail` (ID3v2 attached picture) |
| FLAC | Yes | yt-dlp `--embed-thumbnail` |
| AAC | Yes | yt-dlp `--embed-thumbnail` |
| OGG | Yes | yt-dlp `--embed-thumbnail` |
| M4A | Yes | yt-dlp `--embed-thumbnail` |
| WAV | No | Container does not support thumbnails |
| MP4 | Yes | yt-dlp `--embed-thumbnail` (attached pic) |
| MKV | Yes | yt-dlp `--embed-thumbnail` |
| WebM | No | Container does not support thumbnails |
| AVI | No | Container does not support thumbnails |
| MOV | Limited | Only works with single format (no merge). Merged MOV files lose thumbnail due to ffmpeg limitation. |

## Dependency Management

### yt-dlp
- Downloaded from GitHub releases at runtime
- Binary stored in platform-specific `bin_dir`
- Channels: stable, nightly, master
- Auto-updates silently (configurable via config.toml or `--no-auto-update`)
- HTTP 403 retry with android `--extractor-args youtube:player_client=android`
- Deno used as JS runtime when available

### ffmpeg + ffprobe
- Downloaded from `eugeneware/ffmpeg-static` at runtime
- Gzip archives extracted to `bin_dir`
- ffprobe always downloaded alongside ffmpeg
- Falls back gracefully if download fails

### deno
- Downloaded from denoland/deno at runtime
- Zip archive extracted to `bin_dir`
- Used as JS runtime for yt-dlp (no Node.js required)
- Optional: falls back to system deno if available

### DepManager API

| Method | Description |
|--------|-------------|
| `setup(auto_update, channel)` | Full setup with auto-update check |
| `build(auto_update, channel)` | Same as setup (legacy alias) |
| `setup_blocking(auto_update, channel, quiet)` | With progress bars always shown |
| `status()` | Returns Vec<DepStatus> for all deps |
| `update_all()` | Check and update all deps |
| `yt_dlp()` / `ffmpeg()` / `deno()` | Access individual dep managers |

## Platform Detection

Uses compile-time constants:
- `std::env::consts::OS` -> `"linux"`, `"macos"`, `"windows"`
- `std::env::consts::ARCH` -> `"x86_64"`, `"aarch64"`
- `std::env::consts::EXE_SUFFIX` -> `".exe"` (Windows), `""` (Unix)

## Configuration Paths

| Platform | Config | Data | Cache |
|----------|--------|------|-------|
| Linux | `~/.config/souris-dw/` | `~/.local/share/souris-dw/` | `~/.cache/souris-dw/` |
| macOS | `~/Library/Application Support/souris-dw/` | same | `~/Library/Caches/souris-dw/` |
| Windows | `%APPDATA%\souris-dw\` | `%LOCALAPPDATA%\souris-dw\` | `%LOCALAPPDATA%\souris-dw\cache\` |

## JSON Protocol

All CLI commands support `--json` output for subprocess integration:

```json
{"type":"init","url":"...","platform":"youtube","title":"...","media_type":"video","total_items":1}
{"type":"progress","item":1,"total":1,"percent":45.2,"speed":"2.3MB/s","eta":"00:12"}
{"type":"postprocess","item":1,"total":1,"stage":"converting","format":"mp4"}
{"type":"metadata","item":1,"total":1,"stage":"embedding_tags"}
{"type":"complete","item":1,"total":1,"path":"/path/to/file.mp4","size":125000000}
{"type":"error","item":1,"total":1,"code":"DOWNLOAD_FAILED","message":"..."}
{"type":"summary","total":10,"success":9,"failed":1,"elapsed":"02:34"}
```

## Error Types

| Error | Exit Code | Description |
|-------|-----------|-------------|
| `DependencyNotFound` | 2 | Required binary not found |
| `DependencyDownloadFailed` | 2 | Failed to download dependency |
| `DependencyUpdateFailed` | 2 | Failed to update dependency |
| `DownloadFailed` | 1 | Download failed |
| `HttpError` | 3 | Network error |
| `Timeout` | 3 | Request timed out |
| `Cancelled` | 0 | User cancelled operation |

## Key Implementation Details

- **WAV 2-step deprecated**: Previously downloaded as vorbis and converted to WAV via ffmpeg. Now downloads directly as WAV. WAV cannot embed thumbnails, so `--embed-thumbnail` is skipped.
- **Trailing period filenames**: `--replace-in-metadata title "\.+$" ""` strips trailing dots from titles before filename construction.
- **Path extraction**: `extract_downloaded_path_static()` parses yt-dlp stdout for `[ExtractAudio] Destination:` first, falls back to `[download] Destination:` to find the final file path after post-processing.
- **Empty postprocess module**: `src/postprocess/mod.rs` exists but is empty (reserved for future use). All post-processing is handled by yt-dlp.
