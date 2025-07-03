use crate::game::GameState::{Menu, Playing, Stop};
use carcasonne_console_input::input_handler::default_input_handler::DefaultConsoleInputHandler;
use carcasonne_console_input::input_handler::game_input_handler::GameConsoleInputHandler;
use carcasonne_console_input::input_handler::menu_input_handler::MenuConsoleInputHandler;

use carcasonne_console_input::input_handler::{read_input_event, InputHandler};
use carcasonne_core::action::Action;
use carcasonne_core::game_state::GameState;
use carcasonne_core::renderer::{Renderer, UIComponent};
use carcasonne_game_ui::ui_component::GameStateRenderer;

pub struct Game<T: Renderer> {
    game_state: GameState,
    renderer: T,
    input_handler: Box<dyn InputHandler>,
}

impl<T: Renderer> Game<T> {
    pub fn new(renderer: T) -> Self {
        Self {
            game_state: Menu,
            renderer,
            input_handler: Box::new(MenuConsoleInputHandler {}),
        }
    }

    pub fn run(&mut self) {
        GameStateRenderer(&self.game_state).render(&self.renderer);
        'main_loop: loop {
            let new_state = match self.game_state {
                Menu => self.handle_menu(),
                Playing => self.handle_playing(),
                Stop => break 'main_loop,
            };

            if let Some(new_state) = new_state {
                self.change_state(new_state);
            }
        }
    }

    fn change_state(&mut self, new_state: GameState) {
        self.game_state = new_state;
        self.input_handler = match self.game_state {
            Menu => Box::new(MenuConsoleInputHandler {}),
            Playing => Box::new(GameConsoleInputHandler {}),
            Stop => Box::new(DefaultConsoleInputHandler {}),
        };
        GameStateRenderer(&self.game_state).render(&self.renderer);
    }

    fn handle_menu(&mut self) -> Option<GameState> {
        match self.input_handler.handle_input(read_input_event()) {
            Action::StartGame => Some(Playing),
            Action::Quit => Some(Stop),
            _ => None,
        }
    }

    fn handle_playing(&mut self /*, playing_state: &mut PlayingState*/) -> Option<GameState> {
        match self.input_handler.handle_input(read_input_event()) {
            Action::StopGame => Some(Menu),
            Action::Quit => Some(Stop),
            _ => None,
        }
    }
}
