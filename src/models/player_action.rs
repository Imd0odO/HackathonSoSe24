use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct PlayerAction {
    pub src: u32,    // uid of source base
    pub dest: u32,   // uid of destination base
    pub amount: u32, // number of bits moved
}

impl Default for PlayerAction {
    fn default() -> Self {
        PlayerAction {
            src: 0,
            dest: 0,
            amount: 0,
        }
    }
}
