use crate::enums;
use crate::rng::{Xoroshiro, Xorshift};
use std::convert::TryFrom;

pub fn generate_pokemon(
    mut seed_rng: Xorshift,
    gender_ratio: enums::GenderRatio,
    set_ivs: bool,
) -> super::stationary::Pokemon {
    let mut shiny = enums::Shiny::None;

    let seed = seed_rng.next();
    let ec = seed;

    let mut rng = Xoroshiro::new_bdsp(seed);

    let shiny_rand = rng.next_bdsp();
    let pid = rng.next_bdsp();

    let psv = shiny_rand & 0xFFFF ^ shiny_rand >> 0x10;
    let tsv = pid >> 0x10 ^ pid & 0xFFFF;
    if (psv ^ tsv) < 0x10 {
        if (psv ^ tsv) == 0 {
            shiny = enums::Shiny::Square
        } else {
            shiny = enums::Shiny::Star
        }
    }

    let mut ivs = vec![32, 32, 32, 32, 32, 32];

    if set_ivs {
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

    let ability = rng.next_bdsp() & 1;

    let gender = match enums::get_set_gender_from_ratio(&gender_ratio) {
        Some(set_gender) => set_gender,
        None => {
            let gender_rand = rng.next_bdsp();
            let gender_num = (gender_rand - (gender_rand / 253) * 253) + 1;
            enums::get_gender_from_ratio(&gender_ratio, gender_num)
        }
    };

    let nature = rng.next_bdsp() % 25;

    super::stationary::Pokemon {
        shiny,
        pid,
        ec,
        nature: enums::Nature::try_from(nature as u16).unwrap_or(enums::Nature::Hardy),
        ivs,
        ability: enums::Ability::try_from(ability).unwrap_or(enums::Ability::Ability0),
        gender,
    }
}
