# Architecture

## Overview

SourisDW is a cross-platform music & video downloader built in Rust. It uses yt-dlp as its download engine and supports YouTube and Spotify.

## Module Structure

```
souris-dw/
├── build.rs                 # Downloads ffmpeg at compile time (embedded via include_bytes!)
├── src/
│   ├── lib.rs              # Public API exports
│   ├── bin/souris-dw/      # CLI binary
│   │   └── main.rs         # Entry point (CLI + TUI)
│   ├── core/               # Core business logic
│   │   ├── downloader.rs   # SourisDW struct + builder
│   │   ├── request.rs      # DownloadRequest (fluent chainable)
│   │   ├── types.rs        # MediaType, Format, Quality, etc.
│   │   ├── progress.rs     # Progress events (JSON streaming)
│   │   └── queue.rs        # Parallel download queue
│   ├── deps/               # Dependency management
│   │   ├── platform.rs     # OS/arch detection
│   │   ├── yt_dlp.rs       # yt-dlp auto-download & update
│   │   ├── ffmpeg.rs       # ffmpeg extraction from embedded binary
│   │   └── mod.rs          # DepManager
│   ├── extractors/         # Platform extractors
│   │   ├── youtube.rs      # YouTube via yt-dlp
│   │   ├── spotify.rs      # Spotify Web API
│   │   └── resolver.rs     # URL detection & routing
│   ├── postprocess/        # Post-processing
│   │   ├── converter.rs    # Audio/video conversion (ffmpeg)
│   │   ├── metadata.rs     # ID3 tag embedding (lofty)
│   │   ├── thumbnail.rs    # Album art embedding
│   │   └── subtitle.rs     # Subtitle download
│   ├── tui/                # Terminal UI
│   │   ├── app.rs          # Application state
│   │   ├── ui.rs           # Rendering (opencode style)
│   │   ├── events.rs       # Keyboard event handling
│   │   └── theme.rs        # Color palette
│   ├── config.rs           # Configuration (TOML)
│   ├── error.rs            # Error types
│   └── utils/              # Utilities
│       ├── fs.rs           # Filesystem helpers
│       └── unicode.rs      # Unicode normalization
```

## Data Flow

```
User Input (URL)
      │
      ▼
┌─────────────┐
│   Resolver   │  Detects platform (YouTube/Spotify)
│              │  Detects type (video/playlist/track)
└──────┬──────┘
       │
       ▼
┌─────────────┐     ┌──────────────┐
│  Extractor   │────▶│  yt-dlp      │  YouTube: direct extraction
│              │     │  (subprocess)│
└──────┬──────┘     └──────────────┘
       │
       │  Spotify: metadata → YouTube search → download
       ▼
┌─────────────┐
│  Downloader  │  Manages download queue
│              │  Parallel execution
└──────┬──────┘
       │
       ▼
┌─────────────┐
│Post-Process  │  Format conversion (ffmpeg)
│              │  Metadata embedding (lofty)
│              │  Thumbnail embedding
└──────┬──────┘
       │
       ▼
┌─────────────┐
│  Output File │  Saved to configured directory
└─────────────┘
```

## Dependency Management

### yt-dlp
- Auto-downloaded on first use from GitHub releases
- Binary stored in platform-specific data directory
- Auto-updates silently in background (configurable)
- Rollback on repeated failures

### ffmpeg
- Static binary embedded at compile time via `include_bytes!` (build.rs downloads from GitHub)
- Extracted to platform data directory on first run
- Passed to yt-dlp via `--ffmpeg-location` for audio conversion
- Falls back to system ffmpeg if embedded version is empty

## Platform Detection

Uses compile-time constants:
- `std::env::consts::OS` → `"linux"`, `"macos"`, `"windows"`
- `std::env::consts::ARCH` → `"x86_64"`, `"aarch64"`
- `std::env::consts::EXE_SUFFIX` → `".exe"` (Windows), `""` (Unix)

## Configuration Paths

| Platform | Config | Data | Cache |
|----------|--------|------|-------|
| Linux | `~/.config/souris-dw/` | `~/.local/share/souris-dw/` | `~/.cache/souris-dw/` |
| macOS | `~/Library/Application Support/souris-dw/` | same | `~/Library/Caches/souris-dw/` |
| Windows | `%APPDATA%\souris-dw\` | `%LOCALAPPDATA%\souris-dw\` | `%LOCALAPPDATA%\souris-dw\cache\` |

## JSON Protocol

All CLI commands support `--json` output for subprocess integration:

```json
{"type":"init",       "url":"...", "platform":"youtube", "title":"...", "total_items":1}
{"type":"progress",   "item":1, "total":1, "percent":45.2, "speed":"2.3MB/s", "eta":"00:12"}
{"type":"complete",   "item":1, "total":1, "path":"/path/to/file.mp4", "size":125000000}
{"type":"error",      "item":1, "total":1, "code":"DOWNLOAD_FAILED", "message":"..."}
{"type":"summary",    "total":10, "success":9, "failed":1, "elapsed":"02:34"}
```
