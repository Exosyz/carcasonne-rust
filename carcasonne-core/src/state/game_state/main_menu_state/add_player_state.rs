use crate::context::game_context::GameContext;
use crate::context::main_menu_context::MainMenuContext;
use crate::state::game_state::main_menu_state::manage_player_state::ManagePlayerOptions;
use crate::state::shared_state::input_state::InputBehaviour;
use crate::state::shared_state::menu_state::MenuState;
use crate::state::shared_state::SharedContext;
use crate::state::StateResult;

#[derive(Clone)]
pub struct AddPlayerState;

impl InputBehaviour<MainMenuContext> for AddPlayerState {
    fn apply_transition(
        value: &str,
        mut ctx: SharedContext<MainMenuContext>,
        game_context: &mut GameContext,
    ) -> Result<StateResult, String> {
        if let SharedContext::Some(menu_context) = &mut ctx {
            match menu_context.get_next_color() {
                Some(color) => match game_context.try_add_player(value.into(), color) {
                    Ok(_) => Ok(StateResult::Transition(Box::new(MenuState::<
                        ManagePlayerOptions,
                        MainMenuContext,
                    >::new_from_context(
                        ctx, game_context
                    )))),
                    Err(err) => Err(err),
                },
                None => Err("Max player count reach".into()),
            }
        } else {
            panic!("Invalid context");
        }
    }

    fn title() -> &'static str {
        "Add Player"
    }

    fn validate<'a>(
        value: &str,
        _ctx: &SharedContext<MainMenuContext>,
        game_context: &GameContext,
    ) -> Result<(), &'a str> {
        if value.is_empty() {
            return Err("Player name cannot be empty");
        }
        if game_context.players().iter().any(|p| p.name == value) {
            return Err("Player already exists");
        }
        Ok(())
    }
}
