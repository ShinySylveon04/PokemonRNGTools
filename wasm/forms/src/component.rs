use super::{common_types::*, field_size::FieldSize};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SelectOption {
    label: String,
    value: String,
}

impl SelectOption {
    pub fn new(label: impl ToString) -> Self {
        Self {
            label: label.to_string(),
            value: label.to_string(),
        }
    }

    pub fn new_with_label(label: impl ToString, value: impl ToString) -> Self {
        Self {
            label: label.to_string(),
            value: value.to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldComponent {
    id: String,
    label: String,
    default_value: Option<String>,
    required: bool,
    r#type: String,
    size: String,
    options: Option<Vec<SelectOption>>,
    min_value: Option<u32>,
    max_value: Option<u32>,
}

impl FieldComponent {
    pub fn label(id: impl ToString, label: impl ToString, size: FieldSize) -> Self {
        Self {
            id: id.to_string(),
            label: label.to_string(),
            default_value: None,
            required: true,
            r#type: "label".to_string(),
            options: None,
            size: size.to_string(),
            min_value: None,
            max_value: None,
        }
    }

    pub fn number(
        id: impl ToString,
        label: impl ToString,
        default_value: Option<u32>,
        size: FieldSize,
    ) -> Self {
        Self {
            id: id.to_string(),
            label: label.to_string(),
            default_value: default_value.map(|value| value.to_string()),
            required: true,
            r#type: "number".to_string(),
            options: None,
            size: size.to_string(),
            min_value: Some(0),
            max_value: None,
        }
    }

    pub fn number_with_limits(
        id: impl ToString,
        label: impl ToString,
        default_value: Option<u32>,
        min_value: Option<u32>,
        max_value: Option<u32>,
        size: FieldSize,
    ) -> Self {
        Self {
            id: id.to_string(),
            label: label.to_string(),
            default_value: default_value.map(|num| num.to_string()),
            required: true,
            r#type: "number".to_string(),
            options: None,
            size: size.to_string(),
            min_value,
            max_value,
        }
    }

    pub fn hex_number(
        id: impl ToString,
        label: impl ToString,
        default_value: Option<u32>,
        size: FieldSize,
    ) -> Self {
        Self {
            id: id.to_string(),
            label: label.to_string(),
            default_value: default_value.map(|num| num.to_string()),
            required: true,
            r#type: "hex_number".to_string(),
            options: None,
            size: size.to_string(),
            min_value: None,
            max_value: None,
        }
    }

    pub fn hex_u64(
        id: impl ToString,
        label: impl ToString,
        default_value: Option<u32>,
        size: FieldSize,
    ) -> Self {
        Self {
            id: id.to_string(),
            label: label.to_string(),
            default_value: default_value.map(|num| num.to_string()),
            required: true,
            r#type: "hex_u64".to_string(),
            options: None,
            size: size.to_string(),
            min_value: None,
            max_value: None,
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
            default_value: Some(default_value.to_string()),
            required: true,
            r#type: "text".to_string(),
            options: None,
            size: size.to_string(),
            min_value: None,
            max_value: None,
        }
    }

    pub fn checkbox(id: impl ToString, label: impl ToString, size: FieldSize) -> Self {
        Self {
            id: id.to_string(),
            label: label.to_string(),
            default_value: Some("false".to_string()),
            required: true,
            r#type: "checkbox".to_string(),
            options: None,
            size: size.to_string(),
            min_value: None,
            max_value: None,
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
            default_value: Some(default_value),
            required: true,
            r#type: "select".to_string(),
            options: Some(options),
            size: size.to_string(),
            min_value: None,
            max_value: None,
        }
    }

    pub fn optional_select(
        id: impl ToString,
        label: impl ToString,
        options: Vec<SelectOption>,
        size: FieldSize,
    ) -> Self {
        Self {
            id: id.to_string(),
            label: label.to_string(),
            default_value: None,
            required: true,
            r#type: "optional_select".to_string(),
            options: Some(options),
            size: size.to_string(),
            min_value: None,
            max_value: None,
        }
    }

    pub fn multiselect(
        id: impl ToString,
        label: impl ToString,
        options: Vec<SelectOption>,
        size: FieldSize,
    ) -> Self {
        Self {
            id: id.to_string(),
            label: label.to_string(),
            default_value: None,
            required: false,
            r#type: "multiselect".to_string(),
            options: Some(options),
            size: size.to_string(),
            min_value: None,
            max_value: None,
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

            pub fn number_with_limits(
                id: impl ToString,
                label: impl ToString,
                default_value: Option<u32>,
                min_value: Option<u32>,
                max_value: Option<u32>,
            ) -> FieldComponent {
                FieldComponent::number_with_limits(
                    id,
                    label,
                    default_value,
                    min_value,
                    max_value,
                    FieldSize::$field_size,
                )
            }

            pub fn hex_number(
                id: impl ToString,
                label: impl ToString,
                default_value: Option<u32>,
            ) -> FieldComponent {
                FieldComponent::hex_number(id, label, default_value, FieldSize::$field_size)
            }

            pub fn hex_u64(
                id: impl ToString,
                label: impl ToString,
                default_value: Option<u32>,
            ) -> FieldComponent {
                FieldComponent::hex_u64(id, label, default_value, FieldSize::$field_size)
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

            pub fn optional_select(
                id: impl ToString,
                label: impl ToString,
                options: Vec<SelectOption>,
            ) -> FieldComponent {
                FieldComponent::optional_select(id, label, options, FieldSize::$field_size)
            }

            pub fn multiselect(
                id: impl ToString,
                label: impl ToString,
                options: Vec<SelectOption>,
            ) -> FieldComponent {
                FieldComponent::multiselect(id, label, options, FieldSize::$field_size)
            }

            fn min_iv(id: impl ToString, label: impl ToString) -> FieldComponent {
                $component::number_with_limits(id, label, Some(0), Some(0), Some(31))
            }

            fn max_iv(id: impl ToString, label: impl ToString) -> FieldComponent {
                $component::number_with_limits(id, label, Some(31), Some(0), Some(31))
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
            // Adding for when we implement validation
            pub fn seed_u64_0() -> FieldComponent {
                $component::hex_u64("seed_u64_0", "Seed 0", None)
            }
            // Adding for when we implement validation
            pub fn seed_u64_1() -> FieldComponent {
                $component::hex_u64("seed_u64_1", "Seed 1", None)
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
                $component::number_with_limits("tid", "TID", Some(0), Some(0), Some(99999))
            }
            pub fn sid() -> FieldComponent {
                $component::number_with_limits("sid", "SID", Some(0), Some(0), Some(99999))
            }
            pub fn min_ivs_label() -> FieldComponent {
                $component::label("min_ivs_label", "Min IVs")
            }
            pub fn min_hp_iv() -> FieldComponent {
                $component::min_iv("min_hp_iv", "HP")
            }
            pub fn min_atk_iv() -> FieldComponent {
                $component::min_iv("min_atk_iv", "Attack")
            }
            pub fn min_def_iv() -> FieldComponent {
                $component::min_iv("min_def_iv", "Defense")
            }
            pub fn min_spa_iv() -> FieldComponent {
                $component::min_iv("min_spa_iv", "Special Attack")
            }
            pub fn min_spd_iv() -> FieldComponent {
                $component::min_iv("min_spd_iv", "Special Defense")
            }
            pub fn min_spe_iv() -> FieldComponent {
                $component::min_iv("min_spe_iv", "Speed")
            }
            pub fn max_ivs_label() -> FieldComponent {
                $component::label("max_ivs_label", "Max IVs")
            }
            pub fn max_hp_iv() -> FieldComponent {
                $component::max_iv("max_hp_iv", "HP")
            }
            pub fn max_atk_iv() -> FieldComponent {
                $component::max_iv("max_atk_iv", "Attack")
            }
            pub fn max_def_iv() -> FieldComponent {
                $component::max_iv("max_def_iv", "Defense")
            }
            pub fn max_spa_iv() -> FieldComponent {
                $component::max_iv("max_spa_iv", "Special Attack")
            }
            pub fn max_spd_iv() -> FieldComponent {
                $component::max_iv("max_spd_iv", "Special Defense")
            }
            pub fn max_spe_iv() -> FieldComponent {
                $component::max_iv("max_spe_iv", "Speed")
            }
            pub fn gen3_method() -> FieldComponent {
                $component::select(
                    "gen3_method",
                    "Method",
                    vec![
                        SelectOption::new_with_label("Method H1", Gen3Method::H1),
                        SelectOption::new_with_label("Method H2", Gen3Method::H2),
                        SelectOption::new_with_label("Method H4", Gen3Method::H4),
                    ],
                )
            }
            pub fn gen3_lead() -> FieldComponent {
                $component::optional_select(
                    "gen3_lead",
                    "Lead",
                    vec![SelectOption::new(Gen3Lead::Synchronize)],
                )
            }
            pub fn gen8_id_type() -> FieldComponent {
                $component::optional_select(
                    "gen8_id_type",
                    "ID Filter",
                    vec![
                        SelectOption::new_with_label("TID", IDFilter::TID),
                        SelectOption::new_with_label("SID", IDFilter::SID),
                        SelectOption::new_with_label("TSV", IDFilter::TSV),
                        SelectOption::new_with_label("Gen 8 TID", IDFilter::G8TID),
                    ],
                )
            }
            pub fn shiny_type() -> FieldComponent {
                $component::multiselect(
                    "shiny_type",
                    "Shiny",
                    vec![
                        SelectOption::new(ShinyType::Star),
                        SelectOption::new(ShinyType::Square),
                    ],
                )
            }
            pub fn nature() -> FieldComponent {
                $component::optional_select(
                    "nature",
                    "Nature",
                    vec![
                        SelectOption::new(Nature::Hardy),
                        SelectOption::new(Nature::Lonely),
                        SelectOption::new(Nature::Brave),
                        SelectOption::new(Nature::Adamant),
                        SelectOption::new(Nature::Naughty),
                        SelectOption::new(Nature::Bold),
                        SelectOption::new(Nature::Docile),
                        SelectOption::new(Nature::Relaxed),
                        SelectOption::new(Nature::Impish),
                        SelectOption::new(Nature::Lax),
                        SelectOption::new(Nature::Timid),
                        SelectOption::new(Nature::Hasty),
                        SelectOption::new(Nature::Serious),
                        SelectOption::new(Nature::Jolly),
                        SelectOption::new(Nature::Naive),
                        SelectOption::new(Nature::Modest),
                        SelectOption::new(Nature::Mild),
                        SelectOption::new(Nature::Quiet),
                        SelectOption::new(Nature::Bashful),
                        SelectOption::new(Nature::Rash),
                        SelectOption::new(Nature::Calm),
                        SelectOption::new(Nature::Gentle),
                        SelectOption::new(Nature::Sassy),
                        SelectOption::new(Nature::Careful),
                        SelectOption::new(Nature::Quirky),
                    ],
                )
            }
            pub fn nature_multiselect() -> FieldComponent {
                $component::multiselect(
                    "nature_multiselect",
                    "Nature",
                    vec![
                        SelectOption::new(Nature::Hardy),
                        SelectOption::new(Nature::Lonely),
                        SelectOption::new(Nature::Brave),
                        SelectOption::new(Nature::Adamant),
                        SelectOption::new(Nature::Naughty),
                        SelectOption::new(Nature::Bold),
                        SelectOption::new(Nature::Docile),
                        SelectOption::new(Nature::Relaxed),
                        SelectOption::new(Nature::Impish),
                        SelectOption::new(Nature::Lax),
                        SelectOption::new(Nature::Timid),
                        SelectOption::new(Nature::Hasty),
                        SelectOption::new(Nature::Serious),
                        SelectOption::new(Nature::Jolly),
                        SelectOption::new(Nature::Naive),
                        SelectOption::new(Nature::Modest),
                        SelectOption::new(Nature::Mild),
                        SelectOption::new(Nature::Quiet),
                        SelectOption::new(Nature::Bashful),
                        SelectOption::new(Nature::Rash),
                        SelectOption::new(Nature::Calm),
                        SelectOption::new(Nature::Gentle),
                        SelectOption::new(Nature::Sassy),
                        SelectOption::new(Nature::Careful),
                        SelectOption::new(Nature::Quirky),
                    ],
                )
            }
            pub fn gen3_ability() -> FieldComponent {
                $component::optional_select(
                    "gen3_ability",
                    "Ability",
                    vec![
                        SelectOption::new_with_label("0", Gen3Ability::Ability0),
                        SelectOption::new_with_label("1", Gen3Ability::Ability1),
                    ],
                )
            }
            pub fn encounter_slot() -> FieldComponent {
                $component::optional_select(
                    "encounter_slot",
                    "Encounter Slot",
                    vec![
                        SelectOption::new_with_label("0", EncounterSlot::Slot0),
                        SelectOption::new_with_label("1", EncounterSlot::Slot1),
                        SelectOption::new_with_label("2", EncounterSlot::Slot2),
                        SelectOption::new_with_label("3", EncounterSlot::Slot3),
                        SelectOption::new_with_label("4", EncounterSlot::Slot4),
                        SelectOption::new_with_label("5", EncounterSlot::Slot5),
                        SelectOption::new_with_label("6", EncounterSlot::Slot6),
                        SelectOption::new_with_label("7", EncounterSlot::Slot7),
                        SelectOption::new_with_label("8", EncounterSlot::Slot8),
                        SelectOption::new_with_label("9", EncounterSlot::Slot9),
                        SelectOption::new_with_label("10", EncounterSlot::Slot10),
                        SelectOption::new_with_label("11", EncounterSlot::Slot11),
                    ],
                )
            }
            pub fn gender_ratio() -> FieldComponent {
                $component::select(
                    "gender_ratio",
                    "Gender Ratio",
                    vec![
                        SelectOption::new_with_label("Genderless", GenderRatio::Genderless),
                        SelectOption::new_with_label("50% ♂ / 50% ♀", GenderRatio::Male50Female50),
                        SelectOption::new_with_label("25% ♂ / 75% ♀", GenderRatio::Male25Female75),
                        SelectOption::new_with_label("75% ♂ / 25% ♀", GenderRatio::Male75Female25),
                        SelectOption::new_with_label(
                            "87.5% ♂ / 12.5% ♀",
                            GenderRatio::Male875Female125,
                        ),
                        SelectOption::new_with_label("100% ♂", GenderRatio::Male),
                        SelectOption::new_with_label("100% ♀", GenderRatio::Female),
                    ],
                )
            }
            pub fn gender() -> FieldComponent {
                $component::optional_select(
                    "gender",
                    "Gender",
                    vec![
                        SelectOption::new_with_label("♂", Gender::Male),
                        SelectOption::new_with_label("♀", Gender::Female),
                    ],
                )
            }
            pub fn shiny_charm() -> FieldComponent {
                $component::checkbox("shiny_charm", "Shiny Charm")
            }
        }
    };
}

pub struct LargeComponent;
pub struct SmallComponent;

impl_sized_component!(LargeComponent, Large);
impl_sized_component!(SmallComponent, Small);
