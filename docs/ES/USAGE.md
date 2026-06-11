# Guia de Uso

## Banderas Globales

| Bandera | Descripcion |
|---------|-------------|
| `--json` | Salida JSON legible por maquinas |
| `--quiet` | Suprimir barras de progreso |
| `--no-auto-update` | Saltar actualizacion automatica de dependencias |
| `--no-color` | Deshabilitar salida de color |
| `--timeout <segundos>` | Timeout de descarga (default: 300) |
| `--max-retries <n>` | Reintentos maximos (default: 3) |

## Descarga

```bash
# Descarga basica (defaults: mp4, 1080p)
souris-dw download "https://youtube.com/watch?v=xxx"

# Solo audio
souris-dw download "URL" --audio-only --format mp3

# Video con calidad especifica
souris-dw download "URL" --format mp4 --quality 1080p

# Directorio especifico
souris-dw download "URL" --output ~/Music

# Con metadatos y caratula
souris-dw download "URL" --embed-metadata --embed-thumbnail

# Lista de reproduccion
souris-dw download "PLAYLIST_URL" --parallel 8

# Con subtitulos
souris-dw download "URL" --embed-subtitles

# Con cookies
souris-dw download "URL" --cookies cookies.txt
souris-dw download "URL" --cookies-from-browser firefox

# Progreso en JSON (para integracion)
souris-dw download "URL" --json --format mp4 --quality 1080p
```

**Opciones:**
| Bandera | Descripcion |
|---------|-------------|
| `-f, --format` | Formato de salida (mp3, flac, mp4, mkv, etc.) |
| `-q, --quality` | Calidad (128kbps, 320kbps, 360p, 1080p, 4K) |
| `-o, --output` | Directorio de salida |
| `-p, --parallel` | Descargas paralelas |
| `--audio-only` | Solo audio |
| `--video-only` | Solo video |
| `--embed-metadata` | Incrustar metadatos ID3 |
| `--embed-thumbnail` | Incrustar caratula/album art |
| `--embed-subtitles` | Descargar y incrustar subtitulos |
| `--cookies <archivo>` | Archivo de cookies |
| `--cookies-from-browser <navegador>` | Extraer cookies del navegador |

## Informacion

```bash
# Info como texto legible
souris-dw info "https://youtube.com/watch?v=xxx"

# Info como JSON
souris-dw info "URL" --json
```

## Busqueda

```bash
# Buscar en YouTube
souris-dw search "never gonna give you up"

# Con limite
souris-dw search "query" --limit 20

# Como JSON
souris-dw search "query" --json
```

## Actualizacion

```bash
# Actualizar todas las dependencias
souris-dw update

# Actualizar solo yt-dlp o ffmpeg
souris-dw update --yt-dlp
souris-dw update --ffmpeg

# Verificar sin instalar
souris-dw update --check

# Como JSON
souris-dw update --json
```

## Dependencias

```bash
# Instalar/refrescar dependencias
souris-dw deps install

# Ver estado
souris-dw deps status

# Actualizar
souris-dw deps update

# Estado como JSON
souris-dw deps status --json
```

## Configuracion

```bash
# Mostrar config actual
souris-dw config show

# Obtener valor
souris-dw config get download.default_format

# Establecer valor
souris-dw config set download.default_format mp3
souris-dw config set download.default_quality 320
souris-dw config set download.output_dir ~/Music
souris-dw config set download.parallel 8
souris-dw config set yt_dlp.channel nightly
```

## Desinstalar

```bash
# Eliminar binario (conserva config)
souris-dw uninstall

# Eliminar todo
souris-dw uninstall --keep-config=false
```

## Modo TUI

```bash
souris-dw tui
```

**Atajos de teclado:**
| Tecla | Accion |
|-------|--------|
| `a` | Agregar URL |
| `/` | Buscar |
| `j` / `Down` | Mover abajo |
| `k` / `Up` | Mover arriba |
| `Enter` | Descargar seleccionado |
| `y` | Copiar URL al portapapeles |
| `d` / `Delete` | Eliminar seleccionado |
| `s` | Configuracion |
| `h` / `?` | Ayuda |
| `q` / `Esc` | Atras / Salir |
| `Ctrl+c` | Salir forzado |

## Archivo de Configuracion

Ubicado en:
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

## Formatos Soportados

### Audio

| Formato | Caratula | Notas |
|---------|----------|-------|
| MP3 | Si | ID3v2 attached picture |
| FLAC | Si | |
| AAC | Si | |
| OGG | Si | |
| M4A | Si | |
| WAV | No | No soporta metadatos/caratulas |

### Video

| Formato | Caratula | Notas |
|---------|----------|-------|
| MP4 | Si | |
| MKV | Si | |
| WebM | No | No soporta caratulas |
| AVI | No | No soporta caratulas |
| MOV | Limitado | Solo sin merge |

### Calidad

- Audio: 128kbps, 192kbps, 256kbps, 320kbps, lossless
- Video: 360p, 480p, 720p, 1080p, 1440p, 4K, 8K

## Plataformas Soportadas

- YouTube (videos, listas, canales)
- Spotify (canciones, listas, albumes -> busqueda en YouTube)
