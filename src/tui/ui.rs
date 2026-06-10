use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Gauge, List, ListItem, Paragraph};
use crate::tui::app::AppState;
use crate::tui::theme::{OPENCODE_THEME, progress_bar, format_size};

pub fn draw(f: &mut Frame, app: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(f.area());

    draw_header(f, chunks[0]);
    draw_downloads(f, chunks[1], app);
    draw_footer(f, chunks[2]);
}

fn draw_header(f: &mut Frame, area: Rect) {
    let header = Paragraph::new(vec![
        Line::from(vec![
            Span::styled(
                " SourisDW",
                Style::default()
                    .fg(OPENCODE_THEME.title)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                " v0.1.0",
                Style::default().fg(OPENCODE_THEME.subtitle),
            ),
        ]),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(OPENCODE_THEME.border))
            .style(Style::default().bg(OPENCODE_THEME.background)),
    );

    f.render_widget(header, area);
}

fn draw_downloads(f: &mut Frame, area: Rect, app: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(5),
            Constraint::Length(5),
        ])
        .split(area);

    draw_active_downloads(f, chunks[0], app);
    draw_queue(f, chunks[1], app);
}

fn draw_active_downloads(f: &mut Frame, area: Rect, app: &AppState) {
    let mut items: Vec<ListItem> = Vec::new();

    for (i, dl) in app.downloads.iter().enumerate() {
        let status_icon = match dl.status {
            crate::tui::app::DownloadStatus::Queued => "[ ]",
            crate::tui::app::DownloadStatus::Downloading => "[>]",
            crate::tui::app::DownloadStatus::PostProcessing => "[*]",
            crate::tui::app::DownloadStatus::Complete => "[x]",
            crate::tui::app::DownloadStatus::Error(_) => "[!]",
        };

        let title_line = Line::from(vec![
            Span::styled(
                format!("{} ", status_icon),
                Style::default().fg(OPENCODE_THEME.accent),
            ),
            Span::styled(
                format!("[{}/{}] {}", i + 1, app.downloads.len(), dl.title),
                Style::default().fg(OPENCODE_THEME.foreground),
            ),
        ]);

        let progress_bar = progress_bar(dl.progress, 40);
        let progress_line = Line::from(vec![
            Span::styled(
                format!("  {} {:.1}%  {}", progress_bar, dl.progress, dl.speed),
                Style::default().fg(OPENCODE_THEME.progress),
            ),
        ]);

        let info_line = Line::from(vec![
            Span::styled(
                format!(
                    "  Platform: {} | Format: {} | ETA: {}",
                    dl.platform, dl.format, dl.eta
                ),
                Style::default().fg(OPENCODE_THEME.subtitle),
            ),
        ]);

        items.push(ListItem::new(vec![title_line, progress_line, info_line]));
    }

    let list = List::new(items).block(
        Block::default()
            .title(" Active Downloads ")
            .title_style(Style::default().fg(OPENCODE_THEME.title).add_modifier(Modifier::BOLD))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(OPENCODE_THEME.border))
            .style(Style::default().bg(OPENCODE_THEME.background)),
    );

    f.render_widget(list, area);
}

fn draw_queue(f: &mut Frame, area: Rect, app: &AppState) {
    let mut items: Vec<ListItem> = Vec::new();

    for (i, dl) in app.downloads.iter().enumerate() {
        if matches!(dl.status, crate::tui::app::DownloadStatus::Queued) {
            items.push(ListItem::new(Line::from(vec![
                Span::styled(
                    format!("  {}. {} ({})", i + 1, dl.title, dl.platform),
                    Style::default().fg(OPENCODE_THEME.foreground),
                ),
            ])));
        }
    }

    let list = List::new(items).block(
        Block::default()
            .title(" Queue ")
            .title_style(Style::default().fg(OPENCODE_THEME.title).add_modifier(Modifier::BOLD))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(OPENCODE_THEME.border))
            .style(Style::default().bg(OPENCODE_THEME.background)),
    );

    f.render_widget(list, area);
}

fn draw_footer(f: &mut Frame, area: Rect) {
    let footer = Paragraph::new(vec![Line::from(vec![
        Span::styled(
            " [a] Add URL    [p] Pause    [c] Cancel    [s] Settings    [q] Quit",
            Style::default().fg(OPENCODE_THEME.subtitle),
        ),
    ])])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(OPENCODE_THEME.border))
            .style(Style::default().bg(OPENCODE_THEME.background)),
    );

    f.render_widget(footer, area);
}
