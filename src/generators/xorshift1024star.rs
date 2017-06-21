use super::{Generator, SplitMix64};

pub struct Xorshift1024Star {
    state: [u64; 16],
    pointer: usize,
}

impl Xorshift1024Star {
    pub fn new(seed: u64) -> Xorshift1024Star {
        let mut gen = SplitMix64::new(seed);
        let mut state = [0u64; 16];

        for i in 0..16 {
            state[i] = gen.next();
        }

        Xorshift1024Star {
            state: state,
            pointer: 0,
        }
    }
}

impl Generator for Xorshift1024Star {
    fn next(&mut self) -> u64 {
        let state0 = self.state[self.pointer];
        self.pointer = (self.pointer + 1) & 0b1111;
        let mut state1 = self.state[self.pointer];

        state1 ^= state1 << 31;
        self.state[self.pointer] =
            state1 ^ state0 ^ (state1 >> 11) ^ (state0 >> 30);

        self.state[self.pointer].overflowing_mul(1181783497276652981).0
    }
}

impl Xorshift1024Star {
    pub fn jump(&mut self) {
        let JUMP = [
            0x84242f96eca9c41d, 0xa3c65b8776f96855,
            0x5b34a39f070b5837, 0x4489affce4f31a1e,
            0x2ffeeb0a48316f40, 0xdc2d9891fe68c022,
            0x3659132bb12fea70,	0xaac17d8efa43cab8,
            0xc4cb815590989b13, 0x5ee975283d71c93b,
            0x691548c86c1bd540, 0x7910c41d10a1e6a5,
            0x0b5fc64563b3e2a8,	0x047f7684e9fc949d,
            0xb99181f2d8f685ca, 0x284600e3f30e38c3,
        ];
        let mut t: [u64; 16] = [0; 16];

        for i in 0..2 {
            for j in 0..64 {
                if (JUMP[i] & 1 << j) != 0 {
                    for k in 0..16 {
                        t[k] ^= self.state[(k + self.pointer) & 0b1111];
                    }
                }
                self.next();
            }
        }

        for i in 0..16 {
            self.state[(i + self.pointer) & 0b1111] = t[i];
        }
    }
}
