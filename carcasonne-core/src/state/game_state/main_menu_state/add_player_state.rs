use crate::context::main_menu_context::MainMenuContext;
use crate::state::game_state::main_menu_state::manage_player_state::ManagePlayerOptions;
use crate::state::shared_state::input_state::InputBehaviour;
use crate::state::shared_state::menu_state::MenuState;
use crate::state::shared_state::SharedContext;
use crate::state::StateResult;

#[derive(Clone)]
pub struct AddPlayerState;

impl InputBehaviour<MainMenuContext> for AddPlayerState {
    fn apply_transition(value: &str, ctx: &mut SharedContext<MainMenuContext>) -> StateResult {
        if let SharedContext::Some(context) = ctx {
            context
                .try_add_player(value.into())
                .expect("TODO: panic message")
        }
        StateResult::Transition(Box::new(
            MenuState::<ManagePlayerOptions, MainMenuContext>::new_from_context(std::mem::take(
                ctx,
            )),
        ))
    }

    fn title() -> &'static str {
        "Add Player"
    }

    fn validate<'a>(value: &str, ctx: &SharedContext<MainMenuContext>) -> Option<&'a str> {
        if value.is_empty() {
            return Some("Player name cannot be empty");
        }
        if let SharedContext::Some(context) = ctx
            && context.get_players().iter().any(|p| p.name == value)
        {
            return Some("Player already exists");
        }
        None
    }
}
