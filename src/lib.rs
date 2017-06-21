pub mod generators;
mod tests;

use std::thread;
use generators::Generator;

/// Returns the xoroshiro128+ generator, seeded with `seed`.
pub fn get_generator(seed: u64) -> generators::Xoroshiro128Plus {
    generators::Xoroshiro128Plus::new(seed)
}

/// Generate one pseudorandom, unsigned 64-bit integer
/// from the generator. Note that it has the side-effect of
/// altering the generator's state!
pub fn generate<T: Generator>(generator: &mut T) -> u64 {
    generator.next()
}

fn to_bytes(long: u64) -> [u8; 8] {
    let mut res = [0; 8];

    for i in 0..8 {
        res[i] = (long >> (64 - ((i + 1) * 8))) as u8;
    }

    res
}

/// Generate `num` pseudorandom bytes from `generator`,
/// wrapped up in a Vec<u8>.
pub fn generate_bytes<T: Generator>(num: usize, generator: &mut T) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(num);

    while bytes.len() < num {
        let random = generator.next();
        for byte in to_bytes(random).into_iter() {
            if bytes.len() < num {
                bytes.push(*byte);
            } else {
                break;
            }
        }
    }

    bytes
}

/// Generate `length` pseudorandom bytes from `generator`,
/// and then collect them into a `String` using `String::from_utf8_lossy()`,
/// meaning it may not be valid UTF-8.
pub fn generate_string<T: Generator>(length: usize,
                                     generator: &mut T)
                                    -> String {

    let bytes = generate_bytes(length, generator);

    (*String::from_utf8_lossy(&bytes)).to_owned()
}    

/// Generate `num` pseudorandom, unsigned 64-bit integers from the generator,
/// collecting them into a `Vec<u64>`. This method is parallelized, and at the moment
/// slower than the sequential version... :(
pub fn generate_many<T: Generator>(num: usize, generator: &mut T) -> Vec<u64> {
    let nthreads = 4;
    let thread_load = num / nthreads;
    let mut threads = Vec::with_capacity(nthreads);
    let mut result = Vec::with_capacity(num);

    for _ in 0..nthreads {
        let mut thread_gen = get_generator(generator.next());
        threads.push(thread::spawn(move || {
            let mut thread_results = Vec::new();
            while thread_results.len() < thread_load {
                thread_results.push(thread_gen.next());
            }

            thread_results
        }));
    }

    for t in threads {
        result.append(&mut t.join().unwrap());
    }

    result
}

/// `generate_many` but without the use of threads; strictly sequential
pub fn generate_many_np<T: Generator>(num: usize, generator: &mut T) -> Vec<u64> {
    let mut result = Vec::with_capacity(num);

    for _ in 0..num {
        result.push(generator.next());
    }

    result
}
