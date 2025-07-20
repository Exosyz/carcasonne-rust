use crate::action::Action;
use crate::input_handler::InputEvent;
use crate::layout::node::Node;

pub mod game_state;
pub mod shared_state;

/// Represents the possible outcomes after processing a state update.
///
/// This enum is used to control the flow of state transitions in the application.
pub enum StateResult {
    /// Continue with a new state, replacing the current one.
    ///
    /// The boxed `State` will become the active state.
    Transition(Box<dyn State>),
    Stay(bool),
    /// Exit the current state machine or application gracefully.
    Exit,
}

/// Represents a game state within the application.
///
/// A `State` defines the logic and rendering for a specific part of the game, such as
/// a menu screen, a game turn, or an animation sequence. It handles input, produces
/// actions in response, updates itself based on actions and an optional `Context`, and
/// knows how to render itself.
///
/// # Type Parameters
///
/// * `Context` – Optional shared data or external input passed into the state on update.
///   This allows states to perform logic depending on external factors or shared state.
pub trait State {
    /// Called to update the state based on an `Action` and optional `Context`.
    ///
    /// The state can choose to:
    /// - stay the same (return `StateResult::Skip`)
    /// - transition to a new state (return `StateResult::Continue`)
    /// - signal that the application should exit (return `StateResult::ExitToStop`)
    ///
    /// # Arguments
    ///
    /// * `action` – The action to process.
    /// * `context` – Optional contextual information passed in during state updates.
    ///
    /// # Returns
    ///
    /// A `StateResult` indicating what to do next.
    fn update(&mut self, action: Action) -> StateResult;

    /// Renders the state as a UI `Node` (tree structure).
    ///
    /// This is used to draw the current visual representation of the state.
    fn draw(&'_ self) -> Node<'_>;

    /// Handles a user input event and maps it to an `Action`.
    ///
    /// # Arguments
    ///
    /// * `event` – The raw input event received.
    ///
    /// # Returns
    ///
    /// The `Action` corresponding to the input event.
    fn handle_input(&self, event: InputEvent) -> Action;

    /// Indicates whether the state currently requires to be input.
    ///
    /// Defaults to `false`. Override this method in states where input is expected.
    ///
    /// # Returns
    ///
    /// `true` if the state is waiting for input, `false` otherwise.
    fn need_input(&self) -> bool {
        false
    }
}
