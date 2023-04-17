use super::settings::Settings;
use crate::enums;
use crate::rng::MT;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Pokemon {
    pub ec: u32,
    pub pid: u32,
    pub ivs: Vec<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Result {
    pub advances: usize,
    pub pid: u32,
    pub ivs: Vec<u32>,
}

type IVs = Vec<u32>;
fn check_ivs(ivs: &IVs, min_ivs: &IVs, max_ivs: &IVs) -> bool {
    ivs.iter()
        .zip(min_ivs.iter())
        .zip(max_ivs.iter())
        .all(|((iv, min), max)| min <= iv && iv <= max)
}

pub fn generate_pokemon(rng: &mut MT, settings: &Settings) -> Option<Pokemon> {
    let ec = rng.next();
    let pid = rng.next();

    let mut ivs = vec![32, 32, 32, 32, 32, 32];

    for _ in 0..3 {
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

    Some(Pokemon { ec, pid, ivs })
}

pub fn generate_transporter(settings: Settings) -> Vec<Result> {
    let mut rng = MT::new(settings.rng_state);
    rng.advance(settings.delay);
    let mut results: Vec<Result> = Vec::new();
    let values = settings.min_advances..=settings.max_advances;
    rng.advance(settings.min_advances);

    for value in values {
        let generate_result = generate_pokemon(&mut rng, &settings);
        if let Some(pokemon) = generate_result {
            let result = Result {
                advances: value,
                pid: pokemon.pid,
                ivs: pokemon.ivs,
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
            delay: 27,
            min_advances: 0,
            max_advances: 1000,
            gender_ratio: enums::GenderRatio::Male50Female50,
            min_ivs: vec![0, 0, 0, 0, 0, 0],
            max_ivs: vec![31, 31, 31, 31, 31, 31],
        };

        let result = generate_pokemon(&mut rng, &settings);

        let expected_result = Pokemon {
            ec: 295873325,
            pid: 0x0250eff9,
            ivs: vec![6, 31, 31, 6, 31, 31],
        };
        assert_eq!(result, Some(expected_result))
    }
}
