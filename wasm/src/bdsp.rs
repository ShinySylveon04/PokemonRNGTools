use super::enums;
use super::{Pokemonbdsp, PokemonbdspStationary, Xorshift};
use std::convert::TryFrom;
use wasm_bindgen::prelude::*;

pub fn generate_bdsp_pokemon(
    mut rng: Xorshift,
    gender_ratio: enums::GenderRatioEnum,
) -> Pokemonbdsp {
    let encounter_rand = rng.rand_range(0, 100) as u8;
    rng.advance(84);
    let mut is_shiny = false;
    let ec = rng.next();
    let shiny_rand = rng.next();
    let pid = rng.next();

    if (shiny_rand & 0xFFF0 ^ shiny_rand >> 0x10 ^ pid >> 0x10 ^ pid & 0xFFF0) < 0x10 {
        is_shiny = true
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

    let encounter_slots: [u8; 12] = [20, 40, 50, 60, 70, 80, 85, 90, 94, 98, 99, 100];

    let encounter = encounter_slots
        .iter()
        .position(|enc| encounter_rand < *enc)
        .unwrap_or(0) as u8;

    Pokemonbdsp {
        is_shiny,
        pid,
        ec,
        nature: enums::NatureEnum::try_from(nature).unwrap_or(enums::NatureEnum::Hardy),
        ivs,
        ability: enums::AbilityEnum::try_from(ability).unwrap_or(enums::AbilityEnum::Ability0),
        gender,
        encounter,
    }
}

pub fn generate_bdsp_pokemon_stationary(
    mut rng: Xorshift,
    gender_ratio: enums::GenderRatioEnum,
    set_ivs: bool,
) -> PokemonbdspStationary {
    let mut is_shiny = false;

    let ec = rng.next();
    let shiny_rand = rng.next();
    let pid = rng.next();

    if (shiny_rand & 0xFFF0 ^ shiny_rand >> 0x10 ^ pid >> 0x10 ^ pid & 0xFFF0) < 0x10 {
        is_shiny = true
    }

    let mut ivs = vec![32, 32, 32, 32, 32, 32];

    if set_ivs {
        for _ in 0..3 {
            let mut index = 0;
            loop {
                let iv_rand = rng.next();
                index = iv_rand - (iv_rand / 6) * 6;
                if ivs[index as usize] == 32 {
                    break;
                }
            }
            ivs[index as usize] = 31;
        }
    }

    for i in ivs.iter_mut() {
        if *i == 32 {
            *i = rng.rand_max(32)
        };
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

    PokemonbdspStationary {
        is_shiny,
        pid,
        ec,
        nature: enums::NatureEnum::try_from(nature).unwrap_or(enums::NatureEnum::Hardy),
        ivs,
        ability: enums::AbilityEnum::try_from(ability).unwrap_or(enums::AbilityEnum::Ability0),
        gender,
    }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Debug)]
pub struct UndergroundResults {
    pub is_shiny: bool,
    pub pid: u32,
    pub ec: u32,
    pub nature: enums::NatureEnum,
    pub ivs: Vec<u32>,
    pub ability: enums::AbilityEnum,
    pub gender: enums::GenderEnum,
    pub encounter: u8,
    pub advances: usize,
    pub is_rare: bool,
}

pub fn generate_bdsp_pokemon_underground(
    mut rng: Xorshift,
    gender_ratio: enums::GenderRatioEnum,
    advances: usize,
    tiles: usize,
    large_room: bool,
    diglett_boost: bool,
) -> Vec<UndergroundResults> {
    let mut results: Vec<UndergroundResults> = Vec::new();
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
        gender_ratio: enums::GenderRatioEnum,
        advances: usize,
        diglett_boost: bool,
    ) -> UndergroundResults {
        rng.next(); // slot weight call?
        rng.next(); // level
        let mut is_shiny = false;
        let ec = rng.next();

        let shiny_rolls = if diglett_boost { 2 } else { 1 };
        let mut pid = 0;

        let shiny_rand = rng.next();
        for _ in 0..shiny_rolls {
            pid = rng.next();
            is_shiny =
                (shiny_rand & 0xFFF0 ^ shiny_rand >> 0x10 ^ pid >> 0x10 ^ pid & 0xFFF0) < 0x10;
            if is_shiny {
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

        UndergroundResults {
            is_shiny,
            pid,
            ec,
            nature: enums::NatureEnum::try_from(nature).unwrap_or(enums::NatureEnum::Hardy),
            ivs,
            ability: enums::AbilityEnum::try_from(ability).unwrap_or(enums::AbilityEnum::Ability0),
            gender,
            encounter,
            advances,
            is_rare: false,
        }
    }

    fn generate_rare_underground_pokemon(
        rng: &mut Xorshift,
        gender_ratio: enums::GenderRatioEnum,
        advances: usize,
        diglett_boost: bool,
    ) -> UndergroundResults {
        rng.next(); // level
        let mut is_shiny = false;
        let ec = rng.next();

        let shiny_rolls = if diglett_boost { 2 } else { 1 };
        let mut pid = 0;

        let shiny_rand = rng.next();
        for _ in 0..shiny_rolls {
            pid = rng.next();
            is_shiny =
                (shiny_rand & 0xFFF0 ^ shiny_rand >> 0x10 ^ pid >> 0x10 ^ pid & 0xFFF0) < 0x10;
            if is_shiny {
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

        UndergroundResults {
            is_shiny,
            pid,
            ec,
            nature: enums::NatureEnum::try_from(nature).unwrap_or(enums::NatureEnum::Hardy),
            ivs,
            ability: enums::AbilityEnum::try_from(ability).unwrap_or(enums::AbilityEnum::Ability0),
            gender,
            encounter,
            advances,
            is_rare: true,
        }
    }

    results
}
