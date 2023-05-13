use super::form_settings::Settings;
use crate::rng::Xorshift;
use chatot_forms::IDFilter;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Tid {
    pub tid: u16,
    pub tsv: u16,
    pub g8tid: u32,
    pub sid: u16,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Result {
    pub state0: u32,
    pub state1: u32,
    pub state2: u32,
    pub state3: u32,
    pub advances: usize,
    pub tid: u16,
    pub tsv: u16,
    pub g8tid: u32,
    pub sid: u16,
    pub filter_type: Option<IDFilter>,
}

pub fn generate_tid(settings: Settings) -> Vec<Result> {
    let ids = settings
        .ids
        .split("\n")
        .map(|id| id.parse::<u32>().unwrap_or_default())
        .collect::<Vec<u32>>();
    let states: [u32; 4] = [
        settings.seed_0,
        settings.seed_1,
        settings.seed_2,
        settings.seed_3,
    ];
    let mut rng = Xorshift::from_state(states);
    rng.advance(settings.min_advances);
    let mut results: Vec<Result> = Vec::new();
    let values = settings.min_advances..=settings.max_advances;
    for value in values {
        let generate_result = calculate_tid(rng, &settings, &ids);

        if let Some(tid) = generate_result {
            let rng_state = rng.get_state();
            let result = Result {
                state0: rng_state[0],
                state1: rng_state[1],
                state2: rng_state[2],
                state3: rng_state[3],
                advances: value,
                tid: tid.tid,
                tsv: tid.tsv,
                g8tid: tid.g8tid,
                sid: tid.sid,
                filter_type: settings.gen8_id_type,
            };
            results.push(result);
        }
        rng.next();
    }

    results.into_iter().collect()
}

pub fn calculate_tid(mut rng: Xorshift, settings: &Settings, ids: &[u32]) -> Option<Tid> {
    let sidtid = rng.next();
    let tid = sidtid & 0xFFFF;

    if settings.gen8_id_type == Some(IDFilter::TID) && ids.iter().all(|id| *id != tid) {
        return None;
    };

    let sid = sidtid >> 0x10;

    if settings.gen8_id_type == Some(IDFilter::SID) && ids.iter().all(|id| *id != sid) {
        return None;
    };

    let tsv = ((tid ^ sid) >> 4) as u32;

    if settings.gen8_id_type == Some(IDFilter::TSV) && ids.iter().all(|id| *id != tsv) {
        return None;
    };

    let g8tid = sidtid % 1000000;

    if settings.gen8_id_type == Some(IDFilter::G8TID) && ids.iter().all(|id| *id != g8tid) {
        return None;
    };

    Some(Tid {
        tid: (tid as u16),
        tsv: (tsv as u16),
        g8tid,
        sid: (sid as u16),
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use std::vec;

    #[test]
    fn should_generate_tids() {
        let settings = Settings {
            ids: "0".to_string(),
            gen8_id_type: None,
            max_advances: 10,
            min_advances: 0,
            seed_0: 0x12345678,
            seed_1: 0x12345678,
            seed_2: 0x12345678,
            seed_3: 0x12345678,
        };
        let mut rng = Xorshift::from_state([0x12345678, 0x12345678, 0x12345678, 0x12345678]);

        let expected_results = vec![
            Tid {
                tid: 17872,
                tsv: 1661,
                g8tid: 639824,
                sid: 8707,
            },
            Tid {
                tid: 16510,
                tsv: 3364,
                g8tid: 897918,
                sid: 37428,
            },
            Tid {
                tid: 21462,
                tsv: 1821,
                g8tid: 643414,
                sid: 8707,
            },
            Tid {
                tid: 22136,
                tsv: 3140,
                g8tid: 903544,
                sid: 37428,
            },
        ];

        for (advance, expected_result) in expected_results.iter().enumerate() {
            let result = calculate_tid(rng, &settings, &[0]);

            assert_eq!(
                result.as_ref(),
                Some(expected_result),
                "Mismatch on advance {}",
                advance
            );
            rng.next();
        }
    }

    #[test]
    fn should_filter_tids() {
        let settings = Settings {
            ids: "1234".to_string(),
            gen8_id_type: Some(IDFilter::TSV),
            max_advances: 10000,
            min_advances: 0,
            seed_0: 0x12345678,
            seed_1: 0x12345678,
            seed_2: 0x12345678,
            seed_3: 0x12345678,
        };

        let expected_results = vec![
            Result {
                state0: 3705715888,
                state1: 1942319532,
                state2: 4253276561,
                state3: 3177154437,
                advances: 2672,
                filter_type: Some(IDFilter::TSV),
                tid: 43426,
                tsv: 1234,
                g8tid: 833890,
                sid: 58499,
            },
            Result {
                state0: 403024301,
                state1: 1125726731,
                state2: 327036423,
                state3: 766138671,
                advances: 7940,
                filter_type: Some(IDFilter::TSV),
                tid: 54774,
                tsv: 1234,
                g8tid: 281846,
                sid: 39127,
            },
        ];

        let results = generate_tid(settings);

        assert_eq!(results.len(), expected_results.len());
        for (advance, (result, expected_result)) in
            results.into_iter().zip(expected_results).enumerate()
        {
            assert_eq!(result, expected_result, "Mismatch on advance {}", advance);
        }
    }
}
