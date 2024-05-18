use crate::models::{game_state::GameState, player_action::PlayerAction};
use crate::models::base::Base;
use crate::models::target::Target;

pub fn decide(game_state: GameState) -> Vec<PlayerAction> {
    // all planned attacks
    let mut attacks: Vec<PlayerAction> = Vec::new();

    // a list of all friendly and opponent bases
    let mut own_bases: Vec<Base> = Vec::new();
    let mut opponent_bases: Vec<Base> = Vec::new();

    // iterate over all bases
    game_state.bases.iter().for_each(|base| {
        // sort them into owned and opponent bases
        if base.player == game_state.game.player {
            own_bases.push(base.clone());
        }
        else {
            opponent_bases.push(base.clone());
        }
    });

    // iterate through all owned bases
    own_bases.iter().for_each(|base| {
        // the target of the base, currently none
        let mut target: Option<Target> = None;

        // iterate over all opponents as possible targets
        opponent_bases.iter().for_each(|opponent| {
            // calculate the required bits to conquer the opponents base
            let required_bits: u32 = base.required_to_defeat(&opponent, &game_state.config, &game_state.actions);

            // check that the base could be conquered with at least 1/4 of the population remaining in base
            if required_bits + (game_state.config.base_levels[base.level as usize].max_population / 4) < base.population {
                // check if there is already a target
                if let Some(target_value) = target {
                    // get if the new target is closer than the old one
                    let is_closer: bool = base.distance_to(&opponent) < base.distance_to(&target_value.base);
                    // check if the new target is closer and the required bits are less or the game is free and within grace period
                    if is_closer && (required_bits <= target_value.required_bits || (base.uid == 0 && opponent.distance_to(&base) < game_state.config.paths.grace_period)) {
                        // set the new target
                        target = Some(Target::new(base.clone(), required_bits));
                    }
                }
                // there is no target yet, so this is the first one
                else {
                    // set the new target
                    target = Some(Target::new(base.clone(), required_bits));
                }
            }
        });

        let mut consider_upgrade: bool = false;
        // check if a target has been selected
        if let Some(target) = target {
            //check if the base would die until the attack reaches its target
            if !base.will_die_within_in_n_ticks(base.distance_to(&target.base), &game_state.config, &game_state.actions) {
                // attack
                attacks.push(PlayerAction {
                    src: base.uid,
                    dest: target.base.uid,
                    amount: target.required_bits + 3,
                });
            }
            // consider upgrade
            else {
                consider_upgrade = true;
            }
        }
        // consider upgrade
        else {
            consider_upgrade = true;
        }

        // check if the max population is at its limit and no attacks were made
        if consider_upgrade && base.population > game_state.config.base_levels[base.level as usize].max_population - 1 {
            // upgrade with all bits over limit
            attacks.push(PlayerAction {
                src: base.uid,
                dest: base.uid,
                amount: base.population - (game_state.config.base_levels[base.level as usize].max_population - 1),
            });
        }
    });

    // return attacks
    return attacks;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decide_test() {
        let want = vec![PlayerAction::default()];

        let result = decide(GameState::default());

        assert!(want == result)
    }
}
