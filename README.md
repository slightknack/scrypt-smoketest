# `scrypt` Smoketest
This is just a simple script to find which value of
log₂(`N`) should use for the `scrypt` password hash on your machine.
The paper recommends that each hash take ~100ms -
we simply bench the hash for each value of `N` until we reach
one that takes the recommended time.

## Installation
Assuming you have the [Rust toolchain](https://www.rust-lang.org/tools/install)
installed, simply:

```
$ git clone https://github.com/slightknack/scrypt-smoketest.git
$ cd scrypt-smoketest
$ cargo run --release
Starting smoketest
...
```

The program will terminate after it finds a value of `N`
which takes longer than 100ms/hash to execute.

## So, what *should* `N` be?
I've heard that N should be anywhere from 14 to 20, but I wasn't sure which value
to use in practice.
Of course, these results will be different across machines;
If you'd like to contribute the results of your tests, see the [section below](##Help-Collect-More-Results).

Here are the results for the machine I tested these benchmarks on,
a 2015 MacBook Air with a 1.6 GHz Intel i5:

```
...
i = 13; 100 in 2397275310 ns; 23972753 ns/hash; 23.972753 ms/hash
i = 14; 100 in 4780038600 ns; 47800386 ns/hash; 47.800386 ms/hash
i = 15; 100 in 9491265365 ns; 94912653 ns/hash; 94.912653 ms/hash
i = 16; 100 in 19257962378 ns; 192579623 ns/hash; 192.579623 ms/hash
Smoketested up to i = 16, which took longer that 100ms/hash
```

Based on these results, I recommend you
use `N = 15` **minimum**, preferably higher.
This is an older machine, so I doubt these results are fully relevant.

## Help collect more results
If you'd like to clone this repo and run this smoketest on your machine
(just make sure to `--release` it, y'all),
I'd love to make a nice table or something.
Just open an issue with the title
`Smoketest on <Computer Model>` containing your results.

Thanks and have a wonderful day!  
**Isaac Clayton**
