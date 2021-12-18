use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Xoroshiro {
    s0: u64,
    s1: u64,
}

impl Xoroshiro {
    pub fn new(seed: u64) -> Self {
        Self {
            s0: seed,
            s1: 0x82A2B175229D6A5B,
        }
    }

    pub fn new_bdsp(seed: u32) -> Self {
        let mut s0 = u64::from(seed).wrapping_sub(0x61C8864680B583EB);
        let mut s1 = u64::from(seed).wrapping_add(0x3C6EF372FE94F82A);

        s0 = (s0 ^ (s0 >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        s1 = (s1 ^ (s1 >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);

        s0 = (s0 ^ (s0 >> 27)).wrapping_mul(0x94D049BB133111EB);
        s1 = (s1 ^ (s1 >> 27)).wrapping_mul(0x94D049BB133111EB);

        Self {
            s0: s0 ^ (s0 >> 31),
            s1: s1 ^ (s1 >> 31),
        }
    }

    pub fn from_state(s0: u64, s1: u64) -> Self {
        Self { s0, s1 }
    }

    pub fn next_u64(&mut self) -> u64 {
        let result = self.s0.wrapping_add(self.s1);

        self.s1 ^= self.s0;
        self.s0 = self.s0.rotate_left(24) ^ self.s1 ^ (self.s1 << 16);
        self.s1 = self.s1.rotate_left(37);

        result
    }

    pub fn next(&mut self) -> u32 {
        self.next_u64() as u32
    }

    pub fn next_bdsp(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }

    pub fn get_state(&self) -> (u64, u64) {
        (self.s0, self.s1)
    }

    pub fn advance_to_state(&mut self, state: (u64, u64)) -> Option<u64> {
        let mut advances = 0;

        // 10,000 is an arbitary limit to avoid an infinite loop
        while self.get_state() != state {
            self.next();
            advances += 1;

            if advances > 10_000 {
                return None;
            }
        }

        Some(advances)
    }

    fn get_mask(num: u32) -> u32 {
        let mut result = num - 1;

        for i in 0..5 {
            let shift = 1 << i;
            result |= result >> shift;
        }

        result
    }

    pub fn rand_max(&mut self, max: u32) -> u32 {
        let mask = Self::get_mask(max);
        let mut rand = self.next() & mask;

        while max <= rand {
            rand = self.next() & mask;
        }

        rand
    }
}
