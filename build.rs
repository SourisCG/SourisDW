use std::env;
use std::fs;
use std::io::Read;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();
    let is_musl = target_env == "musl";

    let ffmpeg_path = out_dir.join("ffmpeg_bin");
    if !ffmpeg_path.exists() {
        println!(
            "cargo:warning=Downloading ffmpeg for {} {}...",
            target_os, target_arch
        );

        // ffmpeg-static is always musl-linked, works on any Linux
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

    let deno_path = out_dir.join("deno_bin");
    if !deno_path.exists() {
        if is_musl {
            // deno does not provide musl builds; download at runtime instead
            println!("cargo:warning=Skipping deno download for musl target. Deno will be downloaded at runtime.");
            let _ = fs::write(&deno_path, b"");
        } else {
            println!(
                "cargo:warning=Downloading deno for {} {}...",
                target_os, target_arch
            );

            match download_deno(&target_os, &target_arch, &deno_path) {
                Ok(_) => println!("cargo:warning=deno downloaded successfully"),
                Err(e) => {
                    println!(
                        "cargo:warning=Failed to download deno: {}. \
                         Deno will be downloaded at runtime.",
                        e
                    );
                    let _ = fs::write(&deno_path, b"");
                }
            }
        }
    }

    println!("cargo:rustc-env=DENO_PATH={}", deno_path.display());
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

    download_and_decompress_gz(url, dest)
}

fn download_deno(os: &str, arch: &str, dest: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let target = match (os, arch) {
        ("windows", "x86_64") => "x86_64-pc-windows-msvc",
        ("windows", "aarch64") => "aarch64-pc-windows-msvc",
        ("linux", "x86_64") => "x86_64-unknown-linux-gnu",
        ("linux", "aarch64") => "aarch64-unknown-linux-gnu",
        ("macos", "x86_64") => "x86_64-apple-darwin",
        ("macos", "aarch64") => "aarch64-apple-darwin",
        _ => return Err(format!("No deno available for {} {}", os, arch).into()),
    };

    let url = format!(
        "https://github.com/denoland/deno/releases/latest/download/deno-{}.zip",
        target
    );

    let client = reqwest::blocking::Client::builder()
        .user_agent("SourisDW/0.3.0")
        .build()?;
    let response = client.get(&url).send()?;
    let bytes = response.bytes()?;

    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(bytes))?;
    let binary_name = if os == "windows" { "deno.exe" } else { "deno" };

    for i in 0..archive.len() {
        let mut entry = archive.by_index(i)?;
        if entry.name().ends_with(binary_name) {
            let mut buf = Vec::new();
            entry.read_to_end(&mut buf)?;
            fs::write(dest, &buf)?;

            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                fs::set_permissions(dest, fs::Permissions::from_mode(0o755))?;
            }

            return Ok(());
        }
    }

    Err("deno binary not found in zip archive".into())
}

fn download_and_decompress_gz(url: &str, dest: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let temp_gz = dest.with_extension("gz");

    let client = reqwest::blocking::Client::builder()
        .user_agent("SourisDW/0.3.0")
        .build()?;
    let response = client.get(url).send()?;
    let bytes = response.bytes()?;

    use flate2::read::GzDecoder;
    let mut decoder = GzDecoder::new(&bytes[..]);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed)?;

    fs::write(dest, &decompressed)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(dest, fs::Permissions::from_mode(0o755))?;
    }

    let _ = fs::remove_file(&temp_gz);

    Ok(())
}
