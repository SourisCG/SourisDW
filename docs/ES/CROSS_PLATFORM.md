# Notas Multiplataforma

## Plataformas Soportadas

| Plataforma | Arquitectura | Binario |
|------------|--------------|---------|
| Linux | x86_64 (musl) | `souris-dw-linux-x86_64` |
| Linux | x86_64 (glibc) | `souris-dw-linux-x86_64-glibc` |
| Linux | x86_64 (Fedora) | `souris-dw-linux-x86_64-fedora` |
| Linux | aarch64 (musl) | `souris-dw-linux-aarch64` |
| Linux | aarch64 (glibc) | `souris-dw-linux-aarch64-glibc` |
| macOS | x86_64 | `souris-dw-macos-x86_64` |
| macOS | aarch64 | `souris-dw-macos-aarch64` |
| Windows | x86_64 | `souris-dw-windows-x86_64.exe` |
| Windows | aarch64 | `souris-dw-windows-arm64.exe` |

Nota: `souris-dw-windows-arm64.exe` se compila para Windows ARM64. La instalacion automatica de ffmpeg/ffprobe queda desactivada en esa plataforma hasta que el proveedor upstream publique assets Windows ARM64.

## Diferencias del Sistema de Archivos

| Plataforma | Sensible a mayusculas | Separador |
|------------|----------------------|-----------|
| Linux | Si | `/` |
| macOS | No | `/` |
| Windows | No | `\` |

## Rutas de Configuracion

| Plataforma | Config | Datos | Cache |
|------------|--------|-------|-------|
| Linux | `~/.config/souris-dw/` | `~/.local/share/souris-dw/` | `~/.cache/souris-dw/` |
| macOS | `~/Library/Application Support/souris-dw/` | misma | `~/Library/Caches/souris-dw/` |
| Windows | `%APPDATA%\souris-dw\` | `%LOCALAPPDATA%\souris-dw\` | `%LOCALAPPDATA%\souris-dw\cache\` |

## Extensiones de Ejecutables

| Plataforma | Extension | Ejemplo |
|------------|-----------|---------|
| Linux | (ninguna) | `souris-dw` |
| macOS | (ninguna) | `souris-dw` |
| Windows | `.exe` | `souris-dw.exe` |

## Manejo de Unicode

| Plataforma | Codificacion | Notas |
|------------|-------------|-------|
| Linux | UTF-8 | Generalmente |
| macOS | NFD | `cafe` almacenado como NFD |
| Windows | UTF-16 | WTF-8 en OsString de Rust |

## Certificados TLS

| Plataforma | Fuente |
|------------|--------|
| Linux | `/etc/ssl/certs` (via `rustls-native-certs`) |
| macOS | System Keychain (via `rustls-native-certs`) |
| Windows | Windows Certificate Store (via `rustls-native-certs`) |

SourisDW usa `rustls` (no OpenSSL) para TLS.

## Dependencias en Runtime

SourisDW descarga `yt-dlp`, `ffmpeg`, `ffprobe` y `deno` en runtime si faltan. Las descargas validan el estado HTTP, reintentan y escriben primero a un archivo temporal antes de reemplazar el binario final.

La salida por defecto es la carpeta Downloads/Descargas del usuario en cada sistema. El uso como libreria mantiene ese default seguro si no se configura `.output(...)`, y siempre respeta rutas explicitas.

| Dependencia | Linux x86_64 | Linux aarch64 | macOS x86_64 | macOS aarch64 | Windows x86_64 | Windows aarch64 |
|-------------|--------------|---------------|--------------|---------------|----------------|-----------------|
| yt-dlp | auto | auto | auto | auto | auto | auto |
| deno | auto | auto | auto | auto | auto | auto |
| ffmpeg/ffprobe | auto | auto | auto | auto | auto | sistema/fallback |

## Matriz de Pruebas CI

Las pruebas de CI se ejecutan en:
- Ubuntu (latest) - x86_64
- Fedora 41 - x86_64 (contenedor)
- macOS (latest) - pruebas nativas y checks de target x86_64/aarch64
- Windows (latest) - pruebas nativas y checks de target x86_64/aarch64

Pruebas incluyen:
- Formato (`cargo fmt --check`)
- Linting (`cargo clippy -- -D warnings`)
- Tests unitarios (`cargo test`)
- Compilacion release (`cargo build --release`)
- Checks de target para todos los binarios publicados
- Rutas con espacios
- Mapeo de assets de dependencias sin acceso a red

Los releases incluyen binarios crudos, paquetes Linux `.deb`/`.rpm`, tarballs para macOS y zips para Windows.
