use crate::enums;
use crate::rng::Xoroshiro;
use crate::{calculate_shiny_value, check_is_shiny, Pokemon};
use std::convert::TryFrom;

pub fn generate_static_pokemon(
    mut rng: Xoroshiro,
    tid: u16,
    sid: u16,
    shiny_charm: bool,
) -> Pokemon {
    rng.rand_max(100);
    let tsv = calculate_shiny_value(tid, sid);
    let mut is_shiny = false;

    let shiny_rolls = if shiny_charm { 3 } else { 1 };

    for _ in 0..shiny_rolls {
        let rand = rng.next(); // mock pid
        is_shiny = check_is_shiny(tsv, rand);
        if is_shiny {
            break;
        }
    }

    rng.rand_max(2);
    let nature = rng.rand_max(25); //nature
    let ability = rng.rand_max(2); // ability
    let mut seed = Xoroshiro::new(rng.next() as u64);
    let ec = seed.next();
    let mut pid = seed.next();

    let tsv = tid ^ sid;
    let psv = ((pid >> 16) ^ (pid & 0xFFFF)) as u16;
    if !is_shiny {
        if (psv ^ tsv) < 16 {
            pid ^= 0x10000000; // force pid to not be shiny
        }
    } else {
        // force pid to be shiny
        if !((psv ^ tsv) < 16) {
            let pid_high = (pid & 0xFFFF) ^ tsv as u32;
            pid = (pid_high << 16) as u32 | (pid & 0xFFFF)
        }
    }

    let xor = ((pid >> 16) ^ (pid & 0xFFFF)) as u16 ^ tsv;

    let mut shiny_type = enums::Shiny::None;
    if xor < 0x10 {
        if xor == 0 {
            shiny_type = enums::Shiny::Square;
        } else {
            shiny_type = enums::Shiny::Star;
        }
    }

    Pokemon {
        shiny_type,
        ec,
        pid,
        nature: enums::Nature::try_from(nature as u16).unwrap_or(enums::Nature::Hardy),
        ability: enums::Ability::try_from(ability).unwrap_or(enums::Ability::Ability0),
    }
}

pub fn generate_dynamic_pokemon(
    mut rng: Xoroshiro,
    tid: u16,
    sid: u16,
    shiny_charm: bool,
) -> Pokemon {
    rng.next();
    rng.rand_max(100);
    rng.rand_max(100);
    rng.rand_max(100); // slot rand
    let max_level = 60;
    let min_level = 60;
    let level_diff = max_level - min_level;
    let _test = rng.rand_max(level_diff as u32 + 1);
    rng.rand_max(100);
    rng.rand_max(50);
    rng.rand_max(50);
    rng.rand_max(50);
    rng.rand_max(25);
    rng.rand_max(1000); // brilliant
    let tsv = calculate_shiny_value(tid, sid);
    let mut is_shiny = false;

    let shiny_rolls = if shiny_charm { 3 } else { 1 };

    for _ in 0..shiny_rolls {
        let rand = rng.next(); // mock pid
        is_shiny = check_is_shiny(tsv, rand);
        if is_shiny {
            break;
        }
    }

    rng.rand_max(2);
    let nature = rng.rand_max(25); // nature
    let ability = rng.rand_max(2); // ability
    let mut seed = Xoroshiro::new(rng.next() as u64);
    let ec = seed.next();
    let mut pid = seed.next();

    let tsv = tid ^ sid;
    let psv = ((pid >> 16) ^ (pid & 0xFFFF)) as u16;
    if !is_shiny {
        if (psv ^ tsv) < 16 {
            pid ^= 0x10000000; // force pid to not be shiny
        }
    } else {
        // force pid to be shiny
        if !((psv ^ tsv) < 16) {
            let pid_high = (pid & 0xFFFF) ^ tsv as u32;
            pid = (pid_high << 16) as u32 | (pid & 0xFFFF)
        }
    }

    let xor = ((pid >> 16) ^ (pid & 0xFFFF)) as u16 ^ tsv;

    let mut shiny_type = enums::Shiny::None;
    if xor < 0x10 {
        if xor == 0 {
            shiny_type = enums::Shiny::Square;
        } else {
            shiny_type = enums::Shiny::Star;
        }
    }

    Pokemon {
        shiny_type,
        ec,
        pid,
        nature: enums::Nature::try_from(nature as u16).unwrap_or(enums::Nature::Hardy),
        ability: enums::Ability::try_from(ability).unwrap_or(enums::Ability::Ability0),
    }
}
