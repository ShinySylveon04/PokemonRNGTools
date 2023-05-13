use super::generator;
use crate::utils::format_ivs;
use chatot_forms::{
    EncounterSlot, FieldGroup, Gen3Ability, Gen3Lead, Gen3Method, Gender, GenderRatio,
    LargeComponent, Nature, ShinyType, SmallComponent,
};
use serde::{Deserialize, Serialize};

pub fn get_field_groups() -> Vec<FieldGroup> {
    let rng_info_components = vec![
        LargeComponent::seed(),
        LargeComponent::tid(),
        LargeComponent::sid(),
        LargeComponent::min_advances(),
        LargeComponent::max_advances(),
        LargeComponent::delay(),
        LargeComponent::gen3_method(),
        LargeComponent::gen3_lead(),
    ];

    let filer_components = vec![
        LargeComponent::shiny_type(),
        LargeComponent::nature_multiselect(),
        LargeComponent::gen3_ability(),
        LargeComponent::encounter_slot(),
        LargeComponent::gender_ratio(),
        LargeComponent::gender(),
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
    vec![
        "Advances", "Shiny", "Slot", "Nature", "Ability", "Gender", "IVs", "PID",
    ]
    .into_iter()
    .map(String::from)
    .collect::<Vec<String>>()
}

#[derive(Deserialize, Serialize)]
pub struct Settings {
    pub(super) seed: u32,
    pub(super) tid: u16,
    pub(super) sid: u16,
    pub(super) min_advances: usize,
    pub(super) max_advances: usize,
    pub(super) delay: usize,
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
    pub(super) gen3_method: Gen3Method,
    pub(super) gen3_lead: Option<Gen3Lead>,
    pub(super) shiny_type: Vec<ShinyType>,
    pub(super) nature_multiselect: Vec<Nature>,
    pub(super) gen3_ability: Option<Gen3Ability>,
    pub(super) encounter_slot: Option<EncounterSlot>,
    pub(super) gender_ratio: GenderRatio,
    pub(super) gender: Option<Gender>,
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

pub fn generate_wild(settings: Settings) -> Vec<Vec<String>> {
    let results = generator::generate_wild(settings);
    results
        .into_iter()
        .map(|result| {
            vec![
                result.advances.to_string(),
                result
                    .shiny_value
                    .map(|shiny_type| shiny_type.to_string())
                    .unwrap_or("None".to_string()),
                result.encounter.to_string(),
                result.nature.to_string(),
                u8::from(result.ability).to_string(),
                result.gender.to_string(),
                format_ivs(&result.ivs),
                format!("{:x}", result.pid),
            ]
        })
        .collect()
}
