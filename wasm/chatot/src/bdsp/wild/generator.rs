use super::form_settings::Settings;
use crate::{rng, rng::Xorshift};
use chatot_forms::{
    EncounterSlot, Gen3Ability, Gen3Lead, Gender, MultiFilter, Nature, ShinyType, SingleFilter,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Pokemon {
    pub shiny: Option<ShinyType>,
    pub pid: u32,
    pub ec: u32,
    pub nature: Nature,
    pub ivs: IVs,
    pub ability: Gen3Ability,
    pub gender: Gender,
    pub encounter: u8,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Result {
    pub state0: u32,
    pub state1: u32,
    pub state2: u32,
    pub state3: u32,
    pub advances: usize,
    pub shiny_value: Option<ShinyType>,
    pub pid: u32,
    pub ec: u32,
    pub nature: Nature,
    pub ivs: IVs,
    pub ability: Gen3Ability,
    pub gender: Gender,
    pub encounter: u8,
}

type IVs = [u8; 6];
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
    let shiny = ShinyType::calculate_shiny_gen8(pid, shiny_rand);

    if !ShinyType::passes_filter(&settings.shiny_type, shiny) {
        return None;
    }

    let mut ivs: IVs = [32, 32, 32, 32, 32, 32];
    for i in ivs.iter_mut() {
        *i = rng.rand_max(32) as u8;
    }

    if !check_ivs(&ivs, &settings.min_ivs(), &settings.max_ivs()) {
        return None;
    }

    let ability_rand = rng.next() as u8;
    let ability = Gen3Ability::from(ability_rand - (ability_rand / 2) * 2);
    if !Gen3Ability::passes_filter(settings.gen3_ability, ability) {
        return None;
    }

    let gender = match settings.gender_ratio.get_set_gender() {
        Some(set_gender) => set_gender,
        None => {
            let gender_rand = rng.next();
            let gender_num = (gender_rand - (gender_rand / 253) * 253) + 1;
            settings.gender_ratio.get_gender(gender_num as u8)
        }
    };

    if !Gender::passes_filter(settings.gender, gender) {
        return None;
    }

    let nature = match settings.gen3_lead {
        // Can be anything - we're going to render "Synchronize" to the user
        Some(Gen3Lead::Synchronize) => Nature::Hardy,
        None => {
            let nature_rand = rng.next();
            let nature_rand = (nature_rand - (nature_rand / 25) * 25) as u8;
            Nature::from(nature_rand)
        }
    };

    if !Nature::passes_filter(&settings.nature_multiselect, Some(nature)) {
        return None;
    }

    let encounter_slots: [u8; 12] = [20, 40, 50, 60, 70, 80, 85, 90, 94, 98, 99, 100];
    let encounter = encounter_slots
        .iter()
        .position(|enc| encounter_rand < *enc)
        .unwrap_or(0) as u8;
    let encounter_slot = EncounterSlot::from(encounter);

    if !EncounterSlot::passes_filter(settings.encounter_slot, encounter_slot) {
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
        settings.seed_0,
        settings.seed_1,
        settings.seed_2,
        settings.seed_3,
    ];
    let mut rng = rng::Xorshift::from_state(states);
    rng.advance(settings.delay);
    let mut results: Vec<Result> = Vec::new();
    let values = settings.min_advances..=settings.max_advances;
    rng.advance(settings.min_advances);
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
    use chatot_forms::GenderRatio;
    use std::vec;

    #[test]
    fn should_generate_pokemon() {
        let mut rng = Xorshift::from_state([1, 2, 3, 4]);
        let settings = Settings {
            nature_multiselect: vec![],
            encounter_slot: None,
            seed_0: 1,
            seed_1: 2,
            seed_2: 3,
            seed_3: 4,
            delay: 0,
            min_advances: 0,
            max_advances: 10,
            gender_ratio: GenderRatio::Male50Female50,
            gen3_lead: None,
            shiny_type: vec![],
            gen3_ability: None,
            gender: None,
            min_hp_iv: 0,
            min_atk_iv: 0,
            min_def_iv: 0,
            min_spa_iv: 0,
            min_spd_iv: 0,
            min_spe_iv: 0,
            max_hp_iv: 31,
            max_atk_iv: 31,
            max_def_iv: 31,
            max_spa_iv: 31,
            max_spd_iv: 31,
            max_spe_iv: 31,
        };

        let expected_results = vec![
            Pokemon {
                shiny: None,
                pid: 0x6a88a6e4,
                ec: 0x219cc273,
                nature: Nature::Bashful,
                ivs: [12, 20, 1, 12, 5, 20],
                ability: Gen3Ability::Ability0,
                gender: Gender::Female,
                encounter: 4,
            },
            Pokemon {
                shiny: None,
                pid: 0xa28ce86c,
                ec: 0x032ebce9,
                nature: Nature::Hardy,
                ivs: [20, 1, 12, 5, 20, 16],
                ability: Gen3Ability::Ability0,
                gender: Gender::Female,
                encounter: 5,
            },
            Pokemon {
                shiny: None,
                pid: 0xe5443914,
                ec: 0x6a88a6e4,
                nature: Nature::Quiet,
                ivs: [1, 12, 5, 20, 16, 20],
                ability: Gen3Ability::Ability1,
                gender: Gender::Female,
                encounter: 0,
            },
            Pokemon {
                shiny: None,
                pid: 0x137b08a1,
                ec: 0xa28ce86c,
                nature: Nature::Jolly,
                ivs: [12, 5, 20, 16, 20, 19],
                ability: Gen3Ability::Ability0,
                gender: Gender::Male,
                encounter: 1,
            },
            Pokemon {
                shiny: None,
                pid: 0xbc6b23ac,
                ec: 0xe5443914,
                nature: Nature::Naive,
                ivs: [5, 20, 16, 20, 19, 6],
                ability: Gen3Ability::Ability0,
                gender: Gender::Female,
                encounter: 6,
            },
            Pokemon {
                shiny: None,
                pid: 0xf9e163c5,
                ec: 0x137b08a1,
                nature: Nature::Jolly,
                ivs: [20, 16, 20, 19, 6, 28],
                ability: Gen3Ability::Ability0,
                gender: Gender::Female,
                encounter: 7,
            },
            Pokemon {
                shiny: None,
                pid: 0xbd297974,
                ec: 0xbc6b23ac,
                nature: Nature::Naughty,
                ivs: [16, 20, 19, 6, 28, 0],
                ability: Gen3Ability::Ability0,
                gender: Gender::Male,
                encounter: 3,
            },
            Pokemon {
                shiny: None,
                pid: 0xf65c4070,
                ec: 0xf9e163c5,
                nature: Nature::Naughty,
                ivs: [20, 19, 6, 28, 0, 10],
                ability: Gen3Ability::Ability1,
                gender: Gender::Male,
                encounter: 1,
            },
            Pokemon {
                shiny: None,
                pid: 0x934f7b54,
                ec: 0xbd297974,
                nature: Nature::Bold,
                ivs: [19, 6, 28, 0, 10, 17],
                ability: Gen3Ability::Ability1,
                gender: Gender::Female,
                encounter: 3,
            },
            Pokemon {
                shiny: None,
                pid: 0xe1c2cdb3,
                ec: 0xf65c4070,
                nature: Nature::Adamant,
                ivs: [6, 28, 0, 10, 17, 25],
                ability: Gen3Ability::Ability0,
                gender: Gender::Female,
                encounter: 1,
            },
            Pokemon {
                shiny: None,
                pid: 0x9756fa26,
                ec: 0x934f7b54,
                nature: Nature::Sassy,
                ivs: [28, 0, 10, 17, 25, 10],
                ability: Gen3Ability::Ability0,
                gender: Gender::Female,
                encounter: 1,
            },
            Pokemon {
                shiny: None,
                pid: 0x039d677c,
                ec: 0xe1c2cdb3,
                nature: Nature::Jolly,
                ivs: [0, 10, 17, 25, 10, 12],
                ability: Gen3Ability::Ability1,
                gender: Gender::Female,
                encounter: 1,
            },
            Pokemon {
                shiny: None,
                pid: 0x6b603980,
                ec: 0x9756fa26,
                nature: Nature::Lax,
                ivs: [10, 17, 25, 10, 12, 21],
                ability: Gen3Ability::Ability1,
                gender: Gender::Male,
                encounter: 0,
            },
            Pokemon {
                shiny: None,
                pid: 0x1cb8de0a,
                ec: 0x039d677c,
                nature: Nature::Quirky,
                ivs: [17, 25, 10, 12, 21, 9],
                ability: Gen3Ability::Ability0,
                gender: Gender::Male,
                encounter: 2,
            },
            Pokemon {
                shiny: None,
                pid: 0xbc9f8071,
                ec: 0x6b603980,
                nature: Nature::Bashful,
                ivs: [25, 10, 12, 21, 9, 22],
                ability: Gen3Ability::Ability1,
                gender: Gender::Female,
                encounter: 0,
            },
            Pokemon {
                shiny: None,
                pid: 0xd451a619,
                ec: 0x1cb8de0a,
                nature: Nature::Careful,
                ivs: [10, 12, 21, 9, 22, 21],
                ability: Gen3Ability::Ability0,
                gender: Gender::Female,
                encounter: 2,
            },
            Pokemon {
                shiny: None,
                pid: 0x3e17392a,
                ec: 0xbc9f8071,
                nature: Nature::Quiet,
                ivs: [12, 21, 9, 22, 21, 30],
                ability: Gen3Ability::Ability0,
                gender: Gender::Male,
                encounter: 2,
            },
            Pokemon {
                shiny: None,
                pid: 0x6405e86c,
                ec: 0xd451a619,
                nature: Nature::Calm,
                ivs: [21, 9, 22, 21, 30, 20],
                ability: Gen3Ability::Ability0,
                gender: Gender::Male,
                encounter: 8,
            },
            Pokemon {
                shiny: None,
                pid: 0xa4596095,
                ec: 0x3e17392a,
                nature: Nature::Quiet,
                ivs: [9, 22, 21, 30, 20, 24],
                ability: Gen3Ability::Ability1,
                gender: Gender::Female,
                encounter: 4,
            },
            Pokemon {
                shiny: None,
                pid: 0x7de16b69,
                ec: 0x6405e86c,
                nature: Nature::Timid,
                ivs: [22, 21, 30, 20, 24, 29],
                ability: Gen3Ability::Ability1,
                gender: Gender::Male,
                encounter: 5,
            },
            Pokemon {
                shiny: None,
                pid: 0x7a38c396,
                ec: 0xa4596095,
                nature: Nature::Gentle,
                ivs: [21, 30, 20, 24, 29, 15],
                ability: Gen3Ability::Ability0,
                gender: Gender::Female,
                encounter: 3,
            },
            Pokemon {
                shiny: None,
                pid: 0xb1b51235,
                ec: 0x7de16b69,
                nature: Nature::Bashful,
                ivs: [30, 20, 24, 29, 15, 24],
                ability: Gen3Ability::Ability1,
                gender: Gender::Male,
                encounter: 1,
            },
            Pokemon {
                shiny: None,
                pid: 0x5e07815e,
                ec: 0x7a38c396,
                nature: Nature::Hasty,
                ivs: [20, 24, 29, 15, 24, 21],
                ability: Gen3Ability::Ability1,
                gender: Gender::Female,
                encounter: 0,
            },
            Pokemon {
                shiny: None,
                pid: 0xa84b03d4,
                ec: 0xb1b51235,
                nature: Nature::Mild,
                ivs: [24, 29, 15, 24, 21, 3],
                ability: Gen3Ability::Ability1,
                gender: Gender::Female,
                encounter: 6,
            },
            Pokemon {
                shiny: None,
                pid: 0x94535138,
                ec: 0x5e07815e,
                nature: Nature::Adamant,
                ivs: [29, 15, 24, 21, 3, 21],
                ability: Gen3Ability::Ability1,
                gender: Gender::Male,
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
            nature_multiselect: vec![],
            encounter_slot: None,
            seed_0: 1,
            seed_1: 2,
            seed_2: 3,
            seed_3: 4,
            delay: 0,
            min_advances: 0,
            max_advances: 10000,
            gender_ratio: GenderRatio::Male50Female50,
            gen3_lead: None,
            shiny_type: vec![ShinyType::Star, ShinyType::Square],
            gen3_ability: None,
            gender: None,
            min_hp_iv: 0,
            min_atk_iv: 0,
            min_def_iv: 0,
            min_spa_iv: 0,
            min_spd_iv: 0,
            min_spe_iv: 0,
            max_hp_iv: 31,
            max_atk_iv: 31,
            max_def_iv: 31,
            max_spa_iv: 31,
            max_spd_iv: 31,
            max_spe_iv: 31,
        };

        let expected_results = Result {
            state0: 3298367444,
            state1: 2621101892,
            state2: 3417870565,
            state3: 4276622010,
            advances: 4396,
            shiny_value: Some(ShinyType::Star),
            pid: 0x906f73f0,
            ec: 0x2fa7e388,
            nature: Nature::Hasty,
            ivs: [5, 29, 27, 28, 11, 27],
            ability: Gen3Ability::Ability1,
            gender: Gender::Female,
            encounter: 7,
        };

        let results = generate_wild(settings);
        for (advance, result) in results.into_iter().enumerate() {
            assert_eq!(result, expected_results, "Mismatch on advance {}", advance);
        }
    }
}
