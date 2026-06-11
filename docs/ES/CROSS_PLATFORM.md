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

## Matriz de Pruebas CI

Las pruebas de CI se ejecutan en:
- Ubuntu (latest) - x86_64
- Fedora 41 - x86_64 (contenedor)
- macOS (latest) - x86_64 + aarch64
- Windows (latest) - x86_64 + aarch64

Pruebas incluyen:
- Formato (`cargo fmt --check`)
- Linting (`cargo clippy -- -D warnings`)
- Tests unitarios (`cargo test`)
- Compilacion release (`cargo build --release`)
