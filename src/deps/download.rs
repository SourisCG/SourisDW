use crate::error::{Result, SourisError};
use crate::utils::fs;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::Path;
use tokio_stream::StreamExt;

pub async fn download_binary(url: &str, dest: &Path, name: &str, quiet: bool) -> Result<()> {
    let _ = fs::ensure_dir(dest.parent().unwrap_or(Path::new(".")));

    if quiet {
        let response =
            reqwest::get(url)
                .await
                .map_err(|e| SourisError::DependencyDownloadFailed {
                    name: name.into(),
                    reason: e.to_string(),
                })?;

        if !response.status().is_success() {
            return Err(SourisError::DependencyDownloadFailed {
                name: name.into(),
                reason: format!("HTTP {}", response.status()),
            });
        }

        let bytes = response
            .bytes()
            .await
            .map_err(|e| SourisError::DependencyDownloadFailed {
                name: name.into(),
                reason: e.to_string(),
            })?;

        fs_err::write(dest, &bytes).map_err(|e| SourisError::DependencyDownloadFailed {
            name: name.into(),
            reason: e.to_string(),
        })?;
    } else {
        let client = reqwest::Client::builder()
            .user_agent("SourisDW/0.4.0")
            .build()
            .map_err(|e| SourisError::DependencyDownloadFailed {
                name: name.into(),
                reason: e.to_string(),
            })?;

        let response =
            client
                .get(url)
                .send()
                .await
                .map_err(|e| SourisError::DependencyDownloadFailed {
                    name: name.into(),
                    reason: e.to_string(),
                })?;

        if !response.status().is_success() {
            return Err(SourisError::DependencyDownloadFailed {
                name: name.into(),
                reason: format!("HTTP {}", response.status()),
            });
        }

        let total = response.content_length().unwrap_or(0);
        let pb = ProgressBar::new(total);
        pb.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta}) - {msg}",
                )
                .unwrap()
                .progress_chars("##-"),
        );
        pb.set_message(format!("Downloading {}...", name));

        let mut file =
            fs_err::File::create(dest).map_err(|e| SourisError::DependencyDownloadFailed {
                name: name.into(),
                reason: e.to_string(),
            })?;

        let mut downloaded: u64 = 0;
        let mut stream = response.bytes_stream();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(|e| SourisError::DependencyDownloadFailed {
                name: name.into(),
                reason: e.to_string(),
            })?;
            use std::io::Write;
            file.write_all(&chunk)
                .map_err(|e| SourisError::DependencyDownloadFailed {
                    name: name.into(),
                    reason: e.to_string(),
                })?;
            downloaded += chunk.len() as u64;
            pb.set_position(downloaded);
        }

        pb.finish_with_message(format!("{} downloaded", name));
    }

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

    let _ = fs::ensure_dir(dest.parent().unwrap_or(Path::new(".")));

    if quiet {
        let response =
            reqwest::get(url)
                .await
                .map_err(|e| SourisError::DependencyDownloadFailed {
                    name: name.into(),
                    reason: e.to_string(),
                })?;

        let bytes = response
            .bytes()
            .await
            .map_err(|e| SourisError::DependencyDownloadFailed {
                name: name.into(),
                reason: e.to_string(),
            })?;

        let mut decoder = GzDecoder::new(&bytes[..]);
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed).map_err(|e| {
            SourisError::DependencyDownloadFailed {
                name: name.into(),
                reason: e.to_string(),
            }
        })?;

        fs_err::write(dest, &decompressed).map_err(|e| SourisError::DependencyDownloadFailed {
            name: name.into(),
            reason: e.to_string(),
        })?;
    } else {
        let client = reqwest::Client::builder()
            .user_agent("SourisDW/0.4.0")
            .build()
            .map_err(|e| SourisError::DependencyDownloadFailed {
                name: name.into(),
                reason: e.to_string(),
            })?;

        let response =
            client
                .get(url)
                .send()
                .await
                .map_err(|e| SourisError::DependencyDownloadFailed {
                    name: name.into(),
                    reason: e.to_string(),
                })?;

        let total = response.content_length().unwrap_or(0);
        let pb = ProgressBar::new(total);
        pb.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta}) - {msg}",
                )
                .unwrap()
                .progress_chars("##-"),
        );
        pb.set_message(format!("Downloading {}...", name));

        let mut gz_bytes = Vec::new();
        let mut stream = response.bytes_stream();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(|e| SourisError::DependencyDownloadFailed {
                name: name.into(),
                reason: e.to_string(),
            })?;
            gz_bytes.extend_from_slice(&chunk);
            pb.set_position(gz_bytes.len() as u64);
        }

        let mut decoder = GzDecoder::new(&gz_bytes[..]);
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed).map_err(|e| {
            SourisError::DependencyDownloadFailed {
                name: name.into(),
                reason: e.to_string(),
            }
        })?;

        fs_err::write(dest, &decompressed).map_err(|e| SourisError::DependencyDownloadFailed {
            name: name.into(),
            reason: e.to_string(),
        })?;

        pb.finish_with_message(format!("{} downloaded and extracted", name));
    }

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

    let _ = fs::ensure_dir(dest.parent().unwrap_or(Path::new(".")));

    let bytes =
        if quiet {
            let response =
                reqwest::get(url)
                    .await
                    .map_err(|e| SourisError::DependencyDownloadFailed {
                        name: name.into(),
                        reason: e.to_string(),
                    })?;
            response
                .bytes()
                .await
                .map_err(|e| SourisError::DependencyDownloadFailed {
                    name: name.into(),
                    reason: e.to_string(),
                })?
                .to_vec()
        } else {
            let client = reqwest::Client::builder()
                .user_agent("SourisDW/0.4.0")
                .build()
                .map_err(|e| SourisError::DependencyDownloadFailed {
                    name: name.into(),
                    reason: e.to_string(),
                })?;

            let response = client.get(url).send().await.map_err(|e| {
                SourisError::DependencyDownloadFailed {
                    name: name.into(),
                    reason: e.to_string(),
                }
            })?;

            let total = response.content_length().unwrap_or(0);
            let pb = ProgressBar::new(total);
            pb.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta}) - {msg}",
                )
                .unwrap()
                .progress_chars("##-"),
        );
            pb.set_message(format!("Downloading {}...", name));

            let mut all_bytes = Vec::new();
            let mut stream = response.bytes_stream();

            while let Some(chunk) = stream.next().await {
                let chunk = chunk.map_err(|e| SourisError::DependencyDownloadFailed {
                    name: name.into(),
                    reason: e.to_string(),
                })?;
                all_bytes.extend_from_slice(&chunk);
                pb.set_position(all_bytes.len() as u64);
            }

            pb.finish_with_message(format!("{} downloaded, extracting...", name));
            all_bytes
        };

    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(bytes)).map_err(|e| {
        SourisError::DependencyDownloadFailed {
            name: name.into(),
            reason: format!("Invalid zip: {}", e),
        }
    })?;

    for i in 0..archive.len() {
        let mut entry = archive
            .by_index(i)
            .map_err(|e| SourisError::DependencyDownloadFailed {
                name: name.into(),
                reason: format!("Zip entry error: {}", e),
            })?;

        if entry.name().ends_with(binary_name) {
            let mut buf = Vec::new();
            entry
                .read_to_end(&mut buf)
                .map_err(|e| SourisError::DependencyDownloadFailed {
                    name: name.into(),
                    reason: e.to_string(),
                })?;

            fs_err::write(dest, &buf).map_err(|e| SourisError::DependencyDownloadFailed {
                name: name.into(),
                reason: e.to_string(),
            })?;

            #[cfg(unix)]
            fs::set_executable(dest)?;

            return Ok(());
        }
    }

    Err(SourisError::DependencyDownloadFailed {
        name: name.into(),
        reason: format!("{} binary not found in zip archive", binary_name),
    })
}
