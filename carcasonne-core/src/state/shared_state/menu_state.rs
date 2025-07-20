use crate::action::Action;
use crate::color::Color;
use crate::input_handler::InputEvent;
use crate::layout::node::{Node, NodeTag};
use crate::state::shared_state::SharedContext;
use crate::state::{State, StateResult};

/// Trait representing an individual menu option.
///
/// Provides methods to apply transitions, get a label, and a static menu title.
/// The generic `Context` allows options to operate with external state.
///
/// `Self` must be `Clone` so options can be duplicated safely.
pub trait MenuOptions<Context = ()>
where
    Self: Clone,
{
    /// Apply the transition associated with this option,
    /// potentially modifying the shared context and returning a `StateResult`.
    fn apply_transition(&self, context: SharedContext<Context>) -> StateResult;

    /// Return the title of the menu.
    /// This is shown as a header when drawing the menu.
    fn title() -> &'static str;

    /// Return the label text for this option to be displayed in the menu.
    fn label(&self) -> &str;
}

/// Trait to provide a default static set of menu options.
///
/// Useful for menus that don't require dynamic or context-dependent options.
pub trait MenuDefaultOptions {
    /// Returns a vector of default options.
    fn default_options() -> Vec<Self>
    where
        Self: Sized;
}

/// Trait to provide context-dependent menu options.
///
/// Used when menu options depend on external context (e.g., game state).
pub trait MenuContextualOptions<Context> {
    /// Returns a vector of options appropriates for the given context.
    fn options_for(context: &Context) -> Vec<Self>
    where
        Self: Sized;
}

/// Represents the state of a menu with selectable options.
///
/// Generic over the option type and an optional context type.
/// Maintains the current list of options, the selected index, and shared context.
pub struct MenuState<Options, Context = ()>
where
    Options: MenuOptions<Context>,
{
    /// Current menu options available.
    pub(in crate::state) options: Vec<Options>,
    /// Index of the currently selected option.
    pub(in crate::state) selected_option: usize,
    /// Shared contextual state available to menu options.
    pub(in crate::state) context: SharedContext<Context>,
}

impl<Options, Context> MenuState<Options, Context>
where
    Options: MenuDefaultOptions + MenuOptions<Context>,
{
    /// Create a new menu state with default options and the given context.
    pub fn new(context: SharedContext<Context>) -> Self {
        Self {
            options: Options::default_options(),
            context,
            ..Default::default()
        }
    }
}

impl<Options, Context> MenuState<Options, Context>
where
    Options: MenuContextualOptions<Context> + MenuOptions<Context>,
{
    /// Create a new menu state with options determined by the given context.
    pub fn new_from_context(context: SharedContext<Context>) -> Self {
        Self {
            options: if let SharedContext::Some(ctx) = &context {
                Options::options_for(ctx)
            } else {
                vec![]
            },
            context,
            ..Default::default()
        }
    }
}

impl<Options, Context> Default for MenuState<Options, Context>
where
    Options: MenuOptions<Context>,
{
    /// Returns a default menu state with no options and no context.
    fn default() -> Self {
        Self {
            selected_option: 0,
            options: vec![],
            context: SharedContext::None,
        }
    }
}

impl<Options, Context> MenuState<Options, Context>
where
    Options: MenuOptions<Context>,
{
    /// Change the selected option by `delta` (positive or negative).
    /// The selection saturates at the bounds (0 to options.len()).
    fn move_selection(&mut self, delta: isize) {
        if self.options.is_empty() {
            self.selected_option = 0;
            return;
        }

        let max_index = self.options.len() - 1;
        if delta.is_negative() {
            self.selected_option = self.selected_option.saturating_sub(delta.unsigned_abs());
        } else {
            self.selected_option = (self.selected_option + delta as usize).min(max_index);
        }
    }

    /// Get a clone of the currently selected option.
    fn selected_option(&self) -> Options {
        self.options[self.selected_option].clone()
    }
}

impl<Options, Context> State for MenuState<Options, Context>
where
    Options: MenuOptions<Context>,
{
    /// Update the menu state based on an incoming action.
    ///
    /// Moves selection on `Top`/`Bottom` actions or applies the selected option's
    /// transition on `Validate`. Returns a `StateResult` indicating the next step.
    fn update(&mut self, action: Action) -> StateResult {
        match action {
            Action::Bottom => {
                self.move_selection(1);
                StateResult::Stay(true)
            }
            Action::Top => {
                self.move_selection(-1);
                StateResult::Stay(true)
            }
            Action::Validate => self
                .selected_option()
                .apply_transition(std::mem::take(&mut self.context)),
            _ => StateResult::Stay(false),
        }
    }

    /// Draw the menu UI as a node tree.
    ///
    /// Includes the menu title and a menu list with the current selection highlighted.
    fn draw(&'_ self) -> Node<'_> {
        Node::VerticalContainer(vec![
            Node::RichText(
                Options::title(),
                vec![
                    NodeTag::Bold,
                    NodeTag::Underline,
                    NodeTag::Foreground(Color::Blue),
                ],
            ),
            Node::Menu(
                self.options.iter().map(|s| s.label()).collect(),
                self.selected_option,
            ),
        ])
    }

    /// Map input events to actions relevant for menu navigation and selection.
    ///
    /// Supports quitting with 'q', selection, and up/down movement.
    fn handle_input(&self, event: InputEvent) -> Action {
        match event {
            InputEvent::Char('q') => Action::Quit,
            InputEvent::Select | InputEvent::Right => Action::Validate,
            InputEvent::Down => Action::Bottom,
            InputEvent::Up => Action::Top,
            _ => Action::None,
        }
    }

    /// Indicate that this state requires to be input to progress.
    fn need_input(&self) -> bool {
        true
    }
}
