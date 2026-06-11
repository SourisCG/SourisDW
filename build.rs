fn main() {
    // build.rs no longer downloads ffmpeg/deno at compile time.
    // All dependency management happens at runtime via DepManager.
    println!("cargo:rerun-if-changed=build.rs");
}
