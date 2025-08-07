use crate::color::Color;
use std::collections::VecDeque;

#[derive(Default)]
pub struct MainMenuContext {
    available_colors: VecDeque<Color>,
}

impl MainMenuContext {
    pub fn new() -> Self {
        Self {
            available_colors: vec![Color::Red, Color::Blue, Color::Green, Color::Yellow].into(),
        }
    }

    pub fn get_next_color(&mut self) -> Option<Color> {
        self.available_colors.pop_front()
    }

    pub fn add_color(&mut self, color: Color) {
        self.available_colors.push_back(color);
    }
}
