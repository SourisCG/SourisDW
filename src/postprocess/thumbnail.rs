use crate::error::Result;

pub async fn embed_thumbnail(file_path: &str, thumbnail_path: &str) -> Result<()> {
    use lofty::prelude::*;
    use lofty::probe::Probe;
    use lofty::tag::Tag;

    let tagged_file = Probe::open(file_path)
        .map_err(|e| crate::error::SourisError::MetadataError(e.to_string()))?
        .read()
        .map_err(|e| crate::error::SourisError::MetadataError(e.to_string()))?;

    let mut tag = Tag::new(tagged_file.primary_tag_type());

    let image_data = fs_err::read(thumbnail_path)
        .map_err(|e| crate::error::SourisError::io(thumbnail_path, e))?;

    let picture = lofty::picture::Picture::new_unchecked(
        lofty::picture::PictureType::CoverFront,
        Some(lofty::picture::MimeType::Jpeg),
        None,
        image_data,
    );

    tag.set_picture(0, picture);

    tag.save_to_path(file_path, lofty::config::WriteOptions::default())
        .map_err(|e| crate::error::SourisError::MetadataError(e.to_string()))?;

    Ok(())
}
