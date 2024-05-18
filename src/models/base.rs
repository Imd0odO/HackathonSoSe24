use crate::models::position::Position;
use serde::Deserialize;
use crate::models::board_action::BoardAction;
use crate::models::game_config::GameConfig;

#[derive(Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Base {
    pub position: Position,       // position of the base
    pub uid: u32,                 // uid of the base
    pub player: u32,              // owner of the base
    pub population: u32,          // current population of the base
    pub level: u32,               // level of the base
    pub units_until_upgrade: u32, // number of units required to upgrade
}

impl Default for Base {
    fn default() -> Self {
        Base {
            position: Position::default(),
            uid: 0,
            player: 0,
            population: 0,
            level: 0,
            units_until_upgrade: 0,
        }
    }
}

impl Base {
    // get base population from base in n ticks with current config and attacks on the board
    fn raw_population_in_n_ticks(&self, ticks: u32, config: &GameConfig, attacks: &Vec<BoardAction>) -> i32 {
        // set the population in future to the current population
        let mut population_in_future: i32 = self.population as i32;

        // check if the base has a passive generation rate
        if self.uid != 0 {
            // add the spawned bits to the population
            population_in_future += (ticks * config.base_levels[self.level as usize].spawn_rate) as i32;
        }

        // iterate through all attacks
        attacks.iter().for_each(|attack| {
            // check if the attack will arrive in time
            if attack.arrival_in_ticks() <= ticks {
                // get the amount of bits in the attack on arrival
                let val_on_target: i32 = attack.amount_at_target(&config.paths) as i32;

                // check if the attack is owned by the base
                if attack.player == self.player {
                    // the attack will contribute to the population
                    population_in_future += val_on_target;
                }
                else {
                    // the attack will fight with the base
                    population_in_future -= val_on_target;
                }
            }
        });
        // return abs of population, because on negative population it is likely to be conquered by an opponent
        return population_in_future;
    }

    pub fn population_in_n_ticks(&self, ticks: u32, config: &GameConfig, attacks: &Vec<BoardAction>) -> u32 {
        return self.raw_population_in_n_ticks(ticks, config, attacks).abs() as u32;
    }

    pub fn will_die_within_in_n_ticks(&self, ticks: u32, config: &GameConfig, attacks: &Vec<BoardAction>) -> bool {
        return self.raw_population_in_n_ticks(ticks, config, attacks) < 0;
    }

    // get the distance to another base
    pub fn distance_to(&self, base: &Base) -> u32 {
        // calculate the Euclidean distance between the bases
        return (( (base.position.x - self.position.x).pow(2)
                + (base.position.y - self.position.y).pow(2)
                + (base.position.z - self.position.z).pow(2)
            ) as f64).sqrt() as u32;
    }

    // get the amount of bits required to defeat another base
    pub fn required_to_defeat(&self, target_base: &Base, game_config: &GameConfig, attacks: &Vec<BoardAction>) -> u32 {
        // get the distance between the bases
        let d: u32 = self.distance_to(target_base);

        // get the population of the target base when an own attack would arrive
        let population: u32 = target_base.population_in_n_ticks(d, game_config, attacks);

        // to defeat a base there has to be at least one more bit arriving than the current population is
        let mut requirement: u32 = population + 1;

        // check if the distance is greater than the grace period
        if d > game_config.paths.grace_period {
            // add the loss over the additional distance
            requirement += (d - game_config.paths.grace_period) * game_config.paths.death_rate
        }

        // return required bits
        return  requirement;
    }
}
