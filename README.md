# randOMG - a pseudorandom number generator
A quick-and-easy pseudorandom number generator library, implementing
`splitmix64`, `xoroshiro128+` and `xorshift1024*` algorithms.

Please note: pseudorandom numbers are not cryptographically secure
and you should *absolutely not* use them in that context.
Use `/dev/random` or `/dev/urandom` instead!

Algorithms described [here](http://xoroshiro.di.unimi.it/).

Implementations in C:
* [`splitmix64`](http://xoroshiro.di.unimi.it/splitmix64.c)
* [`xoroshiro128+`](http://xoroshiro.di.unimi.it/xoroshiro128plus.c)
* [`xorshift1024*`](http://xoroshiro.di.unimi.it/xorshift1024star.c)

Thanks to David Blackman and Sebastiano Vigna!
