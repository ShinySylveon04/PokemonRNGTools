#![feature(iter_order_by)]
use js_sys::Array;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use wasm_bindgen::prelude::*;

extern crate console_error_panic_hook;

mod bdsp;
mod enums;
mod rng;
mod swsh;

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

fn check_is_shiny(tsv: u16, rand: u32) -> bool {
    let psv = calculate_shiny_value((rand >> 0x10) as u16, (rand & 0xFFFF) as u16);
    (tsv ^ psv) < 0x10
}

fn calculate_shiny_value(first: u16, second: u16) -> u16 {
    first ^ second
}

#[wasm_bindgen]
pub fn get_wild(settings: &JsValue) -> JsValue {
    init_panic_hook();
    let parsed_settings: bdsp::wild::settings::Settings = settings.into_serde().unwrap();

    let results = bdsp::wild::generator::generate_wild(parsed_settings);

    JsValue::from_serde(&results).unwrap()
}

#[wasm_bindgen]
pub fn get_bdsp_tid(settings: &JsValue) -> JsValue {
    init_panic_hook();
    let parsed_settings: bdsp::tid::settings::Settings = settings.into_serde().unwrap();

    let results = bdsp::tid::generator::generate_tid(parsed_settings);

    JsValue::from_serde(&results).unwrap()
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Pokemon {
    shiny_type: enums::Shiny,
    ec: u32,
    pid: u32,
    nature: enums::Nature,
    ability: enums::Ability,
}

#[wasm_bindgen(getter_with_clone)]
pub struct ShinyResult {
    pub state0: u64,
    pub state1: u64,
    pub advances: u32,
    pub shiny_value: enums::Shiny,
    pub ec: u32,
    pub pid: u32,
    pub nature: enums::Nature,
    pub ability: enums::Ability,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ShinyResultBdspStationary {
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

pub fn filter(
    results: Pokemon,
    shiny_filter: enums::ShinyFilter,
    nature_filter: enums::NatureFilter,
    ability_filter: enums::AbilityFilter,
) -> bool {
    shiny_filter == results.shiny_type
        && nature_filter == results.nature
        && ability_filter == results.ability
}

pub fn filter_bdsp_stationary(
    results: &bdsp::stationary::Pokemon,
    shiny_filter: enums::ShinyFilter,
    natures: &[enums::NatureFilter],
    ability_filter: enums::AbilityFilter,
    gender_filter: enums::GenderFilter,
    min_ivs: &Vec<u32>,
    max_ivs: &Vec<u32>,
) -> bool {
    return ability_filter == results.ability
        && natures.iter().any(|nature| *nature == results.nature)
        && gender_filter == results.gender
        && shiny_filter == results.shiny
        && results
            .ivs
            .iter()
            .eq_by(min_ivs, |&iv, &min_iv| iv >= min_iv)
        && results
            .ivs
            .iter()
            .eq_by(max_ivs, |&iv, &max_iv| iv <= max_iv);
}

pub fn filter_bdsp_underground(
    results: &bdsp::underground::generator::Pokemon,
    shiny_filter: enums::ShinyFilter,
    natures: &[enums::NatureFilter],
    ability_filter: enums::AbilityFilter,
    gender_filter: enums::GenderFilter,
    min_ivs: &Vec<u32>,
    max_ivs: &Vec<u32>,
) -> bool {
    return ability_filter == results.ability
        && natures.iter().any(|nature| *nature == results.nature)
        && gender_filter == results.gender
        && shiny_filter == results.shiny_value
        && results
            .ivs
            .iter()
            .eq_by(min_ivs, |&iv, &min_iv| iv >= min_iv)
        && results
            .ivs
            .iter()
            .eq_by(max_ivs, |&iv, &max_iv| iv <= max_iv);
}

#[wasm_bindgen]
pub fn calculate_pokemon(
    seed1: u64,
    seed2: u64,
    tid: u16,
    sid: u16,
    shiny_filter: enums::ShinyFilter,
    encounter_type: enums::EncounterFilter,
    shiny_charm: bool,
    nature_filter: enums::NatureFilter,
    ability_filter: enums::AbilityFilter,
    min: u32,
    max: u32,
) -> Array {
    let mut rng = rng::Xoroshiro::from_state(seed1, seed2);
    let mut pokemon_results;
    let mut shiny_results: Vec<ShinyResult> = Vec::new();
    let values = min..=max;
    for value in values {
        pokemon_results = match encounter_type {
            enums::EncounterFilter::Static => {
                swsh::generate_static_pokemon(rng, tid, sid, shiny_charm)
            }
            enums::EncounterFilter::Dynamic => {
                swsh::generate_dynamic_pokemon(rng, tid, sid, shiny_charm)
            }
        };

        if filter(pokemon_results, shiny_filter, nature_filter, ability_filter) {
            let shiny_state = rng.get_state();
            let result = ShinyResult {
                state0: shiny_state.0,
                state1: shiny_state.1,
                advances: value,
                shiny_value: pokemon_results.shiny_type,
                ec: pokemon_results.ec,
                pid: pokemon_results.pid,
                nature: pokemon_results.nature,
                ability: pokemon_results.ability,
            };
            shiny_results.push(result);
        }
        rng.next();
    }

    shiny_results.into_iter().map(JsValue::from).collect()
}

#[wasm_bindgen]
pub fn calculate_pokemon_bdsp_stationary(
    seed1: u32,
    seed2: u32,
    seed3: u32,
    seed4: u32,
    shiny_filter: enums::ShinyFilter,
    min: usize,
    max: usize,
    delay: usize,
    nature_filter: Vec<u32>,
    ability_filter: enums::AbilityFilter,
    gender_ratio: enums::GenderRatio,
    gender_filter: enums::GenderFilter,
    set_ivs: bool,
    min_ivs: Vec<u32>,
    max_ivs: Vec<u32>,
    lead: enums::LeadFilter,
) -> JsValue {
    let natures: Vec<enums::NatureFilter> = nature_filter
        .iter()
        .map(|nature| enums::NatureFilter::try_from(*nature).unwrap_or(enums::NatureFilter::Hardy))
        .collect();
    let mut rng = rng::Xorshift::from_state([seed1, seed2, seed3, seed4]);
    rng.advance(delay);
    let mut pokemon_results;
    let mut shiny_results: Vec<ShinyResultBdspStationary> = Vec::new();
    let values = min..=max;
    rng.advance(min);
    for value in values {
        pokemon_results = bdsp::stationary::generate_pokemon(rng, gender_ratio, set_ivs, lead);

        if filter_bdsp_stationary(
            &pokemon_results,
            shiny_filter,
            &natures,
            ability_filter,
            gender_filter,
            &min_ivs,
            &max_ivs,
        ) {
            let shiny_state = rng.get_state();
            let result = ShinyResultBdspStationary {
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
            };
            shiny_results.push(result);
        }
        rng.next();
    }

    let results: Vec<ShinyResultBdspStationary> = shiny_results.into_iter().collect();

    JsValue::from_serde(&results).unwrap()
}

#[wasm_bindgen]
pub fn calculate_pokemon_bdsp_underground(
    seed1: u32,
    seed2: u32,
    seed3: u32,
    seed4: u32,
    shiny_filter: enums::ShinyFilter,
    min: usize,
    max: usize,
    delay: usize,
    nature_filter: Vec<u32>,
    ability_filter: enums::AbilityFilter,
    _encounter_filter: enums::EncounterSlotFilter,
    gender_filter: enums::GenderFilter,
    _tiles: usize,
    diglett_boost: bool,
    min_ivs: Vec<u32>,
    max_ivs: Vec<u32>,
    version: u8,
    room: u8,
    story_flag: u8,
) -> JsValue {
    init_panic_hook();
    let min_ivs = [
        min_ivs[0] as u8,
        min_ivs[1] as u8,
        min_ivs[2] as u8,
        min_ivs[3] as u8,
        min_ivs[4] as u8,
        min_ivs[5] as u8,
    ];
    let max_ivs = [
        max_ivs[0] as u8,
        max_ivs[1] as u8,
        max_ivs[2] as u8,
        max_ivs[3] as u8,
        max_ivs[4] as u8,
        max_ivs[5] as u8,
    ];

    let version = if version == 2 {
        bdsp_ug_generator::Version::BD
    } else {
        bdsp_ug_generator::Version::SP
    };

    let room = match room {
        2 => bdsp_ug_generator::RoomType::SpaciousCave,
        3 => bdsp_ug_generator::RoomType::GrasslandCave,
        4 => bdsp_ug_generator::RoomType::FountainspringCave,
        5 => bdsp_ug_generator::RoomType::RockyCave,
        6 => bdsp_ug_generator::RoomType::VolcanicCave,
        7 => bdsp_ug_generator::RoomType::SwampyCave,
        8 => bdsp_ug_generator::RoomType::DazzlingCave,
        9 => bdsp_ug_generator::RoomType::WhiteoutCave,
        10 => bdsp_ug_generator::RoomType::IcyCave,
        11 => bdsp_ug_generator::RoomType::RiverbankCave,
        12 => bdsp_ug_generator::RoomType::SandsearCave,
        13 => bdsp_ug_generator::RoomType::StillWaterCavern,
        14 => bdsp_ug_generator::RoomType::SunlitCavern,
        15 => bdsp_ug_generator::RoomType::BigBluffCavern,
        16 => bdsp_ug_generator::RoomType::StargleamCavern,
        17 => bdsp_ug_generator::RoomType::GlacialCavern,
        18 => bdsp_ug_generator::RoomType::BogsunkCavern,
        _ => bdsp_ug_generator::RoomType::TyphloCavern,
    };

    let ability = if ability_filter == enums::AbilityFilter::Any {
        None
    } else {
        Some(ability_filter as u8)
    };

    let nature = if !nature_filter.contains(&25) {
        Some(
            nature_filter
                .into_iter()
                .map(|i| i as u8)
                .collect::<Vec<u8>>(),
        )
    } else {
        None
    };

    let gender = match gender_filter {
        enums::GenderFilter::Any => None,
        enums::GenderFilter::Female => Some(1),
        enums::GenderFilter::Male => Some(0),
    };

    let shiny = shiny_filter != enums::ShinyFilter::Any;

    let filter = bdsp_ug_generator::Filter {
        shiny,
        species: None,
        min_ivs,
        max_ivs,
        ability,
        nature,
        item: None,
        egg_move: None,
        gender,
    };

    let mut rng = bdsp_ug_generator::xorshift::XorShift::from_state([seed1, seed2, seed3, seed4]);
    rng.advance(delay);
    rng.advance(min);

    let results = bdsp_ug_generator::run_results(
        max as u32,
        rng,
        version,
        story_flag,
        room,
        filter,
        diglett_boost,
    );

    let results: Vec<bdsp::underground::generator::Pokemon> = results
        .into_iter()
        .map(|a| {
            let mut advance = Vec::with_capacity(a.regular_pokemon.len() + 1);
            for p in a.regular_pokemon.iter() {
                advance.push(bdsp::underground::generator::Pokemon {
                    shiny_value: if p.shiny {
                        enums::Shiny::Star
                    } else {
                        enums::Shiny::None
                    },
                    pid: p.pid,
                    ec: p.ec,
                    nature: enums::Nature::try_from(p.nature as u32).unwrap(),
                    ivs: p.ivs.iter().map(|i| *i as u32).collect(),
                    ability: enums::Ability::try_from(p.ability as u32).unwrap(),
                    gender: match p.gender {
                        0 => enums::Gender::Male,
                        1 => enums::Gender::Female,
                        _ => enums::Gender::Genderless,
                    },
                    encounter: p.species as u32,
                    advances: a.advance as usize + min,
                    is_rare: false,
                })
            }
            if let Some(p) = &a.rare_pokemon {
                advance.push(bdsp::underground::generator::Pokemon {
                    shiny_value: if p.shiny {
                        enums::Shiny::Star
                    } else {
                        enums::Shiny::None
                    },
                    pid: p.pid,
                    ec: p.ec,
                    nature: enums::Nature::try_from(p.nature as u32).unwrap(),
                    ivs: p.ivs.iter().map(|i| *i as u32).collect(),
                    ability: enums::Ability::try_from(p.ability as u32).unwrap(),
                    gender: match p.gender {
                        0 => enums::Gender::Male,
                        1 => enums::Gender::Female,
                        _ => enums::Gender::Genderless,
                    },
                    encounter: p.species as u32,
                    advances: a.advance as usize + min,
                    is_rare: false,
                })
            }
            advance
        })
        .flatten()
        .collect::<Vec<bdsp::underground::generator::Pokemon>>();

    JsValue::from_serde(&results).unwrap()
}

#[wasm_bindgen]
pub fn calculate_pokemon_bdsp_roamer(
    seed1: u32,
    seed2: u32,
    seed3: u32,
    seed4: u32,
    shiny_filter: enums::ShinyFilter,
    min: usize,
    max: usize,
    delay: usize,
    nature_filter: Vec<u32>,
    ability_filter: enums::AbilityFilter,
    gender_ratio: enums::GenderRatio,
    gender_filter: enums::GenderFilter,
    set_ivs: bool,
    min_ivs: Vec<u32>,
    max_ivs: Vec<u32>,
) -> JsValue {
    let natures: Vec<enums::NatureFilter> = nature_filter
        .iter()
        .map(|nature| enums::NatureFilter::try_from(*nature).unwrap_or(enums::NatureFilter::Hardy))
        .collect();
    let mut rng = rng::Xorshift::from_state([seed1, seed2, seed3, seed4]);
    rng.advance(delay);
    let mut pokemon_results;
    let mut shiny_results: Vec<ShinyResultBdspStationary> = Vec::new();
    let values = min..=max;
    rng.advance(min);
    for value in values {
        pokemon_results = bdsp::roamer::generate_pokemon(rng, gender_ratio, set_ivs);

        if filter_bdsp_stationary(
            &pokemon_results,
            shiny_filter,
            &natures,
            ability_filter,
            gender_filter,
            &min_ivs,
            &max_ivs,
        ) {
            let shiny_state = rng.get_state();
            let result = ShinyResultBdspStationary {
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
            };
            shiny_results.push(result);
        }
        rng.next();
    }

    let results: Vec<ShinyResultBdspStationary> = shiny_results.into_iter().collect();

    JsValue::from_serde(&results).unwrap()
}
