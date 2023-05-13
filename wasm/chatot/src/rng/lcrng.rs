#[derive(Clone, Copy, Debug)]
pub struct Lcrng {
    state: u32,
}

impl Lcrng {
    pub fn new(state: u32) -> Self {
        Self { state }
    }

    pub fn get_state(&self) -> u32 {
        self.state
    }

    pub fn next_u32(&mut self) -> u32 {
        self.state = self.state.wrapping_mul(0x41C64E6D).wrapping_add(0x6073);
        self.state
    }

    pub fn next_u16(&mut self) -> u16 {
        (self.next_u32() >> 16) as u16
    }

    pub fn advance(&mut self, advances: usize) {
        for _ in 0..advances {
            self.next();
        }
    }
}

impl Iterator for Lcrng {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        Some(self.next_u32())
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

    #[test]
    fn should_work() {
        let rng = Lcrng::new(0);
        let result = rng.take(4).collect::<Vec<u32>>();
        let expected_result = [0x6073, 0xe97e7b6a, 0x52713895, 0x31b0dde4];

        assert_eq!(result, expected_result)
    }
}
