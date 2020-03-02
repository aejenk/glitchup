<h1 align="center">glitchup</h1>

<div align="center">
<sub>
A databender executable made in Rust. Comes with option loading, and proc macros to help development.
</sub>
</div>

<br/>

<div align="center">
  <a href="https://travis-ci.org/Calmynt/glitchup">
    <img src="https://travis-ci.org/Calmynt/glitchup.svg?branch=master" alt="Build Status">
  </a> 
  <a href="https://crates.io/crates/glitchup">
    <img src="https://img.shields.io/crates/v/glitchup.svg" alt="Glitchup crate">
  </a> 
   <a href="https://docs.rs/crate/glitchup">
    <img src="https://docs.rs/glitchup/badge.svg" alt="Glitchup docs">
  </a>
</div>

<br/>

<!--
[![Build status](https://travis-ci.org/Calmynt/glitchup.svg?branch=master)](https://travis-ci.org/Calmynt/glitchup)
[![glitchup](https://img.shields.io/crates/v/glitchup.svg)](https://crates.io/crates/glitchup)
[![glitchup](https://docs.rs/glitchup/badge.svg)](https://docs.rs/crate/glitchup)
-->

**Note:** This project is still a work in progress. You can install the current executable by using: 

- `cargo install glitchup`
- downloading the program from [here](https://github.com/Calmynt/glitchup/releases)

...however it's incomplete and you may encounter bugs.

If you have any questions, for example *"What's databending?"* or *"Why did you make this?"*, check the [**Q&A**](./Q&A.md)

If you wanna have a headstart on databending, check out the [databending tutorial!](./TUTORIAL.md)

## Options

An example options file can be found at [`Options.toml`](./Options.toml). I'll explain some important parts:

### Input + Output
The `inputfile` option needs to be set in order to specify which file to databend:

```toml
inputfile = "somefile.jpeg"
```

You can also specify a file that's not in the current directory. A file being in subdirectories (`dir/file.jpeg`) was tested, however a file being out of the current directory (`../file.jpeg`) was not. This may work, however some further testing is currently required.

By default, the output file will have the same name as the input file *(along with an appended string)*. If you'd like to save the file with a different name/directory, you can specify the `outputfile` option.

```toml
inputfile = "somefile.jpeg"
outputfile = "otherfile.jpeg"
```

**Note:** The output file's name will not be exactly the same as the name you specified. Currently, the format of the output files name is `name<mutations>.extension`. This is to display what mutations the file underwent, while also avoiding overwriting existing files.

### Global options

Currently there are 4 global options:

- `times`: This specifies how many times to run the program. 
- `iterations`: Specifies how many times the mutation should be performed before saving.
- `chunksize`: The size of each chunk to mutate at a time.
- `mutations`: The mutations to use.

So with the following options:

```toml
times = 2
iterations = [5,10]
chunksize = 1000
mutations = [ ["Reverse"], ["Swap"] ]
```

The program is going to be run **twice**, with each run containing an output for
- A mutation of `1KB` chunks by applying `Reverse` _5 to 10 times_, and
- A mutation of `1KB` chunks by applynig `Swap` _5 to 10 times_

...resulting in **2 * 2** output files in total. *(Times * No. of Mutations)*

#### Ranges

As you've seen above, `iterations` is an `array` of 2 integers. Almost all mutation-specific options can be set up with ranges, meaning an array of 2 numbers:

- `iterations = x`: `x` iterations
- `iterations = [x,y]` : Between `x` and `y` iterations.

A number will be generated randomly between `x` and `y` for each *`time`*. So if you want to randomly generate a setting between two bounds, use `[x,y]`. Otherwise, if
you want your setting to be more concrete, simply use a literal number.

#### Mutations

The `mutations` option has been overhauled from [*BEND++*](https://github.com/Calmynt/BENDPlusPlus)! Now it uses an *array of arrays*

```toml
mutations = [
  ["Reverse", "Swap"],
  ["Shuffle"],
  ...
]
```

Each element of `mutations` is an *array* and represents a single output. This array can have multiple mutations *chained together!*

In the options shown above, it means that the first file will first be mutated by `Reverse`, then by `Swap`, **then** saved. Then a new copy of the original file will be made, mutated by `Shuffle`, **then** saved. And so on...

### Specific options

Some mutations may have their own options that they require. For example, currently there is a `Loops` mutation that requires an option `loops` to be set. Each mutation has its own configuration as `[<Mutation>Config]`, so to configure `Loops`:

```toml
[LoopsConfig]
loops = 10
```

This will set `loops` to be `10` for the `Loop` mutation. If you forget to specify this option, the program will specify which options it requires, and under which name.

```
TODO: Add example of error.
```

**Note:** In the case above, you only need to specify the `loops` option *if you include `"Loops"` in the `mutations` option!* If you exclude `"loops"` then the part above can be excluded as well.

### Overriding global options

What if, for example, you want `Loops` to have *different* values for `chunksize`? You can override them by simply specifying them under `[LoopsConfig]`:

```toml
[LoopsConfig]
loops = 10
chunksize = [10,1000]
```

In this case, the `iterations` used will be the global option set, however the `chunksize` used will be taken from `[LoopsConfig]`.

## Feedback!

This project is currently a prototype. As a result, any sort of feedback is *heavily* appreciated! If you'd like to contact me, you can use [my email](mctech26@gmail.com), or if you're on the *Fediverse* you can hit me up [there!](https://hellsite.site/@andre).

You can also open an issue, and I'll try to respond as fast as possible! Don't worry - any kind of feedback is accepted, be they feature requests, opinions, or criticism.

## TODO
- [ ] Update structure/formatting of code
  - [x] Improve structure to use `CfgMap` instead.
  - [ ] Find a way to improve structure of Mutations, specifically randomly generating settings in ranges.
- [ ] Improve UX by possibly adding a CLI app.
- [ ] Improve UX by possibly adding a GUI.

