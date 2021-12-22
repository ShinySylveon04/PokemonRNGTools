use super::enums;
use super::{Pokemonbdsp, PokemonbdspStationary, TIDbdsp, Xoroshiro, Xorshift};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use wasm_bindgen::prelude::*;

pub fn generate_bdsp_pokemon(
    mut rng: Xorshift,
    gender_ratio: enums::GenderRatioEnum,
    lead: enums::LeadFilterEnum,
) -> Pokemonbdsp {
    let encounter_rand = rng.rand_range(0, 100) as u8;
    rng.advance(84);
    let mut shiny = enums::ShinyEnum::None;
    let ec = rng.next();
    let shiny_rand = rng.next();
    let pid = rng.next();

    let psv = shiny_rand & 0xFFFF ^ shiny_rand >> 0x10;
    let tsv = pid >> 0x10 ^ pid & 0xFFFF;
    if (psv ^ tsv) < 0x10 {
        if (psv ^ tsv) == 0 {
            shiny = enums::ShinyEnum::Square
        } else {
            shiny = enums::ShinyEnum::Star
        }
    }

    let mut ivs = vec![32, 32, 32, 32, 32, 32];
    for i in ivs.iter_mut() {
        *i = rng.rand_max(32);
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
    if lead != enums::LeadFilterEnum::Synchronize {
        let nature_rand = rng.next();
        nature = nature_rand - (nature_rand / 25) * 25;
    } else {
        nature = 25;
    }

    let encounter_slots: [u8; 12] = [20, 40, 50, 60, 70, 80, 85, 90, 94, 98, 99, 100];

    let encounter = encounter_slots
        .iter()
        .position(|enc| encounter_rand < *enc)
        .unwrap_or(0) as u8;

    Pokemonbdsp {
        shiny,
        pid,
        ec,
        nature: enums::NatureEnum::try_from(nature).unwrap_or(enums::NatureEnum::Hardy),
        ivs,
        ability: enums::AbilityEnum::try_from(ability).unwrap_or(enums::AbilityEnum::Ability0),
        gender,
        encounter,
    }
}

pub fn generate_bdsp_pokemon_stationary(
    mut rng: Xorshift,
    gender_ratio: enums::GenderRatioEnum,
    set_ivs: bool,
    lead: enums::LeadFilterEnum,
) -> PokemonbdspStationary {
    let mut shiny = enums::ShinyEnum::None;

    let ec = rng.next();
    let shiny_rand = rng.next();
    let pid = rng.next();

    let psv = shiny_rand & 0xFFFF ^ shiny_rand >> 0x10;
    let tsv = pid >> 0x10 ^ pid & 0xFFFF;
    if (psv ^ tsv) < 0x10 {
        if (psv ^ tsv) == 0 {
            shiny = enums::ShinyEnum::Square
        } else {
            shiny = enums::ShinyEnum::Star
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
    if lead != enums::LeadFilterEnum::Synchronize {
        let nature_rand = rng.next();
        nature = nature_rand - (nature_rand / 25) * 25;
    } else {
        nature = 25;
    }

    PokemonbdspStationary {
        shiny,
        pid,
        ec,
        nature: enums::NatureEnum::try_from(nature).unwrap_or(enums::NatureEnum::Hardy),
        ivs,
        ability: enums::AbilityEnum::try_from(ability).unwrap_or(enums::AbilityEnum::Ability0),
        gender,
    }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UndergroundResults {
    pub shiny_value: enums::ShinyEnum,
    pub pid: u32,
    pub ec: u32,
    pub nature: enums::NatureEnum,
    pub ivs: Vec<u32>,
    pub ability: enums::AbilityEnum,
    pub gender: enums::GenderEnum,
    pub encounter: u8,
    pub advances: usize,
    pub is_rare: bool,
}

pub fn generate_bdsp_pokemon_underground(
    mut rng: Xorshift,
    gender_ratio: enums::GenderRatioEnum,
    advances: usize,
    tiles: usize,
    large_room: bool,
    diglett_boost: bool,
) -> Vec<UndergroundResults> {
    let mut results: Vec<UndergroundResults> = Vec::new();
    let rare_check = rng.rand_range(0, 100);

    if rare_check < 50 {
        rng.next();
    }

    let min_max_rand = rng.rand_range(0, 100);

    let mut poke_num = 5;
    if large_room {
        poke_num = 7;
    }

    let tile_boost = match tiles {
        0 => 0,
        1..=15 => 5,
        16..=30 => 10,
        31..=45 => 15,
        tiles if tiles > 60 => 30,
        _ => 20,
    };

    if 50 - tile_boost as u32 <= min_max_rand {
        if large_room {
            poke_num = 10;
        } else {
            poke_num = 7;
        }
    }

    if rare_check < 50 {
        poke_num = poke_num - 1;
    }

    rng.advance(poke_num * 2);

    let values = 0..poke_num;

    for _ in values {
        let pokemon_results =
            generate_underground_pokemon(&mut rng, gender_ratio, advances, diglett_boost);
        results.push(pokemon_results);
    }

    if rare_check < 50 {
        let pokemon_results =
            generate_rare_underground_pokemon(&mut rng, gender_ratio, advances, diglett_boost);
        results.push(pokemon_results);
    }

    fn generate_underground_pokemon(
        rng: &mut Xorshift,
        gender_ratio: enums::GenderRatioEnum,
        advances: usize,
        diglett_boost: bool,
    ) -> UndergroundResults {
        rng.next(); // slot weight call?
        rng.next(); // level
        let mut shiny = enums::ShinyEnum::None;
        let ec = rng.next();

        let shiny_rolls = if diglett_boost { 2 } else { 1 };
        let mut pid = 0;

        let shiny_rand = rng.next();
        for _ in 0..shiny_rolls {
            pid = rng.next();
            let psv = shiny_rand & 0xFFFF ^ shiny_rand >> 0x10;
            let tsv = pid >> 0x10 ^ pid & 0xFFFF;
            if (psv ^ tsv) < 0x10 {
                shiny = enums::ShinyEnum::Star;
                break;
            }

            if (psv ^ tsv) == 0 {
                shiny = enums::ShinyEnum::Square;
                break;
            }
        }

        let mut ivs = vec![32, 32, 32, 32, 32, 32];
        for i in ivs.iter_mut() {
            *i = rng.rand_max(32);
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
        let nature_rand = rng.next();
        let nature = nature_rand - (nature_rand / 25) * 25;
        rng.next(); // height 1
        rng.next(); // height 2
        rng.next(); // weight 1
        rng.next(); // weight 2
        rng.next(); // item
        rng.next(); // egg move
                    // randNum between 0 and max egg moves, then use as index for egg move
        let encounter = 0;

        UndergroundResults {
            shiny_value: shiny,
            pid,
            ec,
            nature: enums::NatureEnum::try_from(nature).unwrap_or(enums::NatureEnum::Hardy),
            ivs,
            ability: enums::AbilityEnum::try_from(ability).unwrap_or(enums::AbilityEnum::Ability0),
            gender,
            encounter,
            advances,
            is_rare: false,
        }
    }

    fn generate_rare_underground_pokemon(
        rng: &mut Xorshift,
        gender_ratio: enums::GenderRatioEnum,
        advances: usize,
        diglett_boost: bool,
    ) -> UndergroundResults {
        rng.next(); // level
        let mut shiny = enums::ShinyEnum::None;
        let ec = rng.next();

        let shiny_rolls = if diglett_boost { 2 } else { 1 };
        let mut pid = 0;

        let shiny_rand = rng.next();
        for _ in 0..shiny_rolls {
            pid = rng.next();
            let psv = shiny_rand & 0xFFFF ^ shiny_rand >> 0x10;
            let tsv = pid >> 0x10 ^ pid & 0xFFFF;
            if (psv ^ tsv) < 0x10 {
                shiny = enums::ShinyEnum::Star;
                break;
            }

            if (psv ^ tsv) == 0 {
                shiny = enums::ShinyEnum::Square;
                break;
            }
        }

        let mut ivs = vec![32, 32, 32, 32, 32, 32];
        for i in ivs.iter_mut() {
            *i = rng.rand_max(32);
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
        let nature_rand = rng.next();
        let nature = nature_rand - (nature_rand / 25) * 25;
        rng.next(); // height 1
        rng.next(); // height 2
        rng.next(); // weight 1
        rng.next(); // weight 2
        rng.next(); // item
        rng.next(); // egg move
        let encounter = 0;

        UndergroundResults {
            shiny_value: shiny,
            pid,
            ec,
            nature: enums::NatureEnum::try_from(nature).unwrap_or(enums::NatureEnum::Hardy),
            ivs,
            ability: enums::AbilityEnum::try_from(ability).unwrap_or(enums::AbilityEnum::Ability0),
            gender,
            encounter,
            advances,
            is_rare: true,
        }
    }

    results
}

pub fn generate_tid(mut rng: Xorshift) -> TIDbdsp {
    let sidtid = rng.next();
    let tid = sidtid & 0xFFFF;
    let sid = sidtid >> 0x10;

    let tsv = ((tid ^ sid) >> 4) as u16;
    let g8tid = sidtid % 1000000;

    TIDbdsp {
        tid: (tid as u16),
        tsv,
        g8tid,
        sid: (sid as u16),
    }
}

pub fn generate_bdsp_pokemon_roamer(
    mut seed_rng: Xorshift,
    gender_ratio: enums::GenderRatioEnum,
    set_ivs: bool,
) -> PokemonbdspStationary {
    let mut shiny = enums::ShinyEnum::None;

    let seed = seed_rng.next();
    let ec = seed;

    let mut rng = Xoroshiro::new_bdsp(seed);

    let shiny_rand = rng.next_bdsp();
    let pid = rng.next_bdsp();

    let psv = shiny_rand & 0xFFFF ^ shiny_rand >> 0x10;
    let tsv = pid >> 0x10 ^ pid & 0xFFFF;
    if (psv ^ tsv) < 0x10 {
        if (psv ^ tsv) == 0 {
            shiny = enums::ShinyEnum::Square
        } else {
            shiny = enums::ShinyEnum::Star
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

    PokemonbdspStationary {
        shiny,
        pid,
        ec,
        nature: enums::NatureEnum::try_from(nature).unwrap_or(enums::NatureEnum::Hardy),
        ivs,
        ability: enums::AbilityEnum::try_from(ability).unwrap_or(enums::AbilityEnum::Ability0),
        gender,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod generate_bdsp_pokemon {
        use super::*;

        #[test]
        fn should_generate_pokemon() {
            let mut rng = Xorshift::from_state([1, 2, 3, 4]);
            let expected_results = vec![
                Pokemonbdsp {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x6a88a6e4,
                    ec: 0x219cc273,
                    nature: enums::NatureEnum::Bashful,
                    ivs: vec![12, 20, 1, 12, 5, 20],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Female,
                    encounter: 4,
                },
                Pokemonbdsp {
                    shiny: enums::ShinyEnum::None,
                    pid: 0xa28ce86c,
                    ec: 0x032ebce9,
                    nature: enums::NatureEnum::Hardy,
                    ivs: vec![20, 1, 12, 5, 20, 16],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Female,
                    encounter: 5,
                },
                Pokemonbdsp {
                    shiny: enums::ShinyEnum::None,
                    pid: 0xe5443914,
                    ec: 0x6a88a6e4,
                    nature: enums::NatureEnum::Quiet,
                    ivs: vec![1, 12, 5, 20, 16, 20],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Female,
                    encounter: 0,
                },
                Pokemonbdsp {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x137b08a1,
                    ec: 0xa28ce86c,
                    nature: enums::NatureEnum::Jolly,
                    ivs: vec![12, 5, 20, 16, 20, 19],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Male,
                    encounter: 1,
                },
                Pokemonbdsp {
                    shiny: enums::ShinyEnum::None,
                    pid: 0xbc6b23ac,
                    ec: 0xe5443914,
                    nature: enums::NatureEnum::Naive,
                    ivs: vec![5, 20, 16, 20, 19, 6],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Female,
                    encounter: 6,
                },
                Pokemonbdsp {
                    shiny: enums::ShinyEnum::None,
                    pid: 0xf9e163c5,
                    ec: 0x137b08a1,
                    nature: enums::NatureEnum::Jolly,
                    ivs: vec![20, 16, 20, 19, 6, 28],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Female,
                    encounter: 7,
                },
                Pokemonbdsp {
                    shiny: enums::ShinyEnum::None,
                    pid: 0xbd297974,
                    ec: 0xbc6b23ac,
                    nature: enums::NatureEnum::Naughty,
                    ivs: vec![16, 20, 19, 6, 28, 0],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Male,
                    encounter: 3,
                },
                Pokemonbdsp {
                    shiny: enums::ShinyEnum::None,
                    pid: 0xf65c4070,
                    ec: 0xf9e163c5,
                    nature: enums::NatureEnum::Naughty,
                    ivs: vec![20, 19, 6, 28, 0, 10],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Male,
                    encounter: 1,
                },
                Pokemonbdsp {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x934f7b54,
                    ec: 0xbd297974,
                    nature: enums::NatureEnum::Bold,
                    ivs: vec![19, 6, 28, 0, 10, 17],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Female,
                    encounter: 3,
                },
                Pokemonbdsp {
                    shiny: enums::ShinyEnum::None,
                    pid: 0xe1c2cdb3,
                    ec: 0xf65c4070,
                    nature: enums::NatureEnum::Adamant,
                    ivs: vec![6, 28, 0, 10, 17, 25],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Female,
                    encounter: 1,
                },
                Pokemonbdsp {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x9756fa26,
                    ec: 0x934f7b54,
                    nature: enums::NatureEnum::Sassy,
                    ivs: vec![28, 0, 10, 17, 25, 10],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Female,
                    encounter: 1,
                },
                Pokemonbdsp {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x039d677c,
                    ec: 0xe1c2cdb3,
                    nature: enums::NatureEnum::Jolly,
                    ivs: vec![0, 10, 17, 25, 10, 12],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Female,
                    encounter: 1,
                },
                Pokemonbdsp {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x6b603980,
                    ec: 0x9756fa26,
                    nature: enums::NatureEnum::Lax,
                    ivs: vec![10, 17, 25, 10, 12, 21],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Male,
                    encounter: 0,
                },
                Pokemonbdsp {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x1cb8de0a,
                    ec: 0x039d677c,
                    nature: enums::NatureEnum::Quirky,
                    ivs: vec![17, 25, 10, 12, 21, 9],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Male,
                    encounter: 2,
                },
                Pokemonbdsp {
                    shiny: enums::ShinyEnum::None,
                    pid: 0xbc9f8071,
                    ec: 0x6b603980,
                    nature: enums::NatureEnum::Bashful,
                    ivs: vec![25, 10, 12, 21, 9, 22],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Female,
                    encounter: 0,
                },
                Pokemonbdsp {
                    shiny: enums::ShinyEnum::None,
                    pid: 0xd451a619,
                    ec: 0x1cb8de0a,
                    nature: enums::NatureEnum::Careful,
                    ivs: vec![10, 12, 21, 9, 22, 21],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Female,
                    encounter: 2,
                },
                Pokemonbdsp {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x3e17392a,
                    ec: 0xbc9f8071,
                    nature: enums::NatureEnum::Quiet,
                    ivs: vec![12, 21, 9, 22, 21, 30],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Male,
                    encounter: 2,
                },
                Pokemonbdsp {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x6405e86c,
                    ec: 0xd451a619,
                    nature: enums::NatureEnum::Calm,
                    ivs: vec![21, 9, 22, 21, 30, 20],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Male,
                    encounter: 8,
                },
                Pokemonbdsp {
                    shiny: enums::ShinyEnum::None,
                    pid: 0xa4596095,
                    ec: 0x3e17392a,
                    nature: enums::NatureEnum::Quiet,
                    ivs: vec![9, 22, 21, 30, 20, 24],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Female,
                    encounter: 4,
                },
                Pokemonbdsp {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x7de16b69,
                    ec: 0x6405e86c,
                    nature: enums::NatureEnum::Timid,
                    ivs: vec![22, 21, 30, 20, 24, 29],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Male,
                    encounter: 5,
                },
                Pokemonbdsp {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x7a38c396,
                    ec: 0xa4596095,
                    nature: enums::NatureEnum::Gentle,
                    ivs: vec![21, 30, 20, 24, 29, 15],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Female,
                    encounter: 3,
                },
                Pokemonbdsp {
                    shiny: enums::ShinyEnum::None,
                    pid: 0xb1b51235,
                    ec: 0x7de16b69,
                    nature: enums::NatureEnum::Bashful,
                    ivs: vec![30, 20, 24, 29, 15, 24],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Male,
                    encounter: 1,
                },
                Pokemonbdsp {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x5e07815e,
                    ec: 0x7a38c396,
                    nature: enums::NatureEnum::Hasty,
                    ivs: vec![20, 24, 29, 15, 24, 21],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Female,
                    encounter: 0,
                },
                Pokemonbdsp {
                    shiny: enums::ShinyEnum::None,
                    pid: 0xa84b03d4,
                    ec: 0xb1b51235,
                    nature: enums::NatureEnum::Mild,
                    ivs: vec![24, 29, 15, 24, 21, 3],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Female,
                    encounter: 6,
                },
                Pokemonbdsp {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x94535138,
                    ec: 0x5e07815e,
                    nature: enums::NatureEnum::Adamant,
                    ivs: vec![29, 15, 24, 21, 3, 21],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Male,
                    encounter: 0,
                },
            ];

            for (advance, expected_result) in expected_results.iter().enumerate() {
                let result = generate_bdsp_pokemon(
                    rng.clone(),
                    enums::GenderRatioEnum::Male50Female50,
                    enums::LeadFilterEnum::None,
                );

                assert_eq!(&result, expected_result, "Mismatch on advance {}", advance);
                rng.next();
            }
        }

        #[test]
        fn should_generate_synchronized_pokemon() {
            let mut rng = Xorshift::from_state([1, 2, 3, 4]);
            let expected_results = vec![
                Pokemonbdsp {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x6a88a6e4,
                    ec: 0x219cc273,
                    nature: enums::NatureEnum::Synchronize,
                    ivs: vec![12, 20, 1, 12, 5, 20],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Female,
                    encounter: 4,
                },
                Pokemonbdsp {
                    shiny: enums::ShinyEnum::None,
                    pid: 0xa28ce86c,
                    ec: 0x032ebce9,
                    nature: enums::NatureEnum::Synchronize,
                    ivs: vec![20, 1, 12, 5, 20, 16],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Female,
                    encounter: 5,
                },
                Pokemonbdsp {
                    shiny: enums::ShinyEnum::None,
                    pid: 0xe5443914,
                    ec: 0x6a88a6e4,
                    nature: enums::NatureEnum::Synchronize,
                    ivs: vec![1, 12, 5, 20, 16, 20],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Female,
                    encounter: 0,
                },
                Pokemonbdsp {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x137b08a1,
                    ec: 0xa28ce86c,
                    nature: enums::NatureEnum::Synchronize,
                    ivs: vec![12, 5, 20, 16, 20, 19],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Male,
                    encounter: 1,
                },
                Pokemonbdsp {
                    shiny: enums::ShinyEnum::None,
                    pid: 0xbc6b23ac,
                    ec: 0xe5443914,
                    nature: enums::NatureEnum::Synchronize,
                    ivs: vec![5, 20, 16, 20, 19, 6],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Female,
                    encounter: 6,
                },
                Pokemonbdsp {
                    shiny: enums::ShinyEnum::None,
                    pid: 0xf9e163c5,
                    ec: 0x137b08a1,
                    nature: enums::NatureEnum::Synchronize,
                    ivs: vec![20, 16, 20, 19, 6, 28],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Female,
                    encounter: 7,
                },
                Pokemonbdsp {
                    shiny: enums::ShinyEnum::None,
                    pid: 0xbd297974,
                    ec: 0xbc6b23ac,
                    nature: enums::NatureEnum::Synchronize,
                    ivs: vec![16, 20, 19, 6, 28, 0],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Male,
                    encounter: 3,
                },
                Pokemonbdsp {
                    shiny: enums::ShinyEnum::None,
                    pid: 0xf65c4070,
                    ec: 0xf9e163c5,
                    nature: enums::NatureEnum::Synchronize,
                    ivs: vec![20, 19, 6, 28, 0, 10],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Male,
                    encounter: 1,
                },
                Pokemonbdsp {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x934f7b54,
                    ec: 0xbd297974,
                    nature: enums::NatureEnum::Synchronize,
                    ivs: vec![19, 6, 28, 0, 10, 17],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Female,
                    encounter: 3,
                },
                Pokemonbdsp {
                    shiny: enums::ShinyEnum::None,
                    pid: 0xe1c2cdb3,
                    ec: 0xf65c4070,
                    nature: enums::NatureEnum::Synchronize,
                    ivs: vec![6, 28, 0, 10, 17, 25],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Female,
                    encounter: 1,
                },
                Pokemonbdsp {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x9756fa26,
                    ec: 0x934f7b54,
                    nature: enums::NatureEnum::Synchronize,
                    ivs: vec![28, 0, 10, 17, 25, 10],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Female,
                    encounter: 1,
                },
                Pokemonbdsp {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x039d677c,
                    ec: 0xe1c2cdb3,
                    nature: enums::NatureEnum::Synchronize,
                    ivs: vec![0, 10, 17, 25, 10, 12],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Female,
                    encounter: 1,
                },
                Pokemonbdsp {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x6b603980,
                    ec: 0x9756fa26,
                    nature: enums::NatureEnum::Synchronize,
                    ivs: vec![10, 17, 25, 10, 12, 21],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Male,
                    encounter: 0,
                },
                Pokemonbdsp {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x1cb8de0a,
                    ec: 0x039d677c,
                    nature: enums::NatureEnum::Synchronize,
                    ivs: vec![17, 25, 10, 12, 21, 9],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Male,
                    encounter: 2,
                },
                Pokemonbdsp {
                    shiny: enums::ShinyEnum::None,
                    pid: 0xbc9f8071,
                    ec: 0x6b603980,
                    nature: enums::NatureEnum::Synchronize,
                    ivs: vec![25, 10, 12, 21, 9, 22],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Female,
                    encounter: 0,
                },
                Pokemonbdsp {
                    shiny: enums::ShinyEnum::None,
                    pid: 0xd451a619,
                    ec: 0x1cb8de0a,
                    nature: enums::NatureEnum::Synchronize,
                    ivs: vec![10, 12, 21, 9, 22, 21],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Female,
                    encounter: 2,
                },
                Pokemonbdsp {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x3e17392a,
                    ec: 0xbc9f8071,
                    nature: enums::NatureEnum::Synchronize,
                    ivs: vec![12, 21, 9, 22, 21, 30],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Male,
                    encounter: 2,
                },
                Pokemonbdsp {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x6405e86c,
                    ec: 0xd451a619,
                    nature: enums::NatureEnum::Synchronize,
                    ivs: vec![21, 9, 22, 21, 30, 20],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Male,
                    encounter: 8,
                },
                Pokemonbdsp {
                    shiny: enums::ShinyEnum::None,
                    pid: 0xa4596095,
                    ec: 0x3e17392a,
                    nature: enums::NatureEnum::Synchronize,
                    ivs: vec![9, 22, 21, 30, 20, 24],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Female,
                    encounter: 4,
                },
                Pokemonbdsp {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x7de16b69,
                    ec: 0x6405e86c,
                    nature: enums::NatureEnum::Synchronize,
                    ivs: vec![22, 21, 30, 20, 24, 29],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Male,
                    encounter: 5,
                },
                Pokemonbdsp {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x7a38c396,
                    ec: 0xa4596095,
                    nature: enums::NatureEnum::Synchronize,
                    ivs: vec![21, 30, 20, 24, 29, 15],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Female,
                    encounter: 3,
                },
                Pokemonbdsp {
                    shiny: enums::ShinyEnum::None,
                    pid: 0xb1b51235,
                    ec: 0x7de16b69,
                    nature: enums::NatureEnum::Synchronize,
                    ivs: vec![30, 20, 24, 29, 15, 24],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Male,
                    encounter: 1,
                },
                Pokemonbdsp {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x5e07815e,
                    ec: 0x7a38c396,
                    nature: enums::NatureEnum::Synchronize,
                    ivs: vec![20, 24, 29, 15, 24, 21],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Female,
                    encounter: 0,
                },
                Pokemonbdsp {
                    shiny: enums::ShinyEnum::None,
                    pid: 0xa84b03d4,
                    ec: 0xb1b51235,
                    nature: enums::NatureEnum::Synchronize,
                    ivs: vec![24, 29, 15, 24, 21, 3],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Female,
                    encounter: 6,
                },
                Pokemonbdsp {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x94535138,
                    ec: 0x5e07815e,
                    nature: enums::NatureEnum::Synchronize,
                    ivs: vec![29, 15, 24, 21, 3, 21],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Male,
                    encounter: 0,
                },
            ];

            for (advance, expected_result) in expected_results.iter().enumerate() {
                let result = generate_bdsp_pokemon(
                    rng.clone(),
                    enums::GenderRatioEnum::Male50Female50,
                    enums::LeadFilterEnum::Synchronize,
                );

                rng.next();
                assert_eq!(&result, expected_result, "Mismatch on advance {}", advance);
            }
        }
    }

    mod generate_bdsp_pokemon_stationary {
        use super::*;

        #[test]
        fn should_generate_pokemon() {
            let mut rng = Xorshift::from_state([1, 2, 3, 4]);
            let expected_results: Vec<PokemonbdspStationary> = vec![
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x80000004,
                    ec: 0x8000080d,
                    nature: enums::NatureEnum::Lax,
                    ivs: vec![0, 13, 26, 14, 30, 11],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Female,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x80002020,
                    ec: 0x8000181f,
                    nature: enums::NatureEnum::Hasty,
                    ivs: vec![13, 26, 14, 30, 11, 25],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Female,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x8040004d,
                    ec: 0x80000004,
                    nature: enums::NatureEnum::Rash,
                    ivs: vec![26, 14, 30, 11, 25, 15],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Female,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x808020ba,
                    ec: 0x80002020,
                    nature: enums::NatureEnum::Quiet,
                    ivs: vec![14, 30, 11, 25, 15, 24],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Female,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x8080008e,
                    ec: 0x8040004d,
                    nature: enums::NatureEnum::Gentle,
                    ivs: vec![30, 11, 25, 15, 24, 9],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Female,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x8180219e,
                    ec: 0x808020ba,
                    nature: enums::NatureEnum::Mild,
                    ivs: vec![11, 25, 15, 24, 9, 30],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Male,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x81c20b8b,
                    ec: 0x8080008e,
                    nature: enums::NatureEnum::Naive,
                    ivs: vec![25, 15, 24, 9, 30, 11],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Female,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x80467ef9,
                    ec: 0x8180219e,
                    nature: enums::NatureEnum::Naive,
                    ivs: vec![15, 24, 9, 30, 11, 28],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Female,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x80c28a0f,
                    ec: 0x81c20b8b,
                    nature: enums::NatureEnum::Gentle,
                    ivs: vec![24, 9, 30, 11, 28, 15],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Male,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x804ed758,
                    ec: 0x80467ef9,
                    nature: enums::NatureEnum::Relaxed,
                    ivs: vec![9, 30, 11, 28, 15, 23],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Male,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x91c11a89,
                    ec: 0x80c28a0f,
                    nature: enums::NatureEnum::Docile,
                    ivs: vec![30, 11, 28, 15, 23, 6],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Female,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0xa2431ffe,
                    ec: 0x804ed758,
                    nature: enums::NatureEnum::Lonely,
                    ivs: vec![11, 28, 15, 23, 6, 29],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Female,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0xb6c57b4b,
                    ec: 0x91c11a89,
                    nature: enums::NatureEnum::Bold,
                    ivs: vec![28, 15, 23, 6, 29, 27],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Male,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0xc0479edc,
                    ec: 0xa2431ffe,
                    nature: enums::NatureEnum::Timid,
                    ivs: vec![15, 23, 6, 29, 27, 5],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Male,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0xd94bd10f,
                    ec: 0xb6c57b4b,
                    nature: enums::NatureEnum::Relaxed,
                    ivs: vec![23, 6, 29, 27, 5, 3],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Male,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0xe3cd8937,
                    ec: 0xc0479edc,
                    nature: enums::NatureEnum::Calm,
                    ivs: vec![6, 29, 27, 5, 3, 6],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Female,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0xfecfb926,
                    ec: 0xd94bd10f,
                    nature: enums::NatureEnum::Jolly,
                    ivs: vec![29, 27, 5, 3, 6, 4],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Male,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x8202795d,
                    ec: 0xe3cd8937,
                    nature: enums::NatureEnum::Mild,
                    ivs: vec![27, 5, 3, 6, 4, 16],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Female,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x85c613bb,
                    ec: 0xfecfb926,
                    nature: enums::NatureEnum::Brave,
                    ivs: vec![5, 3, 6, 4, 16, 31],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Male,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x8a4da605,
                    ec: 0x8202795d,
                    nature: enums::NatureEnum::Naughty,
                    ivs: vec![3, 6, 4, 16, 31, 13],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Male,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x894828e3,
                    ec: 0x85c613bb,
                    nature: enums::NatureEnum::Naughty,
                    ivs: vec![6, 4, 16, 31, 13, 14],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Male,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x98917006,
                    ec: 0x8a4da605,
                    nature: enums::NatureEnum::Careful,
                    ivs: vec![4, 16, 31, 13, 14, 9],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Female,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0xadffe364,
                    ec: 0x894828e3,
                    nature: enums::NatureEnum::Lonely,
                    ivs: vec![16, 31, 13, 14, 9, 10],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Female,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0xcae51550,
                    ec: 0x98917006,
                    nature: enums::NatureEnum::Mild,
                    ivs: vec![31, 13, 14, 9, 10, 15],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Female,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x82a223df,
                    ec: 0xadffe364,
                    nature: enums::NatureEnum::Lonely,
                    ivs: vec![13, 14, 9, 10, 15, 0],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Female,
                },
            ];

            for (advance, expected_result) in expected_results.iter().enumerate() {
                let result = generate_bdsp_pokemon_stationary(
                    rng.clone(),
                    enums::GenderRatioEnum::Male50Female50,
                    false,
                    enums::LeadFilterEnum::None,
                );

                assert_eq!(&result, expected_result, "Mismatch on advance {}", advance);
                rng.next();
            }
        }

        #[test]
        fn should_generate_pokemon_with_set_ivs() {
            let mut rng = Xorshift::from_state([1, 2, 3, 4]);
            let expected_results: Vec<PokemonbdspStationary> = vec![
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x80000004,
                    ec: 0x8000080d,
                    nature: enums::NatureEnum::Hasty,
                    ivs: vec![31, 30, 31, 11, 25, 31],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Female,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x80002020,
                    ec: 0x8000181f,
                    nature: enums::NatureEnum::Hasty,
                    ivs: vec![31, 30, 31, 11, 25, 31],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Female,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x8040004d,
                    ec: 0x80000004,
                    nature: enums::NatureEnum::Rash,
                    ivs: vec![31, 11, 31, 25, 31, 15],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Female,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x808020ba,
                    ec: 0x80002020,
                    nature: enums::NatureEnum::Quiet,
                    ivs: vec![25, 15, 31, 24, 31, 31],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Female,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x8080008e,
                    ec: 0x8040004d,
                    nature: enums::NatureEnum::Gentle,
                    ivs: vec![15, 24, 9, 31, 31, 31],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Female,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x8180219e,
                    ec: 0x808020ba,
                    nature: enums::NatureEnum::Mild,
                    ivs: vec![24, 31, 9, 31, 30, 31],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Male,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x81c20b8b,
                    ec: 0x8080008e,
                    nature: enums::NatureEnum::Naive,
                    ivs: vec![9, 31, 31, 31, 30, 11],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Female,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x80467ef9,
                    ec: 0x8180219e,
                    nature: enums::NatureEnum::Naive,
                    ivs: vec![30, 31, 31, 31, 11, 28],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Female,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x80c28a0f,
                    ec: 0x81c20b8b,
                    nature: enums::NatureEnum::Gentle,
                    ivs: vec![11, 28, 31, 31, 31, 15],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Male,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x804ed758,
                    ec: 0x80467ef9,
                    nature: enums::NatureEnum::Relaxed,
                    ivs: vec![28, 31, 15, 31, 31, 23],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Male,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x91c11a89,
                    ec: 0x80c28a0f,
                    nature: enums::NatureEnum::Docile,
                    ivs: vec![15, 31, 31, 23, 31, 6],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Female,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0xa2431ffe,
                    ec: 0x804ed758,
                    nature: enums::NatureEnum::Lonely,
                    ivs: vec![23, 31, 31, 31, 6, 29],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Female,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0xb6c57b4b,
                    ec: 0x91c11a89,
                    nature: enums::NatureEnum::Timid,
                    ivs: vec![31, 29, 31, 31, 27, 5],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Male,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0xc0479edc,
                    ec: 0xa2431ffe,
                    nature: enums::NatureEnum::Relaxed,
                    ivs: vec![31, 31, 27, 31, 5, 3],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Male,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0xd94bd10f,
                    ec: 0xb6c57b4b,
                    nature: enums::NatureEnum::Relaxed,
                    ivs: vec![31, 31, 27, 31, 5, 3],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Male,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0xe3cd8937,
                    ec: 0xc0479edc,
                    nature: enums::NatureEnum::Calm,
                    ivs: vec![31, 31, 5, 31, 3, 6],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Female,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0xfecfb926,
                    ec: 0xd94bd10f,
                    nature: enums::NatureEnum::Jolly,
                    ivs: vec![3, 31, 6, 31, 4, 31],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Male,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x8202795d,
                    ec: 0xe3cd8937,
                    nature: enums::NatureEnum::Brave,
                    ivs: vec![4, 16, 31, 31, 31, 31],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Male,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x85c613bb,
                    ec: 0xfecfb926,
                    nature: enums::NatureEnum::Naughty,
                    ivs: vec![16, 31, 31, 13, 31, 31],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Male,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x8a4da605,
                    ec: 0x8202795d,
                    nature: enums::NatureEnum::Naughty,
                    ivs: vec![16, 31, 31, 13, 31, 31],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Male,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x894828e3,
                    ec: 0x85c613bb,
                    nature: enums::NatureEnum::Careful,
                    ivs: vec![13, 31, 31, 14, 31, 9],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Female,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x98917006,
                    ec: 0x8a4da605,
                    nature: enums::NatureEnum::Careful,
                    ivs: vec![13, 31, 31, 14, 31, 9],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Female,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0xadffe364,
                    ec: 0x894828e3,
                    nature: enums::NatureEnum::Lonely,
                    ivs: vec![14, 31, 9, 10, 31, 31],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Female,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0xcae51550,
                    ec: 0x98917006,
                    nature: enums::NatureEnum::Mild,
                    ivs: vec![31, 31, 9, 10, 15, 31],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Female,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x82a223df,
                    ec: 0xadffe364,
                    nature: enums::NatureEnum::Hasty,
                    ivs: vec![31, 31, 0, 25, 21, 31],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Female,
                },
            ];

            for (advance, expected_result) in expected_results.iter().enumerate() {
                let result = generate_bdsp_pokemon_stationary(
                    rng.clone(),
                    enums::GenderRatioEnum::Male50Female50,
                    true,
                    enums::LeadFilterEnum::None,
                );

                assert_eq!(&result, expected_result, "Mismatch on advance {}", advance);
                rng.next();
            }
        }
    }

    mod generate_tid {
        use super::*;

        #[test]
        fn should_generate_tids() {
            let mut rng = Xorshift::from_state([1, 2, 3, 4]);
            let expected_results = vec![
                TIDbdsp {
                    tid: 2061,
                    tsv: 2176,
                    g8tid: 485709,
                    sid: 32768,
                },
                TIDbdsp {
                    tid: 6175,
                    tsv: 2433,
                    g8tid: 489823,
                    sid: 32768,
                },
                TIDbdsp {
                    tid: 4,
                    tsv: 2048,
                    g8tid: 483652,
                    sid: 32768,
                },
                TIDbdsp {
                    tid: 8224,
                    tsv: 2562,
                    g8tid: 491872,
                    sid: 32768,
                },
                TIDbdsp {
                    tid: 77,
                    tsv: 2048,
                    g8tid: 678029,
                    sid: 32832,
                },
                TIDbdsp {
                    tid: 8378,
                    tsv: 2563,
                    g8tid: 880634,
                    sid: 32896,
                },
                TIDbdsp {
                    tid: 142,
                    tsv: 2048,
                    g8tid: 872398,
                    sid: 32896,
                },
                TIDbdsp {
                    tid: 8606,
                    tsv: 2561,
                    g8tid: 658078,
                    sid: 33152,
                },
                TIDbdsp {
                    tid: 2955,
                    tsv: 2212,
                    g8tid: 977803,
                    sid: 33218,
                },
                TIDbdsp {
                    tid: 32505,
                    tsv: 4075,
                    g8tid: 103673,
                    sid: 32838,
                },
                TIDbdsp {
                    tid: 35343,
                    tsv: 172,
                    g8tid: 232975,
                    sid: 32962,
                },
                TIDbdsp {
                    tid: 55128,
                    tsv: 1393,
                    g8tid: 650584,
                    sid: 32846,
                },
                TIDbdsp {
                    tid: 6793,
                    tsv: 2228,
                    g8tid: 351561,
                    sid: 37313,
                },
                TIDbdsp {
                    tid: 8190,
                    tsv: 3035,
                    g8tid: 308094,
                    sid: 41539,
                },
                TIDbdsp {
                    tid: 31563,
                    tsv: 3288,
                    g8tid: 395467,
                    sid: 46789,
                },
                TIDbdsp {
                    tid: 40668,
                    tsv: 1513,
                    g8tid: 919196,
                    sid: 49223,
                },
                TIDbdsp {
                    tid: 53519,
                    tsv: 132,
                    g8tid: 624591,
                    sid: 55627,
                },
                TIDbdsp {
                    tid: 35127,
                    tsv: 1711,
                    g8tid: 898039,
                    sid: 58317,
                },
                TIDbdsp {
                    tid: 47398,
                    tsv: 1150,
                    g8tid: 26214,
                    sid: 65231,
                },
                TIDbdsp {
                    tid: 31069,
                    tsv: 4021,
                    g8tid: 200221,
                    sid: 33282,
                },
                TIDbdsp {
                    tid: 5051,
                    tsv: 2407,
                    g8tid: 350907,
                    sid: 34246,
                },
                TIDbdsp {
                    tid: 42501,
                    tsv: 708,
                    g8tid: 344581,
                    sid: 35405,
                },
                TIDbdsp {
                    tid: 10467,
                    tsv: 2586,
                    g8tid: 207651,
                    sid: 35144,
                },
                TIDbdsp {
                    tid: 28678,
                    tsv: 3721,
                    g8tid: 668230,
                    sid: 39057,
                },
                TIDbdsp {
                    tid: 58212,
                    tsv: 1257,
                    g8tid: 228260,
                    sid: 44543,
                },
            ];

            for (advance, expected_result) in expected_results.iter().enumerate() {
                let result = generate_tid(rng.clone());
                assert_eq!(&result, expected_result, "Mismatch on advance {}", advance);
                rng.next();
            }
        }
    }

    mod generate_bdsp_pokemon_roamer {
        use super::*;

        #[test]
        fn should_generate_pokemon() {
            let mut rng = Xorshift::from_state([1, 2, 3, 4]);
            let expected_results: Vec<PokemonbdspStationary> = vec![
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x88c59540,
                    ec: 0x8000080d,
                    nature: enums::NatureEnum::Naive,
                    ivs: vec![19, 7, 19, 18, 2, 27],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Male,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0xeb66944a,
                    ec: 0x8000181f,
                    nature: enums::NatureEnum::Bold,
                    ivs: vec![11, 5, 16, 10, 11, 20],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Male,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x2d994828,
                    ec: 0x80000004,
                    nature: enums::NatureEnum::Bold,
                    ivs: vec![20, 6, 19, 16, 14, 24],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Male,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0xece70fe7,
                    ec: 0x80002020,
                    nature: enums::NatureEnum::Relaxed,
                    ivs: vec![6, 28, 17, 20, 19, 1],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Male,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0xf4728816,
                    ec: 0x8040004d,
                    nature: enums::NatureEnum::Hasty,
                    ivs: vec![22, 3, 31, 13, 2, 13],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Male,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0xf3ab101b,
                    ec: 0x808020ba,
                    nature: enums::NatureEnum::Naive,
                    ivs: vec![6, 23, 25, 2, 18, 20],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Male,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0xc0b5dfbe,
                    ec: 0x8080008e,
                    nature: enums::NatureEnum::Quirky,
                    ivs: vec![19, 25, 3, 1, 18, 31],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Female,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0xcb0fe58c,
                    ec: 0x8180219e,
                    nature: enums::NatureEnum::Adamant,
                    ivs: vec![1, 14, 4, 26, 15, 13],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Male,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x2ea9a543,
                    ec: 0x81c20b8b,
                    nature: enums::NatureEnum::Lonely,
                    ivs: vec![13, 3, 10, 5, 5, 18],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Female,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x0340802d,
                    ec: 0x80467ef9,
                    nature: enums::NatureEnum::Gentle,
                    ivs: vec![18, 27, 9, 20, 19, 30],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Female,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x7297c2ab,
                    ec: 0x80c28a0f,
                    nature: enums::NatureEnum::Careful,
                    ivs: vec![11, 28, 6, 21, 1, 17],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Female,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x27f1752e,
                    ec: 0x804ed758,
                    nature: enums::NatureEnum::Sassy,
                    ivs: vec![18, 23, 0, 9, 0, 14],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Female,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x92239010,
                    ec: 0x91c11a89,
                    nature: enums::NatureEnum::Modest,
                    ivs: vec![17, 10, 19, 30, 5, 26],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Female,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x941168e3,
                    ec: 0xa2431ffe,
                    nature: enums::NatureEnum::Jolly,
                    ivs: vec![0, 15, 1, 30, 12, 25],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Male,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x0735d88a,
                    ec: 0xb6c57b4b,
                    nature: enums::NatureEnum::Gentle,
                    ivs: vec![4, 27, 5, 15, 4, 22],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Male,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x94bfc292,
                    ec: 0xc0479edc,
                    nature: enums::NatureEnum::Modest,
                    ivs: vec![31, 20, 28, 15, 30, 8],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Male,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x353afd69,
                    ec: 0xd94bd10f,
                    nature: enums::NatureEnum::Brave,
                    ivs: vec![9, 12, 4, 10, 14, 11],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Male,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0xd397d63a,
                    ec: 0xe3cd8937,
                    nature: enums::NatureEnum::Relaxed,
                    ivs: vec![28, 12, 5, 30, 30, 2],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Female,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0xfb7ea0a4,
                    ec: 0xfecfb926,
                    nature: enums::NatureEnum::Jolly,
                    ivs: vec![29, 17, 22, 13, 9, 29],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Male,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x1d0acfab,
                    ec: 0x8202795d,
                    nature: enums::NatureEnum::Jolly,
                    ivs: vec![5, 5, 20, 6, 9, 5],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Male,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x62fe057c,
                    ec: 0x85c613bb,
                    nature: enums::NatureEnum::Careful,
                    ivs: vec![24, 9, 28, 24, 9, 0],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Male,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x78571fbf,
                    ec: 0x8a4da605,
                    nature: enums::NatureEnum::Gentle,
                    ivs: vec![31, 13, 30, 19, 0, 7],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Male,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x69809b59,
                    ec: 0x894828e3,
                    nature: enums::NatureEnum::Hardy,
                    ivs: vec![6, 1, 25, 18, 17, 4],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Female,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x0bf413f6,
                    ec: 0x98917006,
                    nature: enums::NatureEnum::Jolly,
                    ivs: vec![20, 15, 5, 28, 7, 25],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Male,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x31e845d8,
                    ec: 0xadffe364,
                    nature: enums::NatureEnum::Hardy,
                    ivs: vec![19, 26, 13, 3, 10, 22],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Male,
                },
            ];

            for (advance, expected_result) in expected_results.iter().enumerate() {
                let result = generate_bdsp_pokemon_roamer(
                    rng.clone(),
                    enums::GenderRatioEnum::Male50Female50,
                    false,
                );

                assert_eq!(&result, expected_result, "Mismatch on advance {}", advance);
                rng.next();
            }
        }

        #[test]
        fn should_generate_pokemon_with_set_ivs() {
            let mut rng = Xorshift::from_state([1, 2, 3, 4]);
            let expected_results: Vec<PokemonbdspStationary> = vec![
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x88c59540,
                    ec: 0x8000080d,
                    nature: enums::NatureEnum::Careful,
                    ivs: vec![2, 31, 27, 30, 31, 31],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Male,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0xeb66944a,
                    ec: 0x8000181f,
                    nature: enums::NatureEnum::Naive,
                    ivs: vec![11, 31, 31, 20, 31, 30],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Female,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x2d994828,
                    ec: 0x80000004,
                    nature: enums::NatureEnum::Lonely,
                    ivs: vec![14, 31, 31, 24, 31, 15],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Male,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0xece70fe7,
                    ec: 0x80002020,
                    nature: enums::NatureEnum::Relaxed,
                    ivs: vec![31, 31, 20, 19, 31, 1],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Male,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0xf4728816,
                    ec: 0x8040004d,
                    nature: enums::NatureEnum::Quiet,
                    ivs: vec![31, 13, 27, 8, 31, 31],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Female,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0xf3ab101b,
                    ec: 0x808020ba,
                    nature: enums::NatureEnum::Calm,
                    ivs: vec![31, 18, 31, 31, 20, 6],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Female,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0xc0b5dfbe,
                    ec: 0x8080008e,
                    nature: enums::NatureEnum::Quirky,
                    ivs: vec![18, 31, 31, 31, 22, 31],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Male,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0xcb0fe58c,
                    ec: 0x8180219e,
                    nature: enums::NatureEnum::Adamant,
                    ivs: vec![26, 15, 31, 31, 31, 13],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Male,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x2ea9a543,
                    ec: 0x81c20b8b,
                    nature: enums::NatureEnum::Quiet,
                    ivs: vec![31, 31, 5, 18, 28, 31],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Female,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x0340802d,
                    ec: 0x80467ef9,
                    nature: enums::NatureEnum::Gentle,
                    ivs: vec![20, 31, 19, 31, 31, 30],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Female,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x7297c2ab,
                    ec: 0x80c28a0f,
                    nature: enums::NatureEnum::Quiet,
                    ivs: vec![1, 17, 4, 31, 31, 31],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Male,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x27f1752e,
                    ec: 0x804ed758,
                    nature: enums::NatureEnum::Sassy,
                    ivs: vec![9, 31, 31, 0, 31, 14],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Female,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x92239010,
                    ec: 0x91c11a89,
                    nature: enums::NatureEnum::Modest,
                    ivs: vec![30, 31, 31, 5, 26, 31],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Female,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x941168e3,
                    ec: 0xa2431ffe,
                    nature: enums::NatureEnum::Jolly,
                    ivs: vec![31, 31, 30, 12, 25, 31],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Male,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x0735d88a,
                    ec: 0xb6c57b4b,
                    nature: enums::NatureEnum::Gentle,
                    ivs: vec![15, 31, 4, 22, 31, 31],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Male,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x94bfc292,
                    ec: 0xc0479edc,
                    nature: enums::NatureEnum::Modest,
                    ivs: vec![15, 31, 31, 30, 31, 8],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Male,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x353afd69,
                    ec: 0xd94bd10f,
                    nature: enums::NatureEnum::Brave,
                    ivs: vec![10, 31, 31, 14, 31, 11],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Male,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0xd397d63a,
                    ec: 0xe3cd8937,
                    nature: enums::NatureEnum::Relaxed,
                    ivs: vec![31, 31, 31, 30, 30, 2],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Female,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0xfb7ea0a4,
                    ec: 0xfecfb926,
                    nature: enums::NatureEnum::Jolly,
                    ivs: vec![31, 31, 13, 31, 9, 29],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Male,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x1d0acfab,
                    ec: 0x8202795d,
                    nature: enums::NatureEnum::Jolly,
                    ivs: vec![6, 9, 31, 31, 5, 31],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Male,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x62fe057c,
                    ec: 0x85c613bb,
                    nature: enums::NatureEnum::Careful,
                    ivs: vec![24, 9, 31, 31, 31, 0],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Male,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x78571fbf,
                    ec: 0x8a4da605,
                    nature: enums::NatureEnum::Gentle,
                    ivs: vec![19, 31, 31, 31, 0, 7],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Male,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x69809b59,
                    ec: 0x894828e3,
                    nature: enums::NatureEnum::Docile,
                    ivs: vec![4, 15, 31, 31, 6, 31],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Female,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x0bf413f6,
                    ec: 0x98917006,
                    nature: enums::NatureEnum::Jolly,
                    ivs: vec![31, 28, 7, 31, 25, 31],
                    ability: enums::AbilityEnum::Ability0,
                    gender: enums::GenderEnum::Male,
                },
                PokemonbdspStationary {
                    shiny: enums::ShinyEnum::None,
                    pid: 0x31e845d8,
                    ec: 0xadffe364,
                    nature: enums::NatureEnum::Hardy,
                    ivs: vec![3, 31, 31, 31, 10, 22],
                    ability: enums::AbilityEnum::Ability1,
                    gender: enums::GenderEnum::Male,
                },
            ];

            for (advance, expected_result) in expected_results.iter().enumerate() {
                let result = generate_bdsp_pokemon_roamer(
                    rng.clone(),
                    enums::GenderRatioEnum::Male50Female50,
                    true,
                );

                assert_eq!(&result, expected_result, "Mismatch on advance {}", advance);
                rng.next();
            }
        }
    }
}
