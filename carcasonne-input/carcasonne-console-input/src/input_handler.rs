pub mod default_input_handler;
pub mod game_input_handler;
pub mod menu_input_handler;

use carcasonne_core::action::Action;
use crossterm::event::{read, Event, KeyCode};

pub enum InputEvent {
    Up,
    Down,
    Left,
    Right,
    Quit,
}

pub trait InputHandler {
    fn handle_input(&mut self, event: InputEvent) -> Action;
}

pub fn read_input_event() -> InputEvent {
    loop {
        if let Event::Key(key_event) = read().unwrap() {
            return match key_event.code {
                KeyCode::Up => InputEvent::Up,
                KeyCode::Down => InputEvent::Down,
                KeyCode::Left => InputEvent::Left,
                KeyCode::Right => InputEvent::Right,
                KeyCode::Char('q') => InputEvent::Quit,
                _ => continue,
            };
        }
    }
}
