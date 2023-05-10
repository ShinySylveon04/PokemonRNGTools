#![feature(iter_order_by)]
use js_sys::Array;
use std::convert::TryFrom;
use wasm_bindgen::prelude::*;

extern crate console_error_panic_hook;

mod bdsp;
mod enums;
mod gen3;
mod gen6;
mod rng;
mod swsh;
mod utils;

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
pub fn get_gen3_wild(settings: &JsValue) -> JsValue {
    init_panic_hook();
    let parsed_settings: gen3::settings::Settings = settings.into_serde().unwrap();

    let results = gen3::generator::generate_wild(parsed_settings);

    JsValue::from_serde(&results).unwrap()
}

#[wasm_bindgen]
pub fn get_transporter(settings: &JsValue) -> JsValue {
    init_panic_hook();
    let parsed_settings: gen6::transporter::settings::Settings = settings.into_serde().unwrap();

    let results = gen6::transporter::generator::generate_transporter(parsed_settings);

    JsValue::from_serde(&results).unwrap()
}

// Begin BdSp functions

#[wasm_bindgen]
pub fn get_bdsp_wild(settings: &JsValue) -> JsValue {
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

#[wasm_bindgen]
pub fn get_bdsp_stationary(settings: &JsValue) -> JsValue {
    init_panic_hook();
    let parsed_settings: bdsp::stationary::settings::Settings = settings.into_serde().unwrap();

    let results = bdsp::stationary::generator::generate_stationary(parsed_settings);

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

pub fn filter(
    results: Pokemon,
    shiny_filter: enums::ShinyFilter,
    nature_filter: enums::DeprecatedNatureFilter,
    ability_filter: enums::AbilityFilter,
) -> bool {
    shiny_filter == results.shiny_type
        && nature_filter == results.nature
        && ability_filter == results.ability
}

pub fn filter_bdsp_underground(
    results: &bdsp::underground::generator::Pokemon,
    shiny_filter: enums::ShinyFilter,
    natures: &[enums::DeprecatedNatureFilter],
    ability_filter: enums::AbilityFilter,
    gender_filter: enums::DeprecatedGenderFilter,
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

pub(crate) fn _calculate_pokemon(
    seed1: u64,
    seed2: u64,
    tid: u16,
    sid: u16,
    shiny_filter: enums::ShinyFilter,
    encounter_type: enums::DeprecatedEncounterFilter,
    shiny_charm: bool,
    nature_filter: enums::DeprecatedNatureFilter,
    ability_filter: enums::AbilityFilter,
    min: u32,
    max: u32,
) -> Vec<ShinyResult> {
    let mut rng = rng::Xoroshiro::from_state(seed1, seed2);
    let mut pokemon_results;
    let mut shiny_results: Vec<ShinyResult> = Vec::new();
    let values = min..=max;
    for value in values {
        pokemon_results = match encounter_type {
            enums::DeprecatedEncounterFilter::Static => {
                swsh::generate_static_pokemon(rng, tid, sid, shiny_charm)
            }
            enums::DeprecatedEncounterFilter::Dynamic => {
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

    shiny_results
}

#[wasm_bindgen]
pub fn calculate_pokemon(
    seed1: u64,
    seed2: u64,
    tid: u16,
    sid: u16,
    shiny_filter: enums::ShinyFilter,
    encounter_type: enums::DeprecatedEncounterFilter,
    shiny_charm: bool,
    nature_filter: enums::DeprecatedNatureFilter,
    ability_filter: enums::AbilityFilter,
    min: u32,
    max: u32,
) -> Array {
    _calculate_pokemon(
        seed1,
        seed2,
        tid,
        sid,
        shiny_filter,
        encounter_type,
        shiny_charm,
        nature_filter,
        ability_filter,
        min,
        max,
    )
    .into_iter()
    .map(JsValue::from)
    .collect()
}

pub(crate) fn _calculate_pokemon_bdsp_underground(
    seed1: u32,
    seed2: u32,
    seed3: u32,
    seed4: u32,
    shiny_filter: enums::ShinyFilter,
    min_advances: usize,
    max_advances: usize,
    delay: usize,
    nature_filter: Vec<u32>,
    ability_filter: enums::AbilityFilter,
    _encounter_filter: enums::DeprecatedEncounterSlotFilter,
    gender_ratio: enums::DeprecatedGenderRatio,
    gender_filter: enums::DeprecatedGenderFilter,
    tiles: usize,
    large_room: bool,
    diglett_boost: bool,
    min_ivs: Vec<u32>,
    max_ivs: Vec<u32>,
) -> Vec<bdsp::underground::generator::Pokemon> {
    let natures: Vec<enums::DeprecatedNatureFilter> = nature_filter
        .iter()
        .map(|nature| {
            enums::DeprecatedNatureFilter::try_from(*nature as u16)
                .unwrap_or(enums::DeprecatedNatureFilter::Hardy)
        })
        .collect();
    let mut rng = rng::Xorshift::from_state([seed1, seed2, seed3, seed4]);
    rng.advance(delay);
    let mut pokemon_results = Vec::new();
    let values = min_advances..=max_advances;
    rng.advance(min_advances);
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

    pokemon_results.into_iter().collect()
}

#[wasm_bindgen]
pub fn calculate_pokemon_bdsp_underground(
    seed1: u32,
    seed2: u32,
    seed3: u32,
    seed4: u32,
    shiny_filter: enums::ShinyFilter,
    min_advances: usize,
    max_advances: usize,
    delay: usize,
    nature_filter: Vec<u32>,
    ability_filter: enums::AbilityFilter,
    _encounter_filter: enums::DeprecatedEncounterSlotFilter,
    gender_ratio: enums::DeprecatedGenderRatio,
    gender_filter: enums::DeprecatedGenderFilter,
    tiles: usize,
    large_room: bool,
    diglett_boost: bool,
    min_ivs: Vec<u32>,
    max_ivs: Vec<u32>,
) -> JsValue {
    init_panic_hook();
    let results = _calculate_pokemon_bdsp_underground(
        seed1,
        seed2,
        seed3,
        seed4,
        shiny_filter,
        min_advances,
        max_advances,
        delay,
        nature_filter,
        ability_filter,
        _encounter_filter,
        gender_ratio,
        gender_filter,
        tiles,
        large_room,
        diglett_boost,
        min_ivs,
        max_ivs,
    );
    JsValue::from_serde(&results).unwrap()
}

#[wasm_bindgen]
pub fn get_bdsp_tid_field_groups() -> JsValue {
    let result = bdsp::tid::form_settings::get_field_groups();
    JsValue::from_serde(&result).unwrap()
}

#[wasm_bindgen]
pub fn get_bdsp_tid_result_columns() -> JsValue {
    let result = bdsp::tid::form_settings::get_result_columns();
    JsValue::from_serde(&result).unwrap()
}

#[wasm_bindgen]
pub fn generate_tid(settings: &JsValue) -> JsValue {
    init_panic_hook();
    let parsed_settings: bdsp::tid::form_settings::Settings = settings.into_serde().unwrap();
    let results = bdsp::tid::form_settings::generate_tid(parsed_settings);
    JsValue::from_serde(&results).unwrap()
}

#[wasm_bindgen]
pub fn get_transporter_field_groups() -> JsValue {
    let result = gen6::transporter::get_field_groups();
    JsValue::from_serde(&result).unwrap()
}

#[wasm_bindgen]
pub fn get_transporter_result_columns() -> JsValue {
    let result = gen6::transporter::get_result_columns();
    JsValue::from_serde(&result).unwrap()
}

#[wasm_bindgen]
pub fn generate_transporter(settings: &JsValue) -> JsValue {
    init_panic_hook();
    let parsed_settings: gen6::transporter::form_settings::Settings =
        settings.into_serde().unwrap();
    let results = gen6::transporter::form_settings::generate_transporter(parsed_settings);
    JsValue::from_serde(&results).unwrap()
}

#[wasm_bindgen]
pub fn get_gen3_wild_field_groups() -> JsValue {
    let result = gen3::get_field_groups();
    JsValue::from_serde(&result).unwrap()
}

#[wasm_bindgen]
pub fn get_gen3_wild_result_columns() -> JsValue {
    let result = gen3::get_result_columns();
    JsValue::from_serde(&result).unwrap()
}

#[wasm_bindgen]
pub fn generate_gen3_wild(settings: &JsValue) -> JsValue {
    init_panic_hook();
    let parsed_settings: gen3::form_settings::Settings = settings.into_serde().unwrap();
    let results = gen3::form_settings::generate_wild(parsed_settings);
    JsValue::from_serde(&results).unwrap()
}

#[wasm_bindgen]
pub fn get_bdsp_wild_field_groups() -> JsValue {
    let result = bdsp::wild::form_settings::get_field_groups();
    JsValue::from_serde(&result).unwrap()
}

#[wasm_bindgen]
pub fn get_bdsp_wild_result_columns() -> JsValue {
    let result = bdsp::wild::form_settings::get_result_columns();
    JsValue::from_serde(&result).unwrap()
}

#[wasm_bindgen]
pub fn generate_bdsp_wild(settings: &JsValue) -> JsValue {
    init_panic_hook();
    let parsed_settings: bdsp::wild::form_settings::Settings = settings.into_serde().unwrap();
    let results = bdsp::wild::form_settings::generate_wild(parsed_settings);
    JsValue::from_serde(&results).unwrap()
}

#[wasm_bindgen]
pub fn get_bdsp_static_field_groups() -> JsValue {
    let result = bdsp::stationary::form_settings::get_field_groups();
    JsValue::from_serde(&result).unwrap()
}

#[wasm_bindgen]
pub fn get_bdsp_static_result_columns() -> JsValue {
    let result = bdsp::stationary::form_settings::get_result_columns();
    JsValue::from_serde(&result).unwrap()
}

#[wasm_bindgen]
pub fn generate_bdsp_static(settings: &JsValue) -> JsValue {
    init_panic_hook();
    let parsed_settings: bdsp::stationary::form_settings::Settings = settings.into_serde().unwrap();
    let results = bdsp::stationary::form_settings::generate_stationary(parsed_settings);
    JsValue::from_serde(&results).unwrap()
}

#[wasm_bindgen]
pub fn get_bdsp_underground_field_groups() -> JsValue {
    let result = bdsp::underground::form_settings::get_field_groups();
    JsValue::from_serde(&result).unwrap()
}

#[wasm_bindgen]
pub fn get_bdsp_underground_result_columns() -> JsValue {
    let result = bdsp::underground::form_settings::get_result_columns();
    JsValue::from_serde(&result).unwrap()
}

#[wasm_bindgen]
pub fn generate_bdsp_underground(settings: &JsValue) -> JsValue {
    init_panic_hook();
    let parsed_settings: bdsp::underground::form_settings::Settings =
        settings.into_serde().unwrap();
    let results = bdsp::underground::form_settings::generate_underground(parsed_settings);
    JsValue::from_serde(&results).unwrap()
}

#[wasm_bindgen]
pub fn get_swsh_overworld_field_groups() -> JsValue {
    let result = swsh::form_settings::get_field_groups();
    JsValue::from_serde(&result).unwrap()
}

#[wasm_bindgen]
pub fn get_swsh_overworld_result_columns() -> JsValue {
    let result = swsh::form_settings::get_result_columns();
    JsValue::from_serde(&result).unwrap()
}

#[wasm_bindgen]
pub fn generate_swsh_overworld(settings: &JsValue) -> JsValue {
    init_panic_hook();
    let parsed_settings: swsh::form_settings::Settings = settings.into_serde().unwrap();
    let results = swsh::form_settings::generate_overworld(parsed_settings);
    JsValue::from_serde(&results).unwrap()
}
