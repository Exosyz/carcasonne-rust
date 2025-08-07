use carcasonne_console_input::input_handler::read_input_event;
use carcasonne_core::action::Action;
use carcasonne_core::context::game_context::GameContext;
use carcasonne_core::context::main_menu_context::MainMenuContext;
use carcasonne_core::factory::game_factory::GameTilesFactory;
use carcasonne_core::renderer::Renderer;
use carcasonne_core::state::State;
use carcasonne_core::state::StateResult::{Exit, Stay, Transition};
use carcasonne_core::state::game_state::main_menu_state::{MainMenuOptions, MenuState};
use carcasonne_core::state::shared_state::SharedContext;
use std::cell::RefCell;

/// Main game engine struct managing the state and rendering.
///
/// This struct holds the current state and a renderer instance.
/// It drives the main game loop, processes input events, updates the state,
/// and triggers rendering accordingly.
pub struct Game<'context, T: Renderer> {
    /// The current active state.
    state: Option<Box<dyn State>>,
    /// Renderer used to draw the current state.
    renderer: RefCell<T>,

    context: GameContext<'context>,
}

impl<'context, T: Renderer> Game<'context, T> {
    /// Creates a new game instance with the given renderer.
    ///
    /// Initializes the state to the main menu (`MenuState`).
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
            state: Some(Box::new(
                MenuState::<MainMenuOptions, MainMenuContext>::new(SharedContext::Some(
                    MainMenuContext::new(),
                )),
            )),
            renderer,
            context: GameContext::new(GameTilesFactory::build_base_game()),
        }
    }
    /// Returns a reference to the current state.
    ///
    /// # Panics
    ///
    /// Panics if the internal state is `None`, which should never happen during normal operation.
    fn state(&self) -> &dyn State {
        self.state.as_deref().expect("State should always be set")
    }

    /// Takes ownership of the current state, leaving it temporarily empty.
    ///
    /// This is used to update the state during the game loop.
    ///
    /// # Panics
    ///
    /// Panics if the internal state is `None`, which should never happen during normal operation.
    fn take_state(&mut self) -> Box<dyn State> {
        self.state.take().expect("State should always be set")
    }

    /// Renders the current game state using the associated renderer.
    fn rerender(&mut self) {
        self.renderer.borrow_mut().render(self.state().draw());
    }

    /// Runs the main game loop.
    ///
    /// The loop:
    /// - Renders the current state.
    /// - Checks if input is needed; if so, reads input and produces an action.
    /// - Updates the current state based on the action.
    /// - Changes the state or exits the loop based on the state's response.
    ///
    /// The loop continues until an `Action::Quit` or `ExitToStop` a state result occurs.
    pub fn run(&mut self) {
        self.rerender();
        'main_loop: loop {
            match self.state().need_input() {
                true => {
                    if let Some(action) = read_input_event()
                        && !self.apply_transition(self.state().handle_input(action))
                    {
                        break 'main_loop;
                    }
                }
                false => {
                    if !self.apply_transition(Action::None) {
                        break 'main_loop;
                    }
                }
            }
        }
    }

    /// Changes the current state and triggers re-rendering.
    ///
    /// # Arguments
    ///
    /// * `new_state` - The new state to replace the current one.
    fn change_state(&mut self, new_state: Box<dyn State>, rerender: bool) {
        self.state = Some(new_state);
        if rerender {
            self.rerender();
        }
    }

    fn apply_transition(&mut self, action: Action) -> bool {
        let mut current_state = self.take_state();

        match current_state.update(&mut self.context, action) {
            Transition(state) => self.change_state(state, true),
            Stay(rerender) => {
                self.change_state(current_state, rerender);
            }
            Exit => return false,
        }

        true
    }
}
