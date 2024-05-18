use crate::models::base_level::BaseLevel;
use crate::models::path_config::PathConfig;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct GameConfig {
    pub base_levels: Vec<BaseLevel>, // all available base levels
    pub paths: PathConfig,           // settings containing paths between bases
}

impl Default for GameConfig {
    fn default() -> Self {
        GameConfig {
            base_levels: vec![BaseLevel::default()],
            paths: PathConfig::default(),
        }
    }
}
