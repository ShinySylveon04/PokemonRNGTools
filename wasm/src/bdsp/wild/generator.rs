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

fn generate_pokemon(
    mut rng: Xorshift,
    gender_ratio: enums::GenderRatio,
    lead: enums::LeadFilter,
) -> Pokemon {
    let encounter_rand = rng.rand_range(0, 100) as u8;
    rng.advance(84);
    let mut shiny = enums::Shiny::None;
    let ec = rng.next();
    let shiny_rand = rng.next();
    let pid = rng.next();

    let psv = shiny_rand & 0xFFFF ^ shiny_rand >> 0x10;
    let tsv = pid >> 0x10 ^ pid & 0xFFFF;
    if (psv ^ tsv) < 0x10 {
        if (psv ^ tsv) == 0 {
            shiny = enums::Shiny::Square
        } else {
            shiny = enums::Shiny::Star
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

    let nature;
    if lead != enums::LeadFilter::Synchronize {
        let nature_rand = rng.next();
        nature = nature_rand - (nature_rand / 25) * 25;
    } else {
        nature = 25;
    }

    let encounter_slots: [u8; 12] = [20, 40, 50, 60, 70, 80, 85, 90, 94, 98, 99, 100];

    let encounter = encounter_slots
        .iter()
        .position(|enc| encounter_rand < *enc)
        .unwrap_or(0) as u8;

    Pokemon {
        shiny,
        pid,
        ec,
        nature: enums::Nature::try_from(nature).unwrap_or(enums::Nature::Hardy),
        ivs,
        ability: enums::Ability::try_from(ability).unwrap_or(enums::Ability::Ability0),
        gender,
        encounter,
    }
}

fn filter(
    results: &Pokemon,
    shiny_filter: enums::ShinyFilter,
    natures: &Vec<enums::NatureFilter>,
    ability_filter: enums::AbilityFilter,
    encounters: &Vec<enums::EncounterSlotFilter>,
    gender_filter: enums::GenderFilter,
    min_ivs: &Vec<u32>,
    max_ivs: &Vec<u32>,
) -> bool {
    if ability_filter == results.ability
        && natures.iter().any(|nature| *nature == results.nature)
        && encounters
            .iter()
            .any(|encounter| *encounter == results.encounter)
        && gender_filter == results.gender
        && shiny_filter == results.shiny
        && results
            .ivs
            .iter()
            .eq_by(min_ivs, |&iv, &min_iv| iv >= min_iv)
        && results
            .ivs
            .iter()
            .eq_by(max_ivs, |&iv, &max_iv| iv <= max_iv)
    {
        return true;
    } else {
        return false;
    }
}

pub fn generate_wild(settings: Settings) -> Vec<Result> {
    let natures: Vec<enums::NatureFilter> = settings
        .nature_filter
        .iter()
        .map(|nature| enums::NatureFilter::try_from(*nature).unwrap_or(enums::NatureFilter::Hardy))
        .collect();
    let encounters: Vec<enums::EncounterSlotFilter> = settings
        .encounter_filter
        .iter()
        .map(|encounter| {
            enums::EncounterSlotFilter::try_from(*encounter)
                .unwrap_or(enums::EncounterSlotFilter::Slot0)
        })
        .collect();
    let states: [u32; 4] = [
        settings.rng_state[0],
        settings.rng_state[1],
        settings.rng_state[2],
        settings.rng_state[3],
    ];
    let mut rng = rng::Xorshift::from_state(states);
    rng.advance(settings.delay);
    let mut pokemon_results;
    let mut shiny_results: Vec<Result> = Vec::new();
    let values = settings.min..=settings.max;
    rng.advance(settings.min);
    for value in values {
        pokemon_results = generate_pokemon(rng.clone(), settings.gender_ratio, settings.lead);

        if filter(
            &pokemon_results,
            settings.shiny_filter,
            &natures,
            settings.ability_filter,
            &encounters,
            settings.gender_filter.into(),
            &settings.min_ivs,
            &settings.max_ivs,
        ) {
            let shiny_state = rng.get_state();
            let result = Result {
                state0: shiny_state[0],
                state1: shiny_state[1],
                state2: shiny_state[2],
                state3: shiny_state[3],
                advances: value,
                pid: pokemon_results.pid,
                shiny_value: pokemon_results.shiny,
                ec: pokemon_results.ec,
                nature: pokemon_results.nature,
                ivs: pokemon_results.ivs,
                ability: pokemon_results.ability,
                gender: pokemon_results.gender,
                encounter: pokemon_results.encounter,
            };
            shiny_results.push(result);
        }
        rng.next();
    }

    let results: Vec<Result> = shiny_results.into_iter().collect();

    results
}
