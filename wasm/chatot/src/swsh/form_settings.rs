use crate::_calculate_pokemon as calculate_swsh_pokemon;
use chatot_forms::{
    impl_display, FieldGroup, Gen3Ability, LargeComponent, Nature, SelectOption, ShinyType,
};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[repr(u32)]
pub enum EncounterFilter {
    Static = 0,
    Dynamic = 1,
}

impl_display!(EncounterFilter);

pub fn get_field_groups() -> Vec<FieldGroup> {
    let rng_info_components = vec![
        LargeComponent::seed_u64_0(),
        LargeComponent::seed_u64_1(),
        LargeComponent::min_advances(),
        LargeComponent::max_advances(),
        LargeComponent::select(
            "encounter_type",
            "Encounter Type",
            vec![
                SelectOption::new(EncounterFilter::Static),
                SelectOption::new(EncounterFilter::Dynamic),
            ],
        ),
        LargeComponent::tid(),
        LargeComponent::sid(),
        LargeComponent::shiny_charm(),
    ];

    let filer_components = vec![
        LargeComponent::shiny_type(),
        LargeComponent::nature_multiselect(),
        LargeComponent::gen3_ability(),
    ];

    vec![
        FieldGroup::new("RNG Info", rng_info_components),
        FieldGroup::new("Filters", filer_components),
    ]
}

pub fn get_result_columns() -> Vec<String> {
    vec![
        "Advances", "Shiny", "Nature", "Ability", "State 0", "State 1", "EC", "PID",
    ]
    .into_iter()
    .map(String::from)
    .collect::<Vec<String>>()
}

#[derive(Deserialize, Serialize)]
pub struct Settings {
    tid: u16,
    sid: u16,
    shiny_charm: bool,
    seed_u64_0: String,
    seed_u64_1: String,
    encounter_type: EncounterFilter,
    min_advances: usize,
    max_advances: usize,
    shiny_type: Vec<ShinyType>,
    nature_multiselect: Vec<Nature>,
    gen3_ability: Option<Gen3Ability>,
}

pub struct ParsedSettings {
    pub tid: u16,
    pub sid: u16,
    pub shiny_charm: bool,
    pub seed_u64_0: u64,
    pub seed_u64_1: u64,
    pub encounter_type: EncounterFilter,
    pub min_advances: usize,
    pub max_advances: usize,
    pub shiny_type: Vec<ShinyType>,
    pub nature_multiselect: Vec<Nature>,
    pub gen3_ability: Option<Gen3Ability>,
}

impl From<Settings> for ParsedSettings {
    fn from(value: Settings) -> Self {
        Self {
            tid: value.tid,
            sid: value.sid,
            shiny_charm: value.shiny_charm,
            seed_u64_0: u64::from_str_radix(&value.seed_u64_0, 16).unwrap_or_default(),
            seed_u64_1: u64::from_str_radix(&value.seed_u64_1, 16).unwrap_or_default(),
            encounter_type: value.encounter_type,
            min_advances: value.min_advances,
            max_advances: value.max_advances,
            shiny_type: value.shiny_type,
            nature_multiselect: value.nature_multiselect,
            gen3_ability: value.gen3_ability,
        }
    }
}

pub fn generate_overworld(settings: Settings) -> Vec<Vec<String>> {
    let results = calculate_swsh_pokemon(&settings.into());
    results
        .into_iter()
        .map(|result| {
            vec![
                result.advances.to_string(),
                result
                    .shiny_value
                    .map(|shiny_type| shiny_type.to_string())
                    .unwrap_or("None".to_string()),
                result.nature.to_string(),
                result.ability.to_string(),
                format!("{:x}", result.state0),
                format!("{:x}", result.state1),
                format!("{:x}", result.ec),
                format!("{:x}", result.pid),
            ]
        })
        .collect()
}
