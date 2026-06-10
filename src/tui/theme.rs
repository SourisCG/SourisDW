use ratatui::style::Color;

pub struct Theme {
    pub background: Color,
    pub foreground: Color,
    pub accent: Color,
    pub highlight: Color,
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub info: Color,
    pub progress: Color,
    pub border: Color,
    pub title: Color,
    pub subtitle: Color,
}

pub const OPENCODE_THEME: Theme = Theme {
    background: Color::Rgb(30, 30, 35),
    foreground: Color::Rgb(207, 206, 205),
    accent: Color::Rgb(101, 99, 99),
    highlight: Color::Rgb(241, 236, 236),
    success: Color::Rgb(134, 239, 172),
    warning: Color::Rgb(253, 224, 71),
    error: Color::Rgb(252, 165, 165),
    info: Color::Rgb(147, 197, 253),
    progress: Color::Rgb(196, 181, 253),
    border: Color::Rgb(75, 70, 70),
    title: Color::Rgb(241, 236, 236),
    subtitle: Color::Rgb(156, 163, 175),
};

pub fn progress_bar(percent: f64, width: usize) -> String {
    let filled = (percent / 100.0 * width as f64) as usize;
    let empty = width - filled;
    format!("{}{}", "█".repeat(filled), "░".repeat(empty))
}

pub fn format_speed(bytes_per_sec: f64) -> String {
    if bytes_per_sec >= 1_000_000.0 {
        format!("{:.1} MB/s", bytes_per_sec / 1_000_000.0)
    } else if bytes_per_sec >= 1_000.0 {
        format!("{:.1} KB/s", bytes_per_sec / 1_000.0)
    } else {
        format!("{:.0} B/s", bytes_per_sec)
    }
}

pub fn format_size(bytes: u64) -> String {
    if bytes >= 1_000_000_000 {
        format!("{:.1} GB", bytes as f64 / 1_000_000_000.0)
    } else if bytes >= 1_000_000 {
        format!("{:.1} MB", bytes as f64 / 1_000_000.0)
    } else if bytes >= 1_000 {
        format!("{:.1} KB", bytes as f64 / 1_000.0)
    } else {
        format!("{} B", bytes)
    }
}

pub fn format_duration(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;

    if hours > 0 {
        format!("{:02}:{:02}:{:02}", hours, minutes, secs)
    } else {
        format!("{:02}:{:02}", minutes, secs)
    }
}
