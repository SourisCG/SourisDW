# Cross-Platform Notes

## Supported Platforms

| Platform | Architecture | Binary |
|----------|-------------|--------|
| Linux | x86_64 (musl) | `souris-dw-linux-x86_64` |
| Linux | x86_64 (glibc) | `souris-dw-linux-x86_64-glibc` |
| Linux | x86_64 (Fedora) | `souris-dw-linux-x86_64-fedora` |
| Linux | aarch64 (musl) | `souris-dw-linux-aarch64` |
| Linux | aarch64 (glibc) | `souris-dw-linux-aarch64-glibc` |
| macOS | x86_64 | `souris-dw-macos-x86_64` |
| macOS | aarch64 | `souris-dw-macos-aarch64` |
| Windows | x86_64 | `souris-dw-windows-x86_64.exe` |
| Windows | aarch64 | `souris-dw-windows-arm64.exe` |

**Binary compatibility:**
- `souris-dw-linux-x86_64` - Statically linked with musl, works on ALL Linux distros. This is the primary Linux binary.
- `souris-dw-linux-x86_64-glibc` - glibc-linked, built on Ubuntu. Works on Ubuntu/Debian and similar glibc-based distros.
- `souris-dw-linux-x86_64-fedora` - Built natively on Fedora 41. Works on Fedora/RHEL/CentOS with newer glibc.

## File System Differences

### Case Sensitivity

| Platform | Case Sensitive | Behavior |
|----------|---------------|----------|
| Linux | Yes | `File.txt` != `file.txt` |
| macOS | No (APFS) | `File.txt` == `file.txt` |
| Windows | No (NTFS) | `File.txt` == `file.txt` |

**Implication:** Never rely on filename case for uniqueness. SourisDW uses `sanitize-filename` to normalize names.

### Path Separators

| Platform | Separator | Example |
|----------|-----------|---------|
| Linux | `/` | `/home/user/Music` |
| macOS | `/` | `/Users/user/Music` |
| Windows | `\` | `C:\Users\user\Music` |

**Implication:** Always use `Path::join()` in Rust, never string concatenation.

### MAX_PATH Length

| Platform | Limit |
|----------|-------|
| Linux | ~4096 |
| macOS | ~1024 |
| Windows | 260 (default) |

**Implication:** SourisDW uses `dunce::canonicalize()` to avoid Windows UNC paths (`\\?\`).

## Configuration Paths

| Platform | Config | Data | Cache |
|----------|--------|------|-------|
| Linux | `~/.config/souris-dw/` | `~/.local/share/souris-dw/` | `~/.cache/souris-dw/` |
| macOS | `~/Library/Application Support/souris-dw/` | same | `~/Library/Caches/souris-dw/` |
| Windows | `%APPDATA%\souris-dw\` | `%LOCALAPPDATA%\souris-dw\` | `%LOCALAPPDATA%\souris-dw\cache\` |

These paths are determined using the `directories` crate which follows platform conventions:
- Linux: XDG Base Directory Specification
- macOS: Apple File System conventions
- Windows: Known Folder API

## Binary Paths

yt-dlp, ffmpeg, ffprobe, and deno binaries are stored in:

| Platform | Path |
|----------|------|
| Linux | `~/.local/share/souris-dw/bin/` |
| macOS | `~/Library/Application Support/souris-dw/bin/` |
| Windows | `%LOCALAPPDATA%\souris-dw\bin\` |

## Executable Extensions

| Platform | Extension | Example |
|----------|-----------|---------|
| Linux | (none) | `souris-dw` |
| macOS | (none) | `souris-dw` |
| Windows | `.exe` | `souris-dw.exe` |

Determined at runtime via `std::env::consts::EXE_SUFFIX`.

## File Permissions

| Platform | Mechanism | Notes |
|----------|-----------|-------|
| Linux | `chmod` | `PermissionsExt::set_mode(0o755)` |
| macOS | `chmod` | Same as Linux |
| Windows | ACLs | No direct chmod equivalent |

SourisDW sets executable permission on Unix after downloading binaries using `#[cfg(unix)]` gates.

## Unicode Handling

| Platform | Filename Encoding | Notes |
|----------|------------------|-------|
| Linux | UTF-8 | Usually |
| macOS | NFD (decomposed) | `cafe` stored NFD |
| Windows | UTF-16 | WTF-8 in Rust's OsString |

**Implication:** SourisDW normalizes Unicode with `unicode-normalization` (NFC) before comparing filenames.

## Terminal Differences

| Feature | Linux/macOS | Windows |
|---------|-------------|---------|
| ANSI colors | Native | Requires `ENABLE_VIRTUAL_TERMINAL_PROCESSING` |
| UTF-8 output | Default | May need `SetConsoleOutputCP(CP_UTF8)` |
| Raw mode | `termios` | `SetConsoleMode` |

SourisDW uses `crossterm` which handles all these differences automatically.

## Signal Handling

| Signal | Linux/macOS | Windows |
|--------|-------------|---------|
| Ctrl+C | `SIGINT` | `CTRL_C_EVENT` |
| Terminal resize | `SIGWINCH` | (none) |
| Graceful shutdown | `SIGTERM` | (none) |

SourisDW uses the `ctrlc` crate for cross-platform Ctrl+C handling.

## TLS Certificates

| Platform | Source |
|----------|--------|
| Linux | `/etc/ssl/certs` (via `rustls-native-certs`) |
| macOS | System Keychain (via `rustls-native-certs`) |
| Windows | Windows Certificate Store (via `rustls-native-certs`) |

SourisDW uses `rustls` (not OpenSSL) for TLS. System certificates are loaded via `rustls-native-certs`.

## Testing Matrix

CI tests run on:
- Ubuntu (latest) - x86_64
- Fedora 41 - x86_64 (container)
- macOS (latest) - x86_64 + aarch64
- Windows (latest) - x86_64 + aarch64

Tests include:
- Formatting (`cargo fmt --check`)
- Linting (`cargo clippy -- -D warnings`)
- Unit tests (`cargo test`)
- Release build (`cargo build --release`)
- Paths with spaces
- Paths with Unicode characters
- Long paths
- Non-TTY output (piped)
