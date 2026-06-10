use crate::error::Result;
use crate::tui::app::InputMode;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::time::Duration;

#[derive(Debug, Clone)]
pub enum AppEvent {
    Key(KeyEvent),
    Tick,
    Quit,
}

pub fn poll_event(timeout: Duration) -> Result<Option<AppEvent>> {
    if event::poll(timeout).map_err(|e| crate::error::SourisError::Other(e.into()))? {
        if let Event::Key(key) =
            event::read().map_err(|e| crate::error::SourisError::Other(e.into()))?
        {
            if key.kind == KeyEventKind::Press {
                return Ok(Some(AppEvent::Key(key)));
            }
        }
    }
    Ok(Some(AppEvent::Tick))
}

pub fn handle_key_event(key: KeyEvent, mode: &InputMode) -> Option<Action> {
    match mode {
        InputMode::Normal => handle_normal_key(key),
        InputMode::Input => handle_input_key(key),
        InputMode::Search => handle_search_key(key),
    }
}

fn handle_normal_key(key: KeyEvent) -> Option<Action> {
    match key.code {
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            Some(Action::ForceQuit)
        }
        KeyCode::Char('f') if key.modifiers.contains(KeyModifiers::CONTROL) => Some(Action::Search),
        KeyCode::Esc | KeyCode::Char('q') => Some(Action::Back),
        KeyCode::Char('a') => Some(Action::AddUrl),
        KeyCode::Char('h') | KeyCode::Char('?') => Some(Action::Help),
        KeyCode::Char('s') => Some(Action::Settings),
        KeyCode::Char('y') => Some(Action::CopyUrl),
        KeyCode::Char('j') | KeyCode::Down => Some(Action::MoveDown),
        KeyCode::Char('k') | KeyCode::Up => Some(Action::MoveUp),
        KeyCode::Char('g') | KeyCode::Home => Some(Action::MoveFirst),
        KeyCode::Char('G') | KeyCode::End => Some(Action::MoveLast),
        KeyCode::Enter => Some(Action::Confirm),
        KeyCode::Char('d') | KeyCode::Delete => Some(Action::Delete),
        KeyCode::Char('p') => Some(Action::Pause),
        _ => None,
    }
}

fn handle_input_key(key: KeyEvent) -> Option<Action> {
    match key.code {
        KeyCode::Esc => Some(Action::Back),
        KeyCode::Enter => Some(Action::Confirm),
        KeyCode::Backspace => Some(Action::DeleteChar),
        KeyCode::Char(c) => Some(Action::CharInput(c)),
        _ => None,
    }
}

fn handle_search_key(key: KeyEvent) -> Option<Action> {
    match key.code {
        KeyCode::Esc => Some(Action::Back),
        KeyCode::Enter => Some(Action::Confirm),
        KeyCode::Backspace => Some(Action::DeleteChar),
        KeyCode::Down => Some(Action::MoveDown),
        KeyCode::Up => Some(Action::MoveUp),
        KeyCode::Char(c) => Some(Action::CharInput(c)),
        _ => None,
    }
}

#[derive(Debug, Clone)]
pub enum Action {
    Back,
    ForceQuit,
    AddUrl,
    Search,
    Help,
    Settings,
    CopyUrl,
    MoveDown,
    MoveUp,
    MoveFirst,
    MoveLast,
    Confirm,
    Delete,
    Pause,
    Cancel,
    CharInput(char),
    DeleteChar,
}
