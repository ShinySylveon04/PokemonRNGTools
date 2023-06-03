use num_enum::{FromPrimitive, IntoPrimitive};
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

pub trait SingleFilter: Sized + PartialEq {
    fn passes_filter(filter: Option<Self>, value: Self) -> bool {
        filter.map(|filter| filter == value).unwrap_or(true)
    }
}

pub trait MultiFilter: Sized + PartialEq {
    fn passes_filter(filter: &[Self], value: Option<Self>) -> bool {
        if filter.len() == 0 {
            return true;
        }

        value
            .map(|value| filter.contains(&value))
            .unwrap_or_default()
    }
}

#[wasm_bindgen]
#[derive(
    Copy, Clone, Debug, Eq, PartialEq, FromPrimitive, IntoPrimitive, Serialize, Deserialize,
)]
#[repr(u8)]
pub enum Gen3Ability {
    #[num_enum(default)]
    Ability0 = 0,
    Ability1 = 1,
}

impl SingleFilter for Gen3Ability {}

impl_display!(Gen3Ability);

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
pub enum ShinyType {
    Star,
    Square,
}

impl ShinyType {
    pub fn from_xor(xor: u16, compare: u16) -> Option<Self> {
        if xor == 0 {
            return Some(Self::Square);
        }

        if xor < compare {
            return Some(Self::Star);
        }

        None
    }

    pub fn calculate_shiny_gen8(pid: u32, shiny_rand: u32) -> Option<Self> {
        let psv = shiny_rand & 0xFFFF ^ shiny_rand >> 0x10;
        let tsv = pid >> 0x10 ^ pid & 0xFFFF;
        Self::from_xor((psv ^ tsv) as u16, 0x10)
    }

    pub fn calculate_shiny_gen3(pid: u32, tsv: u16) -> Option<Self> {
        let psv = ((pid & 0xFFFF) ^ (pid >> 0x10)) as u16;
        Self::from_xor(psv ^ tsv, 8)
    }
}

impl MultiFilter for ShinyType {}

impl_display!(ShinyType);

#[wasm_bindgen]
#[derive(
    Copy, Clone, Debug, Eq, PartialEq, FromPrimitive, IntoPrimitive, Serialize, Deserialize,
)]
#[repr(u8)]
pub enum Nature {
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

impl MultiFilter for Nature {}

impl_display!(Nature);

#[wasm_bindgen]
#[derive(
    Copy, Clone, Debug, Eq, PartialEq, FromPrimitive, IntoPrimitive, Serialize, Deserialize,
)]
#[repr(u8)]
pub enum AbilityNumber {
    #[num_enum(default)]
    Ability0 = 0,
    Ability1 = 1,
}

impl std::fmt::Display for AbilityNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self as u8))
    }
}

#[wasm_bindgen]
#[derive(
    Copy, Clone, Debug, Eq, PartialEq, FromPrimitive, IntoPrimitive, Serialize, Deserialize,
)]
#[repr(u8)]
pub enum EncounterSlot {
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

impl SingleFilter for EncounterSlot {}

impl std::fmt::Display for EncounterSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self as u8).to_string())
    }
}

#[wasm_bindgen]
#[derive(
    Copy,
    Clone,
    Debug,
    Eq,
    PartialEq,
    PartialOrd,
    FromPrimitive,
    IntoPrimitive,
    Serialize,
    Deserialize,
)]
#[repr(u8)]
pub enum GenderRatio {
    #[num_enum(default)]
    Genderless = 255,
    Male50Female50 = 127,
    Male25Female75 = 191,
    Male75Female25 = 63,
    Male875Female125 = 31,
    Male = 0,
    Female = 254,
}

impl GenderRatio {
    pub fn get_set_gender(&self) -> Option<Gender> {
        match self {
            Self::Male => Some(Gender::Male),
            Self::Female => Some(Gender::Female),
            Self::Genderless => Some(Gender::Genderless),
            _ => None,
        }
    }

    pub fn get_gender(&self, gender_num: u8) -> Gender {
        match self {
            Self::Male => Gender::Male,
            Self::Female => Gender::Female,
            Self::Genderless => Gender::Genderless,
            _ => {
                if gender_num < *self as u8 {
                    Gender::Female
                } else {
                    Gender::Male
                }
            }
        }
    }
}

impl_display!(GenderRatio);

#[wasm_bindgen]
#[derive(PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Gender {
    Genderless,
    Male,
    Female,
}

impl SingleFilter for Gender {}

impl_display!(Gender);

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum IDFilter {
    TID,
    SID,
    TSV,
    G8TID,
}

impl_display!(IDFilter);

#[derive(Clone, Copy, Debug, Eq, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum HiddenPower {
    #[num_enum(default)]
    Fighting = 0,
    Flying = 1,
    Poison = 2,
    Ground = 3,
    Rock = 4,
    Bug = 5,
    Ghost = 6,
    Steel = 7,
    Fire = 8,
    Water = 9,
    Grass = 10,
    Electric = 11,
    Psychic = 12,
    Ice = 13,
    Dragon = 14,
    Dark = 15,
}

impl_display!(HiddenPower);

impl MultiFilter for HiddenPower {}
