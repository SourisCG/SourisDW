# Guia de Uso

Referencia completa de CLI y TUI. Cada comando tiene su **proposito**, sus **flags** y **ejemplos**.

---

## Instalacion

### Instalacion en Una Linea

**Linux y macOS:**
```bash
curl -fsSL https://raw.githubusercontent.com/SourisCG/SourisDW/main/install.sh | bash
```

**Windows (PowerShell):**
```powershell
irm https://raw.githubusercontent.com/SourisCG/SourisDW/main/install.ps1 | iex
```

### Descarga Manual

**Linux x86_64 (musl - funciona en cualquier distro):**
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
Descarga `souris-dw-windows-x86_64.exe` desde [releases](https://github.com/SourisCG/SourisDW/releases) y agregalo a tu PATH.

---

## Flags Globales

Estos flags funcionan en **todos** los comandos.

| Flag | Descripcion |
|------|-------------|
| `--json` | Salida JSON para maquinas |
| `--quiet` | Suprime barras de progreso y salida no esencial |
| `--no-auto-update` | Omite actualizaciones automaticas de dependencias (ejecuciones deterministas) |
| `--no-color` | Desactiva salida con colores |
| `--timeout <segundos>` | Timeout de descarga (default: 300) |
| `--max-retries <n>` | Maximos reintentos al fallar (default: 3) |

---

## download

**Proposito:** descargar video, audio, playlist o album de YouTube o Spotify.

**Opciones:**
| Flag | Descripcion |
|------|-------------|
| `-f, --format` | Formato de salida (mp3, flac, mp4, mkv, etc.) |
| `-q, --quality` | Calidad (128kbps, 320kbps, 360p, 1080p, 4K, lossless) |
| `-o, --output` | Directorio de salida |
| `-p, --parallel` | Descargas paralelas para playlists (default: 4) |
| `--audio-only` | Solo audio |
| `--video-only` | Solo video |
| `--embed-metadata` | Incrustar etiquetas ID3/metadatos |
| `--embed-thumbnail` | Incrustar caratula/album art |
| `--embed-subtitles` | Descargar e incrustar subtitulos |
| `--cookies <archivo>` | Archivo de cookies para descargas autenticadas |
| `--cookies-from-browser <navegador>` | Extraer cookies de un navegador (firefox, chrome, etc.) |

**Ejemplos:**
```bash
# Descarga basica (defaults: mp4, 1080p)
souris-dw download "https://youtube.com/watch?v=xxx"

# Solo audio como MP3
souris-dw download "URL" --audio-only --format mp3

# Audio lossless
souris-dw download "URL" --audio-only --format flac --quality lossless

# Video en 4K
souris-dw download "URL" --format mp4 --quality 4K

# A un directorio especifico con tags y caratula
souris-dw download "URL" --output ~/Music --embed-metadata --embed-thumbnail

# Playlist con 8 descargas paralelas
souris-dw download "PLAYLIST_URL" --parallel 8

# Album de Spotify (requiere credenciales de la API en la config)
souris-dw download "https://open.spotify.com/album/xxx" --audio-only --format mp3

# Descargas autenticadas/privadas
souris-dw download "URL" --cookies cookies.txt
souris-dw download "URL" --cookies-from-browser firefox

# Progreso JSON para integracion con subprocess
souris-dw download "URL" --json --format mp4 --quality 1080p
```

---

## info

**Proposito:** obtener metadatos de una URL **sin descargarla** (titulo, duracion, autor, formatos, subtitulos).

```bash
# Legible para humanos
souris-dw info "https://youtube.com/watch?v=xxx"

# JSON (objeto MediaInfo completo, incluye formatos disponibles)
souris-dw info "URL" --json
```

---

## search

**Proposito:** buscar medios en YouTube.

**Opciones:**
| Flag | Descripcion |
|------|-------------|
| `-p, --platform` | Plataforma a buscar (solo se soporta `youtube`) |
| `-l, --limit` | Cantidad de resultados (default: 10) |

```bash
# Buscar
souris-dw search "never gonna give you up"

# Mas resultados
souris-dw search "query" --limit 20

# Como JSON (array de resultados con id/titulo/url/duracion/autor)
souris-dw search "query" --json
```

---

## update

**Proposito:** actualizar las dependencias de runtime de SourisDW (yt-dlp, ffmpeg, deno).

**Opciones:**
| Flag | Descripcion |
|------|-------------|
| `--yt-dlp` | Actualizar solo yt-dlp |
| `--ffmpeg` | Actualizar solo ffmpeg (y ffprobe) |
| `--self` | Actualizar el binario de SourisDW (aun no soportado) |
| `--check` | Verificar actualizaciones sin instalar |

```bash
# Actualizar todo
souris-dw update

# Actualizar solo yt-dlp
souris-dw update --yt-dlp

# Ver que esta desactualizado (reporta update_available + latest por dependencia)
souris-dw update --check

# Salida JSON (array de objetos DepStatus)
souris-dw update --json
```

---

## deps

**Proposito:** administrar dependencias de runtime (yt-dlp, ffmpeg/ffprobe, deno). Se descargan solas en el primer uso; este comando permite controlarlas explicitamente.

**Subcomandos:**
| Comando | Proposito |
|---------|-----------|
| `deps install` | Forzar la re-descarga de todas las dependencias |
| `deps status` | Mostrar el estado de instalacion de cada dependencia |
| `deps update` | Actualizar todas las dependencias |

```bash
# Forzar re-descarga (p. ej. tras un binario corrupto)
souris-dw deps install

# Estado como JSON
souris-dw deps status --json

# Actualizar
souris-dw deps update
```

---

## config

**Proposito:** ver y editar el archivo de configuracion (`config.toml`).

**Subcomandos:**
| Comando | Proposito |
|---------|-----------|
| `config show` | Imprimir la config completa como TOML |
| `config get <clave>` | Imprimir un solo valor |
| `config set <clave> <valor>` | Establecer un valor (se guarda al instante) |

**Claves soportadas:**
| Clave | Tipo | Default |
|-------|------|---------|
| `yt_dlp.auto_update` | bool | false |
| `yt_dlp.channel` | stable/nightly/master | stable |
| `ffmpeg.auto_update` | bool | false |
| `download.default_format` | string | mp4 |
| `download.default_quality` | string | 1080p |
| `download.output_dir` | ruta | carpeta de Descargas del sistema |
| `download.parallel` | numero | 4 |
| `download.embed_metadata` | bool | true |
| `download.embed_thumbnail` | bool | true |
| `download.embed_subtitles` | bool | false |
| `download.timeout` | numero (segundos) | 300 |
| `download.max_retries` | numero | 3 |

```bash
# Mostrar config actual
souris-dw config show

# Obtener un valor
souris-dw config get download.default_format

# Establecer valores
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

**Proposito:** inicializacion unica: crea el archivo de config, los directorios de dependencias y apunta las descargas a tu carpeta de Descargas. El instalador lo ejecuta automaticamente.

```bash
# Setup no interactivo
souris-dw setup --quiet
```

---

## uninstall

**Proposito:** eliminar el binario de SourisDW. Por defecto tambien elimina tu config y datos; usa `--keep-config` para conservarlos.

**Opciones:**
| Flag | Descripcion |
|------|-------------|
| `--keep-config` | Conservar config y datos (por defecto se eliminan) |

```bash
# Eliminar todo (binario + config + datos)
souris-dw uninstall

# Eliminar binario pero conservar config y datos
souris-dw uninstall --keep-config
```

---

## tui

**Proposito:** lanzar la interfaz de terminal interactiva.

```bash
souris-dw tui
```

**Atajos de teclado:**
| Tecla | Accion |
|-------|--------|
| `a` | Agregar URL |
| `/` / `Ctrl+F` | Buscar |
| `j` / `Down` | Mover abajo |
| `k` / `Up` | Mover arriba |
| `g` / `Home` | Ir al primero |
| `G` / `End` | Ir al ultimo |
| `Enter` | Descargar seleccionado |
| `y` | Copiar URL al portapapeles |
| `d` / `Delete` | Eliminar seleccionado |
| `s` | Configuracion (los cambios persisten en config) |
| `h` / `?` | Ayuda |
| `q` / `Esc` | Atras / Salir |
| `Ctrl+C` | Salir forzado |

---

## Salida JSON

Todos los comandos soportan `--json` para salida legible por maquinas:

```bash
# Info como JSON
souris-dw info "URL" --json

# Descarga con progreso JSON
souris-dw download "URL" --json --format mp4

# Busqueda como JSON
souris-dw search "query" --json

# Estado de dependencias
souris-dw deps status --json
```

### Eventos de Progreso

Al usar `download --json`, el progreso se transmite como **un objeto JSON por linea**:

```json
{"type":"init","url":"...","platform":"youtube","title":"...","media_type":"video","total_items":1}
{"type":"progress","item":1,"total":1,"percent":45.2,"speed":"2.3MB/s","eta":"00:12"}
{"type":"postprocess","item":1,"total":1,"stage":"ExtractAudio","format":"mp3"}
{"type":"metadata","item":1,"total":1,"stage":"Metadata"}
{"type":"complete","item":1,"total":1,"path":"/ruta/al/archivo.mp4","size":125000000}
{"type":"error","item":1,"total":1,"code":"DOWNLOAD_FAILED","message":"..."}
{"type":"summary","total":10,"success":9,"failed":1,"elapsed":"02:34"}
```

### Codigos de Salida

| Codigo | Significado |
|--------|-------------|
| 0 | Exito o cancelado por el usuario |
| 1 | Error general |
| 2 | Error de dependencia (binario faltante, fallo de descarga) |
| 3 | Error de red o timeout |

---

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

---

## Formatos Soportados

### Audio

| Formato | Caratula | Notas |
|---------|----------|-------|
| MP3 | Si | ID3v2 attached picture (caratula del album) |
| FLAC | Si | |
| AAC | Si | |
| OGG | Si | |
| M4A | Si | |
| WAV | No | El contenedor no soporta metadatos/caratulas |

### Video

| Formato | Caratula | Notas |
|---------|----------|-------|
| MP4 | Si | Attached picture |
| MKV | Si | |
| WebM | No | El contenedor no soporta caratulas |
| AVI | No | El contenedor no soporta caratulas |
| MOV | Limitado | Solo con formato unico (sin merge). Al hacer merge se pierde. |

### Calidad

- Audio: 128kbps, 192kbps, 256kbps, 320kbps, lossless
- Video: 360p, 480p, 720p, 1080p, 1440p, 4K, 8K

---

## Plataformas Soportadas

- YouTube (videos, playlists, canales)
- Spotify (canciones, playlists, albumes -> busqueda en YouTube; requiere credenciales de la API de Spotify)
