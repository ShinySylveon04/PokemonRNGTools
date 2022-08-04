use super::settings::Settings;
use crate::rng::Xorshift;
use crate::{enums, rng};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Pokemon {
    pub shiny: enums::Shiny,
    pub pid: u32,
    pub ec: u32,
    pub nature: enums::Nature,
    pub ivs: Vec<u32>,
    pub ability: enums::Ability,
    pub gender: enums::Gender,
    pub encounter: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
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

type IVs = Vec<u32>;
fn check_ivs(ivs: &IVs, min_ivs: &IVs, max_ivs: &IVs) -> bool {
    ivs.iter()
        .zip(min_ivs.iter())
        .zip(max_ivs.iter())
        .all(|((iv, min), max)| min <= iv && iv <= max)
}

fn generate_pokemon(mut rng: Xorshift, settings: &Settings) -> Option<Pokemon> {
    let encounter_rand = rng.rand_range(0, 100) as u8;
    rng.advance(84);
    let ec = rng.next();
    let shiny_rand = rng.next();
    let pid = rng.next();
    let shiny = enums::Shiny::from_pid_shiny_rand(pid, shiny_rand);

    if settings.shiny_filter != shiny {
        return None;
    }

    let mut ivs = vec![32, 32, 32, 32, 32, 32];
    for i in ivs.iter_mut() {
        *i = rng.rand_max(32);
    }

    if !check_ivs(&ivs, &settings.min_ivs, &settings.max_ivs) {
        return None;
    }

    let ability_rand = rng.next();
    let ability = enums::Ability::try_from(ability_rand - (ability_rand / 2) * 2)
        .unwrap_or(enums::Ability::Ability0);

    if settings.ability_filter != ability {
        return None;
    }

    let gender = match enums::get_set_gender_from_ratio(&settings.gender_ratio) {
        Some(set_gender) => set_gender,
        None => {
            let gender_rand = rng.next();
            let gender_num = (gender_rand - (gender_rand / 253) * 253) + 1;
            enums::get_gender_from_ratio(&settings.gender_ratio, gender_num)
        }
    };

    if settings.gender_filter != gender {
        return None;
    }

    let nature = match enums::get_sync_nature(&settings.lead_filter) {
        Some(set_nature) => set_nature,
        None => {
            let nature_rand = rng.next();
            enums::Nature::try_from(nature_rand - (nature_rand / 25) * 25)
                .unwrap_or(enums::Nature::Hardy)
        }
    };

    let natures: Vec<enums::NatureFilter> = settings
        .nature_filter
        .iter()
        .map(|nature| enums::NatureFilter::try_from(*nature).unwrap_or(enums::NatureFilter::Hardy))
        .collect();

    if !natures.iter().any(|nat| *nat == nature) {
        return None;
    }

    let encounter_slots: [u8; 12] = [20, 40, 50, 60, 70, 80, 85, 90, 94, 98, 99, 100];

    let encounter = encounter_slots
        .iter()
        .position(|enc| encounter_rand < *enc)
        .unwrap_or(0) as u8;

    let encounters: Vec<enums::EncounterSlotFilter> = settings
        .encounter_filter
        .iter()
        .map(|encounter| {
            enums::EncounterSlotFilter::try_from(*encounter)
                .unwrap_or(enums::EncounterSlotFilter::Slot0)
        })
        .collect();

    if !encounters.iter().any(|slot| *slot == encounter) {
        return None;
    }

    Some(Pokemon {
        shiny,
        pid,
        ec,
        nature,
        ivs,
        ability,
        gender,
        encounter,
    })
}

pub fn generate_wild(settings: Settings) -> Vec<Result> {
    let states: [u32; 4] = [
        settings.rng_state[0],
        settings.rng_state[1],
        settings.rng_state[2],
        settings.rng_state[3],
    ];
    let mut rng = rng::Xorshift::from_state(states);
    rng.advance(settings.delay);
    let mut results: Vec<Result> = Vec::new();
    let values = settings.min..=settings.max;
    rng.advance(settings.min);
    for value in values {
        let generate_result = generate_pokemon(rng, &settings);
        if let Some(pokemon) = generate_result {
            let rng_state = rng.get_state();
            let result = Result {
                state0: rng_state[0],
                state1: rng_state[1],
                state2: rng_state[2],
                state3: rng_state[3],
                advances: value,
                pid: pokemon.pid,
                shiny_value: pokemon.shiny,
                ec: pokemon.ec,
                nature: pokemon.nature,
                ivs: pokemon.ivs,
                ability: pokemon.ability,
                gender: pokemon.gender,
                encounter: pokemon.encounter,
            };
            results.push(result);
        }

        rng.next();
    }

    results.into_iter().collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::vec;

    #[test]
    fn should_generate_pokemon() {
        let mut rng = Xorshift::from_state([1, 2, 3, 4]);
        let settings = Settings {
            nature_filter: vec![25],
            encounter_filter: vec![12],
            rng_state: vec![1, 2, 3, 4],
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
                pid: 0x6a88a6e4,
                ec: 0x219cc273,
                nature: enums::Nature::Bashful,
                ivs: vec![12, 20, 1, 12, 5, 20],
                ability: enums::Ability::Ability0,
                gender: enums::Gender::Female,
                encounter: 4,
            },
            Pokemon {
                shiny: enums::Shiny::None,
                pid: 0xa28ce86c,
                ec: 0x032ebce9,
                nature: enums::Nature::Hardy,
                ivs: vec![20, 1, 12, 5, 20, 16],
                ability: enums::Ability::Ability0,
                gender: enums::Gender::Female,
                encounter: 5,
            },
            Pokemon {
                shiny: enums::Shiny::None,
                pid: 0xe5443914,
                ec: 0x6a88a6e4,
                nature: enums::Nature::Quiet,
                ivs: vec![1, 12, 5, 20, 16, 20],
                ability: enums::Ability::Ability1,
                gender: enums::Gender::Female,
                encounter: 0,
            },
            Pokemon {
                shiny: enums::Shiny::None,
                pid: 0x137b08a1,
                ec: 0xa28ce86c,
                nature: enums::Nature::Jolly,
                ivs: vec![12, 5, 20, 16, 20, 19],
                ability: enums::Ability::Ability0,
                gender: enums::Gender::Male,
                encounter: 1,
            },
            Pokemon {
                shiny: enums::Shiny::None,
                pid: 0xbc6b23ac,
                ec: 0xe5443914,
                nature: enums::Nature::Naive,
                ivs: vec![5, 20, 16, 20, 19, 6],
                ability: enums::Ability::Ability0,
                gender: enums::Gender::Female,
                encounter: 6,
            },
            Pokemon {
                shiny: enums::Shiny::None,
                pid: 0xf9e163c5,
                ec: 0x137b08a1,
                nature: enums::Nature::Jolly,
                ivs: vec![20, 16, 20, 19, 6, 28],
                ability: enums::Ability::Ability0,
                gender: enums::Gender::Female,
                encounter: 7,
            },
            Pokemon {
                shiny: enums::Shiny::None,
                pid: 0xbd297974,
                ec: 0xbc6b23ac,
                nature: enums::Nature::Naughty,
                ivs: vec![16, 20, 19, 6, 28, 0],
                ability: enums::Ability::Ability0,
                gender: enums::Gender::Male,
                encounter: 3,
            },
            Pokemon {
                shiny: enums::Shiny::None,
                pid: 0xf65c4070,
                ec: 0xf9e163c5,
                nature: enums::Nature::Naughty,
                ivs: vec![20, 19, 6, 28, 0, 10],
                ability: enums::Ability::Ability1,
                gender: enums::Gender::Male,
                encounter: 1,
            },
            Pokemon {
                shiny: enums::Shiny::None,
                pid: 0x934f7b54,
                ec: 0xbd297974,
                nature: enums::Nature::Bold,
                ivs: vec![19, 6, 28, 0, 10, 17],
                ability: enums::Ability::Ability1,
                gender: enums::Gender::Female,
                encounter: 3,
            },
            Pokemon {
                shiny: enums::Shiny::None,
                pid: 0xe1c2cdb3,
                ec: 0xf65c4070,
                nature: enums::Nature::Adamant,
                ivs: vec![6, 28, 0, 10, 17, 25],
                ability: enums::Ability::Ability0,
                gender: enums::Gender::Female,
                encounter: 1,
            },
            Pokemon {
                shiny: enums::Shiny::None,
                pid: 0x9756fa26,
                ec: 0x934f7b54,
                nature: enums::Nature::Sassy,
                ivs: vec![28, 0, 10, 17, 25, 10],
                ability: enums::Ability::Ability0,
                gender: enums::Gender::Female,
                encounter: 1,
            },
            Pokemon {
                shiny: enums::Shiny::None,
                pid: 0x039d677c,
                ec: 0xe1c2cdb3,
                nature: enums::Nature::Jolly,
                ivs: vec![0, 10, 17, 25, 10, 12],
                ability: enums::Ability::Ability1,
                gender: enums::Gender::Female,
                encounter: 1,
            },
            Pokemon {
                shiny: enums::Shiny::None,
                pid: 0x6b603980,
                ec: 0x9756fa26,
                nature: enums::Nature::Lax,
                ivs: vec![10, 17, 25, 10, 12, 21],
                ability: enums::Ability::Ability1,
                gender: enums::Gender::Male,
                encounter: 0,
            },
            Pokemon {
                shiny: enums::Shiny::None,
                pid: 0x1cb8de0a,
                ec: 0x039d677c,
                nature: enums::Nature::Quirky,
                ivs: vec![17, 25, 10, 12, 21, 9],
                ability: enums::Ability::Ability0,
                gender: enums::Gender::Male,
                encounter: 2,
            },
            Pokemon {
                shiny: enums::Shiny::None,
                pid: 0xbc9f8071,
                ec: 0x6b603980,
                nature: enums::Nature::Bashful,
                ivs: vec![25, 10, 12, 21, 9, 22],
                ability: enums::Ability::Ability1,
                gender: enums::Gender::Female,
                encounter: 0,
            },
            Pokemon {
                shiny: enums::Shiny::None,
                pid: 0xd451a619,
                ec: 0x1cb8de0a,
                nature: enums::Nature::Careful,
                ivs: vec![10, 12, 21, 9, 22, 21],
                ability: enums::Ability::Ability0,
                gender: enums::Gender::Female,
                encounter: 2,
            },
            Pokemon {
                shiny: enums::Shiny::None,
                pid: 0x3e17392a,
                ec: 0xbc9f8071,
                nature: enums::Nature::Quiet,
                ivs: vec![12, 21, 9, 22, 21, 30],
                ability: enums::Ability::Ability0,
                gender: enums::Gender::Male,
                encounter: 2,
            },
            Pokemon {
                shiny: enums::Shiny::None,
                pid: 0x6405e86c,
                ec: 0xd451a619,
                nature: enums::Nature::Calm,
                ivs: vec![21, 9, 22, 21, 30, 20],
                ability: enums::Ability::Ability0,
                gender: enums::Gender::Male,
                encounter: 8,
            },
            Pokemon {
                shiny: enums::Shiny::None,
                pid: 0xa4596095,
                ec: 0x3e17392a,
                nature: enums::Nature::Quiet,
                ivs: vec![9, 22, 21, 30, 20, 24],
                ability: enums::Ability::Ability1,
                gender: enums::Gender::Female,
                encounter: 4,
            },
            Pokemon {
                shiny: enums::Shiny::None,
                pid: 0x7de16b69,
                ec: 0x6405e86c,
                nature: enums::Nature::Timid,
                ivs: vec![22, 21, 30, 20, 24, 29],
                ability: enums::Ability::Ability1,
                gender: enums::Gender::Male,
                encounter: 5,
            },
            Pokemon {
                shiny: enums::Shiny::None,
                pid: 0x7a38c396,
                ec: 0xa4596095,
                nature: enums::Nature::Gentle,
                ivs: vec![21, 30, 20, 24, 29, 15],
                ability: enums::Ability::Ability0,
                gender: enums::Gender::Female,
                encounter: 3,
            },
            Pokemon {
                shiny: enums::Shiny::None,
                pid: 0xb1b51235,
                ec: 0x7de16b69,
                nature: enums::Nature::Bashful,
                ivs: vec![30, 20, 24, 29, 15, 24],
                ability: enums::Ability::Ability1,
                gender: enums::Gender::Male,
                encounter: 1,
            },
            Pokemon {
                shiny: enums::Shiny::None,
                pid: 0x5e07815e,
                ec: 0x7a38c396,
                nature: enums::Nature::Hasty,
                ivs: vec![20, 24, 29, 15, 24, 21],
                ability: enums::Ability::Ability1,
                gender: enums::Gender::Female,
                encounter: 0,
            },
            Pokemon {
                shiny: enums::Shiny::None,
                pid: 0xa84b03d4,
                ec: 0xb1b51235,
                nature: enums::Nature::Mild,
                ivs: vec![24, 29, 15, 24, 21, 3],
                ability: enums::Ability::Ability1,
                gender: enums::Gender::Female,
                encounter: 6,
            },
            Pokemon {
                shiny: enums::Shiny::None,
                pid: 0x94535138,
                ec: 0x5e07815e,
                nature: enums::Nature::Adamant,
                ivs: vec![29, 15, 24, 21, 3, 21],
                ability: enums::Ability::Ability1,
                gender: enums::Gender::Male,
                encounter: 0,
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

    #[test]
    fn should_filter_pokemon() {
        let settings = Settings {
            nature_filter: vec![25],
            encounter_filter: vec![12],
            rng_state: vec![1, 2, 3, 4],
            delay: 0,
            min: 0,
            max: 10000,
            gender_ratio: enums::GenderRatio::Male50Female50,
            lead_filter: enums::LeadFilter::None,
            shiny_filter: enums::ShinyFilter::Both,
            ability_filter: enums::AbilityFilter::Any,
            gender_filter: enums::GenderFilter::Any,
            min_ivs: vec![0, 0, 0, 0, 0, 0],
            max_ivs: vec![31, 31, 31, 31, 31, 31],
        };

        let expected_results = Result {
            state0: 3298367444,
            state1: 2621101892,
            state2: 3417870565,
            state3: 4276622010,
            advances: 4396,
            shiny_value: enums::Shiny::Star,
            pid: 0x906f73f0,
            ec: 0x2fa7e388,
            nature: enums::Nature::Hasty,
            ivs: vec![5, 29, 27, 28, 11, 27],
            ability: enums::Ability::Ability1,
            gender: enums::Gender::Female,
            encounter: 7,
        };

        let results = generate_wild(settings);
        for (advance, result) in results.into_iter().enumerate() {
            assert_eq!(result, expected_results, "Mismatch on advance {}", advance);
        }
    }
}
