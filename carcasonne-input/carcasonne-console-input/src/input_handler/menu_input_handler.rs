use crate::input_handler::{InputEvent, InputHandler};
use carcasonne_core::action::Action;

pub struct MenuConsoleInputHandler;
impl InputHandler for MenuConsoleInputHandler {
    fn handle_input(&mut self, event: InputEvent) -> Action {
        match event {
            InputEvent::Quit => Action::Quit,
            InputEvent::Up => Action::StartGame,
            _ => Action::None,
        }
    }
}
