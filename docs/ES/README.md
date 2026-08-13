# SourisDW

Descargador de musica y video multiplataforma para YouTube y Spotify.

<p align="center">
  <img src="https://img.shields.io/badge/version-0.4.0-blue?style=flat-square" />
  <a href="https://github.com/SourisCG/SourisDW/blob/main/LICENSE"><img src="https://img.shields.io/badge/licencia-MIT-green?style=flat-square" /></a>
  <img src="https://img.shields.io/badge/plataforma-linux%20%7C%20macos%20%7C%20windows-lightgrey?style=flat-square" />
</p>

> Documentacion principal en ingles: [README.md](../../README.md)

## Caracteristicas

- Descarga musica y video de **YouTube** y **Spotify**
- Soporte completo para playlists y **albumes** en ambas plataformas
- Formatos de audio: MP3, FLAC, AAC, OGG, M4A, WAV
- Formatos de video: MP4, MKV, WebM, AVI, MOV
- Calidad seleccionable: 128kbps a lossless (audio), 360p a 8K (video)
- Incrustacion automatica de metadatos (ID3, caratulas)
- Subtitulos
- Descargas paralelas
- Modo CLI y TUI interactivo con progreso en tiempo real
- Usa como **libreria** en tus proyectos Rust
- Usa como **motor** desde cualquier lenguaje via subprocess + JSON
- Sin dependencias externas que instalar (yt-dlp, ffmpeg, deno se descargan solos al ejecutar)
- Actualizacion automatica de dependencias (configurable)
- Multiplataforma: Linux (musl + glibc), macOS (Apple Silicon), Windows (x64)

## Instalacion Rapida

**Linux y macOS:**
```bash
curl -fsSL https://raw.githubusercontent.com/SourisCG/SourisDW/main/install.sh | bash
```

**Windows (PowerShell):**
```powershell
irm https://raw.githubusercontent.com/SourisCG/SourisDW/main/install.ps1 | iex
```

## Uso

```bash
# Descargar video
souris-dw download "https://youtube.com/watch?v=xxx"

# Solo audio
souris-dw download "URL" --audio-only --format mp3

# Con calidad especifica
souris-dw download "URL" --format mp4 --quality 1080p

# Playlist o album con descargas paralelas
souris-dw download "URL" --parallel 8

# Descargas autenticadas
souris-dw download "URL" --cookies cookies.txt
souris-dw download "URL" --cookies-from-browser firefox

# Buscar
souris-dw search "never gonna give you up"

# Informacion sin descargar
souris-dw info "https://youtube.com/watch?v=xxx"

# Interfaz TUI
souris-dw tui

# Instalar/actualizar dependencias
souris-dw deps install
souris-dw deps status
souris-dw update
souris-dw update --check
souris-dw update --yt-dlp

# Configuracion
souris-dw config show
souris-dw config set download.default_format mp3

# Setup y desinstalacion
souris-dw setup --quiet
souris-dw uninstall
souris-dw uninstall --keep-config

# Salida JSON (para integracion con otros lenguajes)
souris-dw download "URL" --json --format mp4

# Ejecucion determinista (sin auto-actualizaciones)
souris-dw download "URL" --no-auto-update
```

### Banderas Globales

| Bandera | Descripcion |
|---------|-------------|
| `--json` | Salida JSON legible por maquinas |
| `--quiet` | Suprimir barras de progreso |
| `--no-auto-update` | Saltar actualizacion automatica |
| `--no-color` | Deshabilitar salida de color |
| `--timeout <segundos>` | Timeout de descarga (default: 300) |
| `--max-retries <n>` | Reintentos maximos (default: 3) |

### Comandos de Descarga

| Bandera | Descripcion |
|---------|-------------|
| `-f, --format` | Formato de salida (mp3, flac, mp4, etc.) |
| `-q, --quality` | Calidad (128kbps, 320kbps, 360p, 1080p, 4K, lossless) |
| `-o, --output` | Directorio de salida |
| `-p, --parallel` | Descargas paralelas (default: 4) |
| `--audio-only` | Solo audio |
| `--video-only` | Solo video |
| `--embed-metadata` | Incrustar metadatos ID3 |
| `--embed-thumbnail` | Incrustar caratula/album art |
| `--embed-subtitles` | Descargar e incrustar subtitulos |
| `--cookies <archivo>` | Archivo de cookies |
| `--cookies-from-browser <navegador>` | Extraer cookies del navegador |

## Formato de Caratulas

| Formato | Caratula | Notas |
|---------|----------|-------|
| MP3 | Si | ID3v2 attached picture |
| FLAC | Si | |
| AAC | Si | |
| OGG | Si | |
| M4A | Si | |
| WAV | No | El contenedor no soporta caratulas |
| MP4 | Si | |
| MKV | Si | |
| WebM | No | No soporta caratulas |
| AVI | No | No soporta caratulas |
| MOV | Limitado | Solo sin merge (formato unico) |

## Configuracion

Archivo en:
- Linux: `~/.config/souris-dw/config.toml`
- macOS: `~/Library/Application Support/souris-dw/config.toml`
- Windows: `%APPDATA%\souris-dw\config.toml`

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

[spotify]
client_id = ""
client_secret = ""
```

## Compilacion desde Codigo Fuente

```bash
git clone https://github.com/SourisCG/SourisDW.git
cd SourisDW
cargo build --release
./target/release/souris-dw --version
```

Requiere Rust 1.70+. No necesita otras dependencias (yt-dlp, ffmpeg y deno se descargan en runtime).

## Binarios por Plataforma

| Binario | Plataforma | Libc |
|---------|------------|------|
| `souris-dw-linux-x86_64` | Linux x86_64 | musl (todas las distros, incl. Fedora) |
| `souris-dw-linux-x86_64-glibc` | Linux x86_64 | glibc (Ubuntu, Debian, Fedora, etc.) |
| `souris-dw-macos-aarch64` | macOS Apple Silicon | - |
| `souris-dw-windows-x86_64.exe` | Windows x64 | - |

## Documentacion

- [Arquitectura](ARCHITECTURE.md)
- [Guia de uso](USAGE.md)
- [Uso como libreria](LIBRARY.md)
- [Integracion con otros lenguajes](INTEGRATION.md)
- [Notas multiplataforma](CROSS_PLATFORM.md)

## Licencia

MIT
