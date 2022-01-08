use crate::enums;
use crate::rng::Xorshift;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pokemon {
    pub shiny_value: enums::Shiny,
    pub pid: u32,
    pub ec: u32,
    pub nature: enums::Nature,
    pub ivs: Vec<u32>,
    pub ability: enums::Ability,
    pub gender: enums::Gender,
    pub encounter: u8,
    pub advances: usize,
    pub is_rare: bool,
}

pub fn generate_pokemon(
    mut rng: Xorshift,
    gender_ratio: enums::GenderRatio,
    advances: usize,
    tiles: usize,
    large_room: bool,
    diglett_boost: bool,
) -> Vec<Pokemon> {
    let mut results: Vec<Pokemon> = Vec::new();
    let rare_check = rng.rand_range(0, 100);

    if rare_check < 50 {
        rng.next();
    }

    let min_max_rand = rng.rand_range(0, 100);

    let mut poke_num = 5;
    if large_room {
        poke_num = 7;
    }

    let tile_boost = match tiles {
        0 => 0,
        1..=15 => 5,
        16..=30 => 10,
        31..=45 => 15,
        tiles if tiles > 60 => 30,
        _ => 20,
    };

    if 50 - tile_boost as u32 <= min_max_rand {
        if large_room {
            poke_num = 10;
        } else {
            poke_num = 7;
        }
    }

    if rare_check < 50 {
        poke_num = poke_num - 1;
    }

    rng.advance(poke_num * 2);

    let values = 0..poke_num;

    for _ in values {
        let pokemon_results =
            generate_underground_pokemon(&mut rng, gender_ratio, advances, diglett_boost);
        results.push(pokemon_results);
    }

    if rare_check < 50 {
        let pokemon_results =
            generate_rare_underground_pokemon(&mut rng, gender_ratio, advances, diglett_boost);
        results.push(pokemon_results);
    }

    fn generate_underground_pokemon(
        rng: &mut Xorshift,
        gender_ratio: enums::GenderRatio,
        advances: usize,
        diglett_boost: bool,
    ) -> Pokemon {
        rng.next(); // slot weight call?
        rng.next(); // level
        let mut shiny = enums::Shiny::None;
        let ec = rng.next();

        let shiny_rolls = if diglett_boost { 2 } else { 1 };
        let mut pid = 0;

        let shiny_rand = rng.next();
        for _ in 0..shiny_rolls {
            pid = rng.next();
            let psv = shiny_rand & 0xFFFF ^ shiny_rand >> 0x10;
            let tsv = pid >> 0x10 ^ pid & 0xFFFF;
            if (psv ^ tsv) < 0x10 {
                shiny = enums::Shiny::Star;
                break;
            }

            if (psv ^ tsv) == 0 {
                shiny = enums::Shiny::Square;
                break;
            }
        }

        let mut ivs = vec![32, 32, 32, 32, 32, 32];
        for i in ivs.iter_mut() {
            *i = rng.rand_max(32);
        }

        let ability_rand = rng.next();
        let ability = ability_rand - (ability_rand / 2) * 2;

        let gender = match enums::get_set_gender_from_ratio(&gender_ratio) {
            Some(set_gender) => set_gender,
            None => {
                let gender_rand = rng.next();
                let gender_num = (gender_rand - (gender_rand / 253) * 253) + 1;
                enums::get_gender_from_ratio(&gender_ratio, gender_num)
            }
        };
        let nature_rand = rng.next();
        let nature = nature_rand - (nature_rand / 25) * 25;
        rng.next(); // height 1
        rng.next(); // height 2
        rng.next(); // weight 1
        rng.next(); // weight 2
        rng.next(); // item
        rng.next(); // egg move
                    // randNum between 0 and max egg moves, then use as index for egg move
        let encounter = 0;

        Pokemon {
            shiny_value: shiny,
            pid,
            ec,
            nature: enums::Nature::try_from(nature).unwrap_or(enums::Nature::Hardy),
            ivs,
            ability: enums::Ability::try_from(ability).unwrap_or(enums::Ability::Ability0),
            gender,
            encounter,
            advances,
            is_rare: false,
        }
    }

    fn generate_rare_underground_pokemon(
        rng: &mut Xorshift,
        gender_ratio: enums::GenderRatio,
        advances: usize,
        diglett_boost: bool,
    ) -> Pokemon {
        rng.next(); // level
        let mut shiny = enums::Shiny::None;
        let ec = rng.next();

        let shiny_rolls = if diglett_boost { 2 } else { 1 };
        let mut pid = 0;

        let shiny_rand = rng.next();
        for _ in 0..shiny_rolls {
            pid = rng.next();
            let psv = shiny_rand & 0xFFFF ^ shiny_rand >> 0x10;
            let tsv = pid >> 0x10 ^ pid & 0xFFFF;
            if (psv ^ tsv) < 0x10 {
                shiny = enums::Shiny::Star;
                break;
            }

            if (psv ^ tsv) == 0 {
                shiny = enums::Shiny::Square;
                break;
            }
        }

        let mut ivs = vec![32, 32, 32, 32, 32, 32];
        for i in ivs.iter_mut() {
            *i = rng.rand_max(32);
        }

        let ability_rand = rng.next();
        let ability = ability_rand - (ability_rand / 2) * 2;

        let gender = match enums::get_set_gender_from_ratio(&gender_ratio) {
            Some(set_gender) => set_gender,
            None => {
                let gender_rand = rng.next();
                let gender_num = (gender_rand - (gender_rand / 253) * 253) + 1;
                enums::get_gender_from_ratio(&gender_ratio, gender_num)
            }
        };
        let nature_rand = rng.next();
        let nature = nature_rand - (nature_rand / 25) * 25;
        rng.next(); // height 1
        rng.next(); // height 2
        rng.next(); // weight 1
        rng.next(); // weight 2
        rng.next(); // item
        rng.next(); // egg move
        let encounter = 0;

        Pokemon {
            shiny_value: shiny,
            pid,
            ec,
            nature: enums::Nature::try_from(nature).unwrap_or(enums::Nature::Hardy),
            ivs,
            ability: enums::Ability::try_from(ability).unwrap_or(enums::Ability::Ability0),
            gender,
            encounter,
            advances,
            is_rare: true,
        }
    }

    results
}
