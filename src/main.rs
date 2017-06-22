extern crate randomg;

use randomg::generators::Generator;

struct Man {
    x: i64,
    y: i64,
}

impl Man {
    fn new() -> Man {
        Man {
            x: 0,
            y: 0,
        }
    }

    fn step<T: Generator>(&mut self, gen: &mut T) {
        if randomg::generate_bool(gen) {
            if randomg::generate_bool(gen) {
                self.x += 1;
            } else {
                self.x -= 1;
            }
        } else {
            if randomg::generate_bool(gen) {
                self.y += 1;
            } else {
                self.y -= 1;
            }
        }
    }
}

fn main() {
    let mut gen = randomg::get_generator(randomg::get_seed());

    let mut man = Man::new();
    let mut steps = 0u64;

    man.step(&mut gen);

    while !(man.x == 0 && man.y == 0) {
        man.step(&mut gen);
        steps += 1;
    }

    println!("{}", steps);
}
