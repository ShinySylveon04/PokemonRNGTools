use crate::bdsp::stationary::settings::Settings;
use crate::enums;
use crate::rng::{Xoroshiro, Xorshift};
use std::convert::TryFrom;

use crate::bdsp::stationary::generator::Pokemon;

type IVs = Vec<u32>;
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

    let shiny = enums::Shiny::from_pid_shiny_rand(pid, shiny_rand);

    if settings.shiny_filter != shiny {
        return None;
    }

    let mut ivs = vec![32, 32, 32, 32, 32, 32];

    if settings.set_ivs {
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
            *i = rng.next_bdsp() % 32
        };
    }

    if !check_ivs(&ivs, &settings.min_ivs, &settings.max_ivs) {
        return None;
    }

    let ability_rand = rng.next_bdsp() & 1;

    let ability = enums::Ability::try_from(ability_rand - (ability_rand / 2) * 2)
        .unwrap_or(enums::Ability::Ability0);

    if settings.ability_filter != ability {
        return None;
    }

    let gender = match enums::get_set_gender_from_ratio(&settings.gender_ratio) {
        Some(set_gender) => set_gender,
        None => {
            let gender_rand = rng.next_bdsp();
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
            let nature_rand = rng.next_bdsp() % 25;
            enums::Nature::try_from(nature_rand).unwrap_or(enums::Nature::Hardy)
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
    use std::vec;

    #[test]
    fn should_generate_pokemon() {
        let mut rng = Xorshift::from_state([1, 2, 3, 4]);
        let settings = Settings {
            nature_filter: vec![25],
            rng_state: vec![1, 2, 3, 4],
            delay: 0,
            min: 0,
            max: 10,
            gender_ratio: enums::GenderRatio::Genderless,
            lead_filter: enums::LeadFilter::None,
            shiny_filter: enums::ShinyFilter::None,
            ability_filter: enums::AbilityFilter::Any,
            gender_filter: enums::GenderFilter::Any,
            min_ivs: vec![0, 0, 0, 0, 0, 0],
            max_ivs: vec![31, 31, 31, 31, 31, 31],
            set_ivs: true,
            is_roamer: true,
        };

        let expected_results = vec![
            Pokemon {
                shiny: enums::Shiny::None,
                pid: 0x88C59540,
                ec: 2147485709,
                nature: enums::Nature::Naive,
                ivs: vec![2, 31, 27, 30, 31, 31],
                ability: enums::Ability::Ability0,
                gender: enums::Gender::Genderless,
            },
            Pokemon {
                shiny: enums::Shiny::None,
                pid: 0xEB66944A,
                ec: 2147489823,
                nature: enums::Nature::Bold,
                ivs: vec![11, 31, 31, 20, 31, 30],
                ability: enums::Ability::Ability1,
                gender: enums::Gender::Genderless,
            },
            Pokemon {
                shiny: enums::Shiny::None,
                pid: 0x2D994828,
                ec: 2147483652,
                nature: enums::Nature::Bold,
                ivs: vec![14, 31, 31, 24, 31, 15],
                ability: enums::Ability::Ability0,
                gender: enums::Gender::Genderless,
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
