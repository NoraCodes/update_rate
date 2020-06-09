# update_rate

[![Build Status](https://travis-ci.org/NoraCodes/update_rate.svg?branch=master)](https://travis-ci.org/NoraCodes/update_rate)
[![docs.rs documented](https://docs.rs/update_rate/badge.svg)](https://docs.rs/update_rate/)
[![available on crates.io](https://img.shields.io/crates/v/update_rate.svg)](https://crates.io/crates/update_rate/)

A generic, low-overhead rate counter for frames-per-second indicators,
measurement averaging, and more.

```rust
use update_rate::{RateCounter, RollingRateCounter};

let mut c = RollingRateCounter::new(10);

loop {
    c.update();
    mycrate.work(); // Perform the slow operation
    println!("Updating at {}", c); 
}
```

