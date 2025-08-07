mod add_player_state;
pub mod manage_player_state;
mod remove_player_state;

use crate::context::game_context::GameContext;
use crate::context::main_menu_context::MainMenuContext;
use crate::state::game_state::game_running_state::select_tile_state::SelectTileState;
use crate::state::game_state::game_running_state::GameRunningState;
use crate::state::game_state::main_menu_state::manage_player_state::ManagePlayerOptions;
use crate::state::shared_state::menu_state::MenuDefaultOptions;
pub use crate::state::shared_state::menu_state::{MenuOptions, MenuState};
use crate::state::shared_state::SharedContext;
use crate::state::StateResult;

#[derive(Clone)]
pub enum MainMenuOptions {
    StartGame,
    ManagePlayers,
    Quit,
}

impl MenuDefaultOptions for MainMenuOptions {
    fn default_options() -> Vec<Self>
    where
        Self: Sized,
    {
        vec![
            MainMenuOptions::StartGame,
            MainMenuOptions::ManagePlayers,
            MainMenuOptions::Quit,
        ]
    }
}

impl MenuOptions<MainMenuContext> for MainMenuOptions {
    fn apply_transition(
        &self,
        context: SharedContext<MainMenuContext>,
        game_context: &mut GameContext,
    ) -> StateResult {
        match self {
            MainMenuOptions::StartGame => StateResult::Transition(Box::new(GameRunningState::new(
                Box::new(SelectTileState::new()),
            ))),
            MainMenuOptions::ManagePlayers => StateResult::Transition(Box::new(MenuState::<
                ManagePlayerOptions,
                MainMenuContext,
            >::new_from_context(
                context,
                game_context,
            ))),
            MainMenuOptions::Quit => StateResult::Exit,
        }
    }

    fn title() -> &'static str {
        "Carcassonne game"
    }
    fn label(&self) -> &str {
        match self {
            MainMenuOptions::StartGame => "Start game",
            MainMenuOptions::ManagePlayers => "Manage players",
            MainMenuOptions::Quit => "Quit",
        }
    }
}
