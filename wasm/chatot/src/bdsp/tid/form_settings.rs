use super::{generator, settings};
use crate::enums::IDFilter;
use chatot_forms::{FieldGroup, LargeComponent, SmallComponent};
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

pub fn get_field_groups() -> Vec<FieldGroup> {
    let rng_info_components = vec![
        LargeComponent::seed_0(),
        LargeComponent::seed_1(),
        LargeComponent::seed_2(),
        LargeComponent::seed_3(),
        LargeComponent::min_advances(),
        LargeComponent::max_advances(),
    ];
    let filer_components = vec![
        SmallComponent::gen8_id_type(),
        SmallComponent::text("ids", "IDs", "None"),
    ];

    vec![
        FieldGroup::new("RNG Info", rng_info_components),
        FieldGroup::new("Filters", filer_components),
    ]
}

pub fn get_result_columns() -> Vec<String> {
    vec!["Advances", "Gen8TID", "TID", "SID", "TSV"]
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
    gen8_id_type: Option<chatot_forms::IDFilter>,
    ids: String,
}

impl From<Settings> for settings::Settings {
    fn from(value: Settings) -> Self {
        Self {
            id: value
                .ids
                .split("\n")
                .map(|id| id.parse::<u32>().unwrap_or_default())
                .collect(),
            filter_type: value
                .gen8_id_type
                .map(|id_type| id_type.into())
                .unwrap_or(IDFilter::None),
            rng_state: vec![value.seed_0, value.seed_1, value.seed_2, value.seed_3],
            min_advances: value.min_advances.try_into().unwrap_or_default(),
            max_advances: value.max_advances.try_into().unwrap_or_default(),
        }
    }
}

type ResultRow = Vec<String>;

pub fn generate_tid(settings: Settings) -> Vec<ResultRow> {
    let results = generator::generate_tid(settings.into());
    results
        .into_iter()
        .map(|result| {
            vec![
                result.advances.to_string(),
                result.g8tid.to_string(),
                result.tid.to_string(),
                result.sid.to_string(),
                result.tsv.to_string(),
            ]
        })
        .collect()
}
