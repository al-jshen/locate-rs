# locate-rs
quick lookup of files based on pre-constructed cache


## Optimizations todo list

- identify candidate lines to match using non-prefix/inner literals from regex pattern before running regex pattern on line (see [this](https://blog.burntsushi.net/ripgrep/))
- [compression](https://dl.acm.org/doi/pdf/10.1145/348751.348754)
