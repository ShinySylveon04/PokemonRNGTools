use std::convert::TryInto;

use crate::bdsp::tid::settings::Settings;
use crate::enums;
use crate::rng::Xorshift;
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
    pub filter_type: enums::IDFilter,
}

pub fn generate_tid(settings: Settings) -> Vec<Result> {
    let states: [u32; 4] = settings.rng_state[0..4].try_into().unwrap_or_default();
    let mut rng = Xorshift::from_state(states);
    rng.advance(settings.min);
    let mut results: Vec<Result> = Vec::new();
    let values = settings.min..=settings.max;
    for value in values {
        let generate_result = calculate_tid(rng, &settings);

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
                filter_type: settings.filter_type,
            };
            results.push(result);
        }
        rng.next();
    }

    results.into_iter().collect()
}

pub fn calculate_tid(mut rng: Xorshift, settings: &Settings) -> Option<Tid> {
    let sidtid = rng.next();
    let tid = sidtid & 0xFFFF;

    if settings.filter_type == enums::IDFilter::TID && settings.id.iter().all(|id| *id != tid) {
        return None;
    };

    let sid = sidtid >> 0x10;

    if settings.filter_type == enums::IDFilter::SID && settings.id.iter().all(|id| *id != sid) {
        return None;
    };

    let tsv = ((tid ^ sid) >> 4) as u32;

    if settings.filter_type == enums::IDFilter::TSV && settings.id.iter().all(|id| *id != tsv) {
        return None;
    };

    let g8tid = sidtid % 1000000;

    if settings.filter_type == enums::IDFilter::G8TID && settings.id.iter().all(|id| *id != g8tid) {
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
            id: vec![0],
            filter_type: enums::IDFilter::None,
            max: 10,
            min: 0,
            rng_state: vec![0x12345678, 0x12345678, 0x12345678, 0x12345678],
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
            let result = calculate_tid(rng, &settings);

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
            id: vec![1234],
            filter_type: enums::IDFilter::TSV,
            max: 10000,
            min: 0,
            rng_state: vec![0x12345678, 0x12345678, 0x12345678, 0x12345678],
        };

        let expected_results = vec![
            Result {
                state0: 3705715888,
                state1: 1942319532,
                state2: 4253276561,
                state3: 3177154437,
                advances: 2672,
                filter_type: enums::IDFilter::TSV,
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
                filter_type: enums::IDFilter::TSV,
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
