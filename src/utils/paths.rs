use std::path::{Path, PathBuf};

pub fn default_download_dir() -> PathBuf {
    if let Some(dir) = dirs::download_dir() {
        return dir;
    }

    if let Some(home) = dirs::home_dir() {
        return home.join("Downloads");
    }

    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

pub fn is_legacy_default_download_dir(path: &Path) -> bool {
    if path == Path::new("downloads") || path == Path::new("./downloads") {
        return true;
    }

    let normalized = crate::utils::fs::normalize_path(path);
    if normalized.ends_with("downloads") {
        if let Some(data_dir) = crate::deps::platform::data_dir() {
            return normalized == crate::utils::fs::normalize_path(&data_dir.join("downloads"));
        }
    }

    false
}

pub fn ensure_default_dirs() -> crate::error::Result<()> {
    crate::utils::fs::ensure_dir(&default_download_dir())?;
    if let Some(dir) = crate::deps::platform::config_dir() {
        crate::utils::fs::ensure_dir(&dir)?;
    }
    if let Some(dir) = crate::deps::platform::data_dir() {
        crate::utils::fs::ensure_dir(&dir)?;
    }
    if let Some(dir) = crate::deps::platform::cache_dir() {
        crate::utils::fs::ensure_dir(&dir)?;
    }
    if let Some(dir) = crate::deps::platform::bin_dir() {
        crate::utils::fs::ensure_dir(&dir)?;
    }
    Ok(())
}
