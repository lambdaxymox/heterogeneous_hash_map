# Heterogeneous Hash Map Library

## Introduction

The **heterogeneous_hash_map** crate is a library for working with heterogeneous hash maps. 
Heterogeneous hash maps allow one to store multiple unrelated types in the same collection using 
hierarchical two level hashing. The first level of hashing is the key's type identifier, the second 
level of hashing is the key's value identifier. In brief, values are not keyed by just the value of 
the key, but also by the type of the key.

Example use cases for heterogeneous hash maps include:

* Plugin or extension registries: Plugins can store arbitrary config, state, or metadata by type, keyed 
  by a plugin or extension identifier.
* Dependency and resource injection: Register and access various service implementations or resources.
* Entity-component systems: Attach components of various types to the same game entity without predefining 
  a giant sum type for every possible variant.
* Dynamic or runtime-typed data stores: Useful in scripting systems, modding infrastructures, or any 
  context where the set of types isn’t fixed at compile time.
* Serialization/deserialization helpers: Map keys to values of unknown or varying types when processing 
  formats like JSON or user input, especially for dynamic UIs or generic data pipelines.
* Resource management for heterogeneous computing applications.

The goal is to provide **type-safe, ergonomic, and flexible storage** for real-world software systems that 
need to mix value types while maintaining safety and clarity.

## Getting Started

To use this library in your project, add the **heterogeneous_hash_map** crate as a dependency in 
your `Cargo.toml` file

```toml
[dependencies]
heterogeneous_hash_map = "1.2.0"
```

This library has a `nightly` feature to unlock using custom memory allocators for all the 
collections in the library. To use `nightly` add

```toml
[dependencies.heterogeneous_hash_map]
version = "1.2.0"
features = ["nightly"]
```

or

```toml
[dependencies]
heterogeneous_hash_map = { version = "1.2.0", features = ["nightly"] }
```

to your `Cargo.toml` file. Optionally, you can add the crate declaration

```rust
extern crate heterogeneous_hash_map;
```

to your `lib.rs` or `main.rs` file, but this is not strictly necessary.
Alternatively, you can use an explicit crate declaration such as

```rust
extern crate heterogeneous_hash_map as hetmap;
```

to shorten the crate name.

## Testing

To run the tests for the library, run

```text
cargo test --workspace
```

on stable Rust. Run

```text
cargo +nightly test --workspace --features "nightly" 
```

to run the tests with nightly features enabled.

## Features

The primary feature of this library is the `HeterogeneousHashMap` data structure itself. 
It can do the following:

* Store data for multiple, unrelated data types in the same collection.
* Access stored data for any data type stored in its collection again
* Access type metadata for any type stored in the collection for introspective purposes. This is 
  not full-on runtime reflection, but it facilitates building systems where runtime reflection is 
  available. 
* Manipulate and query and information about the collection at the type level.
* Access data for individual types in bulk, since every data type is stored in separate, type-erased 
  storage internally.
* Remove data stored in the collection.
* Remove types stored in the collection.
* Add new types to the collection without necessarily adding values of that type.

In summary, the `HeterogeneousHashMap` data structure is a flexible data store accommodating 
multiple data types at runtime without messing with macros, or boxing everything behind 
something like`Box<dyn Any>`. This library does this with clean, type-safe Rust. In addition, data 
is stored internally using compact type-erased data storage.

## Limitations

This crate does **not** provide any kind of full runtime reflection: it cannot enumerate or access 
all stored types and keys dynamically. The user is responsible for knowing or tracking what types 
exist and may need additional logic if you want to manage dynamic sets of types or keys.

The optional type metadata feature is available to help with tracking, but **the language itself 
does not provide runtime introspection of arbitrary types,** since Rust is a systems language 
focused on compile-time safety, not dynamic type management. This is an inherent limitation when 
working with heterogeneous, type-driven data in Rust.

Additionally, this is not a classic ECS or archetype system: the key and value **type** together 
define uniqueness, so there is no enforced aggregation of “components” under a single ID.