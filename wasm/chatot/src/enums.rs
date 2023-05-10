use num_enum::{FromPrimitive, IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};
use serde_repr::*;
use std::fmt;
use wasm_bindgen::prelude::*;

macro_rules! impl_display {
    ($enum:ident) => {
        impl fmt::Display for $enum {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{:?}", self)
            }
        }
    };
}

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

impl_display!(HiddenPower);

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, PartialEq, TryFromPrimitive, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum LeadFilter {
    None = 0,
    Synchronize = 1,
}

impl_display!(LeadFilter);

impl From<chatot_forms::Gen3Lead> for LeadFilter {
    fn from(value: chatot_forms::Gen3Lead) -> Self {
        match value {
            chatot_forms::Gen3Lead::None => Self::None,
            chatot_forms::Gen3Lead::Synchronize => Self::Synchronize,
        }
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, PartialEq, TryFromPrimitive, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum MethodFilter {
    MethodH1 = 1,
    MethodH2 = 2,
    MethodH4 = 4,
}

impl_display!(MethodFilter);

impl From<chatot_forms::Gen3Method> for MethodFilter {
    fn from(value: chatot_forms::Gen3Method) -> Self {
        match value {
            chatot_forms::Gen3Method::H1 => Self::MethodH1,
            chatot_forms::Gen3Method::H2 => Self::MethodH2,
            chatot_forms::Gen3Method::H4 => Self::MethodH4,
        }
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, PartialEq, TryFromPrimitive, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum AbilityFilter {
    Any = 3,
    Ability0 = 0,
    Ability1 = 1,
}

impl_display!(AbilityFilter);

impl PartialEq<Ability> for AbilityFilter {
    fn eq(&self, other: &Ability) -> bool {
        match (self, other) {
            (AbilityFilter::Any, _) => true,
            (_, _) => (*self as u32) == (*other as u32),
        }
    }
}

impl From<chatot_forms::Gen3AbilityFilter> for AbilityFilter {
    fn from(value: chatot_forms::Gen3AbilityFilter) -> Self {
        match value {
            chatot_forms::Gen3AbilityFilter::Any => Self::Any,
            chatot_forms::Gen3AbilityFilter::Ability0 => Self::Ability0,
            chatot_forms::Gen3AbilityFilter::Ability1 => Self::Ability1,
        }
    }
}

#[wasm_bindgen]
#[derive(
    Copy, Clone, Debug, Eq, PartialEq, FromPrimitive, IntoPrimitive, Serialize, Deserialize,
)]
#[repr(u32)]
pub enum Ability {
    #[num_enum(default)]
    Ability0 = 0,
    Ability1 = 1,
}

impl fmt::Display for Ability {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Ability0 => write!(f, "0"),
            Self::Ability1 => write!(f, "1"),
        }
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, PartialEq, FromPrimitive, Serialize, Deserialize)]
#[repr(u16)]
pub enum DeprecatedNatureFilter {
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

impl_display!(DeprecatedNatureFilter);

impl From<chatot_forms::NatureFilter> for DeprecatedNatureFilter {
    fn from(value: chatot_forms::NatureFilter) -> Self {
        match value {
            chatot_forms::NatureFilter::Hardy => Self::Hardy,
            chatot_forms::NatureFilter::Lonely => Self::Lonely,
            chatot_forms::NatureFilter::Brave => Self::Brave,
            chatot_forms::NatureFilter::Adamant => Self::Adamant,
            chatot_forms::NatureFilter::Naughty => Self::Naughty,
            chatot_forms::NatureFilter::Bold => Self::Bold,
            chatot_forms::NatureFilter::Docile => Self::Docile,
            chatot_forms::NatureFilter::Relaxed => Self::Relaxed,
            chatot_forms::NatureFilter::Impish => Self::Impish,
            chatot_forms::NatureFilter::Lax => Self::Lax,
            chatot_forms::NatureFilter::Timid => Self::Timid,
            chatot_forms::NatureFilter::Hasty => Self::Hasty,
            chatot_forms::NatureFilter::Serious => Self::Serious,
            chatot_forms::NatureFilter::Jolly => Self::Jolly,
            chatot_forms::NatureFilter::Naive => Self::Naive,
            chatot_forms::NatureFilter::Modest => Self::Modest,
            chatot_forms::NatureFilter::Mild => Self::Mild,
            chatot_forms::NatureFilter::Quiet => Self::Quiet,
            chatot_forms::NatureFilter::Bashful => Self::Bashful,
            chatot_forms::NatureFilter::Rash => Self::Rash,
            chatot_forms::NatureFilter::Calm => Self::Calm,
            chatot_forms::NatureFilter::Gentle => Self::Gentle,
            chatot_forms::NatureFilter::Sassy => Self::Sassy,
            chatot_forms::NatureFilter::Careful => Self::Careful,
            chatot_forms::NatureFilter::Quirky => Self::Quirky,
            chatot_forms::NatureFilter::Any => Self::Any,
        }
    }
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

impl_display!(Nature);

impl PartialEq<Nature> for DeprecatedNatureFilter {
    fn eq(&self, other: &Nature) -> bool {
        match (self, other) {
            (DeprecatedNatureFilter::Any, _) => true,
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

impl_display!(ShinyFilter);

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

impl From<chatot_forms::ShinyTypeFilter> for ShinyFilter {
    fn from(value: chatot_forms::ShinyTypeFilter) -> Self {
        match value {
            chatot_forms::ShinyTypeFilter::None => Self::None,
            chatot_forms::ShinyTypeFilter::Star => Self::Star,
            chatot_forms::ShinyTypeFilter::Square => Self::Square,
            chatot_forms::ShinyTypeFilter::Both => Self::Both,
            chatot_forms::ShinyTypeFilter::Any => Self::Any,
        }
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum DeprecatedEncounterFilter {
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

impl_display!(Shiny);

#[wasm_bindgen]
#[derive(
    Copy, Clone, Debug, Eq, PartialEq, FromPrimitive, IntoPrimitive, Serialize, Deserialize,
)]
#[repr(u32)]
pub enum DeprecatedEncounterSlotFilter {
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

impl_display!(DeprecatedEncounterSlotFilter);

impl From<chatot_forms::EncounterSlotFilter> for DeprecatedEncounterSlotFilter {
    fn from(value: chatot_forms::EncounterSlotFilter) -> Self {
        match value {
            chatot_forms::EncounterSlotFilter::Any => Self::Any,
            chatot_forms::EncounterSlotFilter::Slot0 => Self::Slot0,
            chatot_forms::EncounterSlotFilter::Slot1 => Self::Slot1,
            chatot_forms::EncounterSlotFilter::Slot2 => Self::Slot2,
            chatot_forms::EncounterSlotFilter::Slot3 => Self::Slot3,
            chatot_forms::EncounterSlotFilter::Slot4 => Self::Slot4,
            chatot_forms::EncounterSlotFilter::Slot5 => Self::Slot5,
            chatot_forms::EncounterSlotFilter::Slot6 => Self::Slot6,
            chatot_forms::EncounterSlotFilter::Slot7 => Self::Slot7,
            chatot_forms::EncounterSlotFilter::Slot8 => Self::Slot8,
            chatot_forms::EncounterSlotFilter::Slot9 => Self::Slot9,
            chatot_forms::EncounterSlotFilter::Slot10 => Self::Slot10,
            chatot_forms::EncounterSlotFilter::Slot11 => Self::Slot11,
        }
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, PartialEq, FromPrimitive)]
#[repr(u32)]
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

impl PartialEq<u8> for DeprecatedEncounterSlotFilter {
    fn eq(&self, other: &u8) -> bool {
        match (self, other) {
            (DeprecatedEncounterSlotFilter::Any, _) => true,
            (_, _) => (*self as u8) == (*other),
        }
    }
}

#[wasm_bindgen]
#[derive(
    Copy, Clone, Debug, Eq, PartialEq, FromPrimitive, PartialOrd, Serialize_repr, Deserialize_repr,
)]
#[repr(u32)]
pub enum DeprecatedGenderRatio {
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

impl_display!(DeprecatedGenderRatio);

impl From<chatot_forms::GenderRatio> for DeprecatedGenderRatio {
    fn from(value: chatot_forms::GenderRatio) -> Self {
        match value {
            chatot_forms::GenderRatio::NoSetGender => Self::NoSetGender,
            chatot_forms::GenderRatio::Genderless => Self::Genderless,
            chatot_forms::GenderRatio::Male50Female50 => Self::Male50Female50,
            chatot_forms::GenderRatio::Male25Female75 => Self::Male25Female75,
            chatot_forms::GenderRatio::Male75Female25 => Self::Male75Female25,
            chatot_forms::GenderRatio::Male875Female125 => Self::Male875Female125,
            chatot_forms::GenderRatio::Male => Self::Male,
            chatot_forms::GenderRatio::Female => Self::Female,
        }
    }
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

impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Genderless => write!(f, "Genderless"),
            Self::Male => write!(f, "♂"),
            Self::Female => write!(f, "♀"),
        }
    }
}

#[wasm_bindgen]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Eq, Clone, Copy, Debug, FromPrimitive)]
#[repr(u32)]
pub enum DeprecatedGenderFilter {
    #[num_enum(default)]
    Any = 256,
    Male = 0,
    Female = 254,
}

impl_display!(DeprecatedGenderFilter);

impl PartialEq<Gender> for DeprecatedGenderFilter {
    fn eq(&self, other: &Gender) -> bool {
        match (self, other) {
            (DeprecatedGenderFilter::Any, _) => true,
            (_, _) => (*self as u32) == (*other as u32),
        }
    }
}

impl From<chatot_forms::GenderFilter> for DeprecatedGenderFilter {
    fn from(value: chatot_forms::GenderFilter) -> Self {
        match value {
            chatot_forms::GenderFilter::Any => Self::Any,
            chatot_forms::GenderFilter::Male => Self::Male,
            chatot_forms::GenderFilter::Female => Self::Female,
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

pub fn get_set_gender_from_ratio(gender_ratio: &DeprecatedGenderRatio) -> Option<Gender> {
    match gender_ratio {
        DeprecatedGenderRatio::Male => Some(Gender::Male),
        DeprecatedGenderRatio::Female => Some(Gender::Female),
        DeprecatedGenderRatio::Genderless => Some(Gender::Genderless),
        _ => None,
    }
}

pub fn get_gender_from_ratio(gender_ratio: &DeprecatedGenderRatio, gender_num: u32) -> Gender {
    match gender_ratio {
        DeprecatedGenderRatio::Male => Gender::Male,
        DeprecatedGenderRatio::Female => Gender::Female,
        DeprecatedGenderRatio::Genderless => Gender::Genderless,
        _ => {
            if gender_num < *gender_ratio as u32 {
                Gender::Female
            } else {
                Gender::Male
            }
        }
    }
}
