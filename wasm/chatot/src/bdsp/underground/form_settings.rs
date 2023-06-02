use crate::{_calculate_pokemon_bdsp_underground, utils::format_ivs};
use chatot_forms::{
    impl_display, EncounterSlot, FieldGroup, Gen3Ability, Gender, GenderRatio, LargeComponent,
    Nature, SelectOption, ShinyType, SmallComponent,
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
    pub seed_0: u32,
    pub seed_1: u32,
    pub seed_2: u32,
    pub seed_3: u32,
    pub min_advances: usize,
    pub max_advances: usize,
    pub delay: usize,
    pub statue_tiles: u8,
    pub room_size: RoomSize,
    pub diglett_boost: bool,
    pub shiny_type: Vec<ShinyType>,
    pub nature_multiselect: Vec<Nature>,
    pub gen3_ability: Option<Gen3Ability>,
    pub encounter_slot: Option<EncounterSlot>,
    pub gender_ratio: GenderRatio,
    pub gender: Option<Gender>,
    pub min_hp_iv: u8,
    pub min_atk_iv: u8,
    pub min_def_iv: u8,
    pub min_spa_iv: u8,
    pub min_spd_iv: u8,
    pub min_spe_iv: u8,
    pub max_hp_iv: u8,
    pub max_atk_iv: u8,
    pub max_def_iv: u8,
    pub max_spa_iv: u8,
    pub max_spd_iv: u8,
    pub max_spe_iv: u8,
}

impl Settings {
    pub fn min_ivs(&self) -> [u8; 6] {
        [
            self.min_hp_iv,
            self.min_atk_iv,
            self.min_def_iv,
            self.min_spa_iv,
            self.min_spd_iv,
            self.min_spe_iv,
        ]
    }

    pub fn max_ivs(&self) -> [u8; 6] {
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

pub fn generate_underground(settings: Settings) -> Vec<Vec<String>> {
    let results = _calculate_pokemon_bdsp_underground(&settings);
    results
        .into_iter()
        .map(|result| {
            vec![
                result.advances.to_string(),
                result
                    .shiny_value
                    .map(|shiny_type| shiny_type.to_string())
                    .unwrap_or("None".to_string()),
                if result.encounter == 0 {
                    "Rare".to_string()
                } else {
                    result.encounter.to_string()
                },
                result.nature.to_string(),
                result.ability.to_string(),
                result.gender.to_string(),
                format_ivs(&result.ivs),
                format!("{:x}", result.pid),
                format!("{:x}", result.ec),
            ]
        })
        .collect()
}
