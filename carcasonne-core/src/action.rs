/// Represents a high-level action triggered by user input or system events.
///
/// This enum is typically used to drive state transitions in the game engine.
/// Actions can be navigational (e.g., movement), structural (e.g., start/stop game),
/// or control-related (e.g., quit, validate).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    /// Start a new game session.
    StartGame,
    /// Stop or end the current game session.
    StopGame,
    /// Move focus or cursor to the bottom.
    Bottom,
    /// Move focus or cursor to the top.
    Top,
    /// Move focus or cursor to the left.
    Left,
    /// Move focus or cursor to the right.
    Right,
    /// Confirm the current selection or input.
    Validate,
    /// Exit the game or current screen.
    Quit,
    /// No action (e.g., idle state or ignored input).
    None,
}
