# Library Guide (Rust)

SourisDW can be used as a Rust library in your own projects.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
souris-dw = "0.3.6"
```

## Quick Start

```rust
use souris_dw::SourisDW;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dw = SourisDW::builder()
        .format("mp4")
        .quality("1080p")
        .output("./downloads")
        .build()
        .await;

    dw.download("https://youtube.com/watch?v=xxx").run().await?;

    Ok(())
}
```

## Builder Pattern

The builder configures default values for all downloads:

```rust
use souris_dw::SourisDW;

let dw = SourisDW::builder()
    .auto_update(true)           // Auto-update yt-dlp/ffmpeg/deno
    .yt_dlp_channel("stable")    // yt-dlp channel: stable, nightly, master
    .format("mp4")               // Default format
    .quality("1080p")            // Default quality
    .output("./downloads")       // Default output directory
    .parallel(4)                 // Parallel downloads
    .embed_metadata(true)        // Embed ID3/metadata tags
    .embed_thumbnail(true)       // Embed album art
    .embed_subtitles(false)      // Embed subtitles
    .timeout(300)                // Timeout in seconds
    .max_retries(3)              // Max retries on failure
    .spotify_credentials(        // Spotify API (optional)
        "client_id".to_string(),
        "client_secret".to_string(),
    )
    .cookies_file("cookies.txt") // Cookies file (optional)
    .cookies_from_browser(       // Browser cookies (optional)
        "firefox".to_string(),
    )
    .build()
    .await?;
```

### Builder Methods

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `auto_update(bool)` | bool | true | Auto-update dependencies |
| `yt_dlp_channel(string)` | string | "stable" | yt-dlp update channel |
| `format(impl Into<Format>)` | Format | Video(Mp4) | Default output format |
| `format_str(&str)` | string | "mp4" | Format from string |
| `quality(impl Into<Quality>)` | Quality | Video(P1080) | Default quality |
| `quality_str(&str)` | string | "1080p" | Quality from string |
| `output(impl Into<PathBuf>)` | Path | "./downloads" | Output directory |
| `parallel(usize)` | number | 4 | Parallel downloads |
| `embed_metadata(bool)` | bool | true | Embed metadata tags |
| `embed_thumbnail(bool)` | bool | true | Embed album art |
| `embed_subtitles(bool)` | bool | false | Embed subtitles |
| `timeout(u64)` | seconds | 300 | Download timeout |
| `max_retries(u32)` | number | 3 | Max retries |
| `on_progress(ProgressSender)` | channel | None | Progress callback |
| `spotify_credentials(id, secret)` | string | None | Spotify API keys |
| `cookies_file(string)` | string | None | Cookies file path |
| `cookies_from_browser(string)` | string | None | Browser name |
| `quiet_deps(bool)` | bool | false | Suppress dependency download progress bars |

## Fluent Download API

Each download method returns a chainable `DownloadRequestBuilder`:

```rust
// Download with defaults
dw.download("URL").run().await?;

// Download with format and quality overrides
dw.download("URL")
    .format("mp3")
    .quality("lossless")
    .run()
    .await?;

// Download audio (hints media type)
dw.download_audio("URL")
    .format("flac")
    .quality("lossless")
    .run()
    .await?;

// Download video
dw.download_video("URL")
    .format("mkv")
    .quality("4K")
    .output("./videos")
    .run()
    .await?;

// Download playlist
dw.download_playlist("PLAYLIST_URL")
    .parallel(8)
    .format("mp3")
    .run()
    .await?;

// Using format_str and quality_str for dynamic input
dw.download("URL")
    .format_str("mp4")?     // Parse from user input
    .quality_str("4K")?     // Parse from user input
    .run()
    .await?;

// With cookies
dw.download("URL")
    .cookies_file("cookies.txt")
    .cookies_from_browser("firefox")
    .run()
    .await?;
```

### DownloadRequestBuilder Methods

| Method | Description |
|--------|-------------|
| `format(impl Into<Format>)` | Override output format |
| `format_str(&str)` | Parse format from string |
| `quality(impl Into<Quality>)` | Override quality |
| `quality_str(&str)` | Parse quality from string |
| `output(impl Into<PathBuf>)` | Override output directory |
| `parallel(usize)` | Override parallel count |
| `embed_metadata(bool)` | Override metadata embedding |
| `embed_thumbnail(bool)` | Override thumbnail embedding |
| `embed_subtitles(bool)` | Override subtitle embedding |
| `timeout(u64)` | Override timeout |
| `max_retries(u32)` | Override max retries |
| `auto_update(bool)` | Override auto-update |
| `on_progress(ProgressSender)` | Attach a progress event channel |
| `media_type(MediaTypeHint)` | Hint media type (Audio/Video/Playlist/Auto) |
| `cookies_file(string)` | Override cookies file |
| `cookies_from_browser(string)` | Override browser cookies |
| `run()` | Execute download directly (without SourisDW) |

## Direct Execution

`DownloadRequestBuilder` can be executed directly without a pre-built `SourisDW`:

```rust
use souris_dw::core::request::DownloadRequestBuilder;

let result = DownloadRequestBuilder::new("https://youtube.com/watch?v=xxx")
    .format("mp4")
    .quality("1080p")
    .output("./downloads")
    .run()
    .await?;

println!("Downloaded: {:?}", result.path);
```

## Info & Search

```rust
// Get media info
let info = dw.info("https://youtube.com/watch?v=xxx").await?;
println!("Title: {}", info.title);
println!("Duration: {:?}", info.duration);
println!("Platform: {}", info.platform);

// Search (limit = number of results)
let results = dw.search("never gonna give you up", 10).await?;
for item in results {
    println!("{}: {}", item.title, item.url);
}
```

## Progress Events

```rust
use souris_dw::ProgressEvent;
use souris_dw::core::progress::create_progress_channel;

let (tx, mut rx) = create_progress_channel();

let dw = SourisDW::builder()
    .on_progress(tx)
    .build()
    .await;

// Spawn progress handler
tokio::spawn(async move {
    while let Some(event) = rx.recv().await {
        match event {
            ProgressEvent::Progress { percent, speed, .. } => {
                println!("Progress: {:.1}% ({})", percent, speed);
            }
            ProgressEvent::Complete { path, size, .. } => {
                println!("Downloaded: {} ({} bytes)", path, size);
            }
            ProgressEvent::Error { message, .. } => {
                eprintln!("Error: {}", message);
            }
            _ => {}
        }
    }
});

dw.download("URL").run().await?;
```

## Dependency Management

```rust
// Check dependency status (includes latest + update_available after check_updates)
let status = dw.update_check().await?;
for dep in &status {
    println!("{}: {} ({})", dep.name, dep.version.as_deref().unwrap_or("?"), dep.path);
}

// Update all dependencies
let updated = dw.update().await?;

// Update only specific dependencies
let updated = dw.update_specific(true, false, false).await?;  // yt-dlp only

// Using DepManager directly
use souris_dw::DepManager;

// Setup with auto-update
let deps = DepManager::setup(true, "stable").await;
println!("yt-dlp: {}", deps.yt_dlp().binary_path().display());

// Check status
let status = deps.status();

// Check for updates without installing (fills latest/update_available)
let status = deps.check_updates().await;
```

`DepStatus` fields: `name`, `installed`, `version`, `path`, plus `latest` and `update_available` (populated by `check_updates`).

## Configuration

```rust
use souris_dw::AppConfig;

// Load config (creates default if not exists)
let config = AppConfig::load()?;

// Get a value
if let Some(fmt) = config.get("download.default_format") {
    println!("Default format: {}", fmt);
}

// Set a value
let mut config = AppConfig::load()?;
config.set("download.default_format", "mp3")?;
config.flush()?;    // Save to disk

// Supported keys:
// - yt_dlp.auto_update
// - yt_dlp.channel
// - ffmpeg.auto_update
// - download.default_format
// - download.default_quality
// - download.output_dir
// - download.parallel
// - download.embed_metadata
// - download.embed_thumbnail
// - download.embed_subtitles
// - download.timeout
// - download.max_retries
```

## Error Handling

```rust
use souris_dw::SourisError;

match dw.download("URL").run().await {
    Ok(_) => println!("Success"),
    Err(SourisError::DownloadFailed { reason }) => {
        eprintln!("Download failed: {}", reason);
    }
    Err(SourisError::DependencyNotFound { name }) => {
        eprintln!("Missing dependency: {}", name);
    }
    Err(SourisError::DependencyDownloadFailed { name, reason }) => {
        eprintln!("Failed to download {}: {}", name, reason);
    }
    Err(SourisError::Timeout { seconds }) => {
        eprintln!("Timeout after {}s", seconds);
    }
    Err(SourisError::Cancelled) => {
        println!("Cancelled by user");
    }
    Err(e) => {
        eprintln!("Error: {}", e);
    }
}
```

## Types

```rust
use souris_dw::core::types::*;

// Media types
MediaType::Audio
MediaType::Video
MediaType::Playlist

// Audio formats
Format::Audio(AudioFormat::Mp3)
Format::Audio(AudioFormat::Flac)
Format::Audio(AudioFormat::Aac)
Format::Audio(AudioFormat::Ogg)
Format::Audio(AudioFormat::M4a)
Format::Audio(AudioFormat::Wav)

// Video formats
Format::Video(VideoFormat::Mp4)
Format::Video(VideoFormat::Mkv)
Format::Video(VideoFormat::Webm)
Format::Video(VideoFormat::Avi)
Format::Video(VideoFormat::Mov)

// Audio quality
Quality::Audio(AudioQuality::Kbps128)
Quality::Audio(AudioQuality::Kbps320)
Quality::Audio(AudioQuality::Lossless)

// Video quality
Quality::Video(VideoQuality::P720)
Quality::Video(VideoQuality::P1080)
Quality::Video(VideoQuality::P4K)

// Parse from string
let format: Format = "mp3".parse()?;
let quality: Quality = "1080p".parse()?;
```
