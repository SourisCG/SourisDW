use crate::core::downloader::SourisDW;
use crate::core::request::DownloadRequestBuilder;
use crate::core::types::DownloadResult;
use crate::error::Result;
use std::sync::Arc;
use tokio::sync::Semaphore;

pub struct DownloadQueue {
    semaphore: Arc<Semaphore>,
}

impl DownloadQueue {
    pub fn new(max_parallel: usize) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(max_parallel)),
        }
    }

    pub async fn execute_batch(
        &self,
        downloader: Arc<SourisDW>,
        requests: Vec<DownloadRequestBuilder>,
    ) -> Vec<Result<DownloadResult>> {
        let mut handles = Vec::new();

        for req in requests {
            let permit = self.semaphore.clone().acquire_owned().await.unwrap();
            let dw = downloader.clone();

            handles.push(tokio::spawn(async move {
                let result = dw.execute_request(req).await;
                drop(permit);
                result
            }));
        }

        let mut results = Vec::new();
        for handle in handles {
            match handle.await {
                Ok(result) => results.push(result),
                Err(e) => results.push(Err(crate::error::SourisError::DownloadFailed {
                    reason: e.to_string(),
                })),
            }
        }

        results
    }
}
