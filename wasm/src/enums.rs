use num_enum::FromPrimitive;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AbilityFilterEnum {
    Any = 3,
    Ability0 = 0,
    Ability1 = 1,
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, PartialEq, FromPrimitive, Serialize, Deserialize)]
#[repr(u32)]
pub enum AbilityEnum {
    #[num_enum(default)]
    Ability0 = 0,
    Ability1 = 1,
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, PartialEq, FromPrimitive)]
#[repr(u32)]
pub enum NatureFilterEnum {
    #[num_enum(default)]
    Hardy = 0,
    Lonely = 1,
    Brave = 2,
    Adamant = 3,
    Naughty = 4,
    Bold = 5,
    Docile = 6,
    Relaxed = 7,
    Impish = 8,
    Lax = 9,
    Timid = 10,
    Hasty = 11,
    Serious = 12,
    Jolly = 13,
    Naive = 14,
    Modest = 15,
    Mild = 16,
    Quiet = 17,
    Bashful = 18,
    Rash = 19,
    Calm = 20,
    Gentle = 21,
    Sassy = 22,
    Careful = 23,
    Quirky = 24,
    Any = 25,
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, PartialEq, FromPrimitive, Serialize, Deserialize)]
#[repr(u32)]
pub enum NatureEnum {
    #[num_enum(default)]
    Hardy = 0,
    Lonely = 1,
    Brave = 2,
    Adamant = 3,
    Naughty = 4,
    Bold = 5,
    Docile = 6,
    Relaxed = 7,
    Impish = 8,
    Lax = 9,
    Timid = 10,
    Hasty = 11,
    Serious = 12,
    Jolly = 13,
    Naive = 14,
    Modest = 15,
    Mild = 16,
    Quiet = 17,
    Bashful = 18,
    Rash = 19,
    Calm = 20,
    Gentle = 21,
    Sassy = 22,
    Careful = 23,
    Quirky = 24,
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ShinyFilterEnum {
    None = 0,
    Star = 1,
    Square = 2,
    Any = 3,
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum EncounterFilterEnum {
    Static = 0,
    Dynamic = 1,
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ShinyEnum {
    None = 0,
    Star = 1,
    Square = 2,
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, PartialEq, FromPrimitive)]
#[repr(usize)]
pub enum EncounterSlotFilterEnum {
    #[num_enum(default)]
    Any = 12,
    Slot0 = 0,
    Slot1 = 1,
    Slot2 = 2,
    Slot3 = 3,
    Slot4 = 4,
    Slot5 = 5,
    Slot6 = 6,
    Slot7 = 7,
    Slot8 = 8,
    Slot9 = 9,
    Slot10 = 10,
    Slot11 = 11,
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, PartialEq, FromPrimitive)]
#[repr(usize)]
pub enum EncounterSlotEnum {
    #[num_enum(default)]
    Slot0 = 0,
    Slot1 = 1,
    Slot2 = 2,
    Slot3 = 3,
    Slot4 = 4,
    Slot5 = 5,
    Slot6 = 6,
    Slot7 = 7,
    Slot8 = 8,
    Slot9 = 9,
    Slot10 = 10,
    Slot11 = 11,
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, PartialEq, FromPrimitive, PartialOrd)]
#[repr(u32)]
pub enum GenderRatioEnum {
    #[num_enum(default)]
    NoSetGender = 256,
    Genderless = 255,
    Male50Female50 = 127,
    Male25Female75 = 191,
    Male75Female25 = 63,
    Male875Female125 = 31,
    Male = 0,
    Female = 254,
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum GenderEnum {
    Genderless = 255,
    Male = 0,
    Female = 254,
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum GenderFilterEnum {
    Any = 256,
    Male = 0,
    Female = 254,
}

pub fn get_set_gender_from_ratio(gender_ratio: &GenderRatioEnum) -> Option<GenderEnum> {
    match gender_ratio {
        GenderRatioEnum::Male => Some(GenderEnum::Male),
        GenderRatioEnum::Female => Some(GenderEnum::Female),
        GenderRatioEnum::Genderless => Some(GenderEnum::Genderless),
        _ => None,
    }
}

pub fn get_gender_from_ratio(gender_ratio: &GenderRatioEnum, gender_num: u32) -> GenderEnum {
    match gender_ratio {
        GenderRatioEnum::Male => GenderEnum::Male,
        GenderRatioEnum::Female => GenderEnum::Female,
        GenderRatioEnum::Genderless => GenderEnum::Genderless,
        _ => {
            if gender_num < *gender_ratio as u32 {
                GenderEnum::Female
            } else {
                GenderEnum::Male
            }
        }
    }
}
