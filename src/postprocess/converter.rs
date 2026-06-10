use crate::error::Result;

pub async fn convert_audio(input: &str, output: &str, format: &str) -> Result<()> {
    let status = tokio::process::Command::new("ffmpeg")
        .args([
            "-i",
            input,
            "-vn",
            "-acodec",
            codec_for_format(format),
            output,
        ])
        .status()
        .await
        .map_err(|e| crate::error::SourisError::FFmpegError(e.to_string()))?;

    if !status.success() {
        return Err(crate::error::SourisError::FFmpegError(
            "Audio conversion failed".to_string(),
        ));
    }

    Ok(())
}

pub async fn convert_video(input: &str, output: &str, _format: &str) -> Result<()> {
    let status = tokio::process::Command::new("ffmpeg")
        .args(["-i", input, "-c:v", "libx264", "-c:a", "aac", output])
        .status()
        .await
        .map_err(|e| crate::error::SourisError::FFmpegError(e.to_string()))?;

    if !status.success() {
        return Err(crate::error::SourisError::FFmpegError(
            "Video conversion failed".to_string(),
        ));
    }

    Ok(())
}

fn codec_for_format(format: &str) -> &str {
    match format {
        "mp3" => "libmp3lame",
        "aac" => "aac",
        "flac" => "flac",
        "ogg" | "vorbis" => "libvorbis",
        "opus" => "libopus",
        "wav" => "pcm_s16le",
        _ => "copy",
    }
}
