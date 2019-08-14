<h1 align="center">glitchup</h1>

<div align="center">
  <a href="https://travis-ci.org/Calmynt/glitchup">
    <img src="https://travis-ci.org/Calmynt/glitchup.svg?branch=master" alt="Build Status">
  </a> 
  |
  <a href="https://crates.io/crates/glitchup">
    <img src="https://img.shields.io/crates/v/glitchup.svg" alt="Glitchup crate">
  </a> 
  |
   <a href="https://docs.rs/crate/glitchup">
    <img src="https://docs.rs/glitchup/badge.svg" alt="Glitchup docs">
  </a>
</div>

<!--
[![Build status](https://travis-ci.org/Calmynt/glitchup.svg?branch=master)](https://travis-ci.org/Calmynt/glitchup)
[![glitchup](https://img.shields.io/crates/v/glitchup.svg)](https://crates.io/crates/glitchup)
[![glitchup](https://docs.rs/glitchup/badge.svg)](https://docs.rs/crate/glitchup)
-->
A databender library/executable made in Rust. Comes with option loading, and proc macros to help development.

- [glitchup](#glitchup)
  - [Some Q&A](#Some-QA)
    - [What's a databender?](#Whats-a-databender)
    - [Why library and executable?](#Why-library-and-executable)
    - [What's in the library?](#Whats-in-the-library)
    - [Why memory mapping?](#Why-memory-mapping)
    - [What if I want to make my own Loader or Options?](#What-if-I-want-to-make-my-own-Loader-or-Options)
    - [So where's the program?](#So-wheres-the-program)

## Some Q&A

### What's a databender?

A *databender* is a program that **databends**. So what's databending?

It consists of modifying parts of a file *(preferrably binary, but it's up to you)*, and seeing what pops up. This has the effect of corrupting, or causing glitch effects to appear - which is something that is not only fun, but can produce some very interesting results.

### Why library and executable?

I wanted to split them up because I felt other people should be able to make their own versions with ease. They could contribute to the program here, but if they'd like to make their own local version then I want to make that as easy as possible. This way, if I want my executable to be a CLI tool, and someone wants to make one with a GUI, then they would be able to do so.

### What's in the library?

So far there are three main parts - `loader`, `options`, and `mutation`.

`mutation` is the simplest. It simply defines a `Mutation` trait.

`options` provides tools to process options easily. For now, it contains a `TomlProcessor`, which serialises a `TOML` file into your own structs to store the configuration in. It also defines a `MutConfig` trait, used to define configurations that a `Mutation` can use. If you do not want to implement it by yourself, you can use [`#[derive(MutConfig)]`](glitchup_derive), offered by the `glitchup_derive` crate. Be sure to check its `README` and `CHANGELOG` to see how it works, or what updates happened.

`loader` facilitates copying files, memory mapping a file for mutation, and reading a file into a string.

### Why memory mapping?

The current expected mutation should work by copying a file, memory mapping the copy, and then mutating it. This avoids the dual cumbersome process of both `loading` and `saving`, but has the significantly more interesting benefit that there is no memory limitation. 

If you want to load a `10GB` file, not only are waiting times cut in half, but those `10GB` are not all loaded into memory, only the parts that are being mutated.

### What if I want to make my own Loader or Options?

Of course, you do not need to use them, you can make your own. I just thought of predefining a structure/design with some tools to help developing. I left them in the library in the case that someone wants to just focus on databending. If you feel they are too limiting, *feel welcome to open an issue!* I'd be happy to look into implementing it. 

### So where's the program?

The current program can be found as `glitchup.exe`. It is quite unrefined, and fair to say is still in its *beta* stage, but it contains support for the following mutation names:

<center> Void, Chaos, Loops, Reverse, Shift, Shuffle, Swap, Increase, Gradient </center>

The following mutations are planned, but not finished yet:

<center> Echo, Anti, Smear, Sort, Magnify, Reflect, Handshake, Speedup, Slowdown </center>

Note that you will need the `Options.toml` configuration file for the program to work. There is an example of such in this repository as well.

You may encounter some **panics**. Some may instruct you to contact me. If a panic doesn't say this message, feel free to ask and I'll try to help out - but if it does, then that means that, _congratulations_ you have found a bug!
