use super::{generator, settings};
use crate::{
    enums::{
        AbilityFilter, DeprecatedEncounterSlotFilter, DeprecatedGenderFilter,
        DeprecatedNatureFilter, LeadFilter, ShinyFilter,
    },
    utils::format_ivs,
};
use chatot_forms::{
    EncounterSlotFilter, FieldGroup, Gen3AbilityFilter, Gen3Lead, GenderFilter, GenderRatio,
    LargeComponent, NatureFilter, ShinyTypeFilter, SmallComponent,
};
use serde::{Deserialize, Serialize};

pub fn get_field_groups() -> Vec<FieldGroup> {
    let rng_info_components = vec![
        LargeComponent::seed_0(),
        LargeComponent::seed_1(),
        LargeComponent::seed_2(),
        LargeComponent::seed_3(),
        LargeComponent::min_advances(),
        LargeComponent::max_advances(),
        LargeComponent::number("delay", "Delay", Some(1)),
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
        "Advances", "Shiny", "Slot", "Nature", "Ability", "Gender", "IVs", "PID", "EC",
    ]
    .into_iter()
    .map(String::from)
    .collect::<Vec<String>>()
}

#[derive(Deserialize, Serialize)]
pub struct Settings {
    seed_0: u32,
    seed_1: u32,
    seed_2: u32,
    seed_3: u32,
    min_advances: u32,
    max_advances: u32,
    delay: u32,
    gen3_lead: Option<Gen3Lead>,
    shiny_type: Option<ShinyTypeFilter>,
    nature_multiselect: Vec<NatureFilter>,
    gen3_ability: Option<Gen3AbilityFilter>,
    encounter_slot: Option<EncounterSlotFilter>,
    gender_ratio: GenderRatio,
    gender: Option<GenderFilter>,
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
}

impl From<Settings> for settings::Settings {
    fn from(value: Settings) -> Self {
        Self {
            nature_filter: value
                .nature_multiselect
                .into_iter()
                .map(|nature| (DeprecatedNatureFilter::from(nature) as u16).into())
                .collect::<Vec<u32>>(),
            encounter_filter: vec![value
                .encounter_slot
                .map(|slot| slot.into())
                .unwrap_or(DeprecatedEncounterSlotFilter::Any)
                .into()],
            rng_state: vec![value.seed_0, value.seed_1, value.seed_2, value.seed_3],
            delay: value.delay as usize,
            min_advances: value.min_advances as usize,
            max_advances: value.max_advances as usize,
            gender_ratio: value.gender_ratio.into(),
            lead_filter: value
                .gen3_lead
                .map(|lead| lead.into())
                .unwrap_or(LeadFilter::None),
            shiny_filter: value
                .shiny_type
                .map(|shiny_type| shiny_type.into())
                .unwrap_or(ShinyFilter::Any),
            ability_filter: value
                .gen3_ability
                .map(|ability| ability.into())
                .unwrap_or(AbilityFilter::Any),
            gender_filter: value
                .gender
                .map(|gender| gender.into())
                .unwrap_or(DeprecatedGenderFilter::Any),
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
        }
    }
}

pub fn generate_wild(settings: Settings) -> Vec<Vec<String>> {
    let results = generator::generate_wild(settings.into());
    results
        .into_iter()
        .map(|result| {
            vec![
                result.advances.to_string(),
                result.shiny_value.to_string(),
                result.encounter.to_string(),
                result.nature.to_string(),
                result.ability.to_string(),
                result.gender.to_string(),
                format_ivs(result.ivs),
                format!("{:x}", result.pid),
                format!("{:x}", result.ec),
            ]
        })
        .collect()
}
