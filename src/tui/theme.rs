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

pub const SYNTHWAVE84_THEME: Theme = Theme {
    background: Color::Rgb(0x26, 0x23, 0x35),
    foreground: Color::Rgb(0xff, 0xff, 0xff),
    accent: Color::Rgb(0x36, 0xf9, 0xf6),
    highlight: Color::Rgb(0xff, 0x7e, 0xdb),
    success: Color::Rgb(0x72, 0xf1, 0xb8),
    warning: Color::Rgb(0xfe, 0xde, 0x5d),
    error: Color::Rgb(0xfe, 0x44, 0x50),
    info: Color::Rgb(0xff, 0x8b, 0x39),
    progress: Color::Rgb(0x36, 0xf9, 0xf6),
    border: Color::Rgb(0x84, 0x8b, 0xbd),
    title: Color::Rgb(0xff, 0x7e, 0xdb),
    subtitle: Color::Rgb(0x84, 0x8b, 0xbd),
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
