use crate::error::Result;

pub async fn embed_metadata(file_path: &str, title: &str, artist: &str, album: &str) -> Result<()> {
    use lofty::prelude::*;
    use lofty::probe::Probe;
    use lofty::tag::Tag;

    let tagged_file = Probe::open(file_path)
        .map_err(|e| crate::error::SourisError::MetadataError(e.to_string()))?
        .read()
        .map_err(|e| crate::error::SourisError::MetadataError(e.to_string()))?;

    let mut tag = Tag::new(tagged_file.primary_tag_type());
    tag.set_title(title.to_string());
    tag.set_artist(artist.to_string());
    tag.set_album(album.to_string());

    tag.save_to_path(file_path, lofty::config::WriteOptions::default())
        .map_err(|e| crate::error::SourisError::MetadataError(e.to_string()))?;

    Ok(())
}
