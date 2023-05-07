use crate::enums;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Settings {
    pub rng_state: Vec<u32>,
    pub shiny_filter: enums::ShinyFilter,
    pub min_advances: usize,
    pub max_advances: usize,
    pub delay: usize,
    pub nature_filter: Vec<u32>,
    pub ability_filter: enums::AbilityFilter,
    pub gender_ratio: enums::GenderRatio,
    pub gender_filter: enums::GenderFilter,
    pub set_ivs: bool,
    pub min_ivs: Vec<u32>,
    pub max_ivs: Vec<u32>,
    pub lead_filter: enums::LeadFilter,
    pub is_roamer: bool,
}
