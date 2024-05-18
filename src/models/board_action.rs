use crate::models::progress::Progress;
use serde::Deserialize;
use uuid::Uuid;
use crate::models::path_config::PathConfig;

#[derive(Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub struct BoardAction {
    pub src: u32,           // uid of source base
    pub dest: u32,          // uid of destination base
    pub amount: u32,        // number of bits moved
    pub uuid: Uuid,         // uuid of the action
    pub player: u32,        // id of the player who took the action
    pub progress: Progress, // progress off the action
}

impl Default for BoardAction {
    fn default() -> Self {
        BoardAction {
            src: 0,
            dest: 0,
            amount: 0,
            uuid: Uuid::default(),
            player: 0,
            progress: Progress::default(),
        }
    }
}

impl BoardAction {
    // get the remaining ticks until arrival
    pub fn arrival_in_ticks(&self) -> u32 {
        // return the remaining ticks
        return self.progress.distance - self.progress.traveled;
    }

    // get the amount of bits that reach a base
    pub fn amount_at_target(&self, config: &PathConfig) -> u32 {
        // if the distance is smaller than the grace period all bits will reach their destination
        if self.progress.distance < config.grace_period {
            return self.amount;
        }

        // get the number of bits that will die until their destination
        let deaths: u32 = config.death_rate * (self.progress.distance - config.grace_period);

        // there are more deaths than the bits in the attack, no bit will reach the destination
        if deaths > self.amount { return 0 }

        // return the bits at destination
        return self.amount - deaths;
    }
}
