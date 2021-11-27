use super::enums;
use super::{Pokemonbdsp, Xorshift};
use std::convert::TryFrom;

pub fn generate_bdsp_pokemon(mut rng: Xorshift) -> Pokemonbdsp {
    let encounter_rand = rng.rand_range(0, 100) as u8;
    rng.advance(84);
    let mut is_shiny = false;
    let ec = rng.next();
    let shiny_rand = rng.next();
    let pid = rng.next();

    if (shiny_rand & 0xFFF0 ^ shiny_rand >> 0x10 ^ pid >> 0x10 ^ pid & 0xFFF0) < 0x10 {
        is_shiny = true
    }

    let mut ivs = vec![32, 32, 32, 32, 32, 32];
    for i in ivs.iter_mut() {
        *i = rng.rand_max(32);
    }

    let ability_rand = rng.next();
    let ability = ability_rand - (ability_rand / 2) * 2;
    let gender_rand = rng.next();
    let gender = (gender_rand - (gender_rand / 252) * 252) + 1;
    let nature_rand = rng.next();
    let nature = nature_rand - (nature_rand / 25) * 25;

    let encounter_slots: [u8; 12] = [20, 40, 50, 60, 70, 80, 85, 90, 94, 98, 99, 100];

    let encounter = encounter_slots
        .iter()
        .position(|enc| encounter_rand < *enc)
        .unwrap_or(0) as u8;

    Pokemonbdsp {
        is_shiny,
        pid,
        ec,
        nature: enums::NatureEnum::try_from(nature).unwrap_or(enums::NatureEnum::Hardy),
        ivs,
        ability: enums::AbilityEnum::try_from(ability).unwrap_or(enums::AbilityEnum::Ability0),
        gender,
        encounter,
    }
}
