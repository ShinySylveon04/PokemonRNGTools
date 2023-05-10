use super::{generator, settings};
use chatot_forms::{FieldGroup, LargeComponent, SmallComponent};
use serde::{Deserialize, Serialize};

pub fn get_field_groups() -> Vec<FieldGroup> {
    let rng_info_components = vec![
        LargeComponent::hex_number("seed0", "Seed 0", None),
        LargeComponent::hex_number("seed1", "Seed 1", None),
        LargeComponent::hex_number("seed2", "Seed 2", None),
        LargeComponent::hex_number("seed3", "Seed 3", None),
        LargeComponent::number("min_advances", "Min Advances", Some(0)),
        LargeComponent::number("max_advances", "Max Advances", Some(10000)),
    ];
    let tid_options = vec!["None", "TID", "SID", "TSV", "G8TID"];
    let filer_components = vec![
        SmallComponent::select("id_type", "ID Filter", &tid_options),
        SmallComponent::text("ids", "IDs", None),
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
    seed0: u32,
    seed1: u32,
    seed2: u32,
    seed3: u32,
    min_advances: usize,
    max_advances: usize,
    id_type: Vec<u32>,
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
            filter_type: value.id_type,
            rng_state: vec![result.seed0, result.seed1, result.seed2, result.seed3],
            min_advances: value.min_advances,
            max_advances: value.max_advances,
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
