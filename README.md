<h1 align="center">
  Singletons
</h1>

<p align="center">
  Type-Keyed Data Structures for Rust
</p>

<p align="center">
<a
  href="https://github.com/FlippingBinaryLLC/singletons-rs/actions?query=branch%3Amain"><img
    alt="Build Status"
    src="https://img.shields.io/github/actions/workflow/status/FlippingBinaryLLC/singletons-rs/ci.yml?branch=main"></a>
<a
  href="https://crates.io/crates/singletons"><img alt="Latest Release on crates.io"
  src="https://img.shields.io/crates/v/singletons.svg"></a>
</p>

<p align="center">
<a href="https://docs.rs/singletons">
  Documentation
</a>
  -
<a href="https://github.com/FlippingBinaryLLC/singletons-rs">
  Repository
</a>
</p>

This crate provides type-keyed data structures that use Rust types as keys,
allowing you to store one value per type.

## Data Structures

### SingletonSet

A `SingletonSet` stores at most one value of each type. Think of it as a
`HashSet` where the type of the value *is* the key.

```rust
use singletons::SingletonSet;

let mut set = SingletonSet::new();

// Insert values of different types
set.insert(42u32);
set.insert("hello");
set.insert(3.14f64);

// Each type has its own slot - inserting again replaces the value
set.insert(100u32);  // Replaces 42

// Retrieve values by type
assert_eq!(set.get::<u32>(), &100);
assert_eq!(set.get::<&str>(), &"hello");

// Check if a type is present
assert!(set.contains::<f64>());
assert!(!set.contains::<i32>());
```

This is useful for creating locally-scoped singletons without polluting the
global scope. It ensures there is only one instance of any type, similar to a
traditional Singleton pattern, but with proper scoping.

### SingletonMap

A `SingletonMap<V>` maps types to values of a single value type `V`. Think of
it as a `HashMap<TypeId, V>` with a more ergonomic API.

```rust
use singletons::SingletonMap;

let mut descriptions: SingletonMap<&str> = SingletonMap::new();

// Map types to descriptions
descriptions.insert::<u8>("An unsigned 8-bit integer");
descriptions.insert::<i8>("A signed 8-bit integer");
descriptions.insert::<String>("A heap-allocated string");

// Retrieve by type
assert_eq!(descriptions.get::<u8>(), Some(&"An unsigned 8-bit integer"));
assert_eq!(descriptions.get::<bool>(), None);

// Use the entry API for conditional insertion
descriptions.entry::<f64>().or_insert("A 64-bit float");
```

This is useful when you need to associate metadata, configuration, or handlers
with specific types.

## Features

- **Type Safety:** Leverages Rust's type system to ensure compile-time safety
  when working with type-keyed collections.
- **Flexible Initialization:** Types implementing `Default` can be
  auto-initialized, or use explicit values and closures for full control.
- **Scoped Singletons:** Unlike global singletons, these collections can be
  scoped as needed, avoiding global state issues.
- **Insertion Order:** Both structures preserve insertion order (backed by
  `IndexMap`).

## Feature Flags

This crate provides two feature flags, both enabled by default:

- `set` - Enables `SingletonSet`
- `map` - Enables `SingletonMap`

## Installation

```sh
cargo add singletons
```

Or add the following to your `Cargo.toml`:

```toml
[dependencies]
singletons = "0.1"
```

## Contributing

Contributions are welcome! Please [open an issue] or submit a pull request if
you have any suggestions, bug reports, or feature requests.

## License

Licensed under either of the [Apache License, Version 2.0][APACHE-2.0] or the
[MIT license][MIT] at your option.

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

[open an issue]: https://github.com/FlippingBinaryLLC/singletons-rs/issues
[APACHE-2.0]: https://www.apache.org/licenses/LICENSE-2.0
[MIT]: https://opensource.org/licenses/MIT
