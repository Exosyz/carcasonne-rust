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
///
/// # Examples
///
/// ```
/// use carcasonne_console_input::input_handler::read_input_event;
/// use carcasonne_core::input_handler::InputEvent;
///
/// let input = read_input_event();
/// match input {
///     InputEvent::Quit => println!("Quitting..."),
///     InputEvent::Up => println!("Moving up"),
///     _ => {}
/// }
/// ```
pub fn read_input_event() -> InputEvent {
    loop {
        if let Event::Key(key_event) = read().unwrap() {
            return match key_event.code {
                KeyCode::Up => InputEvent::Up,
                KeyCode::Down => InputEvent::Down,
                KeyCode::Left => InputEvent::Left,
                KeyCode::Right => InputEvent::Right,
                KeyCode::Enter => InputEvent::Enter,
                KeyCode::Char('q') => InputEvent::Quit,
                _ => continue,
            };
        }
    }
}
