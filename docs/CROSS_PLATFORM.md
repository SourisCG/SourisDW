# Cross-Platform Notes

## Supported Platforms

| Platform | Architecture | Binary |
|----------|-------------|--------|
| Linux | x86_64 | `souris-dw-linux-x86_64` |
| Linux | x86_64 (musl) | `souris-dw-linux-x86_64-musl` |
| Linux | aarch64 | `souris-dw-linux-aarch64` |
| macOS | x86_64 | `souris-dw-macos-x86_64` |
| macOS | aarch64 | `souris-dw-macos-aarch64` |
| Windows | x86_64 | `souris-dw-windows-x86_64.exe` |

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

yt-dlp and ffmpeg binaries are stored in:

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
| macOS | NFD (decomposed) | `café` stored as `café` |
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
| Linux | `/etc/ssl/certs` (via `openssl-probe`) |
| macOS | Keychain (via `security-framework`) |
| Windows | Windows Certificate Store (via `schannel`) |

SourisDW uses `rustls-native-certs` to load system certificates.

## Testing Matrix

CI tests run on all three platforms:
- Ubuntu (latest)
- macOS (latest)
- Windows (latest)

Tests include:
- Paths with spaces
- Paths with Unicode characters
- Long paths (>260 chars on Windows)
- Non-TTY output (piped)
