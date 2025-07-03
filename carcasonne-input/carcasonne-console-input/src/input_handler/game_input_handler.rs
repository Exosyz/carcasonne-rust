use crate::input_handler::{InputEvent, InputHandler};
use carcasonne_core::action::Action;

pub struct GameConsoleInputHandler;
impl InputHandler for GameConsoleInputHandler {
    fn handle_input(&mut self, event: InputEvent) -> Action {
        match event {
            InputEvent::Quit => Action::Quit,
            InputEvent::Down => Action::StopGame,
            _ => Action::None,
        }
    }
}
