use crate::error::{Result, SourisError};
use std::path::PathBuf;

fn shell_config_files() -> Vec<PathBuf> {
    let mut files = Vec::new();

    if let Ok(home) = std::env::var("HOME") {
        let home_path = PathBuf::from(&home);

        let bashrc = home_path.join(".bashrc");
        if bashrc.exists() {
            files.push(bashrc);
        }

        let zshrc = home_path.join(".zshrc");
        if zshrc.exists() {
            files.push(zshrc);
        }

        let fish_config = home_path.join(".config/fish/config.fish");
        if fish_config.exists() {
            files.push(fish_config);
        }

        let profile = home_path.join(".profile");
        if profile.exists() {
            files.push(profile);
        }
    }

    #[cfg(target_os = "windows")]
    {
        if let Ok(profile) = std::env::var("USERPROFILE") {
            let ps_profile = PathBuf::from(&profile)
                .join("Documents/WindowsPowerShell/Microsoft.PowerShell_profile.ps1");
            if ps_profile.exists() {
                files.push(ps_profile);
            }
            let ps5_profile = PathBuf::from(&profile)
                .join("Documents/PowerShell/Microsoft.PowerShell_profile.ps1");
            if ps5_profile.exists() {
                files.push(ps5_profile);
            }
        }
    }

    files
}

fn export_line(bin_dir: &std::path::Path) -> String {
    format!(
        "\n# Added by souris-dw\nexport PATH=\"{}:$PATH\"\n",
        bin_dir.display()
    )
}

fn set_content_line(bin_dir: &std::path::Path) -> String {
    format!(
        "\n# Added by souris-dw\nset -x PATH \"{}\" $PATH\n",
        bin_dir.display()
    )
}

pub fn add_to_path(bin_dir: &std::path::Path) -> Result<()> {
    let files = shell_config_files();

    if files.is_empty() {
        if let Ok(home) = std::env::var("HOME") {
            let bashrc = PathBuf::from(&home).join(".bashrc");
            let line = export_line(bin_dir);
            fs_err::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&bashrc)
                .map_err(|e| {
                    SourisError::ConfigError(format!("Failed to create .bashrc: {}", e))
                })?;
            let mut contents = fs_err::read_to_string(&bashrc)
                .map_err(|e| SourisError::ConfigError(format!("Failed to read .bashrc: {}", e)))?;
            if !contents.contains(bin_dir.to_str().unwrap_or("")) {
                contents.push_str(&line);
                fs_err::write(&bashrc, &contents).map_err(|e| {
                    SourisError::ConfigError(format!("Failed to write .bashrc: {}", e))
                })?;
            }
        }
        return Ok(());
    }

    for file in &files {
        let line = if file.to_string_lossy().ends_with("fish") {
            set_content_line(bin_dir)
        } else {
            export_line(bin_dir)
        };

        let contents = fs_err::read_to_string(file).map_err(|e| {
            SourisError::ConfigError(format!("Failed to read {}: {}", file.display(), e))
        })?;

        if !contents.contains(bin_dir.to_str().unwrap_or("")) {
            let backup = file.with_extension("sdw.bak");
            let _ = fs_err::copy(file, &backup);

            let mut new_contents = contents;
            new_contents.push_str(&line);
            fs_err::write(file, &new_contents).map_err(|e| {
                SourisError::ConfigError(format!("Failed to write {}: {}", file.display(), e))
            })?;
        }
    }

    Ok(())
}

pub fn remove_from_path(bin_dir: &std::path::Path) -> Result<()> {
    let files = shell_config_files();
    let dir_str = bin_dir.to_str().unwrap_or("");

    for file in &files {
        let contents = fs_err::read_to_string(file).map_err(|e| {
            SourisError::ConfigError(format!("Failed to read {}: {}", file.display(), e))
        })?;

        if contents.contains(dir_str) {
            let backup = file.with_extension("sdw.bak");
            let _ = fs_err::copy(file, &backup);

            let new_contents: String = contents
                .lines()
                .filter(|line| !line.contains(dir_str) && !line.contains("# Added by souris-dw"))
                .collect::<Vec<_>>()
                .join("\n");

            fs_err::write(file, &new_contents).map_err(|e| {
                SourisError::ConfigError(format!("Failed to write {}: {}", file.display(), e))
            })?;
        }
    }

    Ok(())
}
