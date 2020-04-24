# summon

[![Crates.io][ci]][cl] ![MIT/Apache][li] [![docs.rs][di]][dl] ![LoC][lo] ![Tests][btl] ![Lints][bll]

[ci]: https://img.shields.io/crates/v/summon.svg
[cl]: https://crates.io/crates/summon/

[li]: https://img.shields.io/crates/l/specs.svg?maxAge=2592000

[di]: https://docs.rs/summon/badge.svg
[dl]: https://docs.rs/summon/

[lo]: https://tokei.rs/b1/github/vadixidav/summon?category=code

[btl]: https://github.com/vadixidav/summon/workflows/unit%20tests/badge.svg
[bll]: https://github.com/vadixidav/summon/workflows/lints/badge.svg

A logic engine designed to magically give you what you ask for

Nightly is required because the code does some questionable things.

Do you want to study the dark arts? If so, then read the code, but beware!

## Example

```rust
#![feature(const_type_id)]
use summon::{Tome, circle};

#[derive(Clone)]
struct ConstantAcceleration(f64);
#[derive(Clone)]
struct InitialVelocity(f64);
#[derive(Clone)]
struct InitialPosition(f64);
#[derive(Clone)]
struct Time(f64);

#[derive(Debug)]
struct Distance(f64);

// The tome is where all the logic and conversions are written in your code.
let mut tome = Tome::new();

// You can use ether() to give types as givens.
tome.ether(ConstantAcceleration(3.0));
tome.ether(InitialVelocity(5.0));
tome.ether(InitialPosition(6.0));
tome.ether(Time(4.0));

// Inscribe is used to describe a conversion between types.
// Several macros are provided for convenience.
// This one lets you destructure and type construct freely.
tome.inscribe(
    circle!(ConstantAcceleration(a), InitialVelocity(v), InitialPosition(p), Time(t) => Distance(0.5 * a * t.powi(2) + v * t + p))
);

// So long as it is possible to produce the result with the given inscriptions, it will be produced.
let summoned = tome.summon::<Distance>().unwrap().0;
assert_eq!(
    0.5 * 3.0 * 4.0f64.powi(2) + 5.0 * 4.0 + 6.0,
    summoned,
);
```

### Code Disclaimer

Rust was not meant to do this. Turn back now!
The code is known to induce illness to the reader if they are a software engineer. Please proceed with caution.
I am not responsible if reading of the code causes chronic facial convulsions and/or death.
The reader must proceed at their own risk. You have been warned!