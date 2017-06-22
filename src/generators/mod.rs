mod splitmix64;
mod xoroshiro128plus;
mod xorshift1024star;

pub use self::splitmix64::SplitMix64;
pub use self::xoroshiro128plus::Xoroshiro128Plus;
pub use self::xorshift1024star::Xorshift1024Star;

/// All a generator needs to be able to do is
/// generate a `next` value; therefore this trait only
/// requires a single method, `next()`.
pub trait Generator {
    /// The `next` method of a generator generates the 'next'
    /// value; this means a value is computed from its state,
    /// and the state is altered to allow a future call to `next()`
    /// to return a different value, and so on.
    fn next(&mut self) -> u64;
}
