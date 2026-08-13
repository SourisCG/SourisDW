# Guia de Libreria (Rust)

SourisDW puede usarse como libreria Rust en tus propios proyectos.

## Instalacion

Agrega a tu `Cargo.toml`:

```toml
[dependencies]
souris-dw = "0.3.6"
```

## Inicio Rapido

```rust
use souris_dw::SourisDW;

let dw = SourisDW::builder()
    .format("mp4")
    .quality("1080p")
    .output("./downloads")
    .build()
    .await;

dw.download("https://youtube.com/watch?v=xxx").run().await?;
```

## Patron Builder

```rust
use souris_dw::SourisDW;

let dw = SourisDW::builder()
    .auto_update(true)
    .yt_dlp_channel("stable")
    .format("mp4")
    .quality("1080p")
    .output("./downloads")
    .parallel(4)
    .embed_metadata(true)
    .embed_thumbnail(true)
    .embed_subtitles(false)
    .timeout(300)
    .max_retries(3)
    .spotify_credentials("client_id".into(), "client_secret".into())
    .cookies_file("cookies.txt")
    .cookies_from_browser("firefox")
    .build()
    .await;
```

## API Fluida

```rust
// Descarga con defaults
dw.download("URL").run().await?;

// Solo audio
dw.download_audio("URL").format("flac").quality("lossless").run().await?;

// Solo video
dw.download_video("URL").format("mkv").quality("4K").run().await?;

// Lista de reproduccion
dw.download_playlist("PLAYLIST_URL").parallel(8).format("mp3").run().await?;

// Con parseo dinamico
dw.download("URL").format_str("mp4")?.quality_str("4K")?.run().await?;
```

## Informacion y Busqueda

```rust
let info = dw.info("URL").await?;
println!("Titulo: {}", info.title);

let results = dw.search("never gonna give you up", 10).await?;
```

## Eventos de Progreso

```rust
use souris_dw::ProgressEvent;
use souris_dw::core::progress::create_progress_channel;

let (tx, mut rx) = create_progress_channel();

let dw = SourisDW::builder()
    .on_progress(tx)
    .build()
    .await;

tokio::spawn(async move {
    while let Some(event) = rx.recv().await {
        match event {
            ProgressEvent::Progress { percent, speed, .. } => {
                println!("Progreso: {:.1}% ({})", percent, speed);
            }
            ProgressEvent::Complete { path, size, .. } => {
                println!("Descargado: {} ({} bytes)", path, size);
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

## Manejo de Errores

```rust
use souris_dw::SourisError;

match dw.download("URL").run().await {
    Ok(_) => println!("Exito"),
    Err(SourisError::DownloadFailed { reason }) => {
        eprintln!("Error de descarga: {}", reason);
    }
    Err(SourisError::DependencyNotFound { name }) => {
        eprintln!("Dependencia faltante: {}", name);
    }
    Err(SourisError::Cancelled) => {
        println!("Cancelado por el usuario");
    }
    Err(e) => {
        eprintln!("Error: {}", e);
    }
}
```

## Manejo de Dependencias

```rust
// Verificar estado de dependencias (incluye latest + update_available tras check_updates)
let status = dw.update_check().await?;
for dep in &status {
    println!("{}: {} ({})", dep.name, dep.version.as_deref().unwrap_or("?"), dep.path);
}

// Actualizar todas las dependencias
let updated = dw.update().await?;

// Actualizar solo dependencias especificas
let updated = dw.update_specific(true, false, false).await?;  // solo yt-dlp

// Usando DepManager directamente
use souris_dw::DepManager;

// Setup con auto-update
let deps = DepManager::setup(true, "stable").await;
println!("yt-dlp: {}", deps.yt_dlp().binary_path().display());

// Estado
let status = deps.status();

// Verificar actualizaciones sin instalar (llena latest/update_available)
let status = deps.check_updates().await;
```

Los campos de `DepStatus` son: `name`, `installed`, `version`, `path`, mas `latest` y `update_available` (poblados por `check_updates`).

## Tipos

```rust
use souris_dw::core::types::*;

// Formatos de audio
Format::Audio(AudioFormat::Mp3)
Format::Audio(AudioFormat::Flac)

// Formatos de video
Format::Video(VideoFormat::Mp4)
Format::Video(VideoFormat::Mkv)

// Calidad
Quality::Audio(AudioQuality::Kbps320)
Quality::Video(VideoQuality::P1080)

// Parseo desde string
let format: Format = "mp3".parse()?;
let quality: Quality = "1080p".parse()?;
```
