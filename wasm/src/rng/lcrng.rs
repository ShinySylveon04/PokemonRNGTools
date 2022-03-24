use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Lcrng {
    state: u32,
}

impl Lcrng {
    pub fn new(seed: u32) -> Self {
        Self { state: seed }
    }

    pub fn from_state(state: u32) -> Self {
        Self { state }
    }

    pub fn get_state(&self) -> u32 {
        self.state
    }
}

impl Iterator for Lcrng {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        self.state = self.state.wrapping_mul(0x41C64E6D).wrapping_add(0x6073);
        Some(self.state)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::vec;

    #[test]
    fn should_generate_u32s() {
        let mut rng = Lcrng::new(0x12345678);
        let expected_results = vec![
            0x12345678, 0x0B71C18B, 0x84EA22A2, 0xD98A7B6D, 0xF4E023DC, 0x2684AD1F,
        ];

        for (advance, expected_result) in expected_results.iter().enumerate() {
            let result = rng.get_state();

            assert_eq!(&result, expected_result, "Mismatch on advance {}", advance);
            rng.next();
        }
    }

    #[test]
    fn should_work_as_an_iterator() {
        let rng = Lcrng::new(0x12345678);
        let result = rng.skip(100).take(4).collect::<Vec<u32>>();
        assert_eq!(result, [0xa13699ff, 0x2165a406, 0x92e50b01, 0x18a65de0]);
    }
}
