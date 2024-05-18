use crate::models::base::Base;
use crate::models::board_action::BoardAction;
use crate::models::game::Game;
use crate::models::game_config::GameConfig;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct GameState {
    pub actions: Vec<BoardAction>, // list of all actions in progress
    pub bases: Vec<Base>,          // list of all bases
    pub config: GameConfig,        // settings for this game
    pub game: Game,                // information about the game
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            actions: vec![BoardAction::default()],
            bases: vec![Base::default()],
            config: GameConfig::default(),
            game: Game::default(),
        }
    }
}
