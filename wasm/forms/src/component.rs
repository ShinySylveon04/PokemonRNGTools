use super::{common_types::*, field_size::FieldSize};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SelectOption {
    label: String,
    value: String,
}

impl SelectOption {
    pub fn new(label: impl ToString, value: impl ToString) -> Self {
        Self {
            label: label.to_string(),
            value: value.to_string(),
        }
    }

    pub fn new_simple(label: impl ToString) -> Self {
        Self {
            label: label.to_string(),
            value: label.to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldComponent {
    id: String,
    label: String,
    default_value: String,
    required: bool,
    r#type: String,
    size: String,
    options: Option<Vec<SelectOption>>,
}

impl FieldComponent {
    pub fn label(id: impl ToString, label: impl ToString, size: FieldSize) -> Self {
        Self {
            id: id.to_string(),
            label: label.to_string(),
            default_value: "".to_string(),
            required: true,
            r#type: "label".to_string(),
            options: None,
            size: size.to_string(),
        }
    }

    pub fn number(
        id: impl ToString,
        label: impl ToString,
        default_value: Option<u32>,
        size: FieldSize,
    ) -> Self {
        let default_value = match default_value {
            Some(num) => num.to_string(),
            None => "".to_string(),
        };
        Self {
            id: id.to_string(),
            label: label.to_string(),
            default_value: default_value,
            required: true,
            r#type: "number".to_string(),
            options: None,
            size: size.to_string(),
        }
    }

    pub fn hex_number(
        id: impl ToString,
        label: impl ToString,
        default_value: Option<u32>,
        size: FieldSize,
    ) -> Self {
        let default_value = match default_value {
            Some(num) => num.to_string(),
            None => "".to_string(),
        };
        Self {
            id: id.to_string(),
            label: label.to_string(),
            default_value: default_value,
            required: true,
            r#type: "hex_number".to_string(),
            options: None,
            size: size.to_string(),
        }
    }

    pub fn text(
        id: impl ToString,
        label: impl ToString,
        default_value: impl ToString,
        size: FieldSize,
    ) -> Self {
        Self {
            id: id.to_string(),
            label: label.to_string(),
            default_value: default_value.to_string(),
            required: true,
            r#type: "text".to_string(),
            options: None,
            size: size.to_string(),
        }
    }

    pub fn checkbox(id: impl ToString, label: impl ToString, size: FieldSize) -> Self {
        Self {
            id: id.to_string(),
            label: label.to_string(),
            default_value: "false".to_string(),
            required: true,
            r#type: "checkbox".to_string(),
            options: None,
            size: size.to_string(),
        }
    }

    pub fn select(
        id: impl ToString,
        label: impl ToString,
        options: Vec<SelectOption>,
        size: FieldSize,
    ) -> Self {
        let default_value = options
            .first()
            .map(|string| string.value.to_string())
            .unwrap_or_default();
        Self {
            id: id.to_string(),
            label: label.to_string(),
            default_value: default_value,
            required: true,
            r#type: "select".to_string(),
            options: Some(options),
            size: size.to_string(),
        }
    }

    pub fn multiselect(
        id: impl ToString,
        label: impl ToString,
        options: Vec<SelectOption>,
        size: FieldSize,
    ) -> Self {
        let default_value = options
            .first()
            .map(|string| string.value.to_string())
            .unwrap_or_default();
        Self {
            id: id.to_string(),
            label: label.to_string(),
            default_value: default_value,
            required: true,
            r#type: "multiselect".to_string(),
            options: Some(options),
            size: size.to_string(),
        }
    }
}

macro_rules! impl_sized_component {
    ($component:ident, $field_size:tt) => {
        impl $component {
            pub fn label(id: impl ToString, label: impl ToString) -> FieldComponent {
                FieldComponent::label(id, label, FieldSize::$field_size)
            }

            pub fn number(
                id: impl ToString,
                label: impl ToString,
                default_value: Option<u32>,
            ) -> FieldComponent {
                FieldComponent::number(id, label, default_value, FieldSize::$field_size)
            }

            pub fn hex_number(
                id: impl ToString,
                label: impl ToString,
                default_value: Option<u32>,
            ) -> FieldComponent {
                FieldComponent::hex_number(id, label, default_value, FieldSize::$field_size)
            }

            pub fn text(
                id: impl ToString,
                label: impl ToString,
                default_value: impl ToString,
            ) -> FieldComponent {
                FieldComponent::text(id, label, default_value, FieldSize::$field_size)
            }

            pub fn checkbox(id: impl ToString, label: impl ToString) -> FieldComponent {
                FieldComponent::checkbox(id, label, FieldSize::$field_size)
            }

            pub fn select(
                id: impl ToString,
                label: impl ToString,
                options: Vec<SelectOption>,
            ) -> FieldComponent {
                FieldComponent::select(id, label, options, FieldSize::$field_size)
            }

            pub fn multiselect(
                id: impl ToString,
                label: impl ToString,
                options: Vec<SelectOption>,
            ) -> FieldComponent {
                FieldComponent::multiselect(id, label, options, FieldSize::$field_size)
            }

            pub fn seed() -> FieldComponent {
                $component::hex_number("seed", "Seed", None)
            }
            pub fn seed_0() -> FieldComponent {
                $component::hex_number("seed_0", "Seed 0", None)
            }
            pub fn seed_1() -> FieldComponent {
                $component::hex_number("seed_1", "Seed 1", None)
            }
            pub fn seed_2() -> FieldComponent {
                $component::hex_number("seed_2", "Seed 2", None)
            }
            pub fn seed_3() -> FieldComponent {
                $component::hex_number("seed_3", "Seed 3", None)
            }
            pub fn min_advances() -> FieldComponent {
                $component::number("min_advances", "Min Advances", Some(0))
            }
            pub fn max_advances() -> FieldComponent {
                $component::number("max_advances", "Max Advances", Some(10000))
            }
            pub fn delay() -> FieldComponent {
                $component::number("delay", "Delay", Some(0))
            }
            pub fn tid() -> FieldComponent {
                $component::number("tid", "TID", Some(0))
            }
            pub fn sid() -> FieldComponent {
                $component::number("sid", "SID", Some(0))
            }
            pub fn min_ivs_label() -> FieldComponent {
                $component::label("min_ivs_label", "Min IVs")
            }
            pub fn min_hp_iv() -> FieldComponent {
                $component::number("min_hp_iv", "HP", Some(0))
            }
            pub fn min_atk_iv() -> FieldComponent {
                $component::number("min_atk_iv", "Attack", Some(0))
            }
            pub fn min_def_iv() -> FieldComponent {
                $component::number("min_def_iv", "Defense", Some(0))
            }
            pub fn min_spa_iv() -> FieldComponent {
                $component::number("min_spa_iv", "Special Attack", Some(0))
            }
            pub fn min_spd_iv() -> FieldComponent {
                $component::number("min_spd_iv", "Special Defense", Some(0))
            }
            pub fn min_spe_iv() -> FieldComponent {
                $component::number("min_spe_iv", "Speed", Some(0))
            }
            pub fn max_ivs_label() -> FieldComponent {
                $component::label("max_ivs_label", "Max IVs")
            }
            pub fn max_hp_iv() -> FieldComponent {
                $component::number("max_hp_iv", "HP", Some(31))
            }
            pub fn max_atk_iv() -> FieldComponent {
                $component::number("max_atk_iv", "Attack", Some(31))
            }
            pub fn max_def_iv() -> FieldComponent {
                $component::number("max_def_iv", "Defense", Some(31))
            }
            pub fn max_spa_iv() -> FieldComponent {
                $component::number("max_spa_iv", "Special Attack", Some(31))
            }
            pub fn max_spd_iv() -> FieldComponent {
                $component::number("max_spd_iv", "Special Defense", Some(31))
            }
            pub fn max_spe_iv() -> FieldComponent {
                $component::number("max_spe_iv", "Speed", Some(31))
            }
            pub fn gen3_method() -> FieldComponent {
                $component::select(
                    "gen3_method",
                    "Method",
                    vec![
                        SelectOption::new("Method H1", Gen3Method::H1),
                        SelectOption::new("Method H2", Gen3Method::H2),
                        SelectOption::new("Method H4", Gen3Method::H4),
                    ],
                )
            }
            pub fn gen3_lead() -> FieldComponent {
                $component::select(
                    "gen3_lead",
                    "Lead",
                    vec![
                        SelectOption::new("None", Gen3Lead::None),
                        SelectOption::new("Synchronize", Gen3Lead::Synchronize),
                    ],
                )
            }
            pub fn shiny_type() -> FieldComponent {
                $component::select(
                    "shiny_type",
                    "Shiny",
                    vec![
                        SelectOption::new("Any", ShinyTypeFilter::Any),
                        SelectOption::new("Star", ShinyTypeFilter::Star),
                        SelectOption::new("Square", ShinyTypeFilter::Square),
                        SelectOption::new("Star/Square", ShinyTypeFilter::Both),
                    ],
                )
            }
            pub fn nature_multiselect() -> FieldComponent {
                $component::multiselect(
                    "nature_multiselect",
                    "Nature",
                    vec![
                        SelectOption::new("Any", NatureFilter::Any),
                        SelectOption::new("Hardy", NatureFilter::Hardy),
                        SelectOption::new("Lonely", NatureFilter::Lonely),
                        SelectOption::new("Brave", NatureFilter::Brave),
                        SelectOption::new("Adamant", NatureFilter::Adamant),
                        SelectOption::new("Naughty", NatureFilter::Naughty),
                        SelectOption::new("Bold", NatureFilter::Bold),
                        SelectOption::new("Docile", NatureFilter::Docile),
                        SelectOption::new("Relaxed", NatureFilter::Relaxed),
                        SelectOption::new("Impish", NatureFilter::Impish),
                        SelectOption::new("Lax", NatureFilter::Lax),
                        SelectOption::new("Timid", NatureFilter::Timid),
                        SelectOption::new("Hasty", NatureFilter::Hasty),
                        SelectOption::new("Serious", NatureFilter::Serious),
                        SelectOption::new("Jolly", NatureFilter::Jolly),
                        SelectOption::new("Naive", NatureFilter::Naive),
                        SelectOption::new("Modest", NatureFilter::Modest),
                        SelectOption::new("Mild", NatureFilter::Mild),
                        SelectOption::new("Quiet", NatureFilter::Quiet),
                        SelectOption::new("Bashful", NatureFilter::Bashful),
                        SelectOption::new("Rash", NatureFilter::Rash),
                        SelectOption::new("Calm", NatureFilter::Calm),
                        SelectOption::new("Gentle", NatureFilter::Gentle),
                        SelectOption::new("Sassy", NatureFilter::Sassy),
                        SelectOption::new("Careful", NatureFilter::Careful),
                        SelectOption::new("Quirky", NatureFilter::Quirky),
                    ],
                )
            }
            pub fn gen3_ability() -> FieldComponent {
                $component::select(
                    "gen3_ability",
                    "Ability",
                    vec![
                        SelectOption::new("Any", Gen3AbilityFilter::Any),
                        SelectOption::new("0", Gen3AbilityFilter::Ability0),
                        SelectOption::new("1", Gen3AbilityFilter::Ability1),
                    ],
                )
            }
            pub fn encounter_slot() -> FieldComponent {
                $component::select(
                    "encounter_slot",
                    "Encounter Slot",
                    vec![
                        SelectOption::new("Any", EncounterSlotFilter::Any),
                        SelectOption::new("0", EncounterSlotFilter::Slot0),
                        SelectOption::new("1", EncounterSlotFilter::Slot1),
                        SelectOption::new("2", EncounterSlotFilter::Slot2),
                        SelectOption::new("3", EncounterSlotFilter::Slot3),
                        SelectOption::new("4", EncounterSlotFilter::Slot4),
                        SelectOption::new("5", EncounterSlotFilter::Slot5),
                        SelectOption::new("6", EncounterSlotFilter::Slot6),
                        SelectOption::new("7", EncounterSlotFilter::Slot7),
                        SelectOption::new("8", EncounterSlotFilter::Slot8),
                        SelectOption::new("9", EncounterSlotFilter::Slot9),
                        SelectOption::new("10", EncounterSlotFilter::Slot10),
                        SelectOption::new("11", EncounterSlotFilter::Slot11),
                    ],
                )
            }
            pub fn gender_ratio() -> FieldComponent {
                $component::select(
                    "gender_ratio",
                    "Gender Ratio",
                    vec![
                        SelectOption::new("No Set Gender", GenderRatio::NoSetGender),
                        SelectOption::new("Genderless", GenderRatio::Genderless),
                        SelectOption::new("50% ♂ / 50% ♀", GenderRatio::Male50Female50),
                        SelectOption::new("25% ♂ / 75% ♀", GenderRatio::Male25Female75),
                        SelectOption::new("75% ♂ / 25% ♀", GenderRatio::Male75Female25),
                        SelectOption::new("87.5% ♂ / 12.5% ♀", GenderRatio::Male875Female125),
                        SelectOption::new("100% ♂", GenderRatio::Male),
                        SelectOption::new("100% ♀", GenderRatio::Female),
                    ],
                )
            }
            pub fn gender() -> FieldComponent {
                $component::select(
                    "gender",
                    "Gender",
                    vec![
                        SelectOption::new("Any", GenderFilter::Any),
                        SelectOption::new("♂", GenderFilter::Male),
                        SelectOption::new("♀", GenderFilter::Female),
                    ],
                )
            }
        }
    };
}

pub struct LargeComponent;
pub struct SmallComponent;

impl_sized_component!(LargeComponent, Large);
impl_sized_component!(SmallComponent, Small);
