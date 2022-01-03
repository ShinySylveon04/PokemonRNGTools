use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Xorshift {
    state: [u32; 4],
}

impl Xorshift {
    pub fn from_state(state: [u32; 4]) -> Self {
        Self { state }
    }

    pub fn get_state(&self) -> [u32; 4] {
        self.state
    }

    pub fn next(&mut self) -> u32 {
        let s0 = self.state[0];
        self.state[0] = self.state[1];
        self.state[1] = self.state[2];
        self.state[2] = self.state[3];

        let tmp = s0 ^ s0 << 11;
        let tmp = tmp ^ tmp >> 8 ^ self.state[2] ^ self.state[2] >> 19;

        self.state[3] = tmp;

        (tmp % 0xffffffff).wrapping_add(0x80000000)
    }

    pub fn advance(&mut self, advances: usize) {
        for _ in 0..advances {
            self.next();
        }
    }

    pub fn advance_to_state(&mut self, state: [u32; 4]) -> Option<usize> {
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

    pub fn rand_range(&mut self, min: u32, max: u32) -> u32 {
        let s0 = self.state[0];
        self.state[0] = self.state[1];
        self.state[1] = self.state[2];
        self.state[2] = self.state[3];

        let tmp = s0 ^ s0 << 11;
        let tmp = tmp ^ tmp >> 8 ^ self.state[2] ^ self.state[2] >> 19;

        self.state[3] = tmp;

        let diff = max - min;

        (tmp % diff).wrapping_add(min)
    }
}
