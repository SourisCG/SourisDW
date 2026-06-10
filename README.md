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
  <img src="https://img.shields.io/badge/version-0.1.0-blue?style=flat-square" />
  <img src="https://img.shields.io/badge/rust-1.70%2B-orange?style=flat-square&logo=rust" />
  <img src="https://img.shields.io/badge/license-MIT-green?style=flat-square" />
  <img src="https://img.shields.io/badge/platform-linux%20%7C%20macos%20%7C%20windows-lightgrey?style=flat-square" />
</p>

---

## Features

- Download music and video from **YouTube** and **Spotify**
- Full playlist support for both platforms
- Audio formats: MP3, FLAC, AAC, OGG, M4A, WAV
- Video formats: MP4, MKV, WebM, AVI, MOV
- Quality selection: 128kbps to lossless (audio), 360p to 8K (video)
- Automatic metadata embedding (ID3 tags, album art, lyrics)
- Subtitles support
- Parallel downloads
- CLI and interactive TUI modes
- Use as a **library** in your own Rust projects
- Use as a **motor** from any programming language via subprocess + JSON
- Zero external dependencies to install - everything is bundled
- Auto-updates yt-dlp and ffmpeg silently
- Cross-platform: Linux, macOS, Windows

---

## Quick Start

### Installation

```bash
# Linux
curl -sL https://github.com/souris/souris-dw/releases/latest/download/souris-dw-linux-x86_64 -o souris-dw
chmod +x souris-dw

# macOS
curl -sL https://github.com/souris/souris-dw/releases/latest/download/souris-dw-macos-universal -o souris-dw
chmod +x souris-dw

# Windows
# Download souris-dw-windows-x86_64.exe from releases
```

### Basic Usage

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
```

---

## Usage

### CLI Mode

```bash
# Download video with specific format and quality
souris-dw download "URL" --format mp4 --quality 1080p

# Download audio only
souris-dw download "URL" --audio-only --format flac --quality 320

# Download to specific directory
souris-dw download "URL" --output ~/Music

# Download with metadata
souris-dw download "URL" --embed-metadata --embed-thumbnail

# Download playlist with parallel downloads
souris-dw download "PLAYLIST_URL" --parallel 8

# Get info as JSON (for scripting)
souris-dw info "URL" --json

# Search and get results as JSON
souris-dw search "query" --json

# Configuration
souris-dw config show
souris-dw config set download.default_format mp3
souris-dw config set download.default_quality 320

# Dependencies
souris-dw deps status
souris-dw deps update
```

### TUI Mode

```bash
souris-dw tui
```

Interactive terminal interface with:
- Real-time download progress
- Queue management
- Keyboard shortcuts
- Settings panel

### As a Library (Rust)

```rust
use souris_dw::SourisDW;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Builder: configure defaults
    let dw = SourisDW::builder()
        .auto_update(true)
        .embed_metadata(true)
        .format("mp4")
        .quality("1080p")
        .output("./downloads")
        .parallel(4)
        .build()
        .await?;

    // Download with defaults
    dw.download("https://youtube.com/watch?v=xxx").await?;

    // Download with overrides
    dw.download_audio("https://youtube.com/watch?v=xxx")
        .format("flac")
        .quality("lossless")
        .await?;

    // Download playlist
    dw.download_playlist("https://youtube.com/playlist?list=xxx")
        .parallel(8)
        .await?;

    // Get info
    let info = dw.info("https://youtube.com/watch?v=xxx").await?;
    println!("Title: {}", info.title);

    // Search
    let results = dw.search("never gonna give you up").await?;
    for item in results {
        println!("{}: {}", item.title, item.url);
    }

    // Update dependencies
    dw.update().await?;

    Ok(())
}
```

### As a Motor (Any Language)

SourisDW can be used from any programming language via subprocess with JSON output.

```bash
# Download with JSON progress output
souris-dw download "URL" --json --format mp4 --quality 1080p

# Get info as JSON
souris-dw info "URL" --json
```

#### Python

```python
import subprocess, json

proc = subprocess.Popen(
    ["souris-dw", "download", url, "--json", "--format", "mp4"],
    stdout=subprocess.PIPE, text=True
)
for line in proc.stdout:
    event = json.loads(line)
    if event["type"] == "progress":
        print(f"{event['percent']}%")
```

#### Node.js

```javascript
const { spawn } = require("child_process");
const proc = spawn("souris-dw", ["download", url, "--json", "--format", "mp4"]);
proc.stdout.on("data", (data) => {
  const event = JSON.parse(data.toString());
  if (event.type === "progress") console.log(`${event.percent}%`);
});
```

#### Java

```java
ProcessBuilder pb = new ProcessBuilder(
    "souris-dw", "download", url, "--json", "--format", "mp4"
);
Process proc = pb.start();
BufferedReader reader = new BufferedReader(new InputStreamReader(proc.getInputStream()));
String line;
while ((line = reader.readLine()) != null) {
    JsonObject event = JsonParser.parseString(line).getAsJsonObject();
    if ("progress".equals(event.get("type").getAsString())) {
        System.out.println(event.get("percent").getAsDouble() + "%");
    }
}
```

#### Go

```go
cmd := exec.Command("souris-dw", "download", url, "--json", "--format", "mp4")
stdout, _ := cmd.StdoutPipe()
cmd.Start()
scanner := bufio.NewScanner(stdout)
for scanner.Scan() {
    var event map[string]interface{}
    json.Unmarshal([]byte(scanner.Text()), &event)
    if event["type"] == "progress" {
        fmt.Printf("%.1f%%\n", event["percent"])
    }
}
```

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
      | - Spotify                        | - ffmpeg (bundled)
      | - Playlist                       |
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

---

## Configuration

Configuration is stored at:
- Linux: `~/.config/souris-dw/config.toml`
- macOS: `~/Library/Application Support/souris-dw/config.toml`
- Windows: `%APPDATA%\souris-dw\config.toml`

```toml
[yt_dlp]
auto_update = true
channel = "nightly"

[ffmpeg]
auto_update = true

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

---

## Building from Source

```bash
# Prerequisites
# - Rust 1.70+
# - ffmpeg (for runtime)

# Clone
git clone https://github.com/souris/souris-dw.git
cd souris-dw

# Build
cargo build --release

# Run
./target/release/souris-dw --help
```

---

## Cross-Platform Notes

| Feature | Linux | macOS | Windows |
|---------|-------|-------|---------|
| Config path | `~/.config/souris-dw/` | `~/Library/Application Support/souris-dw/` | `%APPDATA%\souris-dw\` |
| Binary path | `~/.local/share/souris-dw/bin/` | `~/Library/Application Support/souris-dw/bin/` | `%LOCALAPPDATA%\souris-dw\bin\` |
| Case sensitivity | Yes | No | No |
| Path separator | `/` | `/` | `\` |

---

## Documentation

- [Architecture](docs/ARCHITECTURE.md) - System architecture and module structure
- [Usage Guide](docs/USAGE.md) - Complete CLI and TUI usage guide
- [Library Guide](docs/LIBRARY.md) - Using SourisDW as a Rust library
- [Integration Guide](docs/INTEGRATION.md) - Using SourisDW from other languages
- [Cross-Platform](docs/CROSS_PLATFORM.md) - Platform-specific notes

---

## License

MIT License - see [LICENSE](LICENSE) for details.
