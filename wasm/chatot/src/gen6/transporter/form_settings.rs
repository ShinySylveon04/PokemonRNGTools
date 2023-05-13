use super::generator;
use crate::utils::format_ivs;
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
    pub(super) min_hp_iv: u8,
    pub(super) min_atk_iv: u8,
    pub(super) min_def_iv: u8,
    pub(super) min_spa_iv: u8,
    pub(super) min_spd_iv: u8,
    pub(super) min_spe_iv: u8,
    pub(super) max_hp_iv: u8,
    pub(super) max_atk_iv: u8,
    pub(super) max_def_iv: u8,
    pub(super) max_spa_iv: u8,
    pub(super) max_spd_iv: u8,
    pub(super) max_spe_iv: u8,
    pub(super) seed: u32,
    pub(super) min_advances: usize,
    pub(super) max_advances: usize,
    pub(super) delay: usize,
    pub(super) mew_or_celebi: bool,
    pub(super) shiny_pokemon: bool,
    pub(super) tid: u16,
}

impl Settings {
    pub(super) fn min_ivs(&self) -> [u8; 6] {
        [
            self.min_hp_iv,
            self.min_atk_iv,
            self.min_def_iv,
            self.min_spa_iv,
            self.min_spd_iv,
            self.min_spe_iv,
        ]
    }

    pub(super) fn max_ivs(&self) -> [u8; 6] {
        [
            self.max_hp_iv,
            self.max_atk_iv,
            self.max_def_iv,
            self.max_spa_iv,
            self.max_spd_iv,
            self.max_spe_iv,
        ]
    }
}

pub fn generate_transporter(settings: Settings) -> Vec<Vec<String>> {
    let results = generator::generate_transporter(settings);
    results
        .into_iter()
        .map(|result| {
            vec![
                result.advances.to_string(),
                format_ivs(&result.ivs),
                result.hidden_power.to_string(),
                result.psv.to_string(),
                format!("{:x}", result.pid),
            ]
        })
        .collect()
}
