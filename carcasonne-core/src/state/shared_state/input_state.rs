use crate::action::Action;
use crate::color::Color;
use crate::context::game_context::GameContext;
use crate::input_handler::InputEvent;
use crate::layout::node::{Node, NodeTag};
use crate::layout::point::Point;
use crate::state::shared_state::SharedContext;
use crate::state::{State, StateResult};
use std::marker::PhantomData;

/// Trait defining behavior for input states.
///
/// `Context` is a generic type parameter that carries shared context info.
/// `apply_transition` handles the transition logic after input validation.
/// `title` returns a static title for the input UI.
/// `validate` optionally validates the input string and returns an error message if invalid.
pub trait InputBehaviour<Context = ()>
where
    Self: Clone,
{
    fn apply_transition(
        value: &str,
        ctx: SharedContext<Context>,
        game_context: &mut GameContext,
    ) -> Result<StateResult, String>;
    fn title() -> &'static str;
    fn validate<'a>(
        value: &str,
        ctx: &SharedContext<Context>,
        game_context: &GameContext,
    ) -> Result<(), &'a str>;
}

/// A state for managing input UI and logic, parameterized by the input behavior and context.
pub struct InputState<Input, Context = ()>
where
    Input: InputBehaviour<Context>,
{
    context: SharedContext<Context>,
    value: String,
    x: usize, // Cursor position in the input string
    error: Option<String>,
    _input: PhantomData<Input>, // Marker to keep the Input generic parameter
}

impl<Input, Context> InputState<Input, Context>
where
    Input: InputBehaviour<Context>,
{
    pub fn new(context: SharedContext<Context>) -> Self {
        Self {
            context,
            value: String::new(),
            x: 0,
            error: None,
            _input: PhantomData,
        }
    }
}

impl<Input, Context> State for InputState<Input, Context>
where
    Input: InputBehaviour<Context>,
{
    fn update(&mut self, game_context: &mut GameContext, action: Action) -> StateResult {
        match action {
            Action::Right => {
                if self.x < self.value.len() {
                    self.x += 1;
                    StateResult::Stay(true)
                } else {
                    StateResult::Stay(false)
                }
            }
            Action::Left => {
                if self.x > 0 {
                    self.x -= 1;
                    StateResult::Stay(true)
                } else {
                    StateResult::Stay(false)
                }
            }
            Action::Validate => match Input::validate(&self.value, &self.context, game_context) {
                Ok(()) => match Input::apply_transition(
                    &self.value,
                    std::mem::take(&mut self.context),
                    game_context,
                ) {
                    Ok(result) => result,
                    Err(error) => {
                        self.error = Some(error.to_string());
                        StateResult::Stay(true)
                    }
                },
                Err(error) => {
                    self.error = Some(error.to_string());
                    StateResult::Stay(true)
                }
            },
            Action::Push(c) => {
                self.value.insert(self.x, c);
                self.x += 1;
                StateResult::Stay(true)
            }
            Action::Remove => {
                if self.x > 0 {
                    self.x -= 1;
                    self.value.remove(self.x);
                    StateResult::Stay(true)
                } else {
                    StateResult::Stay(false)
                }
            }
            _ => StateResult::Stay(false),
        }
    }

    fn draw(&'_ self) -> Node<'_> {
        let mut elems = vec![
            Node::RichText(
                Input::title(),
                vec![
                    NodeTag::Underline,
                    NodeTag::Bold,
                    NodeTag::Foreground(Color::Blue),
                ],
            ),
            Node::Input(self.value.as_str(), Point::new(self.x, 0)),
        ];

        if let Some(error) = &self.error {
            elems.push(Node::Error(error));
        }

        Node::VerticalContainer(elems)
    }

    fn handle_input(&self, event: InputEvent) -> Action {
        match event {
            InputEvent::Char(c) => Action::Push(c),
            InputEvent::Backspace => Action::Remove,
            InputEvent::Select => Action::Validate,
            InputEvent::Left => Action::Left,
            InputEvent::Right => Action::Right,
            _ => Action::None,
        }
    }

    fn need_input(&self) -> bool {
        true
    }
}
