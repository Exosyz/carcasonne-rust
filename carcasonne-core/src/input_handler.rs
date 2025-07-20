/// Represents a user input event, typically from a keyboard or controller.
///
/// These events are used to drive the interaction logic of the application.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputEvent {
    /// Move focus or selection up.
    Up,
    /// Move focus or selection down.
    Down,
    /// Move focus or selection left.
    Left,
    /// Move focus or selection right.
    Right,
    /// Select the current option or confirm the action.
    Select,
    /// A character key input.
    Char(char),
    /// Backspace key input (typically used for deletion).
    Backspace,
}
