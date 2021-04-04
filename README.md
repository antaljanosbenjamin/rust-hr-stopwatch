# High resolution stopwatch

![build](https://github.com/antaljanosbenjamin/rust-hr-stopwatch/actions/workflows/build.yml/badge.svg)

This library is a simple stopwatch implementation based on the [time]([time](https://docs.rs/crate/time/0.1.42)) crate. This library can be also found on [crates.io](https://crates.io/crates/hrsw).

To use this library you only have to add to your Cargo.toml as a dependency:

```
[dependencies]
hrsw = "0.1.0"
```

## Example usage
```rust
use hrsw::Stopwatch;
let mut stopwatch = Stopwatch::new();
stopwatch.start();
// do something and get the elapsed time
let elapsed = stopwatch.elapsed();
// do something other and get the total elapsed time
stopwatch.stop();
let total_elapsed = stopwatch.elapsed();
```

## Contributing [![contributions welcome](https://img.shields.io/badge/contributions-welcome-brightgreen.svg?style=flat)](https://github.com/antaljanosbenjamin/rust-hr-stopwatch/issues)

All kinds of contribution is very welcomed, so please feel free to create an issue, a pull request or ask me anything about the library!
