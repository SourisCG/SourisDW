use souris_dw::SourisDW;

#[test]
fn test_builder_new() {
    let _builder = SourisDW::builder();
}

#[test]
fn test_builder_chain_format_str() {
    let builder = SourisDW::builder().format_str("mp3").unwrap();
    let _ = builder;
}

#[test]
fn test_builder_chain_quality_str() {
    let builder = SourisDW::builder().quality_str("320").unwrap();
    let _ = builder;
}

#[test]
fn test_builder_chain_all() {
    let builder = SourisDW::builder()
        .format_str("mp3")
        .unwrap()
        .quality_str("128")
        .unwrap()
        .output("./music")
        .parallel(2)
        .embed_metadata(true)
        .embed_thumbnail(false)
        .embed_subtitles(false);
    let _ = builder;
}
