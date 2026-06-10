# Usage Guide

## Installation

```bash
# Linux x86_64 (Ubuntu/Debian)
curl -sL https://github.com/SourisCG/SourisDW/releases/latest/download/souris-dw-linux-x86_64 -o souris-dw
chmod +x souris-dw
sudo mv souris-dw /usr/local/bin/

# Fedora/RHEL (native glibc build)
curl -sL https://github.com/SourisCG/SourisDW/releases/latest/download/souris-dw-linux-x86_64-fedora -o souris-dw
chmod +x souris-dw
sudo mv souris-dw /usr/local/bin/

# Linux (musl, works on any distro)
curl -sL https://github.com/SourisCG/SourisDW/releases/latest/download/souris-dw-linux-x86_64-musl -o souris-dw
chmod +x souris-dw
sudo mv souris-dw /usr/local/bin/

# Linux aarch64
curl -sL https://github.com/SourisCG/SourisDW/releases/latest/download/souris-dw-linux-aarch64 -o souris-dw
chmod +x souris-dw

# macOS
curl -sL https://github.com/SourisCG/SourisDW/releases/latest/download/souris-dw-macos-x86_64 -o souris-dw
chmod +x souris-dw

# Windows
# Download souris-dw-windows-x86_64.exe from releases
```

## CLI Commands

### Download

```bash
# Basic download
souris-dw download "https://youtube.com/watch?v=xxx"

# Download audio only
souris-dw download "URL" --audio-only --format mp3

# Download video with specific quality
souris-dw download "URL" --format mp4 --quality 1080p

# Download to specific directory
souris-dw download "URL" --output ~/Music

# Download with metadata
souris-dw download "URL" --embed-metadata --embed-thumbnail

# Download playlist
souris-dw download "PLAYLIST_URL" --parallel 8

# Download with subtitles
souris-dw download "URL" --embed-subtitles
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
| `--embed-metadata` | Embed ID3 metadata |
| `--embed-thumbnail` | Embed thumbnail/album art |
| `--embed-subtitles` | Download and embed subtitles |

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

# Search as JSON
souris-dw search "query" --json
```

### Update

```bash
# Update all dependencies
souris-dw update

# Check for updates without installing
souris-dw update --check

# Update as JSON
souris-dw update --json
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
souris-dw config set download.output ~/Music
souris-dw config set download.parallel 8
souris-dw config set download.embed_metadata true
souris-dw config set yt_dlp.channel nightly
```

### Dependencies

```bash
# Show dependency status
souris-dw deps status

# Update dependencies
souris-dw deps update
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
| `q` / `Esc` | Back (double-Esc to quit) |
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
```

### Progress Events

When using `--json`, progress is streamed as one JSON object per line:

```json
{"type":"init","url":"...","platform":"youtube","title":"...","total_items":1}
{"type":"progress","item":1,"total":1,"percent":45.2,"speed":"2.3MB/s","eta":"00:12"}
{"type":"complete","item":1,"total":1,"path":"/path/to/file.mp4","size":125000000}
{"type":"summary","total":1,"success":1,"failed":0,"elapsed":"00:15"}
```

## Configuration File

Located at:
- Linux: `~/.config/souris-dw/config.toml`
- macOS: `~/Library/Application Support/souris-dw/config.toml`
- Windows: `%APPDATA%\souris-dw\config.toml`

```toml
[yt_dlp]
auto_update = true
channel = "nightly"  # stable, nightly, master

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
MP3, FLAC, AAC, OGG, M4A, WAV

### Video
MP4, MKV, WebM, AVI, MOV

### Quality
- Audio: 128kbps, 192kbps, 256kbps, 320kbps, lossless
- Video: 360p, 480p, 720p, 1080p, 1440p, 4K, 8K

## Supported Platforms

- YouTube (videos, playlists, channels)
- Spotify (tracks, playlists, albums → searched on YouTube)
