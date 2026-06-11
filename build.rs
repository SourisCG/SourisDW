use std::env;
use std::fs;
use std::path::PathBuf;

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
                    "cargo:warning=Failed to download ffmpeg: {}. \
                     Using empty placeholder. SourisDW will fall back to system ffmpeg.",
                    e
                );
                let _ = fs::write(&ffmpeg_path, b"");
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
        ("windows", "x86_64") => {
            "https://github.com/eugeneware/ffmpeg-static/releases/latest/download/ffmpeg-win32-x64.gz"
        }
        ("windows", "aarch64") => {
            "https://github.com/eugeneware/ffmpeg-static/releases/latest/download/ffmpeg-win32-arm64.gz"
        }
        _ => {
            return Err(format!("No ffmpeg available for {} {}", os, arch).into());
        }
    };

    download_and_decompress(url, dest)
}

fn download_and_decompress(
    url: &str,
    dest: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let temp_gz = dest.with_extension("gz");

    let client = reqwest::blocking::Client::builder()
        .user_agent("SourisDW/0.2.0")
        .build()?;
    let response = client.get(url).send()?;
    let bytes = response.bytes()?;

    // Decompress gzip
    use flate2::read::GzDecoder;
    use std::io::Read;
    let mut decoder = GzDecoder::new(&bytes[..]);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed)?;

    fs::write(dest, &decompressed)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(dest, fs::Permissions::from_mode(0o755))?;
    }

    // Clean up temp file if it exists
    let _ = fs::remove_file(&temp_gz);

    Ok(())
}
