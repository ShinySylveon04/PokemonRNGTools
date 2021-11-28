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
) -> PokemonbdspStationary {
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
}

pub fn generate_bdsp_pokemon_underground(
    mut rng: Xorshift,
    gender_ratio: enums::GenderRatioEnum,
    advances: usize,
) -> Vec<UndergroundResults> {
    let mut results: Vec<UndergroundResults> = Vec::new();
    let rare_check = rng.rand_range(0, 100);

    if rare_check < 50 {
        rng.next();
    }

    let secret_base_tiles_used = 0;

    let min_max_rand = rng.rand_range(0, 100);

    let mut poke_num = 5;

    if 50 - secret_base_tiles_used <= min_max_rand {
        poke_num = 7;
    }

    if rare_check < 50 {
        poke_num = poke_num - 1;
    }

    rng.advance(poke_num * 2);

    let values = 0..poke_num;

    for _ in values {
        let pokemon_results = generate_underground_pokemon(&mut rng, gender_ratio, advances);
        results.push(pokemon_results);
    }

    if rare_check < 50 {
        let pokemon_results = generate_rare_underground_pokemon(&mut rng, gender_ratio, advances);
        results.push(pokemon_results);
    }

    fn generate_underground_pokemon(
        rng: &mut Xorshift,
        gender_ratio: enums::GenderRatioEnum,
        advances: usize,
    ) -> UndergroundResults {
        rng.next(); // slot weight call?
        rng.next(); // level
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
        }
    }

    fn generate_rare_underground_pokemon(
        rng: &mut Xorshift,
        gender_ratio: enums::GenderRatioEnum,
        advances: usize,
    ) -> UndergroundResults {
        rng.next(); // level
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
        }
    }

    results
}
