use crate::utils::format_ivs;

use super::{generator, settings};
use chatot_forms::{FieldGroup, LargeComponent, SmallComponent};
use serde::{Deserialize, Serialize};

pub fn get_field_groups() -> Vec<FieldGroup> {
    let rng_info_components = vec![
        LargeComponent::seed(),
        LargeComponent::min_advances(),
        LargeComponent::max_advances(),
        LargeComponent::delay(),
        LargeComponent::tid(),
        LargeComponent::checkbox("shiny_pokemon", "Shiny Pokemon"),
        LargeComponent::checkbox("mew_or_celebi", "Mew or Celebi"),
    ];

    let filer_components = vec![
        LargeComponent::min_ivs_label(),
        SmallComponent::min_hp_iv(),
        SmallComponent::min_atk_iv(),
        SmallComponent::min_def_iv(),
        SmallComponent::min_spa_iv(),
        SmallComponent::min_spd_iv(),
        SmallComponent::min_spe_iv(),
        LargeComponent::max_ivs_label(),
        SmallComponent::max_hp_iv(),
        SmallComponent::max_atk_iv(),
        SmallComponent::max_def_iv(),
        SmallComponent::max_spa_iv(),
        SmallComponent::max_spd_iv(),
        SmallComponent::max_spe_iv(),
    ];

    vec![
        FieldGroup::new("RNG Info", rng_info_components),
        FieldGroup::new("Filters", filer_components),
    ]
}

pub fn get_result_columns() -> Vec<String> {
    vec!["Advances", "IVs", "Hidden Power", "PSV", "PID"]
        .into_iter()
        .map(String::from)
        .collect::<Vec<String>>()
}

#[derive(Deserialize, Serialize)]
pub struct Settings {
    min_hp_iv: u32,
    min_atk_iv: u32,
    min_def_iv: u32,
    min_spa_iv: u32,
    min_spd_iv: u32,
    min_spe_iv: u32,
    max_hp_iv: u32,
    max_atk_iv: u32,
    max_def_iv: u32,
    max_spa_iv: u32,
    max_spd_iv: u32,
    max_spe_iv: u32,
    seed: u32,
    min_advances: usize,
    max_advances: usize,
    delay: usize,
    mew_or_celebi: bool,
    shiny_pokemon: bool,
    tid: u32,
}

impl From<Settings> for settings::Settings {
    fn from(value: Settings) -> Self {
        Self {
            min_ivs: vec![
                value.min_hp_iv,
                value.min_atk_iv,
                value.min_def_iv,
                value.min_spa_iv,
                value.min_spd_iv,
                value.min_spe_iv,
            ],
            max_ivs: vec![
                value.max_hp_iv,
                value.max_atk_iv,
                value.max_def_iv,
                value.max_spa_iv,
                value.max_spd_iv,
                value.max_spe_iv,
            ],
            rng_state: value.seed,
            min_advances: value.min_advances,
            max_advances: value.max_advances,
            delay: value.delay,
            iv_rolls: value.mew_or_celebi,
            is_shiny: value.shiny_pokemon,
            tid: value.tid,
        }
    }
}

pub fn generate_transporter(settings: Settings) -> Vec<Vec<String>> {
    let results = generator::generate_transporter(settings.into());
    results
        .into_iter()
        .map(|result| {
            vec![
                result.advances.to_string(),
                format_ivs(result.ivs),
                result.hidden_power.to_string(),
                result.psv.to_string(),
                format!("{:x}", result.pid),
            ]
        })
        .collect()
}
