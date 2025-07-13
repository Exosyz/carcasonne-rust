use crate::action::Action;

/// Represents a user input event, typically from a keyboard or controller.
///
/// These events are used to drive the interaction logic of the application.
pub enum InputEvent {
    /// Move focus or selection up.
    Up,
    /// Move focus or selection down.
    Down,
    /// Move focus or selection left.
    Left,
    /// Move focus or selection right.
    Right,
    /// Confirm or select the current option.
    Enter,
    /// Exit the current screen or quit the application.
    Quit,
}

/// A trait for handling user input and converting it into game actions.
///
/// Implement this trait for any component or state that reacts to input.
/// The handler transforms low-level input events into high-level actions
/// that can be used to update the application state.
pub trait InputHandler {
    /// Handles an input event and returns an associated `Action`.
    ///
    /// This allows the current state or component to interpret input
    /// in context and produce the appropriate result (e.g., move, select, exit).
    ///
    /// # Arguments
    ///
    /// * `event` - The input event to process.
    ///
    /// # Returns
    ///
    /// An `Action` representing the logical intent of the input.
    fn handle_input(&mut self, event: InputEvent) -> Action;
}
