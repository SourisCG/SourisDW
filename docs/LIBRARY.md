# Library Guide (Rust)

SourisDW can be used as a Rust library in your own projects.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
souris-dw = "0.1.0"
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
        .await?;

    dw.download("https://youtube.com/watch?v=xxx").await?;

    Ok(())
}
```

## Builder Pattern

The builder configures default values for all downloads:

```rust
use souris_dw::SourisDW;

let dw = SourisDW::builder()
    .auto_update(true)           // Auto-update yt-dlp
    .format("mp4")               // Default format
    .quality("1080p")            // Default quality
    .output("./downloads")       // Default output directory
    .parallel(4)                 // Parallel downloads
    .embed_metadata(true)        // Embed ID3 tags
    .embed_thumbnail(true)       // Embed album art
    .embed_subtitles(false)      // Embed subtitles
    .timeout(300)                // Timeout in seconds
    .max_retries(3)              // Max retries on failure
    .spotify_credentials(        // Spotify API (optional)
        "client_id".to_string(),
        "client_secret".to_string(),
    )
    .build()
    .await?;
```

## Fluent Download API

Each download method returns a chainable request:

```rust
// Download with defaults
dw.download("URL").await?;

// Download audio with overrides
dw.download_audio("URL")
    .format("flac")
    .quality("lossless")
    .await?;

// Download video with overrides
dw.download_video("URL")
    .format("mkv")
    .quality("4K")
    .output("./videos")
    .await?;

// Download playlist
dw.download_playlist("PLAYLIST_URL")
    .parallel(8)
    .format("mp3")
    .await?;
```

## Info & Search

```rust
// Get media info
let info = dw.info("https://youtube.com/watch?v=xxx").await?;
println!("Title: {}", info.title);
println!("Duration: {:?}", info.duration);
println!("Platform: {}", info.platform);

// Search
let results = dw.search("never gonna give you up").await?;
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
    .await?;

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

dw.download("URL").await?;
```

## Dependency Management

```rust
// Check dependency status
let status = dw.update_check().await?;
for dep in &status {
    println!("{}: {} ({})", dep.name, dep.version.as_deref().unwrap_or("?"), dep.path);
}

// Update all dependencies
let updated = dw.update().await?;
```

## Configuration

```rust
use souris_dw::AppConfig;

// Load config
let config = AppConfig::load()?;

// Get a value
if let Some(fmt) = config.get("download.default_format") {
    println!("Default format: {}", fmt);
}

// Set a value
let mut config = AppConfig::load()?;
config.set("download.default_format", "mp3")?;
```

## Error Handling

```rust
use souris_dw::SourisError;

match dw.download("URL").await {
    Ok(_) => println!("Success"),
    Err(SourisError::DownloadFailed { reason }) => {
        eprintln!("Download failed: {}", reason);
    }
    Err(SourisError::DependencyNotFound { name }) => {
        eprintln!("Missing dependency: {}", name);
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

// Video formats
Format::Video(VideoFormat::Mp4)
Format::Video(VideoFormat::Mkv)

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
