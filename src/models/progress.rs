use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Progress {
    pub distance: u32, // distance between the bases
    pub traveled: u32, // distance already traveled
}

impl Default for Progress {
    fn default() -> Self {
        Progress {
            distance: 0,
            traveled: 0,
        }
    }
}
