use super::generator;
use crate::utils::format_ivs;
use chatot_forms::{
    impl_display, FieldGroup, Gen3Ability, Gen3Lead, Gender, GenderRatio, LargeComponent, Nature,
    SelectOption, ShinyType, SmallComponent,
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
    pub fn is_roamer(&self) -> bool {
        match self {
            Self::Mesprit | Self::Cresselia => true,
            _ => false,
        }
    }

    pub fn ability(&self) -> Option<Gen3Ability> {
        match self {
            Self::Jirachi | Self::Mew => Some(Gen3Ability::Ability1),
            _ => None,
        }
    }

    pub fn set_ivs(&self) -> bool {
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
    pub fn gender_ratio(&self) -> GenderRatio {
        match self {
            Self::Turtwig => GenderRatio::Male875Female125,
            Self::Chimchar => GenderRatio::Male875Female125,
            Self::Piplup => GenderRatio::Male875Female125,
            Self::Eevee => GenderRatio::Male875Female125,
            Self::Happiny => GenderRatio::Female,
            Self::Riolu => GenderRatio::Male875Female125,
            Self::Mew => GenderRatio::Genderless,
            Self::Jirachi => GenderRatio::Genderless,
            Self::Omanyte => GenderRatio::Male875Female125,
            Self::Kabuto => GenderRatio::Male875Female125,
            Self::Aerodactyl => GenderRatio::Male875Female125,
            Self::Lileep => GenderRatio::Male875Female125,
            Self::Anorith => GenderRatio::Male875Female125,
            Self::Cranidos => GenderRatio::Male875Female125,
            Self::Shieldon => GenderRatio::Male875Female125,
            Self::Drifloon => GenderRatio::Male50Female50,
            Self::Spiritomb => GenderRatio::Male50Female50,
            Self::Rotom => GenderRatio::Genderless,
            Self::Mesprit => GenderRatio::Genderless,
            Self::Cresselia => GenderRatio::Genderless,
            Self::Uxie => GenderRatio::Genderless,
            Self::Azelf => GenderRatio::Genderless,
            Self::Dialga => GenderRatio::Genderless,
            Self::Palkia => GenderRatio::Genderless,
            Self::Heatran => GenderRatio::Genderless,
            Self::Regigigas => GenderRatio::Genderless,
            Self::Giratina => GenderRatio::Genderless,
            Self::Articuno => GenderRatio::Genderless,
            Self::Zapdos => GenderRatio::Genderless,
            Self::Moltres => GenderRatio::Genderless,
            Self::Raikou => GenderRatio::Genderless,
            Self::Entei => GenderRatio::Genderless,
            Self::Suicune => GenderRatio::Genderless,
            Self::Regirock => GenderRatio::Genderless,
            Self::Regice => GenderRatio::Genderless,
            Self::Registeel => GenderRatio::Genderless,
            Self::Latias => GenderRatio::Genderless,
            Self::Latios => GenderRatio::Genderless,
            Self::Mewtwo => GenderRatio::Genderless,
            Self::Lugia => GenderRatio::Genderless,
            Self::HoOh => GenderRatio::Genderless,
            Self::Kyogre => GenderRatio::Genderless,
            Self::Groudon => GenderRatio::Genderless,
            Self::Rayquaza => GenderRatio::Genderless,
            Self::Darkrai => GenderRatio::Genderless,
            Self::Shaymin => GenderRatio::Genderless,
            Self::Arceus => GenderRatio::Genderless,
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
                SelectOption::new(StaticPokemon::Turtwig),
                SelectOption::new(StaticPokemon::Chimchar),
                SelectOption::new(StaticPokemon::Piplup),
                SelectOption::new(StaticPokemon::Eevee),
                SelectOption::new(StaticPokemon::Happiny),
                SelectOption::new(StaticPokemon::Riolu),
                SelectOption::new(StaticPokemon::Mew),
                SelectOption::new(StaticPokemon::Jirachi),
                SelectOption::new(StaticPokemon::Omanyte),
                SelectOption::new(StaticPokemon::Kabuto),
                SelectOption::new(StaticPokemon::Aerodactyl),
                SelectOption::new(StaticPokemon::Lileep),
                SelectOption::new(StaticPokemon::Anorith),
                SelectOption::new(StaticPokemon::Cranidos),
                SelectOption::new(StaticPokemon::Shieldon),
                SelectOption::new(StaticPokemon::Drifloon),
                SelectOption::new(StaticPokemon::Spiritomb),
                SelectOption::new(StaticPokemon::Rotom),
                SelectOption::new(StaticPokemon::Mesprit),
                SelectOption::new(StaticPokemon::Cresselia),
                SelectOption::new(StaticPokemon::Uxie),
                SelectOption::new(StaticPokemon::Azelf),
                SelectOption::new(StaticPokemon::Dialga),
                SelectOption::new(StaticPokemon::Palkia),
                SelectOption::new(StaticPokemon::Heatran),
                SelectOption::new(StaticPokemon::Regigigas),
                SelectOption::new(StaticPokemon::Giratina),
                SelectOption::new(StaticPokemon::Articuno),
                SelectOption::new(StaticPokemon::Zapdos),
                SelectOption::new(StaticPokemon::Moltres),
                SelectOption::new(StaticPokemon::Raikou),
                SelectOption::new(StaticPokemon::Entei),
                SelectOption::new(StaticPokemon::Suicune),
                SelectOption::new(StaticPokemon::Regirock),
                SelectOption::new(StaticPokemon::Regice),
                SelectOption::new(StaticPokemon::Registeel),
                SelectOption::new(StaticPokemon::Latias),
                SelectOption::new(StaticPokemon::Latios),
                SelectOption::new(StaticPokemon::Mewtwo),
                SelectOption::new(StaticPokemon::Lugia),
                SelectOption::new(StaticPokemon::HoOh),
                SelectOption::new(StaticPokemon::Kyogre),
                SelectOption::new(StaticPokemon::Groudon),
                SelectOption::new(StaticPokemon::Rayquaza),
                SelectOption::new(StaticPokemon::Darkrai),
                SelectOption::new(StaticPokemon::Shaymin),
                SelectOption::new(StaticPokemon::Arceus),
            ],
        ),
    ];

    let filer_components = vec![
        LargeComponent::shiny_type(),
        LargeComponent::nature_multiselect(),
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
    pub seed_0: u32,
    pub seed_1: u32,
    pub seed_2: u32,
    pub seed_3: u32,
    pub min_advances: usize,
    pub max_advances: usize,
    pub delay: usize,
    pub gen3_lead: Option<Gen3Lead>,
    pub pokemon: StaticPokemon,
    pub shiny_type: Vec<ShinyType>,
    pub nature_multiselect: Vec<Nature>,
    pub gender: Option<Gender>,
    pub min_hp_iv: u8,
    pub min_atk_iv: u8,
    pub min_def_iv: u8,
    pub min_spa_iv: u8,
    pub min_spd_iv: u8,
    pub min_spe_iv: u8,
    pub max_hp_iv: u8,
    pub max_atk_iv: u8,
    pub max_def_iv: u8,
    pub max_spa_iv: u8,
    pub max_spd_iv: u8,
    pub max_spe_iv: u8,
}

impl Settings {
    pub fn min_ivs(&self) -> [u8; 6] {
        [
            self.min_hp_iv,
            self.min_atk_iv,
            self.min_def_iv,
            self.min_spa_iv,
            self.min_spd_iv,
            self.min_spe_iv,
        ]
    }

    pub fn max_ivs(&self) -> [u8; 6] {
        [
            self.max_hp_iv,
            self.max_atk_iv,
            self.max_def_iv,
            self.max_spa_iv,
            self.max_spd_iv,
            self.max_spe_iv,
        ]
    }
}

pub fn generate_stationary(settings: Settings) -> Vec<Vec<String>> {
    let gen3_lead = settings.gen3_lead;
    let results = generator::generate_stationary(settings);
    results
        .into_iter()
        .map(|result| {
            let stringified_nature = match gen3_lead {
                Some(Gen3Lead::Synchronize) => "Synchronize".to_string(),
                None => result.nature.to_string(),
            };
            vec![
                result.advances.to_string(),
                result
                    .shiny_value
                    .map(|shiny_type| shiny_type.to_string())
                    .unwrap_or("None".to_string()),
                stringified_nature,
                result.ability.to_string(),
                result.gender.to_string(),
                format_ivs(&result.ivs),
                format!("{:x}", result.pid),
                format!("{:x}", result.ec),
            ]
        })
        .collect()
}
