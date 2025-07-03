use crate::input_handler::{InputEvent, InputHandler};
use carcasonne_core::action::Action;

pub struct DefaultConsoleInputHandler;

impl InputHandler for DefaultConsoleInputHandler {
    fn handle_input(&mut self, _: InputEvent) -> Action {
        Action::None
    }
}
