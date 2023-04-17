use crate::enums;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Settings {
    pub rng_state: u32,
    pub delay: usize,
    pub min_advances: usize,
    pub max_advances: usize,
    pub gender_ratio: enums::GenderRatio,
    pub min_ivs: Vec<u32>,
    pub max_ivs: Vec<u32>,
}
