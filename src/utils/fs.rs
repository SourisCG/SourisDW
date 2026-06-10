use crate::error::Result;
use crate::error::SourisError;
use std::path::{Path, PathBuf};

pub fn ensure_dir(path: &Path) -> Result<()> {
    if !path.exists() {
        fs_err::create_dir_all(path).map_err(|e| SourisError::io(path, e))?;
    }
    Ok(())
}

pub fn sanitize_filename(name: &str) -> String {
    sanitize_filename::sanitize(name)
}

pub fn normalize_path(path: &Path) -> PathBuf {
    use normalize_path::NormalizePath;
    path.normalize()
}

pub fn canonicalize(path: &Path) -> Result<PathBuf> {
    dunce::canonicalize(path).map_err(|e| SourisError::io(path, e))
}

pub fn file_name_without_ext(path: &Path) -> Option<String> {
    path.file_stem()
        .and_then(|s| s.to_str())
        .map(|s| s.to_string())
}

pub fn extension(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_string())
}

pub fn join_paths(base: &Path, relative: &str) -> PathBuf {
    let mut path = base.to_path_buf();
    for component in relative.split('/') {
        path = path.join(component);
    }
    path
}

#[cfg(unix)]
pub fn set_executable(path: &Path) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;
    let perms = std::fs::Permissions::from_mode(0o755);
    fs_err::set_permissions(path, perms).map_err(|e| SourisError::io(path, e))?;
    Ok(())
}

#[cfg(not(unix))]
pub fn set_executable(_path: &Path) -> Result<()> {
    Ok(())
}

pub fn which(name: &str) -> Option<PathBuf> {
    which::which(name).ok()
}
