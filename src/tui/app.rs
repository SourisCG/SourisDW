use crate::core::progress::ProgressEvent;

pub struct AppState {
    pub downloads: Vec<DownloadState>,
    pub selected_index: usize,
    pub show_help: bool,
    pub show_settings: bool,
    pub input_mode: InputMode,
    pub input_buffer: String,
}

pub struct DownloadState {
    pub url: String,
    pub title: String,
    pub platform: String,
    pub status: DownloadStatus,
    pub progress: f64,
    pub speed: String,
    pub eta: String,
    pub format: String,
    pub quality: String,
}

pub enum DownloadStatus {
    Queued,
    Downloading,
    PostProcessing,
    Complete,
    Error(String),
}

pub enum InputMode {
    Normal,
    Input,
    Search,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            downloads: Vec::new(),
            selected_index: 0,
            show_help: false,
            show_settings: false,
            input_mode: InputMode::Normal,
            input_buffer: String::new(),
        }
    }

    pub fn update_from_event(&mut self, event: ProgressEvent) {
        match event {
            ProgressEvent::Init { url, platform, title, media_type, total_items } => {
                self.downloads.push(DownloadState {
                    url,
                    title,
                    platform,
                    status: DownloadStatus::Downloading,
                    progress: 0.0,
                    speed: String::new(),
                    eta: String::new(),
                    format: String::new(),
                    quality: String::new(),
                });
            }
            ProgressEvent::Progress { item, total, percent, speed, eta } => {
                if let Some(dl) = self.downloads.get_mut(item.saturating_sub(1)) {
                    dl.progress = percent;
                    dl.speed = speed;
                    dl.eta = eta;
                }
            }
            ProgressEvent::Complete { item, total, path, size } => {
                if let Some(dl) = self.downloads.get_mut(item.saturating_sub(1)) {
                    dl.status = DownloadStatus::Complete;
                    dl.progress = 100.0;
                }
            }
            ProgressEvent::Error { item, total, code, message } => {
                if let Some(dl) = self.downloads.get_mut(item.saturating_sub(1)) {
                    dl.status = DownloadStatus::Error(message);
                }
            }
            _ => {}
        }
    }
}
