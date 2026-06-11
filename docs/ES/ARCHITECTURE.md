# Arquitectura

## Vision General

SourisDW es un descargador de musica y video multiplataforma escrito en Rust. Usa yt-dlp como motor de descarga y soporta YouTube y Spotify. Todas las dependencias externas (yt-dlp, ffmpeg, ffprobe, deno) se descargan en tiempo de ejecucion, no estan embebidas en compilacion.

## Estructura de Modulos

```
souris-dw/
├── build.rs                 # No-op (dependencias gestionadas en runtime)
├── src/
│   ├── lib.rs              # Exportaciones de la API publica
│   ├── bin/souris-dw/      # Binario CLI
│   │   └── main.rs         # Punto de entrada (CLI + TUI)
│   ├── core/               # Logica de negocio principal
│   │   ├── downloader.rs   # Struct SourisDW + builder
│   │   ├── request.rs      # DownloadRequestBuilder
│   │   ├── types.rs        # MediaType, Format, Quality, etc.
│   │   ├── progress.rs     # Eventos de progreso (JSON streaming)
│   │   └── queue.rs        # Cola de descargas paralelas
│   ├── deps/               # Gestion de dependencias en runtime
│   │   ├── mod.rs          # DepManager
│   │   ├── platform.rs     # Deteccion SO/arquitectura
│   │   ├── path.rs         # Rutas especificas por plataforma
│   │   ├── download.rs     # Descarga HTTP con barras de progreso
│   │   ├── resolve.rs      # URLs de descarga por plataforma
│   │   ├── versions.rs     # Versionado y cache
│   │   ├── yt_dlp.rs       # Descarga y actualizacion de yt-dlp
│   │   ├── ffmpeg.rs       # Descarga y actualizacion de ffmpeg/ffprobe
│   │   └── deno.rs         # Descarga y actualizacion de deno
│   ├── extractors/         # Extractores por plataforma
│   │   ├── youtube.rs      # YouTube via yt-dlp
│   │   ├── spotify.rs      # Spotify Web API
│   │   └── resolver.rs     # Deteccion de URLs y routing
│   ├── postprocess/        # Post-procesamiento (vacio/reservado)
│   │   └── mod.rs          # Vacio
│   ├── config.rs           # Configuracion (TOML)
│   ├── error.rs            # Tipos de error (SourisError)
│   ├── tui/                # Interfaz de terminal (ratatui)
│   └── utils/              # Utilidades
```

## Flujo de Datos

```
Entrada (URL)
      |
      v
+--------------+
|   Resolver   |  Detecta plataforma y tipo de recurso
+------+-------+
       |
       v
+--------------+     +--------------+
|  Extractor   |---->|  yt-dlp      |  YouTube: extraccion directa
|  (youtube)   |     |  (subprocess)|
+------+-------+     +--------------+
       |
       |  Spotify: metadata -> busqueda YouTube -> descarga
       v
+--------------+
|  Downloader  |  Construye comando yt-dlp con seleccion de formato,
|  (youtube)   |  filtros de calidad, preferencias de codec,
|              |  soporte de cookies y reintentos 403.
+------+-------+
       |
       v
+--------------+
|  Archivo     |  Ruta final determinada por yt-dlp, parseada de stdout
+--------------+
```

## Estrategia de Seleccion de Formato

| Formato | Cadena de Formato | Notas |
|---------|-------------------|-------|
| MP4 | `bestvideo[height<=h]+bestaudio/best[height<=h]` | Default |
| MKV | `bestvideo[height<=h]+bestaudio/best[height<=h]` | Igual que MP4 |
| WebM | `bestvideo[height<=h]+bestaudio/best[height<=h]` | Igual que MP4 |
| AVI | `bestvideo[ext=mp4][height<=h]+bestaudio[ext=m4a]/best[height<=h]` | Fuerza codecs mp4/m4a |
| MOV | `bestvideo[vcodec^=avc1][ext=mp4][height<=h]+bestaudio[ext=m4a]/best[height<=h]` | Fuerza H.264 + `--merge-output-format mov` |

## Manejo de Dependencias

Las dependencias se descargan en tiempo de ejecucion desde GitHub:

- **yt-dlp**: GitHub releases, canales stable/nightly/master
- **ffmpeg/ffprobe**: eugeneware/ffmpeg-static, archivos gzip
- **deno**: denoland/deno, archivos zip (usado como runtime JS para yt-dlp)

## Codigos de Error

| Error | Codigo | Descripcion |
|-------|--------|-------------|
| `DependencyNotFound` | 2 | Binario requerido no encontrado |
| `DownloadFailed` | 1 | Error de descarga |
| `HttpError` | 3 | Error de red |
| `Timeout` | 3 | Tiempo de espera agotado |
| `Cancelled` | 0 | Operacion cancelada por el usuario |

## Rutas de Configuracion

| Plataforma | Config | Datos | Cache |
|------------|--------|-------|-------|
| Linux | `~/.config/souris-dw/` | `~/.local/share/souris-dw/` | `~/.cache/souris-dw/` |
| macOS | `~/Library/Application Support/souris-dw/` | misma | `~/Library/Caches/souris-dw/` |
| Windows | `%APPDATA%\souris-dw\` | `%LOCALAPPDATA%\souris-dw\` | `%LOCALAPPDATA%\souris-dw\cache\` |
