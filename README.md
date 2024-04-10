[![Crates.io](https://img.shields.io/crates/v/rhai-chrono?color=4d76ae)](https://crates.io/crates/rhai-chrono)
[![API](https://docs.rs/rhai-chrono/badge.svg)](https://docs.rs/rhai-chrono)
[![dependency status](https://deps.rs/repo/github/iganev/rhai-chrono/status.svg)](https://deps.rs/repo/github/iganev/rhai-chrono)
[![build and test](https://github.com/iganev/rhai-chrono/actions/workflows/rust.yml/badge.svg)](https://github.com/iganev/rhai-chrono/actions/workflows/rust.yml)
[![codecov](https://codecov.io/github/iganev/rhai-chrono/graph/badge.svg?token=B5P2TAV5BB)](https://codecov.io/github/iganev/rhai-chrono)


# rhai-chrono
[chrono](https://github.com/chronotope/chrono) [DateTime](https://docs.rs/chrono/latest/chrono/struct.DateTime.html) and [TimeDelta](https://docs.rs/chrono/latest/chrono/struct.TimeDelta.html) package for [rhai](https://github.com/rhaiscript/rhai/tree/main)

## Quick Start

Developed and tested with rhai v1.17, chrono v0.4.37 and chrono-tz v0.9.

### Include

Add to `Cargo.toml`:
```toml
rhai = { version = "^1.15" } # in case you've missed it
rhai-chrono = { version = "^0" }
```

### Registration

Include package:
```rust
use rhai::Engine;
use rhai_chrono::ChronoPackage;
```

Register package:
```rust
let mut engine = Engine::new();

let package = ChronoPackage::new();
package.register_into_engine(&mut engine);
```

### Behavior

The package exposes two wrapper types `DateTimeFixed` (wrapping `chrono::DateTime<FixedOffset>`) and `Timedelta` (wrapping `chrono::TimeDelta`).

Each of the two wrapper types can be initialized in a variety of ways using disctinct contructor functions.

Once initialized, the user can call methods and get / set properties on the wrapper.

Certain methods yield output values, either a number, string or boolean.

Many of the methods and properties have aliases or variants for convenience.

All methods and properties follow (but with slight nuance) the way you'd normally use `chrono::DateTime` and `chrono::TimeDelta`.

## API

### DateTime

#### Constructors

#### Setters

#### Getters

#### Methods

### TimeDelta

#### Constructors

#### Setters

#### Getters

#### Methods

## Examples

TODO

## License

This library (rhai-chrono) is open sourced under the BSD 2 License.