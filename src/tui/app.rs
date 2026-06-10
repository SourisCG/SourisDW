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
    pub waiting_for_quit: bool,
    pub settings_index: usize,
    pub search_index: usize,
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

pub const SETTINGS_OPTIONS: &[&str] = &[
    "Format",
    "Quality",
    "Output Dir",
    "Parallel Downloads",
    "Embed Metadata",
    "Embed Thumbnail",
    "Auto Update",
];

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
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
            waiting_for_quit: false,
            settings_index: 0,
            search_index: 0,
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

    pub fn has_overlay(&self) -> bool {
        self.show_help || self.show_search || self.show_settings
    }

    pub fn close_overlay(&mut self) {
        self.show_help = false;
        self.show_search = false;
        self.show_settings = false;
        self.input_mode = InputMode::Normal;
        self.waiting_for_quit = false;
    }

    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
        self.waiting_for_quit = false;
    }

    pub fn toggle_settings(&mut self) {
        self.show_settings = !self.show_settings;
        self.waiting_for_quit = false;
    }

    pub fn toggle_search(&mut self) {
        self.show_search = !self.show_search;
        if self.show_search {
            self.input_mode = InputMode::Search;
            self.input_buffer.clear();
            self.search_index = 0;
        } else {
            self.input_mode = InputMode::Normal;
        }
        self.waiting_for_quit = false;
    }

    pub fn start_input(&mut self) {
        self.input_mode = InputMode::Input;
        self.input_buffer.clear();
        self.waiting_for_quit = false;
    }

    pub fn cancel_input(&mut self) {
        self.input_mode = InputMode::Normal;
        self.input_buffer.clear();
        self.waiting_for_quit = false;
    }

    pub fn copy_selected_url(&self) -> bool {
        if let Some(dl) = self.downloads.get(self.selected_index) {
            if let Ok(mut clipboard) = arboard::Clipboard::new() {
                return clipboard.set_text(&dl.url).is_ok();
            }
        }
        false
    }

    pub fn get_active_count(&self) -> usize {
        self.downloads
            .iter()
            .filter(|d| {
                matches!(
                    d.status,
                    DownloadStatus::Downloading | DownloadStatus::PostProcessing
                )
            })
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

    pub fn get_setting_value(&self, index: usize) -> String {
        match index {
            0 => self.config.default_format.clone(),
            1 => self.config.default_quality.clone(),
            2 => self.config.output_dir.display().to_string(),
            3 => self.config.parallel.to_string(),
            4 => self.config.embed_metadata.to_string(),
            5 => self.config.embed_thumbnail.to_string(),
            6 => self.config.auto_update.to_string(),
            _ => String::new(),
        }
    }

    pub fn cycle_setting_value(&mut self, index: usize) {
        match index {
            0 => {
                let formats = ["mp4", "mkv", "webm", "avi", "mov"];
                let current = formats
                    .iter()
                    .position(|&f| f == self.config.default_format)
                    .unwrap_or(0);
                self.config.default_format = formats[(current + 1) % formats.len()].to_string();
            }
            1 => {
                let qualities = ["360p", "480p", "720p", "1080p", "1440p", "2160p"];
                let current = qualities
                    .iter()
                    .position(|&q| q == self.config.default_quality)
                    .unwrap_or(0);
                self.config.default_quality =
                    qualities[(current + 1) % qualities.len()].to_string();
            }
            3 => {
                self.config.parallel = if self.config.parallel >= 8 {
                    1
                } else {
                    self.config.parallel + 1
                };
            }
            4 => self.config.embed_metadata = !self.config.embed_metadata,
            5 => self.config.embed_thumbnail = !self.config.embed_thumbnail,
            6 => self.config.auto_update = !self.config.auto_update,
            _ => {}
        }
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
