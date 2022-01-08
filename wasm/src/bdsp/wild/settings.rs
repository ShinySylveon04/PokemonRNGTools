use crate::enums;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Settings {
    pub nature_filter: Vec<u32>,
    pub encounter_filter: Vec<usize>,
    pub rng_state: Vec<u32>,
    pub delay: usize,
    pub min: usize,
    pub max: usize,
    pub gender_ratio: enums::GenderRatio,
    pub lead: enums::LeadFilter,
    pub shiny_filter: enums::ShinyFilter,
    pub ability_filter: enums::AbilityFilter,
    pub gender_filter: enums::GenderFilter,
    pub min_ivs: Vec<u32>,
    pub max_ivs: Vec<u32>,
}
