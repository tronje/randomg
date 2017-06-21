# randOMG - a pseudorandom number generator
A quick-and-easy pseudorandom number generator, using the
`xoroshiro128+` algorithm and additionally using the `splitmix64` algorithm
to seed it.
Please note: pseudorandom numbers are not cryptographically secure.
*Do not* use this for anything crypto-related!

Algorithms described [here](http://xoroshiro.di.unimi.it/).

Implementations in C:
[`xoroshiro128+`](http://xoroshiro.di.unimi.it/xoroshiro128plus.c)
[`splitmix64`](http://xoroshiro.di.unimi.it/splitmix64.c)

Thanks to David Blackman and Sebastiano Vigna!
