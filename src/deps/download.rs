use crate::error::{Result, SourisError};
use crate::utils::fs;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::Path;
use std::time::Duration;
use tokio_stream::StreamExt;

const MAX_DOWNLOAD_ATTEMPTS: usize = 3;

fn download_error(name: &str, reason: impl Into<String>) -> SourisError {
    SourisError::DependencyDownloadFailed {
        name: name.into(),
        reason: reason.into(),
    }
}

fn user_agent() -> String {
    format!("SourisDW/{}", env!("CARGO_PKG_VERSION"))
}

fn progress_bar(total: u64) -> ProgressBar {
    let pb = ProgressBar::new(total);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta}) - {msg}")
            .unwrap()
            .progress_chars("##-"),
    );
    pb
}

async fn fetch_bytes(url: &str, name: &str, quiet: bool) -> Result<Vec<u8>> {
    let client = reqwest::Client::builder()
        .user_agent(user_agent())
        .build()
        .map_err(|e| download_error(name, e.to_string()))?;

    let mut last_error = None;
    for attempt in 1..=MAX_DOWNLOAD_ATTEMPTS {
        let result = async {
            let response = client
                .get(url)
                .send()
                .await
                .map_err(|e| download_error(name, e.to_string()))?;

            if !response.status().is_success() {
                return Err(download_error(name, format!("HTTP {}", response.status())));
            }

            if quiet {
                return response
                    .bytes()
                    .await
                    .map(|b| b.to_vec())
                    .map_err(|e| download_error(name, e.to_string()));
            }

            let total = response.content_length().unwrap_or(0);
            let pb = progress_bar(total);
            pb.set_message(format!("Downloading {}...", name));

            let mut bytes = Vec::new();
            let mut stream = response.bytes_stream();
            while let Some(chunk) = stream.next().await {
                let chunk = chunk.map_err(|e| download_error(name, e.to_string()))?;
                bytes.extend_from_slice(&chunk);
                pb.set_position(bytes.len() as u64);
            }

            pb.finish_with_message(format!("{} downloaded", name));
            Ok(bytes)
        }
        .await;

        match result {
            Ok(bytes) => return Ok(bytes),
            Err(err) if attempt < MAX_DOWNLOAD_ATTEMPTS => {
                last_error = Some(err);
                tokio::time::sleep(Duration::from_secs(attempt as u64)).await;
            }
            Err(err) => return Err(err),
        }
    }

    Err(last_error.unwrap_or_else(|| download_error(name, "download failed")))
}

fn write_atomic(dest: &Path, bytes: &[u8], name: &str) -> Result<()> {
    let parent = dest.parent().unwrap_or(Path::new("."));
    fs::ensure_dir(parent)?;

    let file_name = dest
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("download");
    let tmp = parent.join(format!(".{}.{}.tmp", file_name, std::process::id()));

    fs_err::write(&tmp, bytes).map_err(|e| download_error(name, e.to_string()))?;

    if dest.exists() {
        fs_err::remove_file(dest).map_err(|e| {
            let _ = fs_err::remove_file(&tmp);
            download_error(name, e.to_string())
        })?;
    }

    fs_err::rename(&tmp, dest).map_err(|e| {
        let _ = fs_err::remove_file(&tmp);
        download_error(name, e.to_string())
    })?;

    Ok(())
}

pub async fn download_binary(url: &str, dest: &Path, name: &str, quiet: bool) -> Result<()> {
    let bytes = fetch_bytes(url, name, quiet).await?;
    write_atomic(dest, &bytes, name)?;

    #[cfg(unix)]
    fs::set_executable(dest)?;

    Ok(())
}

pub async fn download_and_decompress_gz(
    url: &str,
    dest: &Path,
    name: &str,
    quiet: bool,
) -> Result<()> {
    use flate2::read::GzDecoder;
    use std::io::Read;

    let bytes = fetch_bytes(url, name, quiet).await?;
    let mut decoder = GzDecoder::new(&bytes[..]);
    let mut decompressed = Vec::new();
    decoder
        .read_to_end(&mut decompressed)
        .map_err(|e| download_error(name, e.to_string()))?;

    write_atomic(dest, &decompressed, name)?;

    #[cfg(unix)]
    fs::set_executable(dest)?;

    Ok(())
}

pub async fn download_and_extract_zip(
    url: &str,
    dest: &Path,
    binary_name: &str,
    name: &str,
    quiet: bool,
) -> Result<()> {
    use std::io::Read;

    let bytes = fetch_bytes(url, name, quiet).await?;
    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(bytes))
        .map_err(|e| download_error(name, format!("Invalid zip: {}", e)))?;

    for i in 0..archive.len() {
        let mut entry = archive
            .by_index(i)
            .map_err(|e| download_error(name, format!("Zip entry error: {}", e)))?;

        if entry.name().ends_with(binary_name) {
            let mut extracted = Vec::new();
            entry
                .read_to_end(&mut extracted)
                .map_err(|e| download_error(name, e.to_string()))?;

            write_atomic(dest, &extracted, name)?;

            #[cfg(unix)]
            fs::set_executable(dest)?;

            return Ok(());
        }
    }

    Err(download_error(
        name,
        format!("{} binary not found in zip archive", binary_name),
    ))
}
