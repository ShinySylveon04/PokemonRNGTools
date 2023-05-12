use crate::{
    _calculate_pokemon_bdsp_underground, enums::DeprecatedNatureFilter, utils::format_ivs,
};
use chatot_forms::{
    impl_display, EncounterSlotFilter, FieldGroup, Gen3AbilityFilter, GenderFilter, GenderRatio,
    LargeComponent, NatureFilter, SelectOption, ShinyTypeFilter, SmallComponent,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RoomSize {
    Small,
    Large,
}

impl_display!(RoomSize);

pub fn get_field_groups() -> Vec<FieldGroup> {
    let rng_info_components = vec![
        LargeComponent::seed_0(),
        LargeComponent::seed_1(),
        LargeComponent::seed_2(),
        LargeComponent::seed_3(),
        LargeComponent::min_advances(),
        LargeComponent::max_advances(),
        LargeComponent::delay(),
        LargeComponent::number("statue_tiles", "Statue Tiles", Some(0)),
        LargeComponent::select(
            "room_size",
            "Room Size",
            vec![
                SelectOption::new(RoomSize::Small),
                SelectOption::new(RoomSize::Large),
            ],
        ),
        LargeComponent::checkbox("diglett_boost", "Diglett Boost"),
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
    statue_tiles: u32,
    room_size: RoomSize,
    diglett_boost: bool,
    shiny_type: ShinyTypeFilter,
    nature_multiselect: Vec<NatureFilter>,
    gen3_ability: Gen3AbilityFilter,
    encounter_slot: EncounterSlotFilter,
    gender_ratio: GenderRatio,
    gender: GenderFilter,
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

pub fn generate_underground(settings: Settings) -> Vec<Vec<String>> {
    let results = _calculate_pokemon_bdsp_underground(
        settings.seed_0,
        settings.seed_1,
        settings.seed_2,
        settings.seed_3,
        settings.shiny_type.into(),
        settings.min_advances as usize,
        settings.max_advances as usize,
        settings.delay as usize,
        settings
            .nature_multiselect
            .into_iter()
            .map(|nature| u32::from(DeprecatedNatureFilter::from(nature) as u16))
            .collect(),
        settings.gen3_ability.into(),
        settings.encounter_slot.into(),
        settings.gender_ratio.into(),
        settings.gender.into(),
        settings.statue_tiles as usize,
        settings.room_size == RoomSize::Large,
        settings.diglett_boost,
        vec![
            settings.min_hp_iv,
            settings.min_atk_iv,
            settings.min_def_iv,
            settings.min_spa_iv,
            settings.min_spd_iv,
            settings.min_spe_iv,
        ],
        vec![
            settings.max_hp_iv,
            settings.max_atk_iv,
            settings.max_def_iv,
            settings.max_spa_iv,
            settings.max_spd_iv,
            settings.max_spe_iv,
        ],
    );
    results
        .into_iter()
        .map(|result| {
            vec![
                result.advances.to_string(),
                result.shiny_value.to_string(),
                if result.encounter == 0 {
                    "Rare".to_string()
                } else {
                    result.encounter.to_string()
                },
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
