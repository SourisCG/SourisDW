# Usage Guide

## Installation

### One-Line Install

**Linux & macOS:**
```bash
curl -fsSL https://raw.githubusercontent.com/SourisCG/SourisDW/main/install.sh | bash
```

**Windows (PowerShell):**
```powershell
irm https://raw.githubusercontent.com/SourisCG/SourisDW/main/install.ps1 | iex
```

### Manual Download (Linux)

**musl (works on any distro):**
```bash
curl -sL https://github.com/SourisCG/SourisDW/releases/latest/download/souris-dw-linux-x86_64 -o souris-dw
chmod +x souris-dw
sudo mv souris-dw /usr/local/bin/
```

**glibc (Ubuntu/Debian):**
```bash
curl -sL https://github.com/SourisCG/SourisDW/releases/latest/download/souris-dw-linux-x86_64-glibc -o souris-dw
chmod +x souris-dw
sudo mv souris-dw /usr/local/bin/
```

**Fedora/RHEL:**
```bash
curl -sL https://github.com/SourisCG/SourisDW/releases/latest/download/souris-dw-linux-x86_64-fedora -o souris-dw
chmod +x souris-dw
sudo mv souris-dw /usr/local/bin/
```

**Linux ARM64 (musl):**
```bash
curl -sL https://github.com/SourisCG/SourisDW/releases/latest/download/souris-dw-linux-aarch64 -o souris-dw
chmod +x souris-dw
sudo mv souris-dw /usr/local/bin/
```

### Manual Download (macOS)

**Intel:**
```bash
curl -sL https://github.com/SourisCG/SourisDW/releases/latest/download/souris-dw-macos-x86_64 -o souris-dw
chmod +x souris-dw
sudo mv souris-dw /usr/local/bin/
```

**Apple Silicon:**
```bash
curl -sL https://github.com/SourisCG/SourisDW/releases/latest/download/souris-dw-macos-aarch64 -o souris-dw
chmod +x souris-dw
sudo mv souris-dw /usr/local/bin/
```

### Manual Download (Windows)

Download `souris-dw-windows-x86_64.exe` (or `souris-dw-windows-arm64.exe` for ARM64) from [releases](https://github.com/SourisCG/SourisDW/releases) and add to your PATH.

## CLI Commands

### Global Flags

| Flag | Description |
|------|-------------|
| `--json` | Machine-readable JSON output |
| `--quiet` | Suppress progress bars |
| `--no-auto-update` | Skip automatic dependency updates |
| `--no-color` | Disable colored output |
| `--timeout <seconds>` | Download timeout (default: 300) |
| `--max-retries <n>` | Max retries on failure (default: 3) |

### Download

```bash
# Basic download (defaults: mp4, 1080p)
souris-dw download "https://youtube.com/watch?v=xxx"

# Download audio only
souris-dw download "URL" --audio-only --format mp3

# Download video with specific quality
souris-dw download "URL" --format mp4 --quality 1080p

# Download to specific directory
souris-dw download "URL" --output ~/Music

# Download with metadata and thumbnail
souris-dw download "URL" --embed-metadata --embed-thumbnail

# Download playlist
souris-dw download "PLAYLIST_URL" --parallel 8

# Download with subtitles
souris-dw download "URL" --embed-subtitles

# Download with cookies
souris-dw download "URL" --cookies cookies.txt
souris-dw download "URL" --cookies-from-browser firefox

# JSON progress output (for subprocess integration)
souris-dw download "URL" --json --format mp4 --quality 1080p
```

**Options:**
| Flag | Description |
|------|-------------|
| `-f, --format` | Output format (mp3, flac, mp4, mkv, etc.) |
| `-q, --quality` | Quality (128kbps, 320kbps, 360p, 1080p, 4K) |
| `-o, --output` | Output directory |
| `-p, --parallel` | Number of parallel downloads |
| `--audio-only` | Download audio only |
| `--video-only` | Download video only |
| `--embed-metadata` | Embed ID3/metadata tags |
| `--embed-thumbnail` | Embed thumbnail/album art |
| `--embed-subtitles` | Download and embed subtitles |
| `--cookies <file>` | Cookies file for authentication |
| `--cookies-from-browser <browser>` | Extract cookies from browser |

### Info

```bash
# Get info as human-readable text
souris-dw info "https://youtube.com/watch?v=xxx"

# Get info as JSON
souris-dw info "URL" --json
```

### Search

```bash
# Search YouTube
souris-dw search "never gonna give you up"

# Search with limit
souris-dw search "query" --limit 20

# Search from specific platform
souris-dw search "query" --platform youtube

# Search as JSON
souris-dw search "query" --json
```

### Update

```bash
# Update all dependencies
souris-dw update

# Update specific dependencies
souris-dw update --yt-dlp
souris-dw update --ffmpeg

# Check for updates without installing
souris-dw update --check

# Update as JSON
souris-dw update --json
```

### Dependencies

```bash
# Install dependencies (force re-download)
souris-dw deps install

# Show dependency status
souris-dw deps status

# Update dependencies
souris-dw deps update

# Status as JSON
souris-dw deps status --json
```

### Configuration

```bash
# Show current config
souris-dw config show

# Get a config value
souris-dw config get download.default_format

# Set a config value
souris-dw config set download.default_format mp3
souris-dw config set download.default_quality 320
souris-dw config set download.output_dir ~/Music
souris-dw config set download.parallel 8
souris-dw config set download.embed_metadata true
souris-dw config set yt_dlp.channel nightly
souris-dw config set ffmpeg.auto_update false
```

### Uninstall

```bash
# Remove binary (keeps config)
souris-dw uninstall

# Remove everything (binary + config + data)
souris-dw uninstall --keep-config=false
```

### TUI Mode

```bash
souris-dw tui
```

**Keyboard shortcuts:**
| Key | Action |
|-----|--------|
| `a` | Add URL |
| `/` | Search |
| `j` / `Down` | Move down |
| `k` / `Up` | Move up |
| `g` / `Home` | Go to first |
| `G` / `End` | Go to last |
| `Enter` | Download selected |
| `y` | Copy URL to clipboard |
| `d` / `Delete` | Delete selected |
| `p` | Pause/Resume |
| `s` | Settings |
| `h` / `?` | Help |
| `q` / `Esc` | Back / Quit |
| `Ctrl+c` | Force quit |

## JSON Output

All commands support `--json` for machine-readable output:

```bash
# Get info as JSON
souris-dw info "URL" --json

# Download with JSON progress
souris-dw download "URL" --json --format mp4

# Search as JSON
souris-dw search "query" --json

# Check dependency status
souris-dw deps status --json
```

### Progress Events

When using `--json`, progress is streamed as one JSON object per line:

```json
{"type":"init","url":"...","platform":"youtube","title":"...","media_type":"video","total_items":1}
{"type":"progress","item":1,"total":1,"percent":45.2,"speed":"2.3MB/s","eta":"00:12"}
{"type":"postprocess","item":1,"total":1,"stage":"converting","format":"mp4"}
{"type":"metadata","item":1,"total":1,"stage":"embedding_tags"}
{"type":"complete","item":1,"total":1,"path":"/path/to/file.mp4","size":125000000}
{"type":"error","item":1,"total":1,"code":"DOWNLOAD_FAILED","message":"..."}
{"type":"summary","total":10,"success":9,"failed":1,"elapsed":"02:34"}
```

## Configuration File

Located at:
- Linux: `~/.config/souris-dw/config.toml`
- macOS: `~/Library/Application Support/souris-dw/config.toml`
- Windows: `%APPDATA%\souris-dw\config.toml`

```toml
[yt_dlp]
auto_update = true
channel = "stable"       # stable, nightly, master

[ffmpeg]
auto_update = false

[download]
default_format = "mp4"
default_quality = "1080p"
output_dir = "./downloads"
parallel = 4
embed_metadata = true
embed_thumbnail = true
embed_subtitles = false
timeout = 300
max_retries = 3

[spotify]
client_id = ""
client_secret = ""
```

## Supported Formats

### Audio

| Format | Thumbnail | Notes |
|--------|-----------|-------|
| MP3 | Yes | ID3v2 attached picture (album cover) |
| FLAC | Yes | |
| AAC | Yes | |
| OGG | Yes | |
| M4A | Yes | |
| WAV | No | Container does not support metadata/thumbnails |

### Video

| Format | Thumbnail | Notes |
|--------|-----------|-------|
| MP4 | Yes | Attached picture |
| MKV | Yes | |
| WebM | No | Container does not support thumbnails |
| AVI | No | Container does not support thumbnails |
| MOV | Limited | Only with single format (no merge). Merged files lose thumbnail. |

### Quality

- Audio: 128kbps, 192kbps, 256kbps, 320kbps, lossless
- Video: 360p, 480p, 720p, 1080p, 1440p, 4K, 8K

## Supported Platforms

- YouTube (videos, playlists, channels)
- Spotify (tracks, playlists, albums -> searched on YouTube)
