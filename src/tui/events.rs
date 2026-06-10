use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::time::Duration;
use crate::error::Result;

#[derive(Debug, Clone)]
pub enum AppEvent {
    Key(KeyEvent),
    Tick,
    Quit,
}

pub fn poll_event(timeout: Duration) -> Result<Option<AppEvent>> {
    if event::poll(timeout).map_err(|e| crate::error::SourisError::Other(e.into()))? {
        if let Event::Key(key) = event::read().map_err(|e| crate::error::SourisError::Other(e.into()))? {
            if key.kind == KeyEventKind::Press {
                return Ok(Some(AppEvent::Key(key)));
            }
        }
    }
    Ok(Some(AppEvent::Tick))
}

pub fn handle_key_event(key: KeyEvent) -> Option<Action> {
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => Some(Action::Quit),
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => Some(Action::Quit),
        KeyCode::Char('a') => Some(Action::AddUrl),
        KeyCode::Char('/') => Some(Action::Search),
        KeyCode::Char('h') | KeyCode::Char('?') => Some(Action::Help),
        KeyCode::Char('s') => Some(Action::Settings),
        KeyCode::Char('j') | KeyCode::Down => Some(Action::MoveDown),
        KeyCode::Char('k') | KeyCode::Up => Some(Action::MoveUp),
        KeyCode::Char('g') | KeyCode::Home => Some(Action::MoveFirst),
        KeyCode::Char('G') | KeyCode::End => Some(Action::MoveLast),
        KeyCode::Enter => Some(Action::Confirm),
        KeyCode::Char('d') | KeyCode::Delete => Some(Action::Delete),
        KeyCode::Char('p') => Some(Action::Pause),
        KeyCode::Char('c') => Some(Action::Cancel),
        _ => None,
    }
}

#[derive(Debug, Clone)]
pub enum Action {
    Quit,
    AddUrl,
    Search,
    Help,
    Settings,
    MoveDown,
    MoveUp,
    MoveFirst,
    MoveLast,
    Confirm,
    Delete,
    Pause,
    Cancel,
}
