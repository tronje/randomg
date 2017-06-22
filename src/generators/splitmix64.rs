use super::{Generator};

/// splitmix64 generator, as described and implemented
/// in C here: http://xoroshiro.di.unimi.it/
pub struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    pub fn new(seed: u64) -> SplitMix64 {
        SplitMix64 {
            state: seed,
        }
    }
}

impl Generator for SplitMix64 {
    fn next(&mut self) -> u64 {
        self.state = self.state.overflowing_add(0x9E3779B97F4A7C15).0;

        let mut z: u64 = self.state;
        z = (z ^ (z >> 30)).overflowing_mul(0xBF58476D1CE4E5B9).0;
        z = (z ^ (z >> 27)).overflowing_mul(0x94D049BB133111EB).0;

        z ^ (z >> 31)
    }

}
