pub mod generators;
mod tests;

use std::thread;
use generators::Generator;

/// Returns the xoroshiro128+ generator, seeded with `seed`.
pub fn get_generator(seed: u64) -> generators::Xoroshiro128Plus {
    generators::Xoroshiro128Plus::new(seed)
}

/// Generate one pseudo-random, unsigned 64-bit integer
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

/// Generate `length` pseudo-random bytes from `generator`,
/// and then collect them into a `String` using `String::from_utf8_lossy()`,
/// meaning it may not be valid UTF-8.
pub fn generate_string<T: Generator>(length: usize,
                                     generator: &mut T)
                                    -> String {

    // generate length divided by the number of bytes in a u64,
    // and one extra in case it's not a clean division.
    let to_generate = (length / 8) + 1;
    let mut randoms = Vec::with_capacity(to_generate);
    let mut bytes: Vec<u8> = Vec::with_capacity(length);

    for _ in 0..to_generate {
        randoms.push(generator.next());
    }

    for elem in randoms {
        for byte in to_bytes(elem).into_iter() {
            if bytes.len() < length {
                bytes.push(*byte);
            } else {
                break;
            }
        }
    }

    (*String::from_utf8_lossy(&bytes)).to_owned()
}    

/// Generate `num` pseudo-random, unsigned 64-bit integers from the generator,
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
