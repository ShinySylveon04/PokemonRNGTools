#![feature(iter_order_by)]
use bdsp::underground::form_settings::RoomSize;
use chatot_forms::{self, Gen3Ability, Gender, MultiFilter, Nature, ShinyType, SingleFilter};
use wasm_bindgen::prelude::*;

extern crate console_error_panic_hook;

mod bdsp;
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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Pokemon {
    shiny_type: Option<ShinyType>,
    ec: u32,
    pid: u32,
    nature: Nature,
    ability: Gen3Ability,
}

pub struct ShinyResult {
    pub state0: u64,
    pub state1: u64,
    pub advances: usize,
    pub shiny_value: Option<ShinyType>,
    pub ec: u32,
    pub pid: u32,
    pub nature: Nature,
    pub ability: Gen3Ability,
}

pub fn filter(
    results: Pokemon,
    shiny_filter: &[ShinyType],
    nature_filter: &[Nature],
    ability_filter: Option<Gen3Ability>,
) -> bool {
    ShinyType::passes_filter(shiny_filter, results.shiny_type)
        && Nature::passes_filter(nature_filter, Some(results.nature))
        && Gen3Ability::passes_filter(ability_filter, results.ability)
}

pub fn filter_bdsp_underground(
    results: &bdsp::underground::generator::Pokemon,
    shiny_filter: &[ShinyType],
    natures: &[Nature],
    ability_filter: Option<Gen3Ability>,
    gender_filter: Option<Gender>,
    min_ivs: &[u8],
    max_ivs: &[u8],
) -> bool {
    return Gen3Ability::passes_filter(ability_filter, results.ability)
        && Nature::passes_filter(natures, Some(results.nature))
        && Gender::passes_filter(gender_filter, results.gender)
        && ShinyType::passes_filter(shiny_filter, results.shiny_value)
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
    settings: &crate::swsh::form_settings::ParsedSettings,
) -> Vec<ShinyResult> {
    let mut rng = rng::Xoroshiro::from_state(settings.seed_u64_0, settings.seed_u64_1);
    let mut pokemon_results;
    let mut shiny_results: Vec<ShinyResult> = Vec::new();
    let values = settings.min_advances..=settings.max_advances;
    for value in values {
        pokemon_results = match settings.encounter_type {
            crate::swsh::form_settings::EncounterFilter::Static => {
                swsh::generate_static_pokemon(rng, settings.tid, settings.sid, settings.shiny_charm)
            }
            crate::swsh::form_settings::EncounterFilter::Dynamic => swsh::generate_dynamic_pokemon(
                rng,
                settings.tid,
                settings.sid,
                settings.shiny_charm,
            ),
        };

        if filter(
            pokemon_results,
            &settings.shiny_type,
            &settings.nature_multiselect,
            settings.gen3_ability,
        ) {
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

pub(crate) fn _calculate_pokemon_bdsp_underground(
    settings: &crate::bdsp::underground::form_settings::Settings,
) -> Vec<bdsp::underground::generator::Pokemon> {
    let mut rng = rng::Xorshift::from_state([
        settings.seed_0,
        settings.seed_1,
        settings.seed_2,
        settings.seed_3,
    ]);
    rng.advance(settings.delay);
    let mut pokemon_results = Vec::new();
    let values = settings.min_advances..=settings.max_advances;
    rng.advance(settings.min_advances);
    for value in values {
        let mut result = bdsp::underground::generator::generate_pokemon(
            rng,
            settings.gender_ratio,
            value,
            settings.statue_tiles,
            settings.room_size == RoomSize::Large,
            settings.diglett_boost,
        );

        if result.iter().any(|pokemon| {
            filter_bdsp_underground(
                pokemon,
                &settings.shiny_type,
                &settings.nature_multiselect,
                settings.gen3_ability,
                settings.gender,
                &settings.min_ivs(),
                &settings.max_ivs(),
            )
        }) {
            pokemon_results.append(&mut result);
        }
        rng.next();
    }

    pokemon_results.into_iter().collect()
}

#[wasm_bindgen]
pub fn calculate_pokemon_bdsp_underground(settings: &JsValue) -> JsValue {
    init_panic_hook();
    let parsed_settings: bdsp::underground::form_settings::Settings =
        settings.into_serde().unwrap();
    let results = _calculate_pokemon_bdsp_underground(&parsed_settings);
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
