use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Settings {
    pub rng_state: u32,
    pub delay: usize,
    pub min_advances: usize,
    pub max_advances: usize,
    pub min_ivs: Vec<u32>,
    pub max_ivs: Vec<u32>,
    pub iv_rolls: bool,
}
