use crate::error::Result;
use crate::core::types::MediaInfo;

pub async fn extract_info(url: &str) -> Result<MediaInfo> {
    let downloader = crate::core::downloader::SourisDW::builder()
        .auto_update(false)
        .build()
        .await?;

    downloader.info(url).await
}
