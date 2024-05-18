use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Position {
    pub x: i32, // x coordinate
    pub y: i32, // y coordinate
    pub z: i32, // y coordinate
}

impl Default for Position {
    fn default() -> Self {
        Position { x: 0, y: 0, z: 0 }
    }
}
