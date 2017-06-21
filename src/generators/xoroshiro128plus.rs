use super::{Generator, SplitMix64};

fn rotl(x: u64, k: i32) -> u64 {
    (x << k) | (x >> (64 - k))
}

/// Xoroshiro128+ (xor-rotate-shift-rotate) generator struct, as
/// described and implemented in C here: http://xoroshiro.di.unimi.it/
pub struct Xoroshiro128Plus {
    state: [u64; 2],
}

impl Xoroshiro128Plus {
    pub fn new(seed: u64) -> Xoroshiro128Plus {
        let mut sm64g = SplitMix64::new(seed);

        Xoroshiro128Plus {
            state: [sm64g.next(), sm64g.next()],
        }
    }
}

impl Generator for Xoroshiro128Plus {
    fn next(&mut self) -> u64 {
        let state0 = self.state[0];
        let mut state1 = self.state[1];
        let result = state0.overflowing_add(state1).0;

        state1 ^= state0;
        self.state[0] = rotl(state0, 55) ^ state1 ^ (state1 << 14);
        self.state[1] = rotl(state1, 36);

        result
    }
}

impl Xoroshiro128Plus {
    /// Xoroshiro128+ additionally implements the `jump()` method.
    /// This method's behaviour is identical to calling `next()`
    /// `2^64` times. Note, however, that it doesn't return anything,
    /// it's just a 'skip forward' method.
    pub fn jump(&mut self) {
        let JUMP: [u64; 2] = [0xbeac0467eba5facb, 0xd86b048b86aa9922];

        let mut s0 = 0;
        let mut s1 = 0;

        for i in 0..2 {
            for j in 0..64 {
                if (JUMP[i] & 1 << j) != 0 {
                    s0 ^= self.state[0];
                    s1 ^= self.state[1];
                }
            }
            self.next();
        }

        self.state[0] = s0;
        self.state[1] = s1;
    }
}
