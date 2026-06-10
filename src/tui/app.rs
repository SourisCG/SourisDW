use crate::core::progress::ProgressEvent;
use std::path::PathBuf;

pub struct AppState {
    pub downloads: Vec<DownloadState>,
    pub selected_index: usize,
    pub show_help: bool,
    pub show_settings: bool,
    pub show_search: bool,
    pub input_mode: InputMode,
    pub input_buffer: String,
    pub search_results: Vec<SearchResult>,
    pub status_message: Option<String>,
    pub config: AppConfigState,
}

pub struct DownloadState {
    pub id: usize,
    pub url: String,
    pub title: String,
    pub platform: String,
    pub media_type: String,
    pub status: DownloadStatus,
    pub progress: f64,
    pub speed: String,
    pub eta: String,
    pub format: String,
    pub quality: String,
    pub size: Option<u64>,
    pub path: Option<String>,
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

pub struct SearchResult {
    pub title: String,
    pub url: String,
    pub platform: String,
    pub duration: Option<u64>,
    pub selected: bool,
}

pub struct AppConfigState {
    pub default_format: String,
    pub default_quality: String,
    pub output_dir: PathBuf,
    pub parallel: usize,
    pub embed_metadata: bool,
    pub embed_thumbnail: bool,
    pub auto_update: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            downloads: Vec::new(),
            selected_index: 0,
            show_help: false,
            show_settings: false,
            show_search: false,
            input_mode: InputMode::Normal,
            input_buffer: String::new(),
            search_results: Vec::new(),
            status_message: None,
            config: AppConfigState::default(),
        }
    }

    pub fn add_download(&mut self, url: String, title: String, platform: String) -> usize {
        let id = self.downloads.len();
        self.downloads.push(DownloadState {
            id,
            url,
            title,
            platform,
            media_type: "video".to_string(),
            status: DownloadStatus::Queued,
            progress: 0.0,
            speed: String::new(),
            eta: String::new(),
            format: self.config.default_format.clone(),
            quality: self.config.default_quality.clone(),
            size: None,
            path: None,
        });
        id
    }

    pub fn update_from_event(&mut self, event: ProgressEvent) {
        match event {
            ProgressEvent::Init {
                url: _,
                platform,
                title,
                media_type,
                total_items: _,
            } => {
                if let Some(dl) = self.downloads.last_mut() {
                    dl.title = title;
                    dl.platform = platform;
                    dl.media_type = media_type;
                    dl.status = DownloadStatus::Downloading;
                }
            }
            ProgressEvent::Progress {
                item,
                total: _,
                percent,
                speed,
                eta,
            } => {
                if let Some(dl) = self.downloads.get_mut(item.saturating_sub(1)) {
                    dl.progress = percent;
                    dl.speed = speed;
                    dl.eta = eta;
                    dl.status = DownloadStatus::Downloading;
                }
            }
            ProgressEvent::PostProcess {
                item,
                total: _,
                stage: _,
                format: _,
            } => {
                if let Some(dl) = self.downloads.get_mut(item.saturating_sub(1)) {
                    dl.status = DownloadStatus::PostProcessing;
                }
            }
            ProgressEvent::Metadata {
                item,
                total: _,
                stage: _,
            } => {
                if let Some(dl) = self.downloads.get_mut(item.saturating_sub(1)) {
                    dl.status = DownloadStatus::PostProcessing;
                }
            }
            ProgressEvent::Complete {
                item,
                total: _,
                path,
                size,
            } => {
                if let Some(dl) = self.downloads.get_mut(item.saturating_sub(1)) {
                    dl.status = DownloadStatus::Complete;
                    dl.progress = 100.0;
                    dl.path = Some(path);
                    dl.size = Some(size);
                }
            }
            ProgressEvent::Error {
                item,
                total: _,
                code: _,
                message,
            } => {
                if let Some(dl) = self.downloads.get_mut(item.saturating_sub(1)) {
                    dl.status = DownloadStatus::Error(message);
                }
            }
            ProgressEvent::Summary {
                total,
                success,
                failed: _,
                elapsed,
            } => {
                self.status_message = Some(format!(
                    "Completed: {}/{} successful in {}",
                    success, total, elapsed
                ));
            }
        }
    }

    pub fn move_selection_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub fn move_selection_down(&mut self) {
        if self.selected_index < self.downloads.len().saturating_sub(1) {
            self.selected_index += 1;
        }
    }

    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    pub fn toggle_settings(&mut self) {
        self.show_settings = !self.show_settings;
    }

    pub fn toggle_search(&mut self) {
        self.show_search = !self.show_search;
        if self.show_search {
            self.input_mode = InputMode::Search;
            self.input_buffer.clear();
        } else {
            self.input_mode = InputMode::Normal;
        }
    }

    pub fn start_input(&mut self) {
        self.input_mode = InputMode::Input;
        self.input_buffer.clear();
    }

    pub fn cancel_input(&mut self) {
        self.input_mode = InputMode::Normal;
        self.input_buffer.clear();
    }

    pub fn get_active_count(&self) -> usize {
        self.downloads
            .iter()
            .filter(|d| matches!(d.status, DownloadStatus::Downloading | DownloadStatus::PostProcessing))
            .count()
    }

    pub fn get_completed_count(&self) -> usize {
        self.downloads
            .iter()
            .filter(|d| matches!(d.status, DownloadStatus::Complete))
            .count()
    }

    pub fn get_error_count(&self) -> usize {
        self.downloads
            .iter()
            .filter(|d| matches!(d.status, DownloadStatus::Error(_)))
            .count()
    }
}

impl Default for AppConfigState {
    fn default() -> Self {
        Self {
            default_format: "mp4".to_string(),
            default_quality: "1080p".to_string(),
            output_dir: PathBuf::from("./downloads"),
            parallel: 4,
            embed_metadata: true,
            embed_thumbnail: true,
            auto_update: true,
        }
    }
}
