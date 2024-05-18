use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Game {
    pub uid: u32,               // uid of game
    pub tick: u32,              // tick in game
    pub player_count: u32,      // number of players
    pub remaining_players: u32, // number of players remaining
    pub player: u32,            // uid of your player
}

impl Default for Game {
    fn default() -> Self {
        Game {
            uid: 0,
            tick: 0,
            player_count: 0,
            remaining_players: 0,
            player: 0,
        }
    }
}
