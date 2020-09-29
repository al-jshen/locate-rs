# locate-rs
quick lookup of files based on pre-constructed cache


## Optimizations todo list

- identify candidate lines to match using non-prefix/inner literals from regex pattern before running regex pattern on line (see [this](https://blog.burntsushi.net/ripgrep/) and [this](https://github.com/BurntSushi/ripgrep/issues/162) (`grep` crate))
- [compression](https://dl.acm.org/doi/pdf/10.1145/348751.348754)
- parallelized caching (see [this](https://docs.rs/ignore/0.4.16/ignore/))
- ~~replace regex engine with [Teddy](https://github.com/jneem/teddy) (SIMD accelerated)~~ SIMD is already enabled in the Rust regex crate
