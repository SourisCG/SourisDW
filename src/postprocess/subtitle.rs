use crate::error::Result;
use crate::deps::yt_dlp::YtDlp;

pub async fn download_subtitles(url: &str, output_dir: &str) -> Result<()> {
    let yt_dlp = YtDlp::ensure_installed().await?;

    let status = tokio::process::Command::new(yt_dlp.binary_path())
        .args([
            "--write-sub",
            "--write-auto-sub",
            "--sub-format",
            "vtt",
            "--skip-download",
            "-o",
            &format!("{}/%(title)s.%(ext)s", output_dir),
            url,
        ])
        .status()
        .await
        .map_err(|e| crate::error::SourisError::DownloadFailed {
            reason: e.to_string(),
        })?;

    if !status.success() {
        return Err(crate::error::SourisError::DownloadFailed {
            reason: "Subtitle download failed".to_string(),
        });
    }

    Ok(())
}
