# SourisDW

Descargador de musica y video multiplataforma para YouTube y Spotify.

## Instalacion Rapida

**Linux y macOS:**
```bash
curl -fsSL https://raw.githubusercontent.com/SourisCG/SourisDW/main/install.sh | bash
```

**Windows (PowerShell):**
```powershell
irm https://raw.githubusercontent.com/SourisCG/SourisDW/main/install.ps1 | iex
```

## Uso Basico

```bash
# Descargar video
souris-dw download "https://youtube.com/watch?v=xxx"

# Solo audio
souris-dw download "https://youtube.com/watch?v=xxx" --audio-only --format mp3

# Con calidad especifica
souris-dw download "https://youtube.com/watch?v=xxx" --format mp4 --quality 1080p

# Lista de reproduccion
souris-dw download "https://youtube.com/playlist?list=xxx" --parallel 8

# Buscar
souris-dw search "never gonna give you up"

# Interfaz TUI
souris-dw tui
```

## Caracteristicas

- Descarga musica y video de YouTube y Spotify
- Soporte completo para listas de reproduccion
- Formatos de audio: MP3, FLAC, AAC, OGG, M4A, WAV
- Formatos de video: MP4, MKV, WebM, AVI, MOV
- Calidad seleccionable: 128kbps a lossless (audio), 360p a 8K (video)
- Incrustacion automatica de metadatos
- Descargas paralelas
- Modo CLI y TUI interactivo
- Sin dependencias externas - todo incluido
- Actualizacion automatica de yt-dlp
- Multiplataforma: Linux, macOS, Windows

## Binarios Universales para Linux

SourisDW ofrece binarios estaticos (musl) que funcionan en **cualquier** distribucion de Linux, sin importar la version de glibc. Usa `install.sh` para deteccion automatica.

## Documentacion

- [Guia de uso](USAGE.md)
- [Arquitectura](ARCHITECTURE.md)
- [Uso como libreria](LIBRARY.md)
- [Integracion con otros lenguajes](INTEGRATION.md)
- [Notas multiplataforma](CROSS_PLATFORM.md)

## Licencia

MIT
