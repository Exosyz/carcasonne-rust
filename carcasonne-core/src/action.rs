/// Represents a high-level action triggered by user input or system events.
///
/// This enum is typically used to drive state transitions in the game engine.
/// Actions can be navigational (e.g., movement), structural (e.g., start/stop game),
/// control-related (e.g., quit, validate), or editing-related.
///
/// # Variants
///
/// - `Bottom`: Move focus or cursor to the bottom.
/// - `Top`: Move focus or cursor to the top.
/// - `Left`: Move focus or cursor to the left.
/// - `Right`: Move focus or cursor to the right.
/// - `Validate`: Confirm the current selection or input.
/// - `Quit`: Exit the game or current screen.
/// - `None`: No action (e.g., idle state or ignored input).
/// - `Push(char)`: Insert or type the specified character (typically for text input).
/// - `Remove`: Remove or delete a character (e.g., backspace).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Bottom,
    Top,
    Left,
    Right,
    Validate,
    Quit,
    None,
    Push(char),
    Remove,
}
