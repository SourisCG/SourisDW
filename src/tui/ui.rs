use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph, Clear};
use crate::tui::app::{AppState, DownloadStatus, InputMode};
use crate::tui::theme::{OPENCODE_THEME, progress_bar, format_duration};

pub fn draw(f: &mut Frame, app: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(3),
            Constraint::Length(1),
        ])
        .split(f.area());

    draw_header(f, chunks[0], app);
    draw_main_content(f, chunks[1], app);
    draw_footer(f, chunks[2], app);
    draw_status_bar(f, chunks[3], app);

    if app.show_help {
        draw_help_overlay(f, app);
    }

    if app.show_search {
        draw_search_overlay(f, app);
    }
}

fn draw_header(f: &mut Frame, area: Rect, app: &AppState) {
    let active = app.get_active_count();
    let completed = app.get_completed_count();
    let errors = app.get_error_count();

    let status_text = if active > 0 {
        format!(" | {} downloading | {} completed | {} errors", active, completed, errors)
    } else {
        String::new()
    };

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
            Span::styled(
                &status_text,
                Style::default().fg(OPENCODE_THEME.accent),
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

fn draw_main_content(f: &mut Frame, area: Rect, app: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(60),
            Constraint::Percentage(40),
        ])
        .split(area);

    draw_downloads_panel(f, chunks[0], app);
    draw_details_panel(f, chunks[1], app);
}

fn draw_downloads_panel(f: &mut Frame, area: Rect, app: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(5),
            Constraint::Length(8),
        ])
        .split(area);

    draw_active_downloads(f, chunks[0], app);
    draw_queue(f, chunks[1], app);
}

fn draw_active_downloads(f: &mut Frame, area: Rect, app: &AppState) {
    let mut items: Vec<ListItem> = Vec::new();

    for (i, dl) in app.downloads.iter().enumerate() {
        let is_selected = i == app.selected_index;

        let (status_icon, status_color) = match &dl.status {
            DownloadStatus::Queued => ("[ ]", OPENCODE_THEME.accent),
            DownloadStatus::Downloading => ("[>]", OPENCODE_THEME.info),
            DownloadStatus::PostProcessing => ("[*]", OPENCODE_THEME.warning),
            DownloadStatus::Complete => ("[x]", OPENCODE_THEME.success),
            DownloadStatus::Error(_) => ("[!]", OPENCODE_THEME.error),
        };

        let title_style = if is_selected {
            Style::default()
                .fg(OPENCODE_THEME.highlight)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(OPENCODE_THEME.foreground)
        };

        let title_line = Line::from(vec![
            Span::styled(format!("{} ", status_icon), Style::default().fg(status_color)),
            Span::styled(format!("[{}] {}", i + 1, dl.title), title_style),
        ]);

        let progress_width = 30;
        let bar = progress_bar(dl.progress, progress_width);
        let progress_line = Line::from(vec![
            Span::styled(
                format!("  {} {:.1}%  {}", bar, dl.progress, dl.speed),
                Style::default().fg(OPENCODE_THEME.progress),
            ),
        ]);

        let info_line = Line::from(vec![
            Span::styled(
                format!("  {} | {} | ETA: {}", dl.platform, dl.format, dl.eta),
                Style::default().fg(OPENCODE_THEME.subtitle),
            ),
        ]);

        let bg_color = if is_selected {
            Color::Rgb(40, 40, 48)
        } else {
            OPENCODE_THEME.background
        };

        items.push(
            ListItem::new(vec![title_line, progress_line, info_line])
                .style(Style::default().bg(bg_color)),
        );
    }

    if items.is_empty() {
        items.push(ListItem::new(Line::from(vec![
            Span::styled(
                "  No downloads. Press 'a' to add a URL.",
                Style::default().fg(OPENCODE_THEME.subtitle),
            ),
        ])));
    }

    let list = List::new(items).block(
        Block::default()
            .title(" Downloads ")
            .title_style(
                Style::default()
                    .fg(OPENCODE_THEME.title)
                    .add_modifier(Modifier::BOLD),
            )
            .borders(Borders::ALL)
            .border_style(Style::default().fg(OPENCODE_THEME.border))
            .style(Style::default().bg(OPENCODE_THEME.background)),
    );

    f.render_widget(list, area);
}

fn draw_queue(f: &mut Frame, area: Rect, app: &AppState) {
    let mut items: Vec<ListItem> = Vec::new();

    for (i, dl) in app.downloads.iter().enumerate() {
        if matches!(dl.status, DownloadStatus::Queued) {
            items.push(ListItem::new(Line::from(vec![
                Span::styled(
                    format!("  {}. {} ({})", i + 1, dl.title, dl.platform),
                    Style::default().fg(OPENCODE_THEME.foreground),
                ),
            ])));
        }
    }

    if items.is_empty() {
        items.push(ListItem::new(Line::from(vec![
            Span::styled(
                "  Queue empty",
                Style::default().fg(OPENCODE_THEME.subtitle),
            ),
        ])));
    }

    let list = List::new(items).block(
        Block::default()
            .title(" Queue ")
            .title_style(
                Style::default()
                    .fg(OPENCODE_THEME.title)
                    .add_modifier(Modifier::BOLD),
            )
            .borders(Borders::ALL)
            .border_style(Style::default().fg(OPENCODE_THEME.border))
            .style(Style::default().bg(OPENCODE_THEME.background)),
    );

    f.render_widget(list, area);
}

fn draw_details_panel(f: &mut Frame, area: Rect, app: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8),
            Constraint::Min(5),
        ])
        .split(area);

    draw_selected_details(f, chunks[0], app);
    draw_input_area(f, chunks[1], app);
}

fn draw_selected_details(f: &mut Frame, area: Rect, app: &AppState) {
    let details = if let Some(dl) = app.downloads.get(app.selected_index) {
        let status_text = match &dl.status {
            DownloadStatus::Queued => "Queued".to_string(),
            DownloadStatus::Downloading => "Downloading".to_string(),
            DownloadStatus::PostProcessing => "Post-processing".to_string(),
            DownloadStatus::Complete => "Complete".to_string(),
            DownloadStatus::Error(e) => format!("Error: {}", e),
        };

        vec![
            Line::from(vec![
                Span::styled(" Title: ", Style::default().fg(OPENCODE_THEME.accent)),
                Span::styled(&dl.title, Style::default().fg(OPENCODE_THEME.foreground)),
            ]),
            Line::from(vec![
                Span::styled(" Platform: ", Style::default().fg(OPENCODE_THEME.accent)),
                Span::styled(&dl.platform, Style::default().fg(OPENCODE_THEME.foreground)),
            ]),
            Line::from(vec![
                Span::styled(" Status: ", Style::default().fg(OPENCODE_THEME.accent)),
                Span::styled(status_text, Style::default().fg(OPENCODE_THEME.foreground)),
            ]),
            Line::from(vec![
                Span::styled(" Format: ", Style::default().fg(OPENCODE_THEME.accent)),
                Span::styled(&dl.format, Style::default().fg(OPENCODE_THEME.foreground)),
            ]),
            Line::from(vec![
                Span::styled(" Quality: ", Style::default().fg(OPENCODE_THEME.accent)),
                Span::styled(&dl.quality, Style::default().fg(OPENCODE_THEME.foreground)),
            ]),
            Line::from(vec![
                Span::styled(" Speed: ", Style::default().fg(OPENCODE_THEME.accent)),
                Span::styled(&dl.speed, Style::default().fg(OPENCODE_THEME.foreground)),
            ]),
            Line::from(vec![
                Span::styled(" ETA: ", Style::default().fg(OPENCODE_THEME.accent)),
                Span::styled(&dl.eta, Style::default().fg(OPENCODE_THEME.foreground)),
            ]),
        ]
    } else {
        vec![Line::from(vec![Span::styled(
            "  Select a download to see details",
            Style::default().fg(OPENCODE_THEME.subtitle),
        )])]
    };

    let details_block = Paragraph::new(details).block(
        Block::default()
            .title(" Details ")
            .title_style(
                Style::default()
                    .fg(OPENCODE_THEME.title)
                    .add_modifier(Modifier::BOLD),
            )
            .borders(Borders::ALL)
            .border_style(Style::default().fg(OPENCODE_THEME.border))
            .style(Style::default().bg(OPENCODE_THEME.background)),
    );

    f.render_widget(details_block, area);
}

fn draw_input_area(f: &mut Frame, area: Rect, app: &AppState) {
    let (title, content) = match app.input_mode {
        InputMode::Normal => (
            " Input ",
            Line::from(vec![Span::styled(
                "  Press 'a' to add URL, '/' to search",
                Style::default().fg(OPENCODE_THEME.subtitle),
            )]),
        ),
        InputMode::Input => (
            " Enter URL ",
            Line::from(vec![
                Span::styled("  > ", Style::default().fg(OPENCODE_THEME.accent)),
                Span::styled(&app.input_buffer, Style::default().fg(OPENCODE_THEME.foreground)),
                Span::styled("█", Style::default().fg(OPENCODE_THEME.foreground)),
            ]),
        ),
        InputMode::Search => (
            " Search ",
            Line::from(vec![
                Span::styled("  > ", Style::default().fg(OPENCODE_THEME.accent)),
                Span::styled(&app.input_buffer, Style::default().fg(OPENCODE_THEME.foreground)),
                Span::styled("█", Style::default().fg(OPENCODE_THEME.foreground)),
            ]),
        ),
    };

    let input_block = Paragraph::new(content).block(
        Block::default()
            .title(title)
            .title_style(
                Style::default()
                    .fg(OPENCODE_THEME.title)
                    .add_modifier(Modifier::BOLD),
            )
            .borders(Borders::ALL)
            .border_style(Style::default().fg(OPENCODE_THEME.border))
            .style(Style::default().bg(OPENCODE_THEME.background)),
    );

    f.render_widget(input_block, area);
}

fn draw_footer(f: &mut Frame, area: Rect, app: &AppState) {
    let shortcuts = match app.input_mode {
        InputMode::Normal => " [a] Add URL  [/] Search  [Enter] Download  [p] Pause  [c] Cancel  [s] Settings  [h] Help  [q] Quit ",
        InputMode::Input => " [Enter] Confirm  [Esc] Cancel ",
        InputMode::Search => " [Enter] Search  [Esc] Cancel ",
    };

    let footer = Paragraph::new(vec![Line::from(vec![Span::styled(
        shortcuts,
        Style::default().fg(OPENCODE_THEME.subtitle),
    )])])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(OPENCODE_THEME.border))
            .style(Style::default().bg(OPENCODE_THEME.background)),
    );

    f.render_widget(footer, area);
}

fn draw_status_bar(f: &mut Frame, area: Rect, app: &AppState) {
    let status = app
        .status_message
        .as_deref()
        .unwrap_or("Ready");

    let status_bar = Paragraph::new(Line::from(vec![Span::styled(
        format!(" {} ", status),
        Style::default().fg(OPENCODE_THEME.foreground).bg(OPENCODE_THEME.accent),
    )]));

    f.render_widget(status_bar, area);
}

fn draw_help_overlay(f: &mut Frame, _app: &AppState) {
    let area = centered_rect(60, 70, f.area());

    f.render_widget(Clear, area);

    let help_text = vec![
        Line::from(vec![Span::styled(
            " SourisDW - Help",
            Style::default()
                .fg(OPENCODE_THEME.title)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![Span::styled(
            " Navigation",
            Style::default().fg(OPENCODE_THEME.info),
        )]),
        Line::from("  j/Down    Move down"),
        Line::from("  k/Up      Move up"),
        Line::from("  g/Home    Go to first"),
        Line::from("  G/End     Go to last"),
        Line::from(""),
        Line::from(vec![Span::styled(
            " Actions",
            Style::default().fg(OPENCODE_THEME.info),
        )]),
        Line::from("  a         Add URL"),
        Line::from("  /         Search"),
        Line::from("  Enter     Download selected"),
        Line::from("  d         Delete selected"),
        Line::from("  p         Pause/Resume"),
        Line::from("  c         Cancel"),
        Line::from(""),
        Line::from(vec![Span::styled(
            " Views",
            Style::default().fg(OPENCODE_THEME.info),
        )]),
        Line::from("  s         Settings"),
        Line::from("  h/?       Help"),
        Line::from("  q/Esc     Quit"),
    ];

    let help_block = Paragraph::new(help_text)
        .block(
            Block::default()
                .title(" Help ")
                .title_style(
                    Style::default()
                        .fg(OPENCODE_THEME.title)
                        .add_modifier(Modifier::BOLD),
                )
                .borders(Borders::ALL)
                .border_style(Style::default().fg(OPENCODE_THEME.border))
                .style(Style::default().bg(OPENCODE_THEME.background)),
        )
        .style(Style::default().bg(OPENCODE_THEME.background));

    f.render_widget(help_block, area);
}

fn draw_search_overlay(f: &mut Frame, app: &AppState) {
    let area = centered_rect(70, 60, f.area());

    f.render_widget(Clear, area);

    let mut items: Vec<ListItem> = Vec::new();

    items.push(ListItem::new(Line::from(vec![
        Span::styled(
            format!("  Query: {}", app.input_buffer),
            Style::default().fg(OPENCODE_THEME.info),
        ),
    ])));

    items.push(ListItem::new(Line::from("")));

    if app.search_results.is_empty() {
        items.push(ListItem::new(Line::from(vec![Span::styled(
            "  No results. Type and press Enter to search.",
            Style::default().fg(OPENCODE_THEME.subtitle),
        )])));
    } else {
        for (i, result) in app.search_results.iter().enumerate() {
            let style = if result.selected {
                Style::default()
                    .fg(OPENCODE_THEME.highlight)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(OPENCODE_THEME.foreground)
            };

            let duration_str = result
                .duration
                .map(|d| format_duration(d))
                .unwrap_or_default();

            items.push(ListItem::new(Line::from(vec![
                Span::styled(
                    format!("  {}. {} ", i + 1, result.title),
                    style,
                ),
                Span::styled(
                    format!("[{}] [{}]", result.platform, duration_str),
                    Style::default().fg(OPENCODE_THEME.subtitle),
                ),
            ])));
        }
    }

    let search_block = List::new(items).block(
        Block::default()
            .title(" Search Results ")
            .title_style(
                Style::default()
                    .fg(OPENCODE_THEME.title)
                    .add_modifier(Modifier::BOLD),
            )
            .borders(Borders::ALL)
            .border_style(Style::default().fg(OPENCODE_THEME.border))
            .style(Style::default().bg(OPENCODE_THEME.background)),
    );

    f.render_widget(search_block, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
