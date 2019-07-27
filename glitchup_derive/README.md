# `glitchup_derive`

[![glitchup_derive](https://img.shields.io/crates/v/glitchup_derive.svg)](https://crates.io/crates/glitchup_derive)
[![glitchup_derive](https://docs.rs/glitchup_derive/badge.svg)](https://docs.rs/crate/glitchup_derive)

A group of procedural macros to assist in the use of `glitchup`.

## `#[derive(MutConfig)]`
This derivation macro is used to derive `MutConfig` for any compatible struct. For a struct to be compatible, the following must apply:

- The name of the struct must contain `Config`.
- The fields of the struct must be of the following types:
  - **Primitives**
    - `isize`
    - `String`
    - `bool`
  - **Generics**
    - `Vec<{Supported Primitive}>`
    - `Option<{Supported Primitive}>`

These specific primitives were selected due to the `MutOptionVal` using said values. 

The `MutConfig` trait implements a `to_hashmap` function, where the fields of the struct are converted into a `HashMap<String, MutOptionVal>` to be used by a `Mutation`.

```rust
#[derive(Debug, Deserialize, MutConfig)]
struct MainConfig {
    mutation : MutationConfig,
}

#[derive(Debug, Deserialize, MutConfig)]
struct MutationConfig {
    min : Option<isize>,
    max : Option<isize>,
    chunksize : isize,
}
```
