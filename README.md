<pre align="center">
 _____                 _      __        __   _
/ ____|               (_)     \ \      / /__| |__  _ __ ___
\___ \_   _ _ __  _ __ _  ___ \ \ /\ / / _ \ '_ \| '__/ _ \
 ___) | | | | '_ \| '__| |/ __| \ V  V /  __/ |_) | | |  __/
|____/ \__,_| .__/|_|  |_|\___|  \_/\_/ \___|_.__/|_|  \___|
            |_|
</pre>

<p align="center">
  <strong>Cross-platform music & video downloader for YouTube and Spotify</strong>
</p>

<p align="center">
  <a href="https://github.com/SourisCG/SourisDW/actions/workflows/ci.yml"><img src="https://github.com/SourisCG/SourisDW/actions/workflows/ci.yml/badge.svg" /></a>
  <a href="https://github.com/SourisCG/SourisDW/actions/workflows/release.yml"><img src="https://github.com/SourisCG/SourisDW/actions/workflows/release.yml/badge.svg" /></a>
  <img src="https://img.shields.io/badge/version-0.3.6-blue?style=flat-square" />
  <img src="https://img.shields.io/badge/rust-1.70%2B-orange?style=flat-square&logo=rust" />
  <a href="https://github.com/SourisCG/SourisDW/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-MIT-green?style=flat-square" /></a>
  <img src="https://img.shields.io/badge/platform-linux%20%7C%20macos%20%7C%20windows-lightgrey?style=flat-square" />
</p>

<p align="center">
  <a href="https://github.com/SourisCG/SourisDW/releases/latest/download/souris-dw-linux-x86_64"><img src="https://img.shields.io/badge/Linux_x86__64_(musl)-download-purple?style=for-the-badge&logo=linux&logoColor=white" /></a>
  <a href="https://github.com/SourisCG/SourisDW/releases/latest/download/souris-dw-linux-x86_64-glibc"><img src="https://img.shields.io/badge/Linux_x86__64_(glibc)-download-blue?style=for-the-badge&logo=linux&logoColor=white" /></a>
  <a href="https://github.com/SourisCG/SourisDW/releases/latest/download/souris-dw-linux-x86_64-fedora"><img src="https://img.shields.io/badge/Fedora_x86__64-download-orange?style=for-the-badge&logo=fedora&logoColor=white" /></a>
  <a href="https://github.com/SourisCG/SourisDW/releases/latest/download/souris-dw-linux-aarch64"><img src="https://img.shields.io/badge/Linux_ARM64_(musl)-download-purple?style=for-the-badge&logo=linux&logoColor=white" /></a>
  <a href="https://github.com/SourisCG/SourisDW/releases/latest/download/souris-dw-linux-aarch64-glibc"><img src="https://img.shields.io/badge/Linux_ARM64_(glibc)-download-blue?style=for-the-badge&logo=linux&logoColor=white" /></a>
  <a href="https://github.com/SourisCG/SourisDW/releases/latest/download/souris-dw-macos-x86_64"><img src="https://img.shields.io/badge/macOS_Intel-download-blue?style=for-the-badge&logo=apple&logoColor=white" /></a>
  <a href="https://github.com/SourisCG/SourisDW/releases/latest/download/souris-dw-macos-aarch64"><img src="https://img.shields.io/badge/macOS_Apple_Silicon-download-blue?style=for-the-badge&logo=apple&logoColor=white" /></a>
  <a href="https://github.com/SourisCG/SourisDW/releases/latest/download/souris-dw-windows-x86_64.exe"><img src="https://img.shields.io/badge/Windows_x64-download-blue?style=for-the-badge&logo=windows&logoColor=white" /></a>
  <a href="https://github.com/SourisCG/SourisDW/releases/latest/download/souris-dw-windows-arm64.exe"><img src="https://img.shields.io/badge/Windows_ARM64-download-blue?style=for-the-badge&logo=windows&logoColor=white" /></a>
</p>

---

## Features

- Download music and video from **YouTube** and **Spotify**
- Full playlist support for both platforms
- Audio formats: MP3, FLAC, AAC, OGG, M4A, WAV
- Video formats: MP4, MKV, WebM, AVI, MOV
- Quality selection: 128kbps to lossless (audio), 360p to 8K (video)
- Automatic metadata embedding (ID3 tags, album art)
- Thumbnail embedding for all compatible formats (MP3, MP4, MKV, FLAC, AAC, OGG, M4A)
- Subtitles support
- Parallel downloads
- CLI and interactive TUI modes
- Use as a **library** in your own Rust projects
- Use as a **motor** from any programming language via subprocess + JSON
- Zero external dependencies to install on supported dependency platforms (yt-dlp, ffmpeg, ffprobe, deno auto-downloaded at runtime; Windows ARM64 uses system/fallback ffmpeg until upstream assets exist)
- Runtime dependency manager with progress bars
- Automatic first-run setup (`souris-dw setup --quiet`) creates config, dependency dirs, and uses your system Downloads folder by default
- Auto-updates yt-dlp, ffmpeg, and deno (configurable)
- Cross-platform: Linux (musl + glibc, x86_64 + aarch64), macOS (Intel + Apple Silicon), Windows (x64 + ARM64)
- HTTP 403 auto-retry with android client fallback
- Trailing-dot-safe filenames (`--replace-in-metadata`)

---

## Quick Start

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

**Linux (musl - works on any distro):**
```bash
curl -sL https://github.com/SourisCG/SourisDW/releases/latest/download/souris-dw-linux-x86_64 -o souris-dw
chmod +x souris-dw
sudo mv souris-dw /usr/local/bin/
```

**Linux (glibc - for Ubuntu/Debian):**
```bash
curl -sL https://github.com/SourisCG/SourisDW/releases/latest/download/souris-dw-linux-x86_64-glibc -o souris-dw
chmod +x souris-dw
sudo mv souris-dw /usr/local/bin/
```

**macOS:**
```bash
curl -sL https://github.com/SourisCG/SourisDW/releases/latest/download/souris-dw-macos-x86_64 -o souris-dw
chmod +x souris-dw
sudo mv souris-dw /usr/local/bin/
```

**Windows:**
Download `souris-dw-windows-x86_64.exe` from [releases](https://github.com/SourisCG/SourisDW/releases) and add to your PATH.

### Verify Installation

```bash
souris-dw --version
```

---

## Usage

### CLI Mode

```bash
# Download video
souris-dw download "https://youtube.com/watch?v=xxx"

# Download audio only
souris-dw download "https://youtube.com/watch?v=xxx" --audio-only --format mp3

# Download with specific quality
souris-dw download "https://youtube.com/watch?v=xxx" --format mp4 --quality 1080p

# Download playlist
souris-dw download "https://youtube.com/playlist?list=xxx" --parallel 8

# Search
souris-dw search "never gonna give you up"

# Get info without downloading
souris-dw info "https://youtube.com/watch?v=xxx"

# Launch TUI
souris-dw tui

# Update dependencies
souris-dw update

# Install dependencies (force re-download)
souris-dw deps install

# Configure paths and dependencies without prompts
souris-dw setup --quiet

# Check dependency status
souris-dw deps status

# Configuration
souris-dw config show
souris-dw config get download.default_format
souris-dw config set download.default_format mp3

# Download with JSON output (for subprocess integration)
souris-dw download "URL" --json --format mp4 --quality 1080p

# Skip auto-update (for reproducible behavior)
souris-dw download "URL" --no-auto-update
```

**Global flags:**
| Flag | Description |
|------|-------------|
| `--json` | Machine-readable JSON output |
| `--quiet` | Suppress progress bars |
| `--no-auto-update` | Skip automatic dependency updates |
| `--no-color` | Disable colored output |
| `--timeout <seconds>` | Download timeout (default: 300) |
| `--max-retries <n>` | Max retries on failure (default: 3) |

### TUI Mode

```bash
souris-dw tui
```

Interactive terminal interface with:
- Real-time download progress
- Queue management
- Keyboard shortcuts (vim-style navigation)
- Settings panel
- Clipboard support (y copies URL)

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

### As a Library (Rust)

```rust
use souris_dw::SourisDW;

let dw = SourisDW::builder()
    .format("mp4")
    .quality("1080p")
    .output("./downloads")
    .build()
    .await?;

dw.download("https://youtube.com/watch?v=xxx").await?;
```

See [Library Guide](docs/LIBRARY.md) for the full fluent API.

### As a Motor (Any Language)

SourisDW can be used from any programming language via subprocess with JSON output. Every CLI command supports `--json` for machine-readable output.

```bash
# Download with JSON progress
souris-dw download "URL" --json --format mp4 --quality 1080p

# Get info as JSON
souris-dw info "URL" --json
```

**Progress events** (one JSON object per line):
```json
{"type":"init","url":"...","platform":"youtube","title":"...","total_items":1}
{"type":"progress","item":1,"total":1,"percent":45.2,"speed":"2.3MB/s","eta":"00:12"}
{"type":"postprocess","item":1,"total":1,"stage":"converting","format":"mp4"}
{"type":"metadata","item":1,"total":1,"stage":"embedding_tags"}
{"type":"complete","item":1,"total":1,"path":"/path/file.mp4","size":125000000}
{"type":"error","item":1,"total":1,"code":"DOWNLOAD_FAILED","message":"..."}
{"type":"summary","total":10,"success":9,"failed":1,"elapsed":"02:34"}
```

**Exit codes:**
| Code | Meaning |
|------|---------|
| 0 | Success or cancelled by user |
| 1 | General error |
| 2 | Dependency error |
| 3 | Network error or timeout |

See [Integration Guide](docs/INTEGRATION.md) for examples in Python, Node.js, Java, Go, and C#.

---

## How It Works

```
User Input (URL)
      |
      v
+-----------+     +----------+     +----------+
|  Resolver |---->| Extractor|---->| Downloader|
+-----------+     +----------+     +-----+-----+
      |                                   |
      | Detects:                         | Uses:
      | - YouTube                        | - yt-dlp (auto-downloaded)
      | - Spotify                        | - ffmpeg (auto-downloaded)
      | - Playlist                       | - deno (auto-downloaded)
      v                                   v
+-----------+                      +-----------+
| Metadata  |                      | File Save |
+-----------+                      +-----+-----+
      |                                   |
      v                                   v
+-----------+                      +-----------+
| Post-     |                      | Metadata  |
| Process   |                      | Embedding |
+-----------+                      +-----------+
```

### Runtime Dependency Management

All external dependencies (yt-dlp, ffmpeg, ffprobe, deno) are downloaded at runtime to the platform-specific data directory. The `build.rs` is a no-op. Dependencies are:

- **yt-dlp**: Downloaded from GitHub releases. Supports stable, nightly, and master channels.
- **ffmpeg/ffprobe**: Downloaded from `eugeneware/ffmpeg-static` releases as gzip archives.
- **deno**: Downloaded from denoland/deno releases as zip archives (used as JS runtime for yt-dlp).

`deps install` force re-downloads all dependencies. `--auto-update` (default: on) checks for updates in the background. Use `--no-auto-update` for deterministic behavior.

---

## Configuration

Configuration is stored at:
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

### Supported Formats

| Format | Type | Thumbnail Embedding | Notes |
|--------|------|---------------------|-------|
| MP3 | Audio | Yes (ID3v2 attached picture) | |
| FLAC | Audio | Yes | |
| AAC | Audio | Yes | |
| OGG | Audio | Yes | |
| M4A | Audio | Yes | |
| WAV | Audio | No | Container does not support thumbnails |
| MP4 | Video | Yes (attached pic) | |
| MKV | Video | Yes | |
| WebM | Video | No | Container does not support thumbnails |
| AVI | Video | No | Container does not support thumbnails |
| MOV | Video | Limited | Thumbnail only works without merge (single format) |

### Quality

- Audio: 128kbps, 192kbps, 256kbps, 320kbps, lossless
- Video: 360p, 480p, 720p, 1080p, 1440p, 4K, 8K

---

## Building from Source

```bash
# Prerequisites
# - Rust 1.70+
# No other dependencies needed (ffmpeg, yt-dlp, deno download at runtime)

# Clone
git clone https://github.com/SourisCG/SourisDW.git
cd SourisDW

# Build
cargo build --release

# Run
./target/release/souris-dw --version
```

---

## Cross-Platform Notes

| Feature | Linux | macOS | Windows |
|---------|-------|-------|---------|
| Config path | `~/.config/souris-dw/` | `~/Library/Application Support/souris-dw/` | `%APPDATA%\souris-dw\` |
| Data path | `~/.local/share/souris-dw/` | `~/Library/Application Support/souris-dw/` | `%LOCALAPPDATA%\souris-dw\` |
| Cache path | `~/.cache/souris-dw/` | `~/Library/Caches/souris-dw/` | `%LOCALAPPDATA%\souris-dw\cache\` |
| Binary path | `~/.local/share/souris-dw/bin/` | `~/Library/Application Support/souris-dw/bin/` | `%LOCALAPPDATA%\souris-dw\bin\` |
| Case sensitivity | Yes | No | No |
| Path separator | `/` | `/` | `\` |

Pre-built binaries per platform:

| Binary | Platform | Libc |
|--------|----------|------|
| `souris-dw-linux-x86_64` | Linux x86_64 | musl (works on all distros) |
| `souris-dw-linux-x86_64-glibc` | Linux x86_64 | glibc |
| `souris-dw-linux-x86_64-fedora` | Linux x86_64 | glibc (Fedora 41) |
| `souris-dw-linux-aarch64` | Linux ARM64 | musl |
| `souris-dw-linux-aarch64-glibc` | Linux ARM64 | glibc |
| `souris-dw-macos-x86_64` | macOS Intel | - |
| `souris-dw-macos-aarch64` | macOS Apple Silicon | - |
| `souris-dw-windows-x86_64.exe` | Windows x64 | - |
| `souris-dw-windows-arm64.exe` | Windows ARM64 | - |

---

## SDKs

Official SDKs with fluent API for multiple languages:

| Language | Package |
|----------|---------|
| Python | `pip install souris-dw` |
| Node.js | `npm install souris-dw` |
| Go | `go get github.com/SourisCG/SourisDW-go` |
| Java | Maven: `io.souris:souris-dw` |
| C# | NuGet: `SourisDW` |

---

## Documentation

- [Architecture](docs/ARCHITECTURE.md) - System architecture and module structure
- [Usage Guide](docs/USAGE.md) - Complete CLI and TUI usage reference
- [Library Guide](docs/LIBRARY.md) - Using SourisDW as a Rust library
- [Integration Guide](docs/INTEGRATION.md) - Using SourisDW from other languages
- [Cross-Platform](docs/CROSS_PLATFORM.md) - Platform-specific notes
- [Contributing](CONTRIBUTING.md) - How to contribute
- [Security](SECURITY.md) - Security policy

---

## License

MIT License - see [LICENSE](LICENSE) for details.

---

## Espanol

<pre>
 _____                 _      __        __   _
/ ____|               (_)     \ \      / /__| |__  _ __ ___
\___ \_   _ _ __  _ __ _  ___ \ \ /\ / / _ \ '_ \| '__/ _ \
 ___) | | | | '_ \| '__| |/ __| \ V  V /  __/ |_) | | |  __/
|____/ \__,_| .__/|_|  |_|\___|  \_/\_/ \___|_.__/|_|  \___|
            |_|
</pre>

<p align="center">
  <strong>Descargador de musica y video multiplataforma para YouTube y Spotify</strong>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-0.3.6-blue?style=flat-square" />
  <a href="https://github.com/SourisCG/SourisDW/blob/main/LICENSE"><img src="https://img.shields.io/badge/licencia-MIT-green?style=flat-square" /></a>
  <img src="https://img.shields.io/badge/plataforma-linux%20%7C%20macos%20%7C%20windows-lightgrey?style=flat-square" />
</p>

### Caracteristicas

- Descarga musica y video de **YouTube** y **Spotify**
- Soporte completo para listas de reproduccion
- Formatos de audio: MP3, FLAC, AAC, OGG, M4A, WAV
- Formatos de video: MP4, MKV, WebM, AVI, MOV
- Calidad seleccionable: 128kbps a lossless (audio), 360p a 8K (video)
- Incrustacion automatica de metadatos (ID3, caratulas)
- Subtitulos
- Descargas paralelas
- Modo CLI y TUI interactivo
- Usa como **libreria** en tus proyectos Rust
- Usa como **motor** desde cualquier lenguaje via subprocess + JSON
- Sin dependencias externas (yt-dlp, ffmpeg, deno se descargan solos al ejecutar)
- Actualizacion automatica de dependencias (configurable)
- Multiplataforma: Linux, macOS, Windows

### Instalacion Rapida

**Linux y macOS:**
```bash
curl -fsSL https://raw.githubusercontent.com/SourisCG/SourisDW/main/install.sh | bash
```

**Windows (PowerShell):**
```powershell
irm https://raw.githubusercontent.com/SourisCG/SourisDW/main/install.ps1 | iex
```

### Uso

```bash
# Descargar video
souris-dw download "https://youtube.com/watch?v=xxx"

# Solo audio
souris-dw download "URL" --audio-only --format mp3

# Con calidad especifica
souris-dw download "URL" --format mp4 --quality 1080p

# Lista de reproduccion
souris-dw download "URL" --parallel 8

# Buscar
souris-dw search "never gonna give you up"

# Informacion sin descargar
souris-dw info "https://youtube.com/watch?v=xxx"

# Interfaz TUI
souris-dw tui

# Instalar/actualizar dependencias
souris-dw deps install
souris-dw update

# Configuracion
souris-dw config show
souris-dw config set download.default_format mp3

# Salida JSON (para integracion con otros lenguajes)
souris-dw download "URL" --json --format mp4
```

### Como Funciona

```
Entrada (URL)
      |
      v
+-----------+     +----------+     +----------+
|  Resolver |---->| Extractor|---->| Downloader|
+-----------+     +----------+     +-----+-----+
      |                                   |
      | Detecta:                         | Usa:
      | - YouTube                        | - yt-dlp
      | - Spotify                        | - ffmpeg
      | - Playlist                       | - deno
      v                                   v
+-----------+                      +-----------+
| Metadata  |                      | Archivo   |
+-----------+                      +-----+-----+
      |                                   |
      v                                   v
+-----------+                      +-----------+
| Post-     |                      | Caratula  |
| Proceso   |                      | Metadata  |
+-----------+                      +-----------+
```

### Configuracion

Archivo en `~/.config/souris-dw/config.toml`:

```toml
[yt_dlp]
auto_update = true
channel = "stable"

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
```

### Formatos Soportados

| Formato | Tipo | Caratula | Notas |
|---------|------|----------|-------|
| MP3 | Audio | Si | ID3v2 attached picture |
| FLAC | Audio | Si | |
| AAC | Audio | Si | |
| OGG | Audio | Si | |
| M4A | Audio | Si | |
| WAV | Audio | No | El contenedor no soporta caratulas |
| MP4 | Video | Si | |
| MKV | Video | Si | |
| WebM | Video | No | No soporta caratulas |
| AVI | Video | No | No soporta caratulas |
| MOV | Video | Limitado | Solo sin merge (formato unico) |

### Compilacion desde Codigo Fuente

```bash
git clone https://github.com/SourisCG/SourisDW.git
cd SourisDW
cargo build --release
./target/release/souris-dw --version
```

Requiere Rust 1.70+. No necesita otras dependencias.

### Documentacion en Espanol

- [Arquitectura](docs/ES/ARCHITECTURE.md)
- [Guia de uso](docs/ES/USAGE.md)
- [Uso como libreria](docs/ES/LIBRARY.md)
- [Integracion con otros lenguajes](docs/ES/INTEGRATION.md)
- [Notas multiplataforma](docs/ES/CROSS_PLATFORM.md)

### Licencia

MIT
