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

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TIDResult {
    pub state0: u32,
    pub state1: u32,
    pub state2: u32,
    pub state3: u32,
    pub advances: usize,
    pub tid: u16,
    pub tsv: u16,
    pub g8tid: u32,
    pub sid: u16,
    pub filter_type: enums::IDFilter,
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

pub fn filter_tid(results: &bdsp::tid::Tid, id: &[u32], filter_type: enums::IDFilter) -> bool {
    if filter_type == enums::IDFilter::None {
        true
    } else {
        let filter_id = match filter_type {
            enums::IDFilter::G8TID => results.g8tid,
            enums::IDFilter::SID => results.sid as u32,
            enums::IDFilter::TID => results.tid as u32,
            enums::IDFilter::TSV => results.tsv as u32,
            enums::IDFilter::None => 0, // shouldn't hit this value because of if above
            _ => 0,
        };

        id.contains(&filter_id)
    }
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
    gender_ratio: enums::GenderRatio,
    gender_filter: enums::GenderFilter,
    tiles: usize,
    large_room: bool,
    diglett_boost: bool,
    min_ivs: Vec<u32>,
    max_ivs: Vec<u32>,
) -> JsValue {
    init_panic_hook();
    let natures: Vec<enums::NatureFilter> = nature_filter
        .iter()
        .map(|nature| enums::NatureFilter::try_from(*nature).unwrap_or(enums::NatureFilter::Hardy))
        .collect();
    let mut rng = rng::Xorshift::from_state([seed1, seed2, seed3, seed4]);
    rng.advance(delay);
    let mut pokemon_results = Vec::new();
    let values = min..=max;
    rng.advance(min);
    for value in values {
        let mut result = bdsp::underground::generator::generate_pokemon(
            rng,
            gender_ratio,
            value,
            tiles,
            large_room,
            diglett_boost,
        );

        if result.iter().any(|pokemon| {
            filter_bdsp_underground(
                pokemon,
                shiny_filter,
                &natures,
                ability_filter,
                gender_filter,
                &min_ivs,
                &max_ivs,
            )
        }) {
            pokemon_results.append(&mut result);
        }
        rng.next();
    }

    let results: Vec<bdsp::underground::generator::Pokemon> = pokemon_results.into_iter().collect();

    JsValue::from_serde(&results).unwrap()
}

#[wasm_bindgen]
pub fn calculate_tid(
    seed1: u32,
    seed2: u32,
    seed3: u32,
    seed4: u32,
    min: usize,
    max: usize,
    id: Vec<u32>,
    filter_type: enums::IDFilter,
) -> JsValue {
    let mut rng = rng::Xorshift::from_state([seed1, seed2, seed3, seed4]);
    let mut tid_results;
    let mut results: Vec<TIDResult> = Vec::new();
    let values = min..=max;
    rng.advance(min);
    for value in values {
        tid_results = bdsp::tid::generate_tid(rng);

        if filter_tid(&tid_results, &id, filter_type) {
            let result_state = rng.get_state();
            let result = TIDResult {
                state0: result_state[0],
                state1: result_state[1],
                state2: result_state[2],
                state3: result_state[3],
                advances: value,
                tid: tid_results.tid,
                tsv: tid_results.tsv,
                g8tid: tid_results.g8tid,
                sid: tid_results.sid,
                filter_type,
            };
            results.push(result);
        }
        rng.next();
    }

    let results: Vec<TIDResult> = results.into_iter().collect();

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
