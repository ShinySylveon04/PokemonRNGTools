use crate::rng::Xorshift;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TID {
    pub tid: u16,
    pub tsv: u16,
    pub g8tid: u32,
    pub sid: u16,
}

pub fn generate_tid(mut rng: Xorshift) -> TID {
    let sidtid = rng.next();
    let tid = sidtid & 0xFFFF;
    let sid = sidtid >> 0x10;

    let tsv = ((tid ^ sid) >> 4) as u16;
    let g8tid = sidtid % 1000000;

    TID {
        tid: (tid as u16),
        tsv,
        g8tid,
        sid: (sid as u16),
    }
}
