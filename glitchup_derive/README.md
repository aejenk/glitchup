# `glitchup_derive`

[![glitchup_derive](https://img.shields.io/crates/v/glitchup_derive.svg)](https://crates.io/crates/glitchup_derive)
[![glitchup_derive](https://docs.rs/glitchup_derive/badge.svg)](https://docs.rs/crate/glitchup_derive)

A group of procedural macros to assist in the use of `glitchup`. Check any updates on [the changelog.](CHANGELOG.md)

## `#[derive(MutConfig)]`
This derivation macro is used to derive `MutConfig` for any compatible struct. For a struct to be compatible, the following must apply:

- The name of the struct must contain `Config`.
- The fields of the struct must be of the following types:
  - **Primitives**
    - `isize`
    - `String`
    - `bool`
  - **Generics**
    - `Vec<{Supported Type (incl. Generic)}>`
    - `Option<{Supported Type (incl. Generic)}>`

These specific primitives were selected due to the `MutOptionVal` using said values. 

The `MutConfig` trait implements a `to_hashmap` function, where the fields of the struct are converted into a `HashMap<String, MutOptionVal>` to be used by a `Mutation`.

```rust
#[derive(Debug, Deserialize, MutConfig)]
struct MainConfig {
    mutation : MutationConfig,
    mutations : Vec<Vec<String>> // works!
    mapint   : Vec<Vec<u8>> // fails!
}

#[derive(Debug, Deserialize, MutConfig)]
struct MutationConfig {
    min : Option<isize>,
    max : Option<isize>,
    chunksize : isize,
}
```

In `MainConfig` above, `mapint` would fail, since it uses the incompatible type `u8`. However, we **need** it in order for our application to work. Let's assume that no `Mutation` will use it. Therefore, what we can do is add the `#[ignore]` attribute:

```rust
#[derive(Debug, Deserialize, MutConfig)]
struct MainConfig {
    mutation : MutationConfig,
    mutations : Vec<Vec<String>> // works!
    #[ignore]
    mapint   : Vec<Vec<u8>> // all ok now!
}
...
```

Any field tagged with `#[ignore]` will not be included in `to_hashmap(...)`.


