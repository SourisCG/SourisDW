//! CLI regression tests. These spawn the built `souris-dw` binary.

use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, MutexGuard};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

/// Serializes tests that mutate the process-global environment.
static ENV_LOCK: Mutex<()> = Mutex::new(());

fn binary() -> &'static str {
    env!("CARGO_BIN_EXE_souris-dw")
}

/// Points config/data dirs at a throwaway location so tests never touch
/// the developer's real configuration. Each call returns a unique directory
/// so parallel tests do not clobber each other.
struct IsolatedEnv {
    base: PathBuf,
    _guard: MutexGuard<'static, ()>,
}

fn isolated_env() -> IsolatedEnv {
    let guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let id = NEXT_ID.fetch_add(1, Ordering::SeqCst);
    let base = std::env::temp_dir().join(format!("souris-dw-test-{}-{}", std::process::id(), id));
    let config = base.join("config");
    let home = base.join("home");
    let _ = std::fs::remove_dir_all(&base);
    std::fs::create_dir_all(&config).unwrap();
    std::fs::create_dir_all(&home).unwrap();
    unsafe {
        std::env::set_var("XDG_CONFIG_HOME", &config);
        std::env::set_var("HOME", &home);
    }
    IsolatedEnv {
        base,
        _guard: guard,
    }
}

fn run(args: &[&str]) -> std::process::Output {
    Command::new(binary()).args(args).output().unwrap()
}

#[test]
fn test_config_set_persists() {
    let _env = isolated_env();

    let set = run(&["config", "set", "download.default_format", "mp3"]);
    assert!(set.status.success());

    let get = run(&["config", "get", "download.default_format"]);
    assert!(get.status.success());
    assert_eq!(String::from_utf8_lossy(&get.stdout).trim(), "mp3");

    // Also verify the TOML file itself was written. Uses the same directory
    // resolution as the binary (works on Linux, macOS, and Windows).
    let config_path =
        souris_dw::AppConfig::config_path().expect("config path should be resolvable");
    let contents =
        std::fs::read_to_string(&config_path).expect("config file should exist after config set");
    assert!(contents.contains("default_format = \"mp3\""));
}

#[test]
fn test_config_set_invalid_boolean_errors() {
    let _env = isolated_env();

    let out = run(&["config", "set", "download.embed_metadata", "notabool"]);
    assert!(!out.status.success());
}

#[test]
fn test_unsupported_format_exits_with_general_error() {
    let _env = isolated_env();

    let out = run(&[
        "download",
        "https://youtube.com/watch?v=xxx",
        "--format",
        "bogus",
    ]);
    assert!(!out.status.success());
    assert_eq!(out.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("Unsupported format"));
}

#[test]
fn test_search_platform_rejected() {
    let _env = isolated_env();

    let out = run(&["search", "hello", "--platform", "spotify"]);
    assert!(!out.status.success());
    assert_eq!(out.status.code(), Some(1));
}

#[test]
fn test_update_self_not_supported() {
    let _env = isolated_env();

    let out = run(&["update", "--self"]);
    assert!(!out.status.success());
}

#[test]
fn test_update_check_json_has_update_fields() {
    let _env = isolated_env();

    let out = run(&["update", "--check", "--json"]);
    if !out.status.success() {
        // Network may be unavailable in CI; skip rather than fail.
        return;
    }
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("update_available"),
        "missing update_available: {}",
        stdout
    );
}

#[test]
#[ignore = "requires network access"]
fn test_search_limit_respected() {
    let _env = isolated_env();

    let out = run(&["search", "hello", "--limit", "3", "--json"]);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert_eq!(stdout.matches("\"id\"").count(), 3);
}

#[test]
#[ignore = "requires network access"]
fn test_download_json_streams_events() {
    let env = isolated_env();
    let out_dir = env.base.join("downloads");

    let out = run(&[
        "download",
        "https://www.youtube.com/watch?v=kK42LZqO0wA",
        "--audio-only",
        "--format",
        "mp3",
        "--json",
        "--output",
        out_dir.to_str().unwrap(),
    ]);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("\"type\":\"init\""),
        "missing init: {}",
        stdout
    );
    assert!(
        stdout.contains("\"type\":\"complete\""),
        "missing complete: {}",
        stdout
    );
    assert!(
        stdout.contains("\"type\":\"summary\""),
        "missing summary: {}",
        stdout
    );

    // Complete event must carry a real path and a non-zero size.
    let last_complete = stdout
        .lines()
        .rev()
        .find(|l| l.contains("\"type\":\"complete\""))
        .expect("complete event present");
    assert!(
        last_complete.contains("\"size\":") && !last_complete.contains("\"size\":0,"),
        "complete should carry a real size: {}",
        last_complete
    );
    assert!(
        last_complete.contains("\"path\":\"/"),
        "complete should carry a path"
    );
}
