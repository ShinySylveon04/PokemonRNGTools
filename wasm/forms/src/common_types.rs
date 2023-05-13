use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[macro_export]
macro_rules! impl_display {
    ($enum:ident) => {
        impl std::fmt::Display for $enum {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }
    };
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Gen3AbilityFilter {
    Ability0,
    Ability1,
}

impl_display!(Gen3AbilityFilter);

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Gen3Method {
    H1,
    H2,
    H4,
}

impl_display!(Gen3Method);

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Gen3Lead {
    Synchronize,
}

impl_display!(Gen3Lead);

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ShinyTypeFilter {
    Star,
    Square,
    Both,
}

impl_display!(ShinyTypeFilter);

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum NatureFilter {
    Hardy,
    Lonely,
    Brave,
    Adamant,
    Naughty,
    Bold,
    Docile,
    Relaxed,
    Impish,
    Lax,
    Timid,
    Hasty,
    Serious,
    Jolly,
    Naive,
    Modest,
    Mild,
    Quiet,
    Bashful,
    Rash,
    Calm,
    Gentle,
    Sassy,
    Careful,
    Quirky,
}

impl_display!(NatureFilter);

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum AbilityNumberFilter {
    Ability0,
    Ability1,
}

impl_display!(AbilityNumberFilter);

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum EncounterSlotFilter {
    Slot0,
    Slot1,
    Slot2,
    Slot3,
    Slot4,
    Slot5,
    Slot6,
    Slot7,
    Slot8,
    Slot9,
    Slot10,
    Slot11,
}

impl_display!(EncounterSlotFilter);

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum GenderRatio {
    NoSetGender = 256,
    Genderless = 255,
    Male50Female50 = 127,
    Male25Female75 = 191,
    Male75Female25 = 63,
    Male875Female125 = 31,
    Male = 0,
    Female = 254,
}

impl_display!(GenderRatio);

#[wasm_bindgen]
#[derive(PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum GenderFilter {
    Male,
    Female,
}

impl_display!(GenderFilter);

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum IDFilter {
    TID,
    SID,
    TSV,
    G8TID,
}

impl_display!(IDFilter);
