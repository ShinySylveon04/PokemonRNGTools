use crate::rng::Xorshift;
use chatot_forms::{Gen3Ability, Gender, GenderRatio, Nature, ShinyType};
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pokemon {
    pub shiny_value: Option<ShinyType>,
    pub pid: u32,
    pub ec: u32,
    pub nature: Nature,
    pub ivs: [u8; 6],
    pub ability: Gen3Ability,
    pub gender: Gender,
    pub encounter: u32,
    pub advances: usize,
    pub is_rare: bool,
}

#[derive(PartialEq, Eq, Debug)]
pub struct PokemonSlot {
    pub slot: u32,
    pub poke_type: u8,
}

pub fn generate_pokemon_slots(rng: &mut Xorshift) -> u8 {
    let typerates = vec![
        100, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50,
    ];

    let summed_typerates = vec![
        100, 150, 200, 250, 300, 350, 400, 450, 500, 550, 600, 650, 700, 750, 800, 850, 900, 950,
    ];

    let sum = typerates.iter().sum();

    let typerate_rand = rng.rand_range(0, sum);

    let poke_type = summed_typerates
        .iter()
        .position(|rate| typerate_rand < *rate)
        .unwrap_or(0) as u8;
    // for first rand use with typerates from ugrandmark typerate table

    poke_type
}

pub fn generate_size_slots(rng: &mut Xorshift) -> u32 {
    let mut sizes = vec![0, 0, 0, 1, 1, 1, 2, 2, 2];

    let size_rand = rng.rand_range(0, sizes.len().try_into().unwrap());
    let slot = sizes[size_rand as usize];
    sizes.remove(size_rand.try_into().unwrap());

    slot
}

pub fn generate_pokemon(
    mut rng: Xorshift,
    gender_ratio: GenderRatio,
    advances: usize,
    tiles: u8,
    large_room: bool,
    diglett_boost: bool,
) -> Vec<Pokemon> {
    let mut results: Vec<Pokemon> = Vec::new();
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
        poke_num -= 1;
    }

    // rng.advance(poke_num * 2);

    // let mut poke_results: Vec<PokemonSlot> = Vec::new();

    let mut test_results: Vec<u32> = Vec::new();

    for _i in 0..poke_num {
        let poke_type = generate_pokemon_slots(&mut rng);

        test_results.push(poke_type.into())

        // let result = PokemonSlot { poke_type, slot };

        // poke_results.push(result)
    }

    for _i in 0..poke_num {
        let slot = generate_size_slots(&mut rng);

        test_results.push(slot)
    }

    // assert_ne!(test_results, test_results);

    // assert_ne!(poke_results, poke_results);

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
        gender_ratio: GenderRatio,
        advances: usize,
        diglett_boost: bool,
    ) -> Pokemon {
        let encounter = rng.rand_range(0, 765); // slot weight call?
        rng.next(); // level
        let mut shiny = None;
        let ec = rng.next();

        let shiny_rolls = if diglett_boost { 2 } else { 1 };
        let mut pid = 0;

        let shiny_rand = rng.next();
        for _ in 0..shiny_rolls {
            pid = rng.next();
            let psv = shiny_rand & 0xFFFF ^ shiny_rand >> 0x10;
            let tsv = pid >> 0x10 ^ pid & 0xFFFF;
            if (psv ^ tsv) < 0x10 {
                shiny = Some(ShinyType::Star);
                break;
            }

            if (psv ^ tsv) == 0 {
                shiny = Some(ShinyType::Square);
                break;
            }
        }

        let mut ivs: [u8; 6] = [32, 32, 32, 32, 32, 32];
        for i in ivs.iter_mut() {
            *i = rng.rand_max(32) as u8;
        }

        let ability_rand = rng.next();
        let ability = ability_rand - (ability_rand / 2) * 2;

        let gender = match gender_ratio.get_set_gender() {
            Some(set_gender) => set_gender,
            None => {
                let gender_rand = rng.next();
                let gender_num = (gender_rand - (gender_rand / 253) * 253) + 1;
                gender_ratio.get_gender(gender_num as u8)
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
                    // let encounter = 0;

        Pokemon {
            shiny_value: shiny,
            pid,
            ec,
            nature: (nature as u8).into(),
            ivs,
            ability: (ability as u8).into(),
            gender,
            encounter,
            advances,
            is_rare: false,
        }
    }

    fn generate_rare_underground_pokemon(
        rng: &mut Xorshift,
        gender_ratio: GenderRatio,
        advances: usize,
        diglett_boost: bool,
    ) -> Pokemon {
        rng.next(); // level
        let mut shiny = None;
        let ec = rng.next();

        let shiny_rolls = if diglett_boost { 2 } else { 1 };
        let mut pid = 0;

        let shiny_rand = rng.next();
        for _ in 0..shiny_rolls {
            pid = rng.next();
            let psv = shiny_rand & 0xFFFF ^ shiny_rand >> 0x10;
            let tsv = pid >> 0x10 ^ pid & 0xFFFF;
            if (psv ^ tsv) < 0x10 {
                shiny = Some(ShinyType::Star);
                break;
            }

            if (psv ^ tsv) == 0 {
                shiny = Some(ShinyType::Square);
                break;
            }
        }

        let mut ivs = [32, 32, 32, 32, 32, 32];
        for i in ivs.iter_mut() {
            *i = rng.rand_max(32) as u8;
        }

        let ability_rand = rng.next();
        let ability = ability_rand - (ability_rand / 2) * 2;

        let gender = match gender_ratio.get_set_gender() {
            Some(set_gender) => set_gender,
            None => {
                let gender_rand = rng.next();
                let gender_num = (gender_rand - (gender_rand / 253) * 253) + 1;
                gender_ratio.get_gender(gender_num as u8)
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

        Pokemon {
            shiny_value: shiny,
            pid,
            ec,
            nature: (nature as u8).into(),
            ivs,
            ability: (ability as u8).into(),
            gender,
            encounter,
            advances,
            is_rare: true,
        }
    }

    results
}
