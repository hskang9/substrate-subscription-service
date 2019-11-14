# Substrate-Alarm-Clock

Ethereum-alarm-clock in substrate runtime.

Vitalik says it is valuable [here](https://chronologic.network/).

[![Chronologic](http://img.youtube.com/vi/SSJUIHrkWhY/0.jpg)](https://www.youtube.com/watch?v=SSJUIHrkWhY "Audi R8")

## Future plans
- integrate this with parity ink!. need to look how it works though.

Difference from EAC is that we count time as block, assuming a block is generally finalized in 6 seconds.

Check out the [HOWTO](HOWTO.md) to learn how to use this for your own runtime module.

This README should act as a general template for distributing your module to others.

## Purpose

This module acts as a template for building other runtime modules.

It currently allows a user to put a `u32` value into storage, which triggers a runtime event.

## Dependencies

### Traits

This module does not depend on any externally defined traits.

### Modules

This module does not depend on any other SRML or externally developed modules.

## Installation

### Runtime `Cargo.toml`

To add this module to your runtime, simply include the following to your runtime's `Cargo.toml` file:

```rust
[dependencies.substrate-module-template]
default_features = false
git = 'https://github.com/substrate-developer-hub/substrate-module-template.git'
```

and update your runtime's `std` feature to include this module:

```rust
std = [
    ...
    'example_module/std',
]
```

### Runtime `lib.rs`

You should implement it's trait like so:

```rust
/// Used for the module test_module
impl substrate_module_template::Trait for Runtime {
	type Event = Event;
}
```

and include it in your `construct_runtime!` macro:

```rust
ExampleModule: substrate_module_template::{Module, Call, Storage, Event<T>},
```

### Genesis Configuration

This template module does not have any genesis configuration.

## Reference Docs

You can view the reference docs for this module by running:

```
cargo doc --open
```

or by visiting this site: <Add Your Link>
