use souris_dw::core::types::*;

#[test]
fn test_audio_format_from_str() {
    assert_eq!("mp3".parse::<AudioFormat>().unwrap(), AudioFormat::Mp3);
    assert_eq!("flac".parse::<AudioFormat>().unwrap(), AudioFormat::Flac);
    assert_eq!("aac".parse::<AudioFormat>().unwrap(), AudioFormat::Aac);
    assert_eq!("ogg".parse::<AudioFormat>().unwrap(), AudioFormat::Ogg);
    assert_eq!("m4a".parse::<AudioFormat>().unwrap(), AudioFormat::M4a);
    assert_eq!("wav".parse::<AudioFormat>().unwrap(), AudioFormat::Wav);
}

#[test]
fn test_audio_format_case_insensitive() {
    assert_eq!("MP3".parse::<AudioFormat>().unwrap(), AudioFormat::Mp3);
    assert_eq!("Flac".parse::<AudioFormat>().unwrap(), AudioFormat::Flac);
}

#[test]
fn test_video_format_from_str() {
    assert_eq!("mp4".parse::<VideoFormat>().unwrap(), VideoFormat::Mp4);
    assert_eq!("mkv".parse::<VideoFormat>().unwrap(), VideoFormat::Mkv);
    assert_eq!("webm".parse::<VideoFormat>().unwrap(), VideoFormat::Webm);
    assert_eq!("avi".parse::<VideoFormat>().unwrap(), VideoFormat::Avi);
    assert_eq!("mov".parse::<VideoFormat>().unwrap(), VideoFormat::Mov);
}

#[test]
fn test_format_from_str() {
    assert!(matches!(
        "mp3".parse::<Format>().unwrap(),
        Format::Audio(AudioFormat::Mp3)
    ));
    assert!(matches!(
        "mp4".parse::<Format>().unwrap(),
        Format::Video(VideoFormat::Mp4)
    ));
    assert!(matches!(
        "flac".parse::<Format>().unwrap(),
        Format::Audio(AudioFormat::Flac)
    ));
    assert!(matches!(
        "mkv".parse::<Format>().unwrap(),
        Format::Video(VideoFormat::Mkv)
    ));
}

#[test]
fn test_invalid_format() {
    assert!("xyz".parse::<AudioFormat>().is_err());
    assert!("xyz".parse::<VideoFormat>().is_err());
    assert!("xyz".parse::<Format>().is_err());
}

#[test]
fn test_audio_quality_from_str() {
    assert_eq!(
        "128".parse::<AudioQuality>().unwrap(),
        AudioQuality::Kbps128
    );
    assert_eq!(
        "192".parse::<AudioQuality>().unwrap(),
        AudioQuality::Kbps192
    );
    assert_eq!(
        "256".parse::<AudioQuality>().unwrap(),
        AudioQuality::Kbps256
    );
    assert_eq!(
        "320".parse::<AudioQuality>().unwrap(),
        AudioQuality::Kbps320
    );
    assert_eq!(
        "lossless".parse::<AudioQuality>().unwrap(),
        AudioQuality::Lossless
    );
}

#[test]
fn test_audio_quality_with_suffix() {
    assert_eq!(
        "128kbps".parse::<AudioQuality>().unwrap(),
        AudioQuality::Kbps128
    );
    assert_eq!(
        "320kbps".parse::<AudioQuality>().unwrap(),
        AudioQuality::Kbps320
    );
}

#[test]
fn test_video_quality_from_str() {
    assert_eq!("360p".parse::<VideoQuality>().unwrap(), VideoQuality::P360);
    assert_eq!("480p".parse::<VideoQuality>().unwrap(), VideoQuality::P480);
    assert_eq!("720p".parse::<VideoQuality>().unwrap(), VideoQuality::P720);
    assert_eq!(
        "1080p".parse::<VideoQuality>().unwrap(),
        VideoQuality::P1080
    );
    assert_eq!(
        "1440p".parse::<VideoQuality>().unwrap(),
        VideoQuality::P1440
    );
    assert_eq!("4K".parse::<VideoQuality>().unwrap(), VideoQuality::P4K);
    assert_eq!("8K".parse::<VideoQuality>().unwrap(), VideoQuality::P8K);
}

#[test]
fn test_video_quality_aliases() {
    assert_eq!("FHD".parse::<VideoQuality>().unwrap(), VideoQuality::P1080);
    assert_eq!("2K".parse::<VideoQuality>().unwrap(), VideoQuality::P1440);
    assert_eq!("2160p".parse::<VideoQuality>().unwrap(), VideoQuality::P4K);
    assert_eq!("4320p".parse::<VideoQuality>().unwrap(), VideoQuality::P8K);
}

#[test]
fn test_audio_format_display() {
    assert_eq!(AudioFormat::Mp3.to_string(), "mp3");
    assert_eq!(AudioFormat::Flac.to_string(), "flac");
    assert_eq!(AudioFormat::Aac.to_string(), "aac");
}

#[test]
fn test_video_format_display() {
    assert_eq!(VideoFormat::Mp4.to_string(), "mp4");
    assert_eq!(VideoFormat::Mkv.to_string(), "mkv");
    assert_eq!(VideoFormat::Webm.to_string(), "webm");
}

#[test]
fn test_audio_quality_display() {
    assert_eq!(AudioQuality::Kbps128.to_string(), "128");
    assert_eq!(AudioQuality::Kbps320.to_string(), "320");
    assert_eq!(AudioQuality::Lossless.to_string(), "lossless");
}

#[test]
fn test_video_quality_display() {
    assert_eq!(VideoQuality::P360.to_string(), "360p");
    assert_eq!(VideoQuality::P1080.to_string(), "1080p");
    assert_eq!(VideoQuality::P4K.to_string(), "4K");
}
