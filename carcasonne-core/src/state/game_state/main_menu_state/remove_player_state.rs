use crate::context::game_context::GameContext;
use crate::context::main_menu_context::MainMenuContext;
use crate::state::game_state::main_menu_state::manage_player_state::ManagePlayerOptions;
use crate::state::shared_state::menu_state::{MenuContextualOptions, MenuOptions, MenuState};
use crate::state::shared_state::SharedContext;
use crate::state::StateResult;

#[derive(Clone)]
pub struct RemovePlayerOption {
    name: String,
}

impl RemovePlayerOption {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl MenuContextualOptions<MainMenuContext> for RemovePlayerOption {
    fn options_for(_ctx: &MainMenuContext, game_context: &GameContext) -> Vec<Self> {
        let mut options: Vec<Self> = game_context
            .players()
            .iter()
            .map(|p| RemovePlayerOption::new(p.name.clone()))
            .collect();

        options.push(RemovePlayerOption::new("Go back".into()));

        options
    }
}

impl MenuOptions<MainMenuContext> for RemovePlayerOption {
    fn apply_transition(
        &self,
        mut ctx: SharedContext<MainMenuContext>,
        game_context: &mut GameContext,
    ) -> StateResult {
        match game_context.remove_player(self.name.clone()) {
            Ok(color) => match &mut ctx {
                SharedContext::Some(context) => context.add_color(color),
                SharedContext::None => (),
            },
            Err(e) => {
                println!("Error removing player: {}", e);
            }
        }

        StateResult::Transition(Box::new(
            MenuState::<ManagePlayerOptions, MainMenuContext>::new_from_context(ctx, game_context),
        ))
    }

    fn title() -> &'static str {
        "Select a player to remove"
    }

    fn label(&self) -> &str {
        self.name.as_str()
    }
}
