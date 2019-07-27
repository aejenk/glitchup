//! A databender library to create your very own databender!
//! 
//! The name `glitchconsole` was inspired by a `dev console`, 
//! where people can break a game using its ingame console.
//! This crate facilitates creating a *console* of sorts 
//! that lets you/others glitch any readable file.
//! 
//! - `mutation` declares what a `Mutation` is.
//! - `loaders` facilitate loading/memory-mapping a file.
//! - `options` facilitate loading files from a `TOML` file, 
//!     and serialising them into their own struct.

/// Declares the `Mutation` trait.
pub mod mutation;

/// Facilitate loading and memory-mapping a file.
pub mod loaders;

/// Facilitate loading options and using configurations.
pub mod options;