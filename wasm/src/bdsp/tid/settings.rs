use crate::enums;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Settings {
    pub rng_state: Vec<u32>,
    pub min_advances: usize,
    pub max_advances: usize,
    pub id: Vec<u32>,
    pub filter_type: enums::IDFilter,
}
