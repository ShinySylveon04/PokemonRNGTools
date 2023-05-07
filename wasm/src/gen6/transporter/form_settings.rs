use super::{generator, settings};
use crate::form_config::{FieldGroup, LargeComponent, SmallComponent};
use serde::{Deserialize, Serialize};

pub fn get_field_groups() -> Vec<FieldGroup> {
    let rng_info_components = vec![
        LargeComponent::hex_number("seed", "Seed", None),
        LargeComponent::number("min_advances", "Min Advances", Some(0)),
        LargeComponent::number("max_advances", "Max Advances", Some(10000)),
        LargeComponent::number("delay", "Delay", Some(0)),
        LargeComponent::number("tid", "TID", Some(0)),
        LargeComponent::checkbox("shiny_pokemon", "Shiny Pokemon"),
        LargeComponent::checkbox("mew_or_celebi", "Mew or Celebi"),
    ];

    let filer_components = vec![
        LargeComponent::label("min_ivs_label", "Min IVs"),
        SmallComponent::number("min_hp_iv", "HP", Some(0)),
        SmallComponent::number("min_atk_iv", "Attack", Some(0)),
        SmallComponent::number("min_def_iv", "Defense", Some(0)),
        SmallComponent::number("min_spa_iv", "Special Attack", Some(0)),
        SmallComponent::number("min_spd_iv", "Special Defense", Some(0)),
        SmallComponent::number("min_spe_iv", "Speed", Some(0)),
        LargeComponent::label("max_ivs_label", "Max IVs"),
        SmallComponent::number("max_hp_iv", "HP", Some(31)),
        SmallComponent::number("max_atk_iv", "Attack", Some(31)),
        SmallComponent::number("max_def_iv", "Defense", Some(31)),
        SmallComponent::number("max_spa_iv", "Special Attack", Some(31)),
        SmallComponent::number("max_spd_iv", "Special Defense", Some(31)),
        SmallComponent::number("max_spe_iv", "Speed", Some(31)),
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
                format!(
                    "{} / {} / {} / {} / {} / {}",
                    result.ivs[0],
                    result.ivs[1],
                    result.ivs[2],
                    result.ivs[3],
                    result.ivs[4],
                    result.ivs[5]
                ),
                result.hidden_power.to_string(),
                result.psv.to_string(),
                format!("{:x}", result.pid),
            ]
        })
        .collect()
}
