use crate::context::game_context::GameContext;
use crate::context::main_menu_context::MainMenuContext;
use crate::state::game_state::main_menu_state::add_player_state::AddPlayerState;
use crate::state::game_state::main_menu_state::remove_player_state::RemovePlayerOption;
use crate::state::game_state::main_menu_state::MainMenuOptions;
use crate::state::shared_state::input_state::InputState;
use crate::state::shared_state::menu_state::{MenuContextualOptions, MenuOptions, MenuState};
use crate::state::shared_state::SharedContext;
use crate::state::StateResult;

#[derive(Clone)]
pub enum ManagePlayerOptions {
    AddPlayer,
    RemovePlayer,
    GotoMenu,
}

impl MenuContextualOptions<MainMenuContext> for ManagePlayerOptions {
    fn options_for(_context: &MainMenuContext, game_context: &GameContext) -> Vec<Self>
    where
        Self: Sized,
    {
        let mut options = vec![ManagePlayerOptions::AddPlayer];

        if !game_context.players().is_empty() {
            options.push(ManagePlayerOptions::RemovePlayer);
        }

        options.push(ManagePlayerOptions::GotoMenu);
        options
    }
}

impl MenuOptions<MainMenuContext> for ManagePlayerOptions {
    fn apply_transition(
        &self,
        ctx: SharedContext<MainMenuContext>,
        game_context: &mut GameContext,
    ) -> StateResult {
        match self {
            ManagePlayerOptions::AddPlayer => {
                StateResult::Transition(Box::new(
                    InputState::<AddPlayerState, MainMenuContext>::new(ctx),
                ))
            }
            ManagePlayerOptions::RemovePlayer => StateResult::Transition(Box::new(
                MenuState::<RemovePlayerOption, MainMenuContext>::new_from_context(
                    ctx,
                    game_context,
                ),
            )),
            ManagePlayerOptions::GotoMenu => {
                StateResult::Transition(Box::new(
                    MenuState::<MainMenuOptions, MainMenuContext>::new(ctx),
                ))
            }
        }
    }

    fn title() -> &'static str {
        "Manage players"
    }

    fn label(&self) -> &str {
        match self {
            ManagePlayerOptions::AddPlayer => "Add player",
            ManagePlayerOptions::RemovePlayer => "Remove player",
            ManagePlayerOptions::GotoMenu => "Go back to the menu",
        }
    }
}
