# Usage Guide

Complete CLI and TUI reference. Every command has a **purpose**, its **flags**, and **examples**.

---

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

### Manual Download

**Linux x86_64 (musl - works on any distro):**
```bash
curl -sL https://github.com/SourisCG/SourisDW/releases/latest/download/souris-dw-linux-x86_64 -o souris-dw
chmod +x souris-dw
sudo mv souris-dw /usr/local/bin/
```

**Linux x86_64 (glibc - Ubuntu, Debian, Fedora, etc.):**
```bash
curl -sL https://github.com/SourisCG/SourisDW/releases/latest/download/souris-dw-linux-x86_64-glibc -o souris-dw
chmod +x souris-dw
sudo mv souris-dw /usr/local/bin/
```

**macOS (Apple Silicon):**
```bash
curl -sL https://github.com/SourisCG/SourisDW/releases/latest/download/souris-dw-macos-aarch64 -o souris-dw
chmod +x souris-dw
sudo mv souris-dw /usr/local/bin/
```

**Windows (x64):**
Download `souris-dw-windows-x86_64.exe` from [releases](https://github.com/SourisCG/SourisDW/releases) and add to your PATH.

---

## Global Flags

These flags work on **every** command.

| Flag | Description |
|------|-------------|
| `--json` | Machine-readable JSON output |
| `--quiet` | Suppress progress bars and non-essential output |
| `--no-auto-update` | Skip automatic dependency updates (deterministic runs) |
| `--no-color` | Disable colored output |
| `--timeout <seconds>` | Download timeout (default: 300) |
| `--max-retries <n>` | Max retries on failure (default: 3) |

---

## download

**Purpose:** download a video, audio, playlist, or album from YouTube or Spotify.

**Options:**
| Flag | Description |
|------|-------------|
| `-f, --format` | Output format (mp3, flac, mp4, mkv, etc.) |
| `-q, --quality` | Quality (128kbps, 320kbps, 360p, 1080p, 4K, lossless) |
| `-o, --output` | Output directory |
| `-p, --parallel` | Number of parallel downloads for playlists (default: 4) |
| `--audio-only` | Download audio only |
| `--video-only` | Download video only |
| `--embed-metadata` | Embed ID3/metadata tags |
| `--embed-thumbnail` | Embed thumbnail/album art |
| `--embed-subtitles` | Download and embed subtitles |
| `--cookies <file>` | Cookies file for authenticated downloads |
| `--cookies-from-browser <browser>` | Extract cookies from a browser (firefox, chrome, etc.) |

**Examples:**
```bash
# Basic download (defaults: mp4, 1080p)
souris-dw download "https://youtube.com/watch?v=xxx"

# Audio only as MP3
souris-dw download "URL" --audio-only --format mp3

# Lossless audio
souris-dw download "URL" --audio-only --format flac --quality lossless

# Video at 4K
souris-dw download "URL" --format mp4 --quality 4K

# To a specific directory with tags and album art
souris-dw download "URL" --output ~/Music --embed-metadata --embed-thumbnail

# Playlist with 8 parallel downloads
souris-dw download "PLAYLIST_URL" --parallel 8

# Spotify album (requires Spotify API credentials in config)
souris-dw download "https://open.spotify.com/album/xxx" --audio-only --format mp3

# Authenticated/private downloads
souris-dw download "URL" --cookies cookies.txt
souris-dw download "URL" --cookies-from-browser firefox

# JSON progress for subprocess integration
souris-dw download "URL" --json --format mp4 --quality 1080p
```

---

## info

**Purpose:** get metadata about a URL **without downloading** it (title, duration, uploader, formats, subtitles).

```bash
# Human-readable
souris-dw info "https://youtube.com/watch?v=xxx"

# JSON (full MediaInfo object, including available formats)
souris-dw info "URL" --json
```

---

## search

**Purpose:** search YouTube for media.

**Options:**
| Flag | Description |
|------|-------------|
| `-p, --platform` | Platform to search (only `youtube` is supported) |
| `-l, --limit` | Number of results (default: 10) |

```bash
# Search
souris-dw search "never gonna give you up"

# More results
souris-dw search "query" --limit 20

# As JSON (array of results with id/title/url/duration/uploader)
souris-dw search "query" --json
```

---

## update

**Purpose:** update SourisDW's runtime dependencies (yt-dlp, ffmpeg, deno).

**Options:**
| Flag | Description |
|------|-------------|
| `--yt-dlp` | Update only yt-dlp |
| `--ffmpeg` | Update only ffmpeg (and ffprobe) |
| `--self` | Update the SourisDW binary itself (not supported yet) |
| `--check` | Check for updates without installing |

```bash
# Update everything
souris-dw update

# Update only yt-dlp
souris-dw update --yt-dlp

# Check what's outdated (reports update_available + latest per dependency)
souris-dw update --check

# JSON output (array of DepStatus objects)
souris-dw update --json
```

---

## deps

**Purpose:** manage runtime dependencies (yt-dlp, ffmpeg/ffprobe, deno). These are auto-downloaded on first run; this command lets you control them explicitly.

**Subcommands:**
| Command | Purpose |
|---------|---------|
| `deps install` | Force re-download all dependencies |
| `deps status` | Show installation status of each dependency |
| `deps update` | Update all dependencies |

```bash
# Force re-download (e.g. after a corrupted binary)
souris-dw deps install

# Status as JSON
souris-dw deps status --json

# Update
souris-dw deps update
```

---

## config

**Purpose:** view and edit the configuration file (`config.toml`).

**Subcommands:**
| Command | Purpose |
|---------|---------|
| `config show` | Print the full config as TOML |
| `config get <key>` | Print a single value |
| `config set <key> <value>` | Set a value (persisted immediately) |

**Supported keys:**
| Key | Type | Default |
|-----|------|---------|
| `yt_dlp.auto_update` | bool | false |
| `yt_dlp.channel` | stable/nightly/master | stable |
| `ffmpeg.auto_update` | bool | false |
| `download.default_format` | string | mp4 |
| `download.default_quality` | string | 1080p |
| `download.output_dir` | path | system Downloads folder |
| `download.parallel` | number | 4 |
| `download.embed_metadata` | bool | true |
| `download.embed_thumbnail` | bool | true |
| `download.embed_subtitles` | bool | false |
| `download.timeout` | number (seconds) | 300 |
| `download.max_retries` | number | 3 |

```bash
# Show current config
souris-dw config show

# Get a value
souris-dw config get download.default_format

# Set values
souris-dw config set download.default_format mp3
souris-dw config set download.default_quality 320
souris-dw config set download.output_dir ~/Music
souris-dw config set download.parallel 8
souris-dw config set download.embed_metadata true
souris-dw config set yt_dlp.channel nightly
souris-dw config set ffmpeg.auto_update false
```

---

## setup

**Purpose:** one-time initialization: creates the config file, dependency directories, and points downloads at your system Downloads folder. The installer runs it automatically.

```bash
# Non-interactive setup
souris-dw setup --quiet
```

---

## uninstall

**Purpose:** remove the SourisDW binary. By default it also removes your config and data; use `--keep-config` to keep them.

**Options:**
| Flag | Description |
|------|-------------|
| `--keep-config` | Keep config and data files (default removes them) |

```bash
# Remove everything (binary + config + data)
souris-dw uninstall

# Remove binary but keep config and data
souris-dw uninstall --keep-config
```

---

## tui

**Purpose:** launch the interactive terminal UI.

```bash
souris-dw tui
```

**Keyboard shortcuts:**
| Key | Action |
|-----|--------|
| `a` | Add URL |
| `/` / `Ctrl+F` | Search |
| `j` / `Down` | Move down |
| `k` / `Up` | Move up |
| `g` / `Home` | Go to first |
| `G` / `End` | Go to last |
| `Enter` | Download selected |
| `y` | Copy URL to clipboard |
| `d` / `Delete` | Delete selected |
| `s` | Settings (changes persist to config) |
| `h` / `?` | Help |
| `q` / `Esc` | Back / Quit |
| `Ctrl+C` | Force quit |

---

## JSON Output

All commands support `--json` for machine-readable output:

```bash
# Get info as JSON
souris-dw info "URL" --json

# Download with JSON progress
souris-dw download "URL" --json --format mp4

# Search as JSON
souris-dw search "query" --json

# Dependency status
souris-dw deps status --json
```

### Progress Events

When using `download --json`, progress is streamed as **one JSON object per line**:

```json
{"type":"init","url":"...","platform":"youtube","title":"...","media_type":"video","total_items":1}
{"type":"progress","item":1,"total":1,"percent":45.2,"speed":"2.3MB/s","eta":"00:12"}
{"type":"postprocess","item":1,"total":1,"stage":"ExtractAudio","format":"mp3"}
{"type":"metadata","item":1,"total":1,"stage":"Metadata"}
{"type":"complete","item":1,"total":1,"path":"/path/to/file.mp4","size":125000000}
{"type":"error","item":1,"total":1,"code":"DOWNLOAD_FAILED","message":"..."}
{"type":"summary","total":10,"success":9,"failed":1,"elapsed":"02:34"}
```

### Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success or cancelled by user |
| 1 | General error |
| 2 | Dependency error (missing binary, download failure) |
| 3 | Network error or timeout |

---

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

---

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

---

## Supported Platforms

- YouTube (videos, playlists, channels)
- Spotify (tracks, playlists, albums -> searched on YouTube; requires Spotify API credentials)
