use crate::tui::app::{AppState, DownloadStatus, InputMode, SETTINGS_OPTIONS};
use crate::tui::theme::{format_duration, progress_bar, SYNTHWAVE84_THEME};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap};
use ratatui::Frame;

const MIN_WIDTH: u16 = 60;
const MIN_HEIGHT: u16 = 20;

#[allow(dead_code)]
fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else if max <= 3 {
        s.chars().take(max).collect()
    } else {
        format!("{}...", &s[..max.saturating_sub(3)])
    }
}

pub fn draw(f: &mut Frame, app: &AppState) {
    let area = f.area();

    if area.width < MIN_WIDTH || area.height < MIN_HEIGHT {
        draw_too_small(f, area);
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(3),
            Constraint::Length(1),
        ])
        .split(area);

    draw_header(f, chunks[0], app);
    draw_main_content(f, chunks[1], app);
    draw_footer(f, chunks[2], app);
    draw_status_bar(f, chunks[3], app);

    if app.show_error_popup {
        draw_error_overlay(f, app);
    }

    if app.show_help {
        draw_help_overlay(f, app);
    }

    if app.show_search {
        draw_search_overlay(f, app);
    }

    if app.show_settings {
        draw_settings_overlay(f, app);
    }
}

fn draw_too_small(f: &mut Frame, area: Rect) {
    let text = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            " SourisDW",
            Style::default()
                .fg(SYNTHWAVE84_THEME.title)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![Span::styled(
            " Terminal too small",
            Style::default()
                .fg(SYNTHWAVE84_THEME.error)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            format!(
                " Need at least {}x{} (current: {}x{})",
                MIN_WIDTH, MIN_HEIGHT, area.width, area.height
            ),
            Style::default().fg(SYNTHWAVE84_THEME.subtitle),
        )]),
    ];
    let block = Paragraph::new(text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(SYNTHWAVE84_THEME.border))
            .style(Style::default().bg(SYNTHWAVE84_THEME.background)),
    );
    f.render_widget(block, area);
}

fn draw_header(f: &mut Frame, area: Rect, app: &AppState) {
    let active = app.get_active_count();
    let completed = app.get_completed_count();
    let errors = app.get_error_count();

    let mut spans = vec![
        Span::styled(
            " SourisDW",
            Style::default()
                .fg(SYNTHWAVE84_THEME.title)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!(" v{} ", env!("CARGO_PKG_VERSION")),
            Style::default().fg(SYNTHWAVE84_THEME.subtitle),
        ),
    ];

    if active > 0 || completed > 0 || errors > 0 {
        spans.push(Span::styled(
            " -- ",
            Style::default().fg(SYNTHWAVE84_THEME.border),
        ));
        if active > 0 {
            spans.push(Span::styled(
                format!("{} downloading", active),
                Style::default().fg(SYNTHWAVE84_THEME.info),
            ));
            spans.push(Span::styled(
                " | ",
                Style::default().fg(SYNTHWAVE84_THEME.border),
            ));
        }
        if completed > 0 {
            spans.push(Span::styled(
                format!("{} completed", completed),
                Style::default().fg(SYNTHWAVE84_THEME.success),
            ));
            spans.push(Span::styled(
                " | ",
                Style::default().fg(SYNTHWAVE84_THEME.border),
            ));
        }
        if errors > 0 {
            spans.push(Span::styled(
                format!("{} errors", errors),
                Style::default().fg(SYNTHWAVE84_THEME.error),
            ));
        }
    }

    let header = Paragraph::new(vec![Line::from(spans)]).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(SYNTHWAVE84_THEME.border))
            .style(Style::default().bg(SYNTHWAVE84_THEME.background)),
    );

    f.render_widget(header, area);
}

fn draw_main_content(f: &mut Frame, area: Rect, app: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(area);

    draw_downloads_panel(f, chunks[0], app);
    draw_details_panel(f, chunks[1], app);
}

fn draw_downloads_panel(f: &mut Frame, area: Rect, app: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(5), Constraint::Length(8)])
        .split(area);

    draw_active_downloads(f, chunks[0], app);
    draw_queue(f, chunks[1], app);
}

fn draw_active_downloads(f: &mut Frame, area: Rect, app: &AppState) {
    let mut items: Vec<ListItem> = Vec::new();

    for (i, dl) in app.downloads.iter().enumerate() {
        let is_selected = i == app.selected_index;

        let (status_icon, status_color) = match &dl.status {
            DownloadStatus::Queued => ("\u{25cb}", SYNTHWAVE84_THEME.subtitle),
            DownloadStatus::Resolving => ("\u{25d2}", SYNTHWAVE84_THEME.info),
            DownloadStatus::Downloading => ("\u{25cf}", SYNTHWAVE84_THEME.info),
            DownloadStatus::PostProcessing => ("\u{25d0}", SYNTHWAVE84_THEME.warning),
            DownloadStatus::Complete => ("\u{2713}", SYNTHWAVE84_THEME.success),
            DownloadStatus::Error(_) => ("\u{2717}", SYNTHWAVE84_THEME.error),
        };

        let title_style = if is_selected {
            Style::default()
                .fg(SYNTHWAVE84_THEME.highlight)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(SYNTHWAVE84_THEME.foreground)
        };

        let title_line = Line::from(vec![
            Span::styled(
                format!(" {} ", status_icon),
                Style::default().fg(status_color),
            ),
            Span::styled(format!("{} ", dl.title), title_style),
            Span::styled(
                format!("[{}]", dl.format),
                Style::default().fg(SYNTHWAVE84_THEME.subtitle),
            ),
        ]);

        let progress_width = 25;
        let bar = progress_bar(dl.progress, progress_width);
        let progress_line = Line::from(vec![
            Span::styled(
                format!("   {} ", bar),
                Style::default().fg(SYNTHWAVE84_THEME.progress),
            ),
            Span::styled(
                format!("{:.0}%", dl.progress),
                Style::default()
                    .fg(SYNTHWAVE84_THEME.foreground)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("  {} ", dl.speed),
                Style::default().fg(SYNTHWAVE84_THEME.subtitle),
            ),
            Span::styled(
                format!("ETA {}", dl.eta),
                Style::default().fg(SYNTHWAVE84_THEME.subtitle),
            ),
        ]);

        let bg_color = if is_selected {
            Color::Rgb(40, 40, 48)
        } else {
            SYNTHWAVE84_THEME.background
        };

        items.push(
            ListItem::new(vec![title_line, progress_line]).style(Style::default().bg(bg_color)),
        );
    }

    if items.is_empty() {
        items.push(ListItem::new(Line::from(vec![Span::styled(
            "  No downloads yet",
            Style::default().fg(SYNTHWAVE84_THEME.subtitle),
        )])));
    }

    let list = List::new(items).block(
        Block::default()
            .title(" Downloads ")
            .title_style(
                Style::default()
                    .fg(SYNTHWAVE84_THEME.title)
                    .add_modifier(Modifier::BOLD),
            )
            .borders(Borders::ALL)
            .border_style(Style::default().fg(SYNTHWAVE84_THEME.border))
            .style(Style::default().bg(SYNTHWAVE84_THEME.background)),
    );

    f.render_widget(list, area);
}

fn draw_queue(f: &mut Frame, area: Rect, app: &AppState) {
    let mut items: Vec<ListItem> = Vec::new();

    for (i, dl) in app.downloads.iter().enumerate() {
        if matches!(dl.status, DownloadStatus::Queued) {
            items.push(ListItem::new(Line::from(vec![
                Span::styled(
                    format!(" {} ", i + 1),
                    Style::default()
                        .fg(SYNTHWAVE84_THEME.accent)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    dl.title.clone(),
                    Style::default().fg(SYNTHWAVE84_THEME.foreground),
                ),
                Span::styled(
                    format!(" ({})", dl.platform),
                    Style::default().fg(SYNTHWAVE84_THEME.subtitle),
                ),
            ])));
        }
    }

    if items.is_empty() {
        items.push(ListItem::new(Line::from(vec![Span::styled(
            "  Queue empty",
            Style::default().fg(SYNTHWAVE84_THEME.subtitle),
        )])));
    }

    let list = List::new(items).block(
        Block::default()
            .title(" Queue ")
            .title_style(
                Style::default()
                    .fg(SYNTHWAVE84_THEME.title)
                    .add_modifier(Modifier::BOLD),
            )
            .borders(Borders::ALL)
            .border_style(Style::default().fg(SYNTHWAVE84_THEME.border))
            .style(Style::default().bg(SYNTHWAVE84_THEME.background)),
    );

    f.render_widget(list, area);
}

fn draw_details_panel(f: &mut Frame, area: Rect, app: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(8), Constraint::Min(5)])
        .split(area);

    draw_selected_details(f, chunks[0], app);
    draw_input_area(f, chunks[1], app);
}

fn draw_selected_details(f: &mut Frame, area: Rect, app: &AppState) {
    let details = if let Some(dl) = app.downloads.get(app.selected_index) {
        let (status_text, status_color) = match &dl.status {
            DownloadStatus::Queued => ("Queued", SYNTHWAVE84_THEME.subtitle),
            DownloadStatus::Resolving => ("Resolving", SYNTHWAVE84_THEME.info),
            DownloadStatus::Downloading => ("Downloading", SYNTHWAVE84_THEME.info),
            DownloadStatus::PostProcessing => ("Processing", SYNTHWAVE84_THEME.warning),
            DownloadStatus::Complete => ("Complete", SYNTHWAVE84_THEME.success),
            DownloadStatus::Error(e) => (e.as_str(), SYNTHWAVE84_THEME.error),
        };

        let mut lines = vec![
            Line::from(vec![
                Span::styled("  Title    ", Style::default().fg(SYNTHWAVE84_THEME.accent)),
                Span::styled(&dl.title, Style::default().fg(SYNTHWAVE84_THEME.foreground)),
            ]),
            Line::from(vec![
                Span::styled("  Platform ", Style::default().fg(SYNTHWAVE84_THEME.accent)),
                Span::styled(
                    &dl.platform,
                    Style::default().fg(SYNTHWAVE84_THEME.foreground),
                ),
            ]),
            Line::from(vec![
                Span::styled("  Author   ", Style::default().fg(SYNTHWAVE84_THEME.accent)),
                Span::styled(
                    dl.author.as_deref().unwrap_or("Not available"),
                    Style::default().fg(SYNTHWAVE84_THEME.foreground),
                ),
            ]),
            Line::from(vec![
                Span::styled("  Status   ", Style::default().fg(SYNTHWAVE84_THEME.accent)),
                Span::styled(status_text, Style::default().fg(status_color)),
            ]),
            Line::from(vec![
                Span::styled("  Format   ", Style::default().fg(SYNTHWAVE84_THEME.accent)),
                Span::styled(
                    format!("{} / {}", dl.format, dl.quality),
                    Style::default().fg(SYNTHWAVE84_THEME.foreground),
                ),
            ]),
        ];

        if !dl.speed.is_empty() {
            lines.push(Line::from(vec![
                Span::styled("  Speed    ", Style::default().fg(SYNTHWAVE84_THEME.accent)),
                Span::styled(&dl.speed, Style::default().fg(SYNTHWAVE84_THEME.foreground)),
            ]));
        }
        if !dl.eta.is_empty() {
            lines.push(Line::from(vec![
                Span::styled("  ETA      ", Style::default().fg(SYNTHWAVE84_THEME.accent)),
                Span::styled(&dl.eta, Style::default().fg(SYNTHWAVE84_THEME.foreground)),
            ]));
        }
        if let Some(ref path) = dl.path {
            lines.push(Line::from(vec![
                Span::styled("  Path     ", Style::default().fg(SYNTHWAVE84_THEME.accent)),
                Span::styled(
                    path.as_str(),
                    Style::default().fg(SYNTHWAVE84_THEME.foreground),
                ),
            ]));
        }

        lines
    } else {
        vec![Line::from(vec![Span::styled(
            "  No download selected",
            Style::default().fg(SYNTHWAVE84_THEME.subtitle),
        )])]
    };

    let details_block = Paragraph::new(details).block(
        Block::default()
            .title(" Details ")
            .title_style(
                Style::default()
                    .fg(SYNTHWAVE84_THEME.title)
                    .add_modifier(Modifier::BOLD),
            )
            .borders(Borders::ALL)
            .border_style(Style::default().fg(SYNTHWAVE84_THEME.border))
            .style(Style::default().bg(SYNTHWAVE84_THEME.background)),
    );

    f.render_widget(details_block, area);
}

fn draw_input_area(f: &mut Frame, area: Rect, app: &AppState) {
    let (title, content) = match app.input_mode {
        InputMode::Normal => (
            " Input ",
            Line::from(vec![
                Span::styled(
                    " a",
                    Style::default()
                        .fg(SYNTHWAVE84_THEME.highlight)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    " add URL  ",
                    Style::default().fg(SYNTHWAVE84_THEME.subtitle),
                ),
                Span::styled(
                    "Ctrl+F",
                    Style::default()
                        .fg(SYNTHWAVE84_THEME.highlight)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(" search", Style::default().fg(SYNTHWAVE84_THEME.subtitle)),
            ]),
        ),
        InputMode::Input => (
            " Enter URL ",
            Line::from(vec![
                Span::styled(" > ", Style::default().fg(SYNTHWAVE84_THEME.accent)),
                Span::styled(
                    &app.input_buffer,
                    Style::default().fg(SYNTHWAVE84_THEME.foreground),
                ),
                Span::styled(
                    "\u{2588}",
                    Style::default().fg(SYNTHWAVE84_THEME.foreground),
                ),
            ]),
        ),
        InputMode::Search => (
            " Search ",
            Line::from(vec![
                Span::styled(" > ", Style::default().fg(SYNTHWAVE84_THEME.accent)),
                Span::styled(
                    &app.input_buffer,
                    Style::default().fg(SYNTHWAVE84_THEME.foreground),
                ),
                Span::styled(
                    "\u{2588}",
                    Style::default().fg(SYNTHWAVE84_THEME.foreground),
                ),
            ]),
        ),
    };

    let input_block = Paragraph::new(content).block(
        Block::default()
            .title(title)
            .title_style(
                Style::default()
                    .fg(SYNTHWAVE84_THEME.title)
                    .add_modifier(Modifier::BOLD),
            )
            .borders(Borders::ALL)
            .border_style(Style::default().fg(SYNTHWAVE84_THEME.border))
            .style(Style::default().bg(SYNTHWAVE84_THEME.background)),
    );

    f.render_widget(input_block, area);
}

fn draw_footer(f: &mut Frame, area: Rect, app: &AppState) {
    let shortcuts = if app.waiting_for_quit {
        " Esc/q: Quit   any key: Cancel "
    } else if app.show_help {
        " Esc/q: Back "
    } else if app.show_settings {
        " j/k: Navigate   Enter: Change   Esc/q: Back "
    } else if app.show_search {
        match app.input_mode {
            InputMode::Search => " Enter: Search   j/k: Navigate   Esc/q: Back ",
            _ => " Esc/q: Back ",
        }
    } else {
        match app.input_mode {
            InputMode::Normal => {
                " a: Add URL   Ctrl+F: Search   y: Copy   h: Help   s: Settings   q/Esc: Back "
            }
            InputMode::Input => " Enter: Confirm   Esc/q: Back ",
            InputMode::Search => " Enter: Search   Esc/q: Back ",
        }
    };

    let footer = Paragraph::new(vec![Line::from(vec![Span::styled(
        shortcuts,
        Style::default().fg(SYNTHWAVE84_THEME.subtitle),
    )])])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(SYNTHWAVE84_THEME.border))
            .style(Style::default().bg(SYNTHWAVE84_THEME.background)),
    );

    f.render_widget(footer, area);
}

fn draw_status_bar(f: &mut Frame, area: Rect, app: &AppState) {
    let status = app.status_message.as_deref().unwrap_or("Ready");
    let has_errors = app.get_error_count() > 0;
    let has_active = app.get_active_count() > 0;

    let bg = if has_errors {
        SYNTHWAVE84_THEME.error
    } else if has_active {
        SYNTHWAVE84_THEME.info
    } else {
        SYNTHWAVE84_THEME.accent
    };

    let status_bar = Paragraph::new(Line::from(vec![Span::styled(
        format!(" {} ", status),
        Style::default()
            .fg(SYNTHWAVE84_THEME.background)
            .bg(bg)
            .add_modifier(Modifier::BOLD),
    )]));

    f.render_widget(status_bar, area);
}

fn draw_error_overlay(f: &mut Frame, app: &AppState) {
    let area = centered_rect(65, 50, f.area());

    f.render_widget(Clear, area);

    // Wrap long lines to fit the popup (accounting for 2-char border)
    let wrap_at = area.width.saturating_sub(2).max(40) as usize;

    let wrapped_msg = app
        .error_message
        .as_ref()
        .map(|msg| textwrap::fill(msg, wrap_at));
    let error_text: Vec<Line> = match &wrapped_msg {
        Some(wrapped) => wrapped
            .lines()
            .map(|l| {
                Line::from(vec![Span::styled(
                    l.to_string(),
                    Style::default().fg(SYNTHWAVE84_THEME.foreground),
                )])
            })
            .collect(),
        None => vec![Line::from(vec![Span::styled(
            "No error details",
            Style::default().fg(SYNTHWAVE84_THEME.subtitle),
        )])],
    };

    let mut lines = vec![
        Line::from(vec![Span::styled(
            " Error",
            Style::default()
                .fg(SYNTHWAVE84_THEME.error)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
    ];
    lines.extend(error_text);
    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled(" c", Style::default().fg(SYNTHWAVE84_THEME.highlight)),
        Span::styled(" Copy   ", Style::default().fg(SYNTHWAVE84_THEME.subtitle)),
        Span::styled("Esc", Style::default().fg(SYNTHWAVE84_THEME.highlight)),
        Span::styled(" Close", Style::default().fg(SYNTHWAVE84_THEME.subtitle)),
    ]));

    let error_block = Paragraph::new(lines)
        .block(
            Block::default()
                .title(" Error ")
                .title_style(
                    Style::default()
                        .fg(SYNTHWAVE84_THEME.error)
                        .add_modifier(Modifier::BOLD),
                )
                .borders(Borders::ALL)
                .border_style(Style::default().fg(SYNTHWAVE84_THEME.error))
                .style(Style::default().bg(SYNTHWAVE84_THEME.background)),
        )
        .wrap(Wrap { trim: false })
        .style(Style::default().bg(SYNTHWAVE84_THEME.background));

    f.render_widget(error_block, area);
}

fn draw_help_overlay(f: &mut Frame, _app: &AppState) {
    let area = centered_rect(60, 70, f.area());

    f.render_widget(Clear, area);

    let help_text = vec![
        Line::from(vec![Span::styled(
            " SourisDW - Help",
            Style::default()
                .fg(SYNTHWAVE84_THEME.title)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![Span::styled(
            " Navigation",
            Style::default()
                .fg(SYNTHWAVE84_THEME.info)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::styled(
                "  j / Down",
                Style::default().fg(SYNTHWAVE84_THEME.highlight),
            ),
            Span::styled(
                "     Move down",
                Style::default().fg(SYNTHWAVE84_THEME.foreground),
            ),
        ]),
        Line::from(vec![
            Span::styled("  k / Up", Style::default().fg(SYNTHWAVE84_THEME.highlight)),
            Span::styled(
                "       Move up",
                Style::default().fg(SYNTHWAVE84_THEME.foreground),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "  g / Home",
                Style::default().fg(SYNTHWAVE84_THEME.highlight),
            ),
            Span::styled(
                "   Go to first",
                Style::default().fg(SYNTHWAVE84_THEME.foreground),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "  G / End",
                Style::default().fg(SYNTHWAVE84_THEME.highlight),
            ),
            Span::styled(
                "    Go to last",
                Style::default().fg(SYNTHWAVE84_THEME.foreground),
            ),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            " Actions",
            Style::default()
                .fg(SYNTHWAVE84_THEME.info)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::styled("  a", Style::default().fg(SYNTHWAVE84_THEME.highlight)),
            Span::styled(
                "         Add URL",
                Style::default().fg(SYNTHWAVE84_THEME.foreground),
            ),
        ]),
        Line::from(vec![
            Span::styled("  Ctrl+F", Style::default().fg(SYNTHWAVE84_THEME.highlight)),
            Span::styled(
                "     Search",
                Style::default().fg(SYNTHWAVE84_THEME.foreground),
            ),
        ]),
        Line::from(vec![
            Span::styled("  Enter", Style::default().fg(SYNTHWAVE84_THEME.highlight)),
            Span::styled(
                "     Download selected",
                Style::default().fg(SYNTHWAVE84_THEME.foreground),
            ),
        ]),
        Line::from(vec![
            Span::styled("  y", Style::default().fg(SYNTHWAVE84_THEME.highlight)),
            Span::styled(
                "         Copy URL",
                Style::default().fg(SYNTHWAVE84_THEME.foreground),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "  d / Del",
                Style::default().fg(SYNTHWAVE84_THEME.highlight),
            ),
            Span::styled(
                "   Delete",
                Style::default().fg(SYNTHWAVE84_THEME.foreground),
            ),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            " Views",
            Style::default()
                .fg(SYNTHWAVE84_THEME.info)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::styled("  s", Style::default().fg(SYNTHWAVE84_THEME.highlight)),
            Span::styled(
                "         Settings",
                Style::default().fg(SYNTHWAVE84_THEME.foreground),
            ),
        ]),
        Line::from(vec![
            Span::styled("  h / ?", Style::default().fg(SYNTHWAVE84_THEME.highlight)),
            Span::styled(
                "     Help",
                Style::default().fg(SYNTHWAVE84_THEME.foreground),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "  q / Esc",
                Style::default().fg(SYNTHWAVE84_THEME.highlight),
            ),
            Span::styled(
                "   Back / Quit",
                Style::default().fg(SYNTHWAVE84_THEME.foreground),
            ),
        ]),
        Line::from(vec![
            Span::styled("  Ctrl+C", Style::default().fg(SYNTHWAVE84_THEME.highlight)),
            Span::styled(
                "    Force quit",
                Style::default().fg(SYNTHWAVE84_THEME.foreground),
            ),
        ]),
    ];

    let help_block = Paragraph::new(help_text)
        .block(
            Block::default()
                .title(" Help ")
                .title_style(
                    Style::default()
                        .fg(SYNTHWAVE84_THEME.title)
                        .add_modifier(Modifier::BOLD),
                )
                .borders(Borders::ALL)
                .border_style(Style::default().fg(SYNTHWAVE84_THEME.border))
                .style(Style::default().bg(SYNTHWAVE84_THEME.background)),
        )
        .style(Style::default().bg(SYNTHWAVE84_THEME.background));

    f.render_widget(help_block, area);
}

fn draw_settings_overlay(f: &mut Frame, app: &AppState) {
    let area = centered_rect(50, 70, f.area());

    f.render_widget(Clear, area);

    let visible_height = area.height.saturating_sub(2) as usize;

    let all_items: Vec<ListItem> = SETTINGS_OPTIONS
        .iter()
        .enumerate()
        .map(|(i, option)| {
            let is_selected = i == app.settings_index;
            let value = app.get_setting_value(i);

            let (option_style, value_style) = if is_selected {
                (
                    Style::default()
                        .fg(SYNTHWAVE84_THEME.highlight)
                        .add_modifier(Modifier::BOLD),
                    Style::default()
                        .fg(SYNTHWAVE84_THEME.info)
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                (
                    Style::default().fg(SYNTHWAVE84_THEME.foreground),
                    Style::default().fg(SYNTHWAVE84_THEME.subtitle),
                )
            };

            let prefix = if is_selected { " > " } else { "   " };

            ListItem::new(Line::from(vec![
                Span::styled(format!("{}{} ", prefix, option), option_style),
                Span::styled(format!("[{}]", value), value_style),
            ]))
        })
        .collect();

    let start = app.settings_scroll_offset.min(all_items.len());
    let end = (start + visible_height).min(all_items.len());
    let visible_items = &all_items[start..end];

    let settings_block = List::new(visible_items.to_vec()).block(
        Block::default()
            .title(format!(
                " Settings {}/{} ",
                app.settings_index + 1,
                SETTINGS_OPTIONS.len()
            ))
            .title_style(
                Style::default()
                    .fg(SYNTHWAVE84_THEME.title)
                    .add_modifier(Modifier::BOLD),
            )
            .borders(Borders::ALL)
            .border_style(Style::default().fg(SYNTHWAVE84_THEME.border))
            .style(Style::default().bg(SYNTHWAVE84_THEME.background)),
    );

    f.render_widget(settings_block, area);
}

fn draw_search_overlay(f: &mut Frame, app: &AppState) {
    let area = centered_rect(70, 60, f.area());

    f.render_widget(Clear, area);

    let mut items: Vec<ListItem> = Vec::new();

    items.push(ListItem::new(Line::from(vec![
        Span::styled(
            " Query: ",
            Style::default()
                .fg(SYNTHWAVE84_THEME.accent)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            &app.input_buffer,
            Style::default()
                .fg(SYNTHWAVE84_THEME.foreground)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            "\u{2588}",
            Style::default().fg(SYNTHWAVE84_THEME.foreground),
        ),
    ])));

    items.push(ListItem::new(Line::from("")));

    if app.search_results.is_empty() {
        items.push(ListItem::new(Line::from(vec![Span::styled(
            "  Type and press Enter to search",
            Style::default().fg(SYNTHWAVE84_THEME.subtitle),
        )])));
    } else {
        for (i, result) in app.search_results.iter().enumerate() {
            let is_selected = i == app.search_index;
            let (style, prefix) = if is_selected {
                (
                    Style::default()
                        .fg(SYNTHWAVE84_THEME.highlight)
                        .add_modifier(Modifier::BOLD),
                    " > ",
                )
            } else {
                (Style::default().fg(SYNTHWAVE84_THEME.foreground), "   ")
            };

            let duration_str = result.duration.map(format_duration).unwrap_or_default();

            items.push(ListItem::new(Line::from(vec![
                Span::styled(format!("{}{}. ", prefix, i + 1), style),
                Span::styled(result.title.clone(), style),
                Span::styled(
                    format!(" [{}]", duration_str),
                    Style::default().fg(SYNTHWAVE84_THEME.subtitle),
                ),
            ])));
        }
    }

    let search_block = List::new(items).block(
        Block::default()
            .title(format!(" Search ({} results) ", app.search_results.len()))
            .title_style(
                Style::default()
                    .fg(SYNTHWAVE84_THEME.title)
                    .add_modifier(Modifier::BOLD),
            )
            .borders(Borders::ALL)
            .border_style(Style::default().fg(SYNTHWAVE84_THEME.border))
            .style(Style::default().bg(SYNTHWAVE84_THEME.background)),
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
