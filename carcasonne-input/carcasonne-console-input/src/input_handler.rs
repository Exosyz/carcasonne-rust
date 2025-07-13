use carcasonne_core::input_handler::InputEvent;
use crossterm::event::{read, Event, KeyCode};

/// Blocks until a valid keyboard input is received and returns a corresponding `InputEvent`.
///
/// This function loops indefinitely until a mapped key is pressed (arrow keys, Enter, or 'q').
/// Other key presses are ignored.
///
/// # Behavior
/// - Arrow keys map to directional input events.
/// - Enter maps to `InputEvent::Enter`.
/// - 'q' maps to `InputEvent::Quit`.
/// - All other inputs are ignored.
/// ```
pub fn read_input_event() -> InputEvent {
    loop {
        match read() {
            Ok(Event::Key(key_event)) => match key_event.code {
                KeyCode::Up => return InputEvent::Up,
                KeyCode::Down => return InputEvent::Down,
                KeyCode::Left => return InputEvent::Left,
                KeyCode::Right => return InputEvent::Right,
                KeyCode::Enter => return InputEvent::Enter,
                KeyCode::Char('q') => return InputEvent::Quit,
                _ => continue,
            },
            Ok(_) => continue,
            Err(e) => {
                eprintln!("Fail to read input: {e}");
                panic!("Fail to read key event");
            }
        }
    }
}
