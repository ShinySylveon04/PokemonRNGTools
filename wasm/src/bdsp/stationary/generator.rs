use super::settings::Settings;
use crate::bdsp::roamer;
use crate::enums;
use crate::rng::Xorshift;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Result {
    pub state0: u32,
    pub state1: u32,
    pub state2: u32,
    pub state3: u32,
    pub advances: usize,
    pub shiny_value: enums::Shiny,
    pub pid: u32,
    pub ec: u32,
    pub nature: enums::Nature,
    pub ivs: Vec<u32>,
    pub ability: enums::Ability,
    pub gender: enums::Gender,
}

pub fn generate_stationary(settings: Settings) -> Vec<Result> {
    let states: [u32; 4] = [
        settings.rng_state[0],
        settings.rng_state[1],
        settings.rng_state[2],
        settings.rng_state[3],
    ];

    let mut rng = Xorshift::from_state(states);
    rng.advance(settings.delay);
    let mut results: Vec<Result> = Vec::new();
    let values = settings.min_advances..=settings.max_advances;
    rng.advance(settings.min_advances);

    for value in values {
        let generate_result = match settings.is_roamer {
            true => roamer::generate_pokemon(rng, &settings),
            false => generate_pokemon(rng, &settings),
        };

        if let Some(pokemon) = generate_result {
            let rng_state = rng.get_state();
            let result = Result {
                state0: rng_state[0],
                state1: rng_state[1],
                state2: rng_state[2],
                state3: rng_state[3],
                advances: value,
                pid: pokemon.pid,
                shiny_value: pokemon.shiny,
                ec: pokemon.ec,
                nature: pokemon.nature,
                ivs: pokemon.ivs,
                ability: pokemon.ability,
                gender: pokemon.gender,
            };
            results.push(result);
        }
        rng.next();
    }

    results.into_iter().collect()
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Pokemon {
    pub shiny: enums::Shiny,
    pub pid: u32,
    pub ec: u32,
    pub nature: enums::Nature,
    pub ivs: Vec<u32>,
    pub ability: enums::Ability,
    pub gender: enums::Gender,
}

type IVs = Vec<u32>;
fn check_ivs(ivs: &IVs, min_ivs: &IVs, max_ivs: &IVs) -> bool {
    ivs.iter()
        .zip(min_ivs.iter())
        .zip(max_ivs.iter())
        .all(|((iv, min), max)| min <= iv && iv <= max)
}

pub fn generate_pokemon(mut rng: Xorshift, settings: &Settings) -> Option<Pokemon> {
    let ec = rng.next();
    let shiny_rand = rng.next();
    let pid = rng.next();

    let shiny = enums::Shiny::from_pid_shiny_rand(pid, shiny_rand);

    if settings.shiny_filter != shiny {
        return None;
    }

    let mut ivs = vec![32, 32, 32, 32, 32, 32];

    if settings.set_ivs {
        for _ in 0..3 {
            let mut index;
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

    if !check_ivs(&ivs, &settings.min_ivs, &settings.max_ivs) {
        return None;
    }

    let ability_rand = rng.next();
    let ability = enums::Ability::try_from(ability_rand - (ability_rand / 2) * 2)
        .unwrap_or(enums::Ability::Ability0);

    if settings.ability_filter != ability {
        return None;
    }

    let gender = match enums::get_set_gender_from_ratio(&settings.gender_ratio) {
        Some(set_gender) => set_gender,
        None => {
            let gender_rand = rng.next();
            let gender_num = (gender_rand - (gender_rand / 253) * 253) + 1;
            enums::get_gender_from_ratio(&settings.gender_ratio, gender_num)
        }
    };

    if settings.gender_filter != gender {
        return None;
    }

    let nature = match enums::get_sync_nature(&settings.lead_filter) {
        Some(set_nature) => set_nature,
        None => {
            let nature_rand = rng.next();
            enums::Nature::try_from((nature_rand - (nature_rand / 25) * 25) as u16)
                .unwrap_or(enums::Nature::Hardy)
        }
    };

    let natures: Vec<enums::NatureFilter> = settings
        .nature_filter
        .iter()
        .map(|nature| {
            enums::NatureFilter::try_from(*nature as u16).unwrap_or(enums::NatureFilter::Hardy)
        })
        .collect();

    if !natures.iter().any(|nat| *nat == nature) {
        return None;
    }

    Some(Pokemon {
        shiny,
        pid,
        ec,
        nature,
        ivs,
        ability,
        gender,
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use std::vec;

    #[test]
    fn should_generate_pokemon() {
        let mut rng = Xorshift::from_state([1, 2, 3, 4]);
        let settings = Settings {
            nature_filter: vec![25],
            rng_state: vec![1, 2, 3, 4],
            delay: 0,
            min_advances: 0,
            max_advances: 10,
            gender_ratio: enums::GenderRatio::Male50Female50,
            lead_filter: enums::LeadFilter::None,
            shiny_filter: enums::ShinyFilter::None,
            ability_filter: enums::AbilityFilter::Any,
            gender_filter: enums::GenderFilter::Any,
            min_ivs: vec![0, 0, 0, 0, 0, 0],
            max_ivs: vec![31, 31, 31, 31, 31, 31],
            set_ivs: false,
            is_roamer: false,
        };

        let expected_results = vec![
            Pokemon {
                shiny: enums::Shiny::None,
                pid: 2147483652,
                ec: 2147485709,
                nature: enums::Nature::Lax,
                ivs: vec![0, 13, 26, 14, 30, 11],
                ability: enums::Ability::Ability1,
                gender: enums::Gender::Female,
            },
            Pokemon {
                shiny: enums::Shiny::None,
                pid: 2147491872,
                ec: 2147489823,
                nature: enums::Nature::Hasty,
                ivs: vec![13, 26, 14, 30, 11, 25],
                ability: enums::Ability::Ability1,
                gender: enums::Gender::Female,
            },
            Pokemon {
                shiny: enums::Shiny::None,
                pid: 2151678029,
                ec: 2147483652,
                nature: enums::Nature::Rash,
                ivs: vec![26, 14, 30, 11, 25, 15],
                ability: enums::Ability::Ability0,
                gender: enums::Gender::Female,
            },
        ];

        for (advance, expected_result) in expected_results.iter().enumerate() {
            let result = generate_pokemon(rng.clone(), &settings);

            assert_eq!(
                result.as_ref(),
                Some(expected_result),
                "Mismatch on advance {}",
                advance
            );
            rng.next();
        }
    }

    #[test]
    fn should_generate_pokemon_with_ivs() {
        let mut rng = Xorshift::from_state([1, 2, 3, 4]);
        let settings = Settings {
            nature_filter: vec![25],
            rng_state: vec![1, 2, 3, 4],
            delay: 0,
            min_advances: 0,
            max_advances: 10,
            gender_ratio: enums::GenderRatio::Genderless,
            lead_filter: enums::LeadFilter::None,
            shiny_filter: enums::ShinyFilter::None,
            ability_filter: enums::AbilityFilter::Any,
            gender_filter: enums::GenderFilter::Any,
            min_ivs: vec![0, 0, 0, 0, 0, 0],
            max_ivs: vec![31, 31, 31, 31, 31, 31],
            set_ivs: true,
            is_roamer: false,
        };

        let expected_results = vec![
            Pokemon {
                shiny: enums::Shiny::None,
                pid: 2147483652,
                ec: 2147485709,
                nature: enums::Nature::Lax,
                ivs: vec![31, 30, 31, 11, 25, 31],
                ability: enums::Ability::Ability1,
                gender: enums::Gender::Genderless,
            },
            Pokemon {
                shiny: enums::Shiny::None,
                pid: 2147491872,
                ec: 2147489823,
                nature: enums::Nature::Lax,
                ivs: vec![31, 30, 31, 11, 25, 31],
                ability: enums::Ability::Ability1,
                gender: enums::Gender::Genderless,
            },
            Pokemon {
                shiny: enums::Shiny::None,
                pid: 2151678029,
                ec: 2147483652,
                nature: enums::Nature::Hasty,
                ivs: vec![31, 11, 31, 25, 31, 15],
                ability: enums::Ability::Ability0,
                gender: enums::Gender::Genderless,
            },
        ];

        for (advance, expected_result) in expected_results.iter().enumerate() {
            let result = generate_pokemon(rng.clone(), &settings);

            assert_eq!(
                result.as_ref(),
                Some(expected_result),
                "Mismatch on advance {}",
                advance
            );
            rng.next();
        }
    }

    #[test]
    fn should_filter_pokemon() {
        let settings = Settings {
            nature_filter: vec![25],
            rng_state: vec![1, 2, 3, 4],
            delay: 0,
            min_advances: 0,
            max_advances: 10000,
            gender_ratio: enums::GenderRatio::Male50Female50,
            lead_filter: enums::LeadFilter::None,
            shiny_filter: enums::ShinyFilter::Both,
            ability_filter: enums::AbilityFilter::Any,
            gender_filter: enums::GenderFilter::Any,
            min_ivs: vec![0, 0, 0, 0, 0, 0],
            max_ivs: vec![31, 31, 31, 31, 31, 31],
            set_ivs: false,
            is_roamer: false,
        };

        let expected_results = Result {
            state0: 2326253939,
            state1: 1975907964,
            state2: 1100255917,
            state3: 457147861,
            advances: 4481,
            shiny_value: enums::Shiny::Star,
            pid: 0x906f73f0,
            ec: 0x2fa7e388,
            nature: enums::Nature::Hasty,
            ivs: vec![5, 29, 27, 28, 11, 27],
            ability: enums::Ability::Ability1,
            gender: enums::Gender::Female,
        };

        let results = generate_stationary(settings);
        for (advance, result) in results.into_iter().enumerate() {
            assert_eq!(result, expected_results, "Mismatch on advance {}", advance);
        }
    }
}
