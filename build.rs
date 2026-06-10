use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let ffmpeg_path = out_dir.join("ffmpeg_bin");

    if !ffmpeg_path.exists() {
        let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
        let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();

        println!(
            "cargo:warning=Downloading ffmpeg for {} {}...",
            target_os, target_arch
        );

        match download_ffmpeg(&target_os, &target_arch, &ffmpeg_path) {
            Ok(_) => println!("cargo:warning=ffmpeg downloaded successfully"),
            Err(e) => {
                println!(
                    "cargo:warning=Failed to download ffmpeg: {}. Using empty placeholder.",
                    e
                );
                fs::write(&ffmpeg_path, b"").unwrap();
            }
        }
    }

    println!("cargo:rustc-env=FFMPEG_PATH={}", ffmpeg_path.display());
    println!("cargo:rerun-if-changed=build.rs");
}

fn download_ffmpeg(os: &str, arch: &str, dest: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let url = match (os, arch) {
        ("linux", "x86_64") => {
            "https://github.com/eugeneware/ffmpeg-static/releases/latest/download/ffmpeg-linux-x64.gz"
        }
        ("linux", "aarch64") => {
            "https://github.com/eugeneware/ffmpeg-static/releases/latest/download/ffmpeg-linux-arm64.gz"
        }
        ("macos", "x86_64") => {
            "https://github.com/eugeneware/ffmpeg-static/releases/latest/download/ffmpeg-darwin-x64.gz"
        }
        ("macos", "aarch64") => {
            "https://github.com/eugeneware/ffmpeg-static/releases/latest/download/ffmpeg-darwin-arm64.gz"
        }
        ("windows", _) => {
            "https://github.com/eugeneware/ffmpeg-static/releases/latest/download/ffmpeg-win32-x64.gz"
        }
        _ => {
            return Err(format!("No ffmpeg available for {} {}", os, arch).into());
        }
    };

    download_and_decompress(url, dest, os == "windows")
}

fn download_and_decompress(
    url: &str,
    dest: &PathBuf,
    skip_decompress: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = if skip_decompress {
        dest.with_extension("exe")
    } else {
        dest.with_extension("tmp")
    };

    let status = Command::new("curl")
        .args(["-fsSL", "-o", temp_file.to_str().unwrap(), url])
        .status()?;

    if !status.success() {
        return Err("Failed to download ffmpeg".into());
    }

    if skip_decompress {
        fs::rename(&temp_file, dest)?;
    } else {
        let status = Command::new("gunzip")
            .arg("-f")
            .arg(temp_file.to_str().unwrap())
            .status();

        match status {
            Ok(s) if s.success() => {
                let decompressed = dest.with_extension("");
                if decompressed.exists() {
                    fs::rename(&decompressed, dest)?;
                }
            }
            _ => {
                fs::rename(&temp_file, dest)?;
            }
        }
    }

    #[cfg(unix)]
    {
        Command::new("chmod")
            .args(["+x", dest.to_str().unwrap()])
            .status()?;
    }

    Ok(())
}
