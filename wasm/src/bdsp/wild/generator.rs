use super::settings::Settings;
use crate::rng::Xorshift;
use crate::{enums, rng};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Pokemon {
    pub shiny: enums::Shiny,
    pub pid: u32,
    pub ec: u32,
    pub nature: enums::Nature,
    pub ivs: Vec<u32>,
    pub ability: enums::Ability,
    pub gender: enums::Gender,
    pub encounter: u8,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Debug, Serialize, Deserialize)]
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
    pub encounter: u8,
}

type IVs = Vec<u32>;
fn check_ivs(ivs: &IVs, min_ivs: &IVs, max_ivs: &IVs) -> bool {
    ivs.iter()
        .zip(min_ivs.iter())
        .zip(max_ivs.iter())
        .all(|((iv, min), max)| min <= iv && iv <= max)
}

fn generate_pokemon(mut rng: Xorshift, settings: &Settings) -> Option<Pokemon> {
    let encounter_rand = rng.rand_range(0, 100) as u8;
    rng.advance(84);
    let ec = rng.next();
    let shiny_rand = rng.next();
    let pid = rng.next();
    let shiny = enums::Shiny::from_pid_shiny_rand(pid, shiny_rand);

    if settings.shiny_filter != shiny {
        return None;
    }

    let mut ivs = vec![32, 32, 32, 32, 32, 32];
    for i in ivs.iter_mut() {
        *i = rng.rand_max(32);
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
            enums::Nature::try_from(nature_rand - (nature_rand / 25) * 25)
                .unwrap_or(enums::Nature::Hardy)
        }
    };

    let natures: Vec<enums::NatureFilter> = settings
        .nature_filter
        .iter()
        .map(|nature| enums::NatureFilter::try_from(*nature).unwrap_or(enums::NatureFilter::Hardy))
        .collect();

    if !natures.iter().any(|nat| *nat == nature) {
        return None;
    }

    let encounter_slots: [u8; 12] = [20, 40, 50, 60, 70, 80, 85, 90, 94, 98, 99, 100];

    let encounter = encounter_slots
        .iter()
        .position(|enc| encounter_rand < *enc)
        .unwrap_or(0) as u8;

    let encounters: Vec<enums::EncounterSlotFilter> = settings
        .encounter_filter
        .iter()
        .map(|encounter| {
            enums::EncounterSlotFilter::try_from(*encounter)
                .unwrap_or(enums::EncounterSlotFilter::Slot0)
        })
        .collect();

    if !encounters.iter().any(|slot| *slot == encounter) {
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
        encounter,
    })
}

pub fn generate_wild(settings: Settings) -> Vec<Result> {
    let states: [u32; 4] = [
        settings.rng_state[0],
        settings.rng_state[1],
        settings.rng_state[2],
        settings.rng_state[3],
    ];
    let mut rng = rng::Xorshift::from_state(states);
    rng.advance(settings.delay);
    let mut shiny_results: Vec<Result> = Vec::new();
    let values = settings.min..=settings.max;
    rng.advance(settings.min);
    for value in values {
        let generate_result = generate_pokemon(rng.clone(), &settings);
        if let Some(pokemon) = generate_result {
            let shiny_state = rng.get_state();
            let result = Result {
                state0: shiny_state[0],
                state1: shiny_state[1],
                state2: shiny_state[2],
                state3: shiny_state[3],
                advances: value,
                pid: pokemon.pid,
                shiny_value: pokemon.shiny,
                ec: pokemon.ec,
                nature: pokemon.nature,
                ivs: pokemon.ivs,
                ability: pokemon.ability,
                gender: pokemon.gender,
                encounter: pokemon.encounter,
            };
            shiny_results.push(result);
        }

        rng.next();
    }

    let results: Vec<Result> = shiny_results.into_iter().collect();

    results
}
