use super::generator;
use chatot_forms::{FieldGroup, LargeComponent, SmallComponent};
use serde::{Deserialize, Serialize};

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
    pub(super) seed_0: u32,
    pub(super) seed_1: u32,
    pub(super) seed_2: u32,
    pub(super) seed_3: u32,
    pub(super) min_advances: usize,
    pub(super) max_advances: usize,
    pub(super) gen8_id_type: Option<chatot_forms::IDFilter>,
    pub(super) ids: String,
}

type ResultRow = Vec<String>;

pub fn generate_tid(settings: Settings) -> Vec<ResultRow> {
    let results = generator::generate_tid(settings);
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
