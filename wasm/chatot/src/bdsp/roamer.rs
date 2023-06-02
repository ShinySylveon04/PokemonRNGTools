use crate::bdsp::stationary::form_settings::Settings;
use crate::rng::{Xoroshiro, Xorshift};
use chatot_forms::{
    self, Gen3Ability, Gen3Lead, Gender, MultiFilter, Nature, ShinyType, SingleFilter,
};

use crate::bdsp::stationary::generator::Pokemon;

type IVs = [u8; 6];
fn check_ivs(ivs: &IVs, min_ivs: &IVs, max_ivs: &IVs) -> bool {
    ivs.iter()
        .zip(min_ivs.iter())
        .zip(max_ivs.iter())
        .all(|((iv, min), max)| min <= iv && iv <= max)
}

pub fn generate_pokemon(mut seed_rng: Xorshift, settings: &Settings) -> Option<Pokemon> {
    let seed = seed_rng.next();
    let ec = seed;

    let mut rng = Xoroshiro::new_bdsp(seed);

    let shiny_rand = rng.next_bdsp();
    let pid = rng.next_bdsp();

    let shiny = ShinyType::calculate_shiny_gen8(pid, shiny_rand);
    if !ShinyType::passes_filter(&settings.shiny_type, shiny) {
        return None;
    }

    let mut ivs: IVs = [32, 32, 32, 32, 32, 32];
    if settings.pokemon.set_ivs() {
        for _ in 0..3 {
            let mut index: usize;
            loop {
                index = (rng.next_bdsp() % 6) as usize;
                if ivs[index] == 32 {
                    break;
                }
            }
            ivs[index] = 31;
        }
    }

    for i in ivs.iter_mut() {
        if *i == 32 {
            *i = (rng.next_bdsp() % 32) as u8
        };
    }

    if !check_ivs(&ivs, &settings.min_ivs(), &settings.max_ivs()) {
        return None;
    }

    let ability_rand = rng.next_bdsp() & 1;
    let ability_rand = (ability_rand - (ability_rand / 2) * 2) as u8;
    let ability: Gen3Ability = ability_rand.into();
    if !Gen3Ability::passes_filter(settings.pokemon.ability(), ability) {
        return None;
    }

    let gender_ratio = settings.pokemon.gender_ratio();
    let gender = match gender_ratio.get_set_gender() {
        Some(set_gender) => set_gender,
        None => {
            let gender_rand = rng.next_bdsp();
            let gender_num = ((gender_rand - (gender_rand / 253) * 253) + 1) as u8;
            gender_ratio.get_gender(gender_num)
        }
    };

    if !Gender::passes_filter(settings.gender, gender) {
        return None;
    }

    let nature = match settings.gen3_lead {
        // Can be anything - we're going to render "Synchronize" to the user
        Some(Gen3Lead::Synchronize) => Nature::Hardy,
        None => {
            let nature_rand = rng.next_bdsp() % 25;
            Nature::from(nature_rand as u8)
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
            pokemon: StaticPokemon::Mesprit,
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
                pid: 0x88C59540,
                ec: 2147485709,
                nature: Nature::Naive,
                ivs: [2, 31, 27, 30, 31, 31],
                ability: Gen3Ability::Ability0,
                gender: Gender::Genderless,
            },
            Pokemon {
                shiny: None,
                pid: 0xEB66944A,
                ec: 2147489823,
                nature: Nature::Bold,
                ivs: [11, 31, 31, 20, 31, 30],
                ability: Gen3Ability::Ability1,
                gender: Gender::Genderless,
            },
            Pokemon {
                shiny: None,
                pid: 0x2D994828,
                ec: 2147483652,
                nature: Nature::Bold,
                ivs: [14, 31, 31, 24, 31, 15],
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
}
