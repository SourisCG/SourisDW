use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::time::Duration;
use crate::error::Result;

pub fn poll_event(timeout: Duration) -> Result<Option<KeyCode>> {
    if event::poll(timeout).map_err(|e| crate::error::SourisError::Other(e.into()))? {
        if let Event::Key(key) = event::read().map_err(|e| crate::error::SourisError::Other(e.into()))? {
            if key.kind == KeyEventKind::Press {
                return Ok(Some(key.code));
            }
        }
    }
    Ok(None)
}
