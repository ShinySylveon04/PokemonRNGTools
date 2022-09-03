use super::settings::Settings;
use crate::enums;
use crate::rng::Lcrng;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Pokemon {
    pub shiny: enums::Shiny,
    pub pid: u32,
    pub ec: u32,
    pub nature: enums::Nature,
    pub ivs: Vec<u16>,
    pub ability: enums::Ability,
    pub gender: enums::Gender,
    pub encounter: u8,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Result {
    pub state0: u32,
    pub state1: u32,
    pub state2: u32,
    pub state3: u32,
    pub advances: usize,
    pub shiny_value: enums::Shiny,
    pub pid: u32,
    pub ec: u32,
    pub nature: enums::Nature,
    pub ivs: Vec<u32>,
    pub ability: enums::Ability,
    pub gender: enums::Gender,
    pub encounter: u8,
}

type IVs = Vec<u16>;
fn check_ivs(ivs: &IVs, min_ivs: &IVs, max_ivs: &IVs) -> bool {
    ivs.iter()
        .zip(min_ivs.iter())
        .zip(max_ivs.iter())
        .all(|((iv, min), max)| min <= iv && iv <= max)
}

fn generate_pokemon(mut rng: Lcrng, settings: &Settings) -> Option<Pokemon> {
    let settings = Settings {
        nature_filter: vec![25],
        encounter_filter: vec![12],
        rng_state: 0,
        delay: 0,
        min: 0,
        max: 10,
        gender_ratio: enums::GenderRatio::Male50Female50,
        lead_filter: enums::LeadFilter::None,
        shiny_filter: enums::ShinyFilter::None,
        ability_filter: enums::AbilityFilter::Any,
        gender_filter: enums::GenderFilter::Any,
        min_ivs: vec![0, 0, 0, 0, 0, 0],
        max_ivs: vec![31, 31, 31, 31, 31, 31],
    };

    rng.next_u32();
    rng.next_u32();
    rng.next_u32();

    let nature_rand = rng.next_u16() % 25;
    let nature = match enums::get_sync_nature(&settings.lead_filter) {
        Some(set_nature) => set_nature,
        None => enums::Nature::try_from(nature_rand as u16).unwrap_or(enums::Nature::Hardy),
    };

    let natures: Vec<enums::NatureFilter> = settings
        .nature_filter
        .iter()
        .map(|nature| {
            enums::NatureFilter::try_from(*nature as u16).unwrap_or(enums::NatureFilter::Hardy)
        })
        .collect();

    if !natures.iter().any(|nat| *nat == nature) {
        return None;
    }

    let mut pid: u32;
    loop {
        let pid_low = rng.next_u16() as u32;
        let pid_high = rng.next_u16() as u32;
        pid = (pid_high << 16) | pid_low;
        if pid % 25 == nature_rand as u32 {
            break;
        }
    }

    let ability_rand = pid & 1;

    let ability = enums::Ability::try_from(ability_rand as u32).unwrap_or(enums::Ability::Ability0);

    if settings.ability_filter != ability {
        return None;
    }

    let gender_rand = pid & 255;

    let gender = match enums::get_set_gender_from_ratio(&settings.gender_ratio) {
        Some(set_gender) => set_gender,
        None => enums::get_gender_from_ratio(&settings.gender_ratio, gender_rand.into()),
    };

    if settings.gender_filter != gender {
        return None;
    }

    let iv1 = rng.next_u16();
    let iv2 = rng.next_u16();

    let mut ivs = vec![32, 32, 32, 32, 32, 32];

    ivs[0] = iv1 & 0x1f;
    ivs[1] = (iv1 >> 5) & 0x1f;
    ivs[2] = (iv1 >> 10) & 0x1f;
    ivs[3] = (iv2 >> 5) & 0x1f;
    ivs[4] = (iv2 >> 10) & 0x1f;
    ivs[5] = iv2 & 0x1f;

    Some(Pokemon {
        shiny: enums::Shiny::None,
        pid: pid.into(),
        ec: 563921523,
        nature,
        ivs,
        ability,
        gender,
        encounter: 4,
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use std::vec;

    #[test]
    fn should_generate_pokemon() {
        let mut rng = Lcrng::from_state(0);
        let settings = Settings {
            nature_filter: vec![25],
            encounter_filter: vec![12],
            rng_state: 0,
            delay: 0,
            min: 0,
            max: 10,
            gender_ratio: enums::GenderRatio::Male50Female50,
            lead_filter: enums::LeadFilter::None,
            shiny_filter: enums::ShinyFilter::None,
            ability_filter: enums::AbilityFilter::Any,
            gender_filter: enums::GenderFilter::Any,
            min_ivs: vec![0, 0, 0, 0, 0, 0],
            max_ivs: vec![31, 31, 31, 31, 31, 31],
        };

        let expected_results = vec![
            Pokemon {
                shiny: enums::Shiny::None,
                pid: 0x60A1E414,
                ec: 563921523,
                nature: enums::Nature::Calm,
                ivs: vec![11, 25, 10, 25, 3, 24],
                ability: enums::Ability::Ability0,
                gender: enums::Gender::Female,
                encounter: 4,
            },
            Pokemon {
                shiny: enums::Shiny::None,
                pid: 0x639E3D69,
                ec: 563921523,
                nature: enums::Nature::Bashful,
                ivs: vec![9, 9, 7, 20, 26, 13],
                ability: enums::Ability::Ability1,
                gender: enums::Gender::Female,
                encounter: 4,
            },
        ];

        for (advance, expected_result) in expected_results.iter().enumerate() {
            let result = generate_pokemon(rng.clone(), &settings);

            assert_eq!(
                result.as_ref(),
                Some(expected_result),
                "Mismatch on advance {}",
                advance
            );
            rng.next();
        }
    }
}
