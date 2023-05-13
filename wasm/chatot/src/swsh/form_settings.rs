use crate::{
    _calculate_pokemon as calculate_swsh_pokemon,
    enums::{AbilityFilter, DeprecatedEncounterFilter, DeprecatedNatureFilter, ShinyFilter},
};
use chatot_forms::{
    FieldGroup, Gen3AbilityFilter, LargeComponent, NatureFilter, SelectOption, ShinyTypeFilter,
};
use serde::{Deserialize, Serialize};

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
                SelectOption::new(DeprecatedEncounterFilter::Static),
                SelectOption::new(DeprecatedEncounterFilter::Dynamic),
            ],
        ),
        LargeComponent::tid(),
        LargeComponent::sid(),
        LargeComponent::shiny_charm(),
    ];

    let filer_components = vec![
        LargeComponent::shiny_type(),
        LargeComponent::nature(),
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
    tid: u32,
    sid: u32,
    shiny_charm: bool,
    seed_u64_0: String,
    seed_u64_1: String,
    encounter_type: DeprecatedEncounterFilter,
    min_advances: u32,
    max_advances: u32,
    shiny_type: Option<ShinyTypeFilter>,
    nature: Option<NatureFilter>,
    gen3_ability: Option<Gen3AbilityFilter>,
}

pub fn generate_overworld(settings: Settings) -> Vec<Vec<String>> {
    let results = calculate_swsh_pokemon(
        u64::from_str_radix(&settings.seed_u64_0, 16).unwrap_or_default(),
        u64::from_str_radix(&settings.seed_u64_1, 16).unwrap_or_default(),
        settings.tid as u16,
        settings.sid as u16,
        settings
            .shiny_type
            .map(|shiny_type| shiny_type.into())
            .unwrap_or(ShinyFilter::Any),
        settings.encounter_type,
        settings.shiny_charm,
        settings
            .nature
            .map(|nature| DeprecatedNatureFilter::from(nature))
            .unwrap_or(DeprecatedNatureFilter::Any),
        settings
            .gen3_ability
            .map(|ability| ability.into())
            .unwrap_or(AbilityFilter::Any),
        settings.min_advances,
        settings.max_advances,
    );
    results
        .into_iter()
        .map(|result| {
            vec![
                result.advances.to_string(),
                result.shiny_value.to_string(),
                result.nature.to_string(),
                u32::from(result.ability).to_string(),
                format!("{:x}", result.state0),
                format!("{:x}", result.state1),
                format!("{:x}", result.ec),
                format!("{:x}", result.pid),
            ]
        })
        .collect()
}
