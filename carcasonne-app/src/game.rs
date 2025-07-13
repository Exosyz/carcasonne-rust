use carcasonne_console_input::input_handler::read_input_event;
use carcasonne_core::action::Action;
use carcasonne_core::renderer::Renderer;
use carcasonne_core::state::game_state::menu_state::MenuState;
use carcasonne_core::state::State;
use carcasonne_core::state::StateResult::{Continue, ExitToStop, Skip};
use std::cell::RefCell;

/// Main game engine struct managing the game state and rendering.
///
/// This struct holds the current game state and a renderer instance.
/// It drives the main game loop, processes input events, updates the state,
/// and triggers rendering accordingly.
pub struct Game<T: Renderer> {
    /// The current active game state.
    game_state: Option<Box<dyn State>>,
    /// Renderer used to draw the current state.
    renderer: RefCell<T>,
}

impl<T: Renderer> Game<T> {
    /// Creates a new game instance with the given renderer.
    ///
    /// Initializes the game state to the main menu (`MenuState`).
    ///
    /// # Arguments
    ///
    /// * `renderer` - A `RefCell` wrapping the renderer implementation.
    ///
    /// # Returns
    ///
    /// A new `Game` instance is ready to run.
    pub fn new(renderer: RefCell<T>) -> Self {
        Self {
            game_state: Some(Box::new(MenuState {})),
            renderer,
        }
    }
    /// Returns a reference to the current game state.
    ///
    /// # Panics
    ///
    /// Panics if the internal state is `None`, which should never happen during normal operation.
    fn game_state(&self) -> &dyn State {
        self.game_state
            .as_deref()
            .expect("Game state should always be set")
    }

    /// Takes ownership of the current game state, leaving it temporarily empty.
    ///
    /// This is used to update the state during the game loop.
    ///
    /// # Panics
    ///
    /// Panics if the internal state is `None`, which should never happen during normal operation.
    fn take_state(&mut self) -> Box<dyn State> {
        self.game_state
            .take()
            .expect("Game state should always be set")
    }

    /// Renders the current game state using the associated renderer.
    fn rerender(&mut self) {
        self.renderer.borrow_mut().render(self.game_state().draw());
    }

    /// Runs the main game loop.
    ///
    /// The loop:
    /// - Renders the current state.
    /// - Checks if input is needed; if so, reads input and produces an action.
    /// - Updates the current state based on the action.
    /// - Changes the game state or exits the loop based on the state's response.
    ///
    /// The loop continues until an `Action::Quit` or `ExitToStop` a state result occurs.
    pub fn run(&mut self) {
        self.rerender();
        'main_loop: loop {
            // TODO find a solution to allow quit without blocking the input
            let action = if self.game_state().need_input() {
                self.game_state().handle_input(read_input_event())
            } else {
                Action::None
            };

            if let Action::Quit = action {
                break 'main_loop;
            }

            let mut current_state = self.take_state();

            match current_state.update(action) {
                Continue(state) => self.change_state(state),
                Skip => self.change_state(current_state),
                ExitToStop => break 'main_loop,
            }
        }
    }

    /// Changes the current game state and triggers re-rendering.
    ///
    /// # Arguments
    ///
    /// * `new_state` - The new game state to replace the current one.
    fn change_state(&mut self, new_state: Box<dyn State>) {
        self.game_state = Some(new_state);
        self.rerender();
    }
}
