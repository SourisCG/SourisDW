use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ProgressEvent {
    #[serde(rename = "init")]
    Init {
        url: String,
        platform: String,
        title: String,
        media_type: String,
        total_items: usize,
    },
    #[serde(rename = "progress")]
    Progress {
        item: usize,
        total: usize,
        percent: f64,
        speed: String,
        eta: String,
    },
    #[serde(rename = "postprocess")]
    PostProcess {
        item: usize,
        total: usize,
        stage: String,
        format: String,
    },
    #[serde(rename = "metadata")]
    Metadata {
        item: usize,
        total: usize,
        stage: String,
    },
    #[serde(rename = "complete")]
    Complete {
        item: usize,
        total: usize,
        path: String,
        size: u64,
    },
    #[serde(rename = "error")]
    Error {
        item: usize,
        total: usize,
        code: String,
        message: String,
    },
    #[serde(rename = "summary")]
    Summary {
        total: usize,
        success: usize,
        failed: usize,
        elapsed: String,
    },
}

pub type ProgressSender = mpsc::UnboundedSender<ProgressEvent>;
pub type ProgressReceiver = mpsc::UnboundedReceiver<ProgressEvent>;

pub fn create_progress_channel() -> (ProgressSender, ProgressReceiver) {
    mpsc::unbounded_channel()
}

impl ProgressEvent {
    pub fn init(url: &str, platform: &str, title: &str, media_type: &str, total_items: usize) -> Self {
        ProgressEvent::Init {
            url: url.to_string(),
            platform: platform.to_string(),
            title: title.to_string(),
            media_type: media_type.to_string(),
            total_items,
        }
    }

    pub fn progress(item: usize, total: usize, percent: f64, speed: &str, eta: &str) -> Self {
        ProgressEvent::Progress {
            item,
            total,
            percent,
            speed: speed.to_string(),
            eta: eta.to_string(),
        }
    }

    pub fn complete(item: usize, total: usize, path: &str, size: u64) -> Self {
        ProgressEvent::Complete {
            item,
            total,
            path: path.to_string(),
            size,
        }
    }

    pub fn error(item: usize, total: usize, code: &str, message: &str) -> Self {
        ProgressEvent::Error {
            item,
            total,
            code: code.to_string(),
            message: message.to_string(),
        }
    }

    pub fn summary(total: usize, success: usize, failed: usize, elapsed: &str) -> Self {
        ProgressEvent::Summary {
            total,
            success,
            failed,
            elapsed: elapsed.to_string(),
        }
    }
}
