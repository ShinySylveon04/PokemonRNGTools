use super::form_settings::Settings;
use crate::{bdsp::roamer, rng::Xorshift};
use chatot_forms::{Gen3Ability, Gen3Lead, Gender, MultiFilter, Nature, ShinyType, SingleFilter};

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
}

pub fn generate_stationary(settings: Settings) -> Vec<Result> {
    let states: [u32; 4] = [
        settings.seed_0,
        settings.seed_1,
        settings.seed_2,
        settings.seed_3,
    ];

    let mut rng = Xorshift::from_state(states);
    rng.advance(settings.delay);
    let mut results: Vec<Result> = Vec::new();
    let values = settings.min_advances..=settings.max_advances;
    rng.advance(settings.min_advances);

    for value in values {
        let generate_result = match settings.pokemon.is_roamer() {
            true => roamer::generate_pokemon(rng, &settings),
            false => generate_pokemon(rng, &settings),
        };

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
            };
            results.push(result);
        }
        rng.next();
    }

    results.into_iter().collect()
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Pokemon {
    pub shiny: Option<ShinyType>,
    pub pid: u32,
    pub ec: u32,
    pub nature: Nature,
    pub ivs: IVs,
    pub ability: Gen3Ability,
    pub gender: Gender,
}

type IVs = [u8; 6];
fn check_ivs(ivs: &IVs, min_ivs: &IVs, max_ivs: &IVs) -> bool {
    ivs.iter()
        .zip(min_ivs.iter())
        .zip(max_ivs.iter())
        .all(|((iv, min), max)| min <= iv && iv <= max)
}

pub fn generate_pokemon(mut rng: Xorshift, settings: &Settings) -> Option<Pokemon> {
    let ec = rng.next();
    let shiny_rand = rng.next();
    let pid = rng.next();

    let shiny = ShinyType::calculate_shiny_gen8(pid, shiny_rand);
    if !ShinyType::passes_filter(&settings.shiny_type, shiny) {
        return None;
    }

    let mut ivs: [u8; 6] = [32, 32, 32, 32, 32, 32];

    if settings.pokemon.set_ivs() {
        for _ in 0..3 {
            let mut index;
            loop {
                let iv_rand = rng.next();
                index = iv_rand - (iv_rand / 6) * 6;
                if ivs[index as usize] == 32 {
                    break;
                }
            }
            ivs[index as usize] = 31;
        }
    }

    for i in ivs.iter_mut() {
        if *i == 32 {
            *i = rng.rand_max(32) as u8
        };
    }

    if !check_ivs(&ivs, &settings.min_ivs(), &settings.max_ivs()) {
        return None;
    }

    let ability_rand = rng.next();
    let ability_num = (ability_rand - (ability_rand / 2) * 2) as u8;
    let ability: Gen3Ability = ability_num.into();
    if !Gen3Ability::passes_filter(settings.pokemon.ability(), ability) {
        return None;
    }

    let gender_ratio = settings.pokemon.gender_ratio();
    let gender = match gender_ratio.get_set_gender() {
        Some(set_gender) => set_gender,
        None => {
            let gender_rand = rng.next();
            let gender_num = (gender_rand - (gender_rand / 253) * 253) + 1;
            gender_ratio.get_gender(gender_num as u8)
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
            let nature_num = (nature_rand - (nature_rand / 25) * 25) as u8;
            Nature::from(nature_num)
        }
    };

    if !Nature::passes_filter(&settings.nature_multiselect, Some(nature)) {
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
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::bdsp::stationary::form_settings::StaticPokemon;
    use std::vec;

    #[test]
    fn should_generate_pokemon() {
        let mut rng = Xorshift::from_state([1, 2, 3, 4]);
        let settings = Settings {
            nature_multiselect: vec![],
            shiny_type: vec![],
            seed_0: 1,
            seed_1: 2,
            seed_2: 3,
            seed_3: 4,
            delay: 0,
            min_advances: 0,
            max_advances: 10,
            gen3_lead: None,
            gender: None,
            pokemon: StaticPokemon::Spiritomb,
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
                pid: 2147483652,
                ec: 2147485709,
                nature: Nature::Lax,
                ivs: [0, 13, 26, 14, 30, 11],
                ability: Gen3Ability::Ability1,
                gender: Gender::Female,
            },
            Pokemon {
                shiny: None,
                pid: 2147491872,
                ec: 2147489823,
                nature: Nature::Hasty,
                ivs: [13, 26, 14, 30, 11, 25],
                ability: Gen3Ability::Ability1,
                gender: Gender::Female,
            },
            Pokemon {
                shiny: None,
                pid: 2151678029,
                ec: 2147483652,
                nature: Nature::Rash,
                ivs: [26, 14, 30, 11, 25, 15],
                ability: Gen3Ability::Ability0,
                gender: Gender::Female,
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
    fn should_generate_pokemon_with_ivs() {
        let mut rng = Xorshift::from_state([1, 2, 3, 4]);
        let settings = Settings {
            nature_multiselect: vec![],
            shiny_type: vec![],
            seed_0: 1,
            seed_1: 2,
            seed_2: 3,
            seed_3: 4,
            delay: 0,
            min_advances: 0,
            max_advances: 10,
            gen3_lead: None,
            gender: None,
            pokemon: StaticPokemon::Palkia,
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
                pid: 2147483652,
                ec: 2147485709,
                nature: Nature::Lax,
                ivs: [31, 30, 31, 11, 25, 31],
                ability: Gen3Ability::Ability1,
                gender: Gender::Genderless,
            },
            Pokemon {
                shiny: None,
                pid: 2147491872,
                ec: 2147489823,
                nature: Nature::Lax,
                ivs: [31, 30, 31, 11, 25, 31],
                ability: Gen3Ability::Ability1,
                gender: Gender::Genderless,
            },
            Pokemon {
                shiny: None,
                pid: 2151678029,
                ec: 2147483652,
                nature: Nature::Hasty,
                ivs: [31, 11, 31, 25, 31, 15],
                ability: Gen3Ability::Ability0,
                gender: Gender::Genderless,
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
            shiny_type: vec![ShinyType::Star, ShinyType::Square],
            seed_0: 1,
            seed_1: 2,
            seed_2: 3,
            seed_3: 4,
            delay: 0,
            min_advances: 0,
            max_advances: 10,
            gen3_lead: None,
            gender: None,
            pokemon: StaticPokemon::Cresselia,
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
            state0: 2326253939,
            state1: 1975907964,
            state2: 1100255917,
            state3: 457147861,
            advances: 4481,
            shiny_value: Some(ShinyType::Star),
            pid: 0x906f73f0,
            ec: 0x2fa7e388,
            nature: Nature::Hasty,
            ivs: [5, 29, 27, 28, 11, 27],
            ability: Gen3Ability::Ability1,
            gender: Gender::Female,
        };

        let results = generate_stationary(settings);
        for (advance, result) in results.into_iter().enumerate() {
            assert_eq!(result, expected_results, "Mismatch on advance {}", advance);
        }
    }
}
