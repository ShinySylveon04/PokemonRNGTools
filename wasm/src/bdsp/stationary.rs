use crate::enums;
use crate::rng::Xorshift;
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
}

pub fn generate_pokemon(
    mut rng: Xorshift,
    gender_ratio: enums::GenderRatio,
    set_ivs: bool,
    lead: enums::LeadFilter,
) -> Pokemon {
    let mut shiny = enums::Shiny::None;

    let ec = rng.next();
    let shiny_rand = rng.next();
    let pid = rng.next();

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
            *i = rng.rand_max(32)
        };
    }

    let ability_rand = rng.next();
    let ability = ability_rand - (ability_rand / 2) * 2;

    let gender = match enums::get_set_gender_from_ratio(&gender_ratio) {
        Some(set_gender) => set_gender,
        None => {
            let gender_rand = rng.next();
            let gender_num = (gender_rand - (gender_rand / 253) * 253) + 1;
            enums::get_gender_from_ratio(&gender_ratio, gender_num)
        }
    };

    let nature;
    if lead != enums::LeadFilter::Synchronize {
        let nature_rand = rng.next();
        nature = nature_rand - (nature_rand / 25) * 25;
    } else {
        nature = 25;
    }

    Pokemon {
        shiny,
        pid,
        ec,
        nature: enums::Nature::try_from(nature).unwrap_or(enums::Nature::Hardy),
        ivs,
        ability: enums::Ability::try_from(ability).unwrap_or(enums::Ability::Ability0),
        gender,
    }
}
