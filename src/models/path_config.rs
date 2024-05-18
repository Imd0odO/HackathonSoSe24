use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct PathConfig {
    pub grace_period: u32, // time until groups of bits take damage
    pub death_rate: u32,   // number of units killed every tick after surpassing the grace period
}

impl Default for PathConfig {
    fn default() -> Self {
        PathConfig {
            grace_period: 0,
            death_rate: 0,
        }
    }
}
