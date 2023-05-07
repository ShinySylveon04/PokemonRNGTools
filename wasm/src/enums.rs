use num_enum::FromPrimitive;
use serde::{Deserialize, Serialize};
use serde_repr::*;
use std::fmt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Eq, PartialEq, FromPrimitive, Serialize, Deserialize)]
#[repr(u8)]
pub enum HiddenPower {
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
    #[num_enum(default)]
    Invalid = 16,
}

impl fmt::Display for HiddenPower {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum LeadFilter {
    None = 0,
    Synchronize = 1,
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum MethodFilter {
    MethodH1 = 1,
    MethodH2 = 2,
    MethodH4 = 4,
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum AbilityFilter {
    Any = 3,
    Ability0 = 0,
    Ability1 = 1,
}

impl PartialEq<Ability> for AbilityFilter {
    fn eq(&self, other: &Ability) -> bool {
        match (self, other) {
            (AbilityFilter::Any, _) => true,
            (_, _) => (*self as u32) == (*other as u32),
        }
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, PartialEq, FromPrimitive, Serialize, Deserialize)]
#[repr(u32)]
pub enum Ability {
    #[num_enum(default)]
    Ability0 = 0,
    Ability1 = 1,
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, PartialEq, FromPrimitive, Serialize, Deserialize)]
#[repr(u16)]
pub enum NatureFilter {
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
#[repr(u16)]
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
    Synchronize = 25,
}

impl PartialEq<Nature> for NatureFilter {
    fn eq(&self, other: &Nature) -> bool {
        match (self, other) {
            (NatureFilter::Any, _) => true,
            (_, _) => (*self as u32) == (*other as u32),
        }
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum ShinyFilter {
    None = 0,
    Star = 1,
    Square = 2,
    Both = 3,
    Any = 4,
}

impl PartialEq<Shiny> for ShinyFilter {
    fn eq(&self, other: &Shiny) -> bool {
        match (self, other) {
            (ShinyFilter::Star, Shiny::Star) => true,
            (ShinyFilter::Square, Shiny::Square) => true,
            (ShinyFilter::None, Shiny::None) => true,
            (ShinyFilter::Both, Shiny::Square) => true,
            (ShinyFilter::Both, Shiny::Star) => true,
            (ShinyFilter::Any, _) => true,
            (_, _) => false,
        }
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum EncounterFilter {
    Static = 0,
    Dynamic = 1,
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Shiny {
    None = 0,
    Star = 1,
    Square = 2,
    Both = 3,
    All = 4,
}

impl Shiny {
    pub fn from_xor(xor: u16, compare: u16) -> Self {
        if xor == 0 {
            return Self::Square;
        }

        if xor < compare {
            return Self::Star;
        }

        Self::None
    }

    pub fn calculate_shiny_gen8(pid: u32, shiny_rand: u32) -> Self {
        let psv = shiny_rand & 0xFFFF ^ shiny_rand >> 0x10;
        let tsv = pid >> 0x10 ^ pid & 0xFFFF;
        Self::from_xor((psv ^ tsv) as u16, 0x10)
    }

    pub fn calculate_shiny_gen3(pid: u32, tsv: u16) -> Self {
        let psv = ((pid & 0xFFFF) ^ (pid >> 0x10)) as u16;
        Self::from_xor(psv ^ tsv, 8)
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, PartialEq, FromPrimitive)]
#[repr(usize)]
pub enum EncounterSlotFilter {
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

impl PartialEq<u8> for EncounterSlotFilter {
    fn eq(&self, other: &u8) -> bool {
        match (self, other) {
            (EncounterSlotFilter::Any, _) => true,
            (_, _) => (*self as u8) == (*other),
        }
    }
}

#[wasm_bindgen]
#[derive(
    Copy, Clone, Debug, Eq, PartialEq, FromPrimitive, PartialOrd, Serialize_repr, Deserialize_repr,
)]
#[repr(u32)]
pub enum GenderRatio {
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
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize, FromPrimitive)]
#[repr(u32)]
pub enum Gender {
    #[num_enum(default)]
    Genderless = 255,
    Male = 0,
    Female = 254,
}

#[wasm_bindgen]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Eq, Clone, Copy, Debug, FromPrimitive)]
#[repr(u32)]
pub enum GenderFilter {
    #[num_enum(default)]
    Any = 256,
    Male = 0,
    Female = 254,
}

impl PartialEq<Gender> for GenderFilter {
    fn eq(&self, other: &Gender) -> bool {
        match (self, other) {
            (GenderFilter::Any, _) => true,
            (_, _) => (*self as u32) == (*other as u32),
        }
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum IDFilter {
    TID = "TID",
    SID = "SID",
    TSV = "TSV",
    G8TID = "G8TID",
    None = "None",
}

pub fn get_sync_nature(lead_filter: &LeadFilter) -> Option<Nature> {
    match lead_filter {
        LeadFilter::Synchronize => Some(Nature::Synchronize),
        _ => None,
    }
}

pub fn get_set_gender_from_ratio(gender_ratio: &GenderRatio) -> Option<Gender> {
    match gender_ratio {
        GenderRatio::Male => Some(Gender::Male),
        GenderRatio::Female => Some(Gender::Female),
        GenderRatio::Genderless => Some(Gender::Genderless),
        _ => None,
    }
}

pub fn get_gender_from_ratio(gender_ratio: &GenderRatio, gender_num: u32) -> Gender {
    match gender_ratio {
        GenderRatio::Male => Gender::Male,
        GenderRatio::Female => Gender::Female,
        GenderRatio::Genderless => Gender::Genderless,
        _ => {
            if gender_num < *gender_ratio as u32 {
                Gender::Female
            } else {
                Gender::Male
            }
        }
    }
}
