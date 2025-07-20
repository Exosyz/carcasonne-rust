use carcasonne_core::input_handler::InputEvent;
use crossterm::event::{read, Event, KeyCode, KeyEventKind};

/// Reads a single input event from the terminal and converts it to an `InputEvent`.
///
/// Only key press events are handled. Other types of events or key releases are ignored.
///
/// # Returns
///
/// * `Some(InputEvent)` if a relevant key press event was detected.
/// * `None` if the event is not a key press or not recognized.
pub fn read_input_event() -> Option<InputEvent> {
    match read() {
        Ok(Event::Key(key_event)) if key_event.kind == KeyEventKind::Press => {
            match key_event.code {
                KeyCode::Up => Some(InputEvent::Up),
                KeyCode::Down => Some(InputEvent::Down),
                KeyCode::Left => Some(InputEvent::Left),
                KeyCode::Right => Some(InputEvent::Right),
                KeyCode::Enter => Some(InputEvent::Select),
                KeyCode::Backspace => Some(InputEvent::Backspace),
                KeyCode::Char(c) => Some(InputEvent::Char(c)),
                _ => None,
            }
        }
        _ => None,
    }
}
