use souris_dw::utils::fs;

#[test]
fn test_ensure_dir_accepts_spaces_and_unicode() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("Souris DW").join("cafe musica");

    fs::ensure_dir(&path).unwrap();

    assert!(path.is_dir());
}

#[test]
fn test_join_paths_uses_platform_separators() {
    let base = std::path::Path::new("downloads");
    let joined = fs::join_paths(base, "artist/album/song.mp3");

    assert_eq!(joined, base.join("artist").join("album").join("song.mp3"));
}

#[test]
fn test_sanitize_filename_removes_windows_reserved_chars() {
    let sanitized = fs::sanitize_filename("bad:name?.mp3");

    assert!(!sanitized.contains(':'));
    assert!(!sanitized.contains('?'));
}

#[test]
fn test_default_download_dir_is_not_relative_downloads() {
    let dir = souris_dw::default_download_dir();

    assert_ne!(dir, std::path::PathBuf::from("./downloads"));
    assert_ne!(dir, std::path::PathBuf::from("downloads"));
}

#[test]
fn test_legacy_download_dirs_are_detected() {
    assert!(souris_dw::utils::paths::is_legacy_default_download_dir(
        std::path::Path::new("./downloads")
    ));
    assert!(souris_dw::utils::paths::is_legacy_default_download_dir(
        std::path::Path::new("downloads")
    ));
}
