use super::settings::Settings;
use crate::enums::HiddenPower;
use crate::rng::MT;
use serde::Deserialize;
use serde::Serialize;
use std::convert::TryFrom;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Pokemon {
    pub pid: u32,
    pub ivs: Vec<u32>,
    pub psv: u32,
    pub hidden_power: HiddenPower,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Result {
    pub advances: usize,
    pub pid: u32,
    pub ivs: Vec<u32>,
    pub psv: u32,
    pub hidden_power: HiddenPower,
}

type IVs = Vec<u32>;
fn check_ivs(ivs: &IVs, min_ivs: &IVs, max_ivs: &IVs) -> bool {
    ivs.iter()
        .zip(min_ivs.iter())
        .zip(max_ivs.iter())
        .all(|((iv, min), max)| min <= iv && iv <= max)
}

pub fn generate_pokemon(rng: &mut MT, settings: &Settings) -> Option<Pokemon> {
    let _ec = rng.next();
    let mut pid = rng.next();
    let mut psv = ((pid >> 16) ^ (pid & 0xffff)) >> 4;
    let tsv = settings.tid >> 4;

    if settings.is_shiny {
        let pid_low = pid & 0xffff;
        pid = ((settings.tid ^ pid_low) << 16) | pid_low;
        psv = (pid >> 16 ^ pid_low) >> 4;
    } else {
        if psv == tsv {
            pid = pid ^ 0x10000000
        }
    }

    let mut ivs = vec![32, 32, 32, 32, 32, 32];

    let iv_rolls = if settings.iv_rolls { 5 } else { 3 };

    for _ in 0..iv_rolls {
        let mut index: usize;
        loop {
            index = (rng.rand_max(6)) as usize;
            if ivs[index] == 32 {
                break;
            }
        }
        ivs[index] = 31;
    }

    for i in ivs.iter_mut() {
        if *i == 32 {
            *i = rng.rand_max(32)
        };
    }

    if !check_ivs(&ivs, &settings.min_ivs, &settings.max_ivs) {
        return None;
    }

    let hidden_power = {
        ((((ivs[0] & 1)
            + ((ivs[1] & 1) << 1)
            + ((ivs[2] & 1) << 2)
            + ((ivs[3] & 1) << 3)
            + ((ivs[4] & 1) << 4)
            + ((ivs[5] & 1) << 5)) as u16
            * 15) as u16
            / 63) as u8
    };

    Some(Pokemon {
        pid,
        ivs,
        psv,
        hidden_power: HiddenPower::try_from(hidden_power as u8).unwrap_or(HiddenPower::Invalid),
    })
}

pub fn generate_transporter(settings: Settings) -> Vec<Result> {
    let mut rng = MT::new(settings.rng_state);
    rng.advance(settings.min_advances);
    rng.advance(settings.delay);
    let mut results: Vec<Result> = Vec::new();
    let values = settings.min_advances..=settings.max_advances;

    for value in values {
        let mut rng_clone = rng.clone();
        let generate_result = generate_pokemon(&mut rng_clone, &settings);
        if let Some(pokemon) = generate_result {
            let result = Result {
                advances: value,
                pid: pokemon.pid,
                ivs: pokemon.ivs,
                psv: pokemon.psv,
                hidden_power: pokemon.hidden_power,
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

    #[test]
    fn should_generate_pokemon() {
        let mut rng = MT::new(0x9ae265ea);
        rng.advance(865);
        // Constant delay with patch (you're welcome :p)
        rng.advance(28);

        let settings = Settings {
            rng_state: 0x9ae265ea,
            delay: 28,
            min_advances: 0,
            max_advances: 1000,
            min_ivs: vec![0, 0, 0, 0, 0, 0],
            max_ivs: vec![31, 31, 31, 31, 31, 31],
            iv_rolls: false,
            is_shiny: false,
            tid: 0,
        };

        let result = generate_pokemon(&mut rng, &settings);

        let expected_result = Pokemon {
            pid: 0x0250eff9,
            ivs: vec![6, 31, 31, 6, 31, 31],
            psv: 3802,
            hidden_power: HiddenPower::Psychic,
        };
        assert_eq!(result, Some(expected_result))
    }

    #[test]
    fn should_generate_shiny_pokemon() {
        let mut rng = MT::new(0xaea136ac);
        rng.advance(2317);
        // Constant delay with patch (you're welcome :p)
        rng.advance(28);

        let settings = Settings {
            rng_state: 0xaea136ac,
            delay: 28,
            min_advances: 0,
            max_advances: 1000,
            min_ivs: vec![0, 0, 0, 0, 0, 0],
            max_ivs: vec![31, 31, 31, 31, 31, 31],
            iv_rolls: false,
            is_shiny: true,
            tid: 14979,
        };

        let result = generate_pokemon(&mut rng, &settings);

        let expected_result = Pokemon {
            pid: 0xb1e08b63,
            ivs: vec![31, 6, 31, 31, 7, 19],
            psv: 0936,
            hidden_power: HiddenPower::Dragon,
        };
        assert_eq!(result, Some(expected_result))
    }
}
