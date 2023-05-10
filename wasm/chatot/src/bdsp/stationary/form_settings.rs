use super::{generator, settings};
use crate::{
    enums::{AbilityFilter, DeprecatedGenderRatio, DeprecatedNatureFilter},
    utils::format_ivs,
};
use chatot_forms::{
    impl_display, FieldGroup, Gen3Lead, GenderFilter, LargeComponent, NatureFilter, SelectOption,
    ShinyTypeFilter, SmallComponent,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum StaticPokemon {
    Turtwig,
    Chimchar,
    Piplup,
    Eevee,
    Happiny,
    Riolu,
    Mew,
    Jirachi,
    Omanyte,
    Kabuto,
    Aerodactyl,
    Lileep,
    Anorith,
    Cranidos,
    Shieldon,
    Drifloon,
    Spiritomb,
    Rotom,
    Mesprit,
    Cresselia,
    Uxie,
    Azelf,
    Dialga,
    Palkia,
    Heatran,
    Regigigas,
    Giratina,
    Articuno,
    Zapdos,
    Moltres,
    Raikou,
    Entei,
    Suicune,
    Regirock,
    Regice,
    Registeel,
    Latias,
    Latios,
    Mewtwo,
    Lugia,
    HoOh,
    Kyogre,
    Groudon,
    Rayquaza,
    Darkrai,
    Shaymin,
    Arceus,
}

impl_display!(StaticPokemon);
impl StaticPokemon {
    fn is_roamer(&self) -> bool {
        match self {
            Self::Mesprit | Self::Cresselia => true,
            _ => false,
        }
    }

    fn ability(&self) -> AbilityFilter {
        match self {
            Self::Jirachi | Self::Mew => AbilityFilter::Ability1,
            _ => AbilityFilter::Any,
        }
    }

    fn set_ivs(&self) -> bool {
        match self {
            Self::Chimchar
            | Self::Drifloon
            | Self::Eevee
            | Self::Happiny
            | Self::Piplup
            | Self::Riolu
            | Self::Rotom
            | Self::Spiritomb
            | Self::Turtwig => false,
            _ => true,
        }
    }
    fn gender_ratio(&self) -> DeprecatedGenderRatio {
        match self {
            Self::Turtwig => DeprecatedGenderRatio::Male875Female125,
            Self::Chimchar => DeprecatedGenderRatio::Male875Female125,
            Self::Piplup => DeprecatedGenderRatio::Male875Female125,
            Self::Eevee => DeprecatedGenderRatio::Male875Female125,
            Self::Happiny => DeprecatedGenderRatio::Female,
            Self::Riolu => DeprecatedGenderRatio::Male875Female125,
            Self::Mew => DeprecatedGenderRatio::Genderless,
            Self::Jirachi => DeprecatedGenderRatio::Genderless,
            Self::Omanyte => DeprecatedGenderRatio::Male875Female125,
            Self::Kabuto => DeprecatedGenderRatio::Male875Female125,
            Self::Aerodactyl => DeprecatedGenderRatio::Male875Female125,
            Self::Lileep => DeprecatedGenderRatio::Male875Female125,
            Self::Anorith => DeprecatedGenderRatio::Male875Female125,
            Self::Cranidos => DeprecatedGenderRatio::Male875Female125,
            Self::Shieldon => DeprecatedGenderRatio::Male875Female125,
            Self::Drifloon => DeprecatedGenderRatio::Male50Female50,
            Self::Spiritomb => DeprecatedGenderRatio::Male50Female50,
            Self::Rotom => DeprecatedGenderRatio::Genderless,
            Self::Mesprit => DeprecatedGenderRatio::Genderless,
            Self::Cresselia => DeprecatedGenderRatio::Genderless,
            Self::Uxie => DeprecatedGenderRatio::Genderless,
            Self::Azelf => DeprecatedGenderRatio::Genderless,
            Self::Dialga => DeprecatedGenderRatio::Genderless,
            Self::Palkia => DeprecatedGenderRatio::Genderless,
            Self::Heatran => DeprecatedGenderRatio::Genderless,
            Self::Regigigas => DeprecatedGenderRatio::Genderless,
            Self::Giratina => DeprecatedGenderRatio::Genderless,
            Self::Articuno => DeprecatedGenderRatio::Genderless,
            Self::Zapdos => DeprecatedGenderRatio::Genderless,
            Self::Moltres => DeprecatedGenderRatio::Genderless,
            Self::Raikou => DeprecatedGenderRatio::Genderless,
            Self::Entei => DeprecatedGenderRatio::Genderless,
            Self::Suicune => DeprecatedGenderRatio::Genderless,
            Self::Regirock => DeprecatedGenderRatio::Genderless,
            Self::Regice => DeprecatedGenderRatio::Genderless,
            Self::Registeel => DeprecatedGenderRatio::Genderless,
            Self::Latias => DeprecatedGenderRatio::Genderless,
            Self::Latios => DeprecatedGenderRatio::Genderless,
            Self::Mewtwo => DeprecatedGenderRatio::Genderless,
            Self::Lugia => DeprecatedGenderRatio::Genderless,
            Self::HoOh => DeprecatedGenderRatio::Genderless,
            Self::Kyogre => DeprecatedGenderRatio::Genderless,
            Self::Groudon => DeprecatedGenderRatio::Genderless,
            Self::Rayquaza => DeprecatedGenderRatio::Genderless,
            Self::Darkrai => DeprecatedGenderRatio::Genderless,
            Self::Shaymin => DeprecatedGenderRatio::Genderless,
            Self::Arceus => DeprecatedGenderRatio::Genderless,
        }
    }
}

pub fn get_field_groups() -> Vec<FieldGroup> {
    let rng_info_components = vec![
        LargeComponent::seed_0(),
        LargeComponent::seed_1(),
        LargeComponent::seed_2(),
        LargeComponent::seed_3(),
        LargeComponent::min_advances(),
        LargeComponent::max_advances(),
        LargeComponent::delay(),
        LargeComponent::gen3_lead(),
        LargeComponent::select(
            "pokemon",
            "Pokemon",
            vec![
                SelectOption::new_simple(StaticPokemon::Turtwig),
                SelectOption::new_simple(StaticPokemon::Chimchar),
                SelectOption::new_simple(StaticPokemon::Piplup),
                SelectOption::new_simple(StaticPokemon::Eevee),
                SelectOption::new_simple(StaticPokemon::Happiny),
                SelectOption::new_simple(StaticPokemon::Riolu),
                SelectOption::new_simple(StaticPokemon::Mew),
                SelectOption::new_simple(StaticPokemon::Jirachi),
                SelectOption::new_simple(StaticPokemon::Omanyte),
                SelectOption::new_simple(StaticPokemon::Kabuto),
                SelectOption::new_simple(StaticPokemon::Aerodactyl),
                SelectOption::new_simple(StaticPokemon::Lileep),
                SelectOption::new_simple(StaticPokemon::Anorith),
                SelectOption::new_simple(StaticPokemon::Cranidos),
                SelectOption::new_simple(StaticPokemon::Shieldon),
                SelectOption::new_simple(StaticPokemon::Drifloon),
                SelectOption::new_simple(StaticPokemon::Spiritomb),
                SelectOption::new_simple(StaticPokemon::Rotom),
                SelectOption::new_simple(StaticPokemon::Mesprit),
                SelectOption::new_simple(StaticPokemon::Cresselia),
                SelectOption::new_simple(StaticPokemon::Uxie),
                SelectOption::new_simple(StaticPokemon::Azelf),
                SelectOption::new_simple(StaticPokemon::Dialga),
                SelectOption::new_simple(StaticPokemon::Palkia),
                SelectOption::new_simple(StaticPokemon::Heatran),
                SelectOption::new_simple(StaticPokemon::Regigigas),
                SelectOption::new_simple(StaticPokemon::Giratina),
                SelectOption::new_simple(StaticPokemon::Articuno),
                SelectOption::new_simple(StaticPokemon::Zapdos),
                SelectOption::new_simple(StaticPokemon::Moltres),
                SelectOption::new_simple(StaticPokemon::Raikou),
                SelectOption::new_simple(StaticPokemon::Entei),
                SelectOption::new_simple(StaticPokemon::Suicune),
                SelectOption::new_simple(StaticPokemon::Regirock),
                SelectOption::new_simple(StaticPokemon::Regice),
                SelectOption::new_simple(StaticPokemon::Registeel),
                SelectOption::new_simple(StaticPokemon::Latias),
                SelectOption::new_simple(StaticPokemon::Latios),
                SelectOption::new_simple(StaticPokemon::Mewtwo),
                SelectOption::new_simple(StaticPokemon::Lugia),
                SelectOption::new_simple(StaticPokemon::HoOh),
                SelectOption::new_simple(StaticPokemon::Kyogre),
                SelectOption::new_simple(StaticPokemon::Groudon),
                SelectOption::new_simple(StaticPokemon::Rayquaza),
                SelectOption::new_simple(StaticPokemon::Darkrai),
                SelectOption::new_simple(StaticPokemon::Shaymin),
                SelectOption::new_simple(StaticPokemon::Arceus),
            ],
        ),
    ];

    let filer_components = vec![
        LargeComponent::shiny_type(),
        LargeComponent::nature_multiselect(),
        LargeComponent::gen3_ability(),
        LargeComponent::gender(),
        LargeComponent::min_ivs_label(),
        SmallComponent::min_hp_iv(),
        SmallComponent::min_atk_iv(),
        SmallComponent::min_def_iv(),
        SmallComponent::min_spa_iv(),
        SmallComponent::min_spd_iv(),
        SmallComponent::min_spe_iv(),
        LargeComponent::max_ivs_label(),
        SmallComponent::max_hp_iv(),
        SmallComponent::max_atk_iv(),
        SmallComponent::max_def_iv(),
        SmallComponent::max_spa_iv(),
        SmallComponent::max_spd_iv(),
        SmallComponent::max_spe_iv(),
    ];

    vec![
        FieldGroup::new("RNG Info", rng_info_components),
        FieldGroup::new("Filters", filer_components),
    ]
}

pub fn get_result_columns() -> Vec<String> {
    vec![
        "Advances", "Shiny", "Nature", "Ability", "Gender", "IVs", "PID", "EC",
    ]
    .into_iter()
    .map(String::from)
    .collect::<Vec<String>>()
}

#[derive(Deserialize, Serialize)]
pub struct Settings {
    seed_0: u32,
    seed_1: u32,
    seed_2: u32,
    seed_3: u32,
    min_advances: u32,
    max_advances: u32,
    delay: u32,
    gen3_lead: Gen3Lead,
    pokemon: StaticPokemon,
    shiny_type: ShinyTypeFilter,
    nature_multiselect: Vec<NatureFilter>,
    gender: GenderFilter,
    min_hp_iv: u32,
    min_atk_iv: u32,
    min_def_iv: u32,
    min_spa_iv: u32,
    min_spd_iv: u32,
    min_spe_iv: u32,
    max_hp_iv: u32,
    max_atk_iv: u32,
    max_def_iv: u32,
    max_spa_iv: u32,
    max_spd_iv: u32,
    max_spe_iv: u32,
}

impl From<Settings> for settings::Settings {
    fn from(value: Settings) -> Self {
        Self {
            rng_state: vec![value.seed_0, value.seed_1, value.seed_2, value.seed_3],
            lead_filter: value.gen3_lead.into(),
            is_roamer: value.pokemon.is_roamer(),
            shiny_filter: value.shiny_type.into(),
            min_advances: value.min_advances as usize,
            max_advances: value.max_advances as usize,
            delay: value.delay as usize,
            nature_filter: value
                .nature_multiselect
                .into_iter()
                .map(|nature| (DeprecatedNatureFilter::from(nature) as u16).into())
                .collect::<Vec<u32>>(),
            ability_filter: value.pokemon.ability(),
            gender_ratio: value.pokemon.gender_ratio(),
            gender_filter: value.gender.into(),
            set_ivs: value.pokemon.set_ivs(),
            min_ivs: vec![
                value.min_hp_iv,
                value.min_atk_iv,
                value.min_def_iv,
                value.min_spa_iv,
                value.min_spd_iv,
                value.min_spe_iv,
            ],
            max_ivs: vec![
                value.max_hp_iv,
                value.max_atk_iv,
                value.max_def_iv,
                value.max_spa_iv,
                value.max_spd_iv,
                value.max_spe_iv,
            ],
        }
    }
}

pub fn generate_stationary(settings: Settings) -> Vec<Vec<String>> {
    let results = generator::generate_stationary(settings.into());
    results
        .into_iter()
        .map(|result| {
            vec![
                result.advances.to_string(),
                result.shiny_value.to_string(),
                result.nature.to_string(),
                result.ability.to_string(),
                result.gender.to_string(),
                format_ivs(result.ivs),
                format!("{:x}", result.pid),
                format!("{:x}", result.ec),
            ]
        })
        .collect()
}
