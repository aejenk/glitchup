<h1 align="center">glitchup</h1>

<div align="center">
<sub>
A databender library/executable made in Rust. Comes with option loading, and proc macros to help development.
</sub>
</div>

<br/>

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

**Note:** This project is still a work in progress. You can *technically* already run it, however it's not polished up to my liking enough for me to actually release it as **V1** yet.

## Inspiration

There's a practice called [*databending*](https://en.wikipedia.org/wiki/Databending) which I've always found to be a nice fun little hobby of mine. This package is meant to make it easier for people to databend in the way that they want.

I already have a [*databender*](https://github.com/Calmynt/BENDPlusPlus) made in C++, however I wanted to remake it in Rust - partly to learn more about the language, and partly to create a superior program in general. Currently, these are the improvements:

- *Files are memory mapped rather than loaded into memory.* This leads to much better performance. As an example, if you load an 8GB file and modify 10MB chunks of data at a time, with the old method you'd need to load the whole 8GB file into memory. The new method would only end up using 10MB instead.
- *The options file is vastly superior.* This databender uses [**TOML**](https://github.com/toml-lang/toml) rather than [**INI**](https://en.wikipedia.org/wiki/INI_file) for the format of its options file. Check the [**options**](#options) section to know more.
- *Performance has an unbelievable boost.* For now benchmarks were only done locally, however in the future I hope to append some benchmarks to this README.

## Library?

*TODO*

## Options

An example options file can be found at [`Options.toml`](./Options.toml). I'll explain some important parts:

### Input + Output
The `inputfile` option needs to be set in order to specify which file to databend:

```
inputfile = "somefile.jpeg"
```

You can also specify a file that's not in the current directory. A file being in subdirectories (`dir/file.jpeg`) was tested, however a file being out of the current directory (`../file.jpeg`) was not. This may work, however some further testing is currently required.

By default, the output file will have the same name as the input file *(along with an appended string)*. If you'd like to save the file with a different name/directory, you can specify the `outputfile` option.

```
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
iterations = [5]
chunksize = [1000]
mutations = [ ["Reverse"], ["Swap"] ]
```

The program is going to be run **twice**, with each run containing an output for
- A mutation of `1KB` chunks by applying `Reverse` 5 times, and
- A mutation of `1KB` chunks by applynig `Swap` 5 times

...resulting in **2 * 2** output files in total. *(Times * No. of Mutations)*

#### Ranges

As you've seen above, `iterations` and `chunksize` need to be an `array`. The reasoning behind this is that they can also have *2 values*. In effect, they have two possible kinds of values:

- `iterations = [x]`: `x` iterations
- `iterations = [x,y]` : Between `x` and `y` iterations.

A number will be generated randomly between `x` and `y` for each *`time`*. Even if you don't want a range, you just need to specify an array of size 1. This is because of the way the *options* are being loaded by the databender.

#### Mutations

The `mutations` option has been overhauled from [*BEND++*](https://github.com/Calmynt/BENDPlusPlus)! Now it uses an *array of arrays*

```
mutations = [
  ["Reverse", "Swap"],
  ["Shuffle"],
  ...
]
```

Each element of `mutations` is an *array* and represents a single output. This array can have multiple mutations *chained together!*

In the options shown above, it means that the first file will first be mutated by `Reverse`, then by `Swap`, **then** saved. Then a new copy of the original file will be made, mutated by `Shuffle`, **then** saved. And so on...

### Specific options

Some mutations may have their own options that they require. For example, currently there is a `Loop` mutation that requires an option `loops` to be set. Each mutation has its own configuration as `[<Mutation>Config]`, so to configure `Loop`:

```
[LoopConfig]
loops = [10]
```

This will set `loops` to be `10` for the `Loop` mutation. If you forget to specify this option, the program will specify which options it requires, and under which name.

```
TODO: Add example of error.
```

**Note:** In the case above, you only need to specify the `loops` option *if you include `"Loops"` in the `mutations` option!* If you exclude `"loops"` then the part above can be excluded as well.

### Overriding global options

What if, for example, you want `Loop` to have *different* values for `chunksize`? You can override them by simply specifying them under `[LoopConfig]`:

```
[LoopConfig]
loops = [10]
chunksize = [10,1000]
```

In this case, the `iterations` used will be the global option set, however the `chunksize` used will be taken from `[LoopConfig]`.

## Feedback!

This project is currently a prototype. As a result, any sort of feedback is *heavily* appreciated! If you'd like to contact me, you can use [my email](mctech26@gmail.com), or if you're on the *Fediverse* you can hit me up [there!](https://cybre.space/@calm).

You can also open an issue, and I'll try to respond as fast as possible! Don't worry - any kind of feedback is accepted, be they feature requests, opinions, or criticism.

## TODO
- Finalize mutations
- Update structure/formatting of code
- Improve UX by possibly adding a CLI app.
- Improve UX by possibly adding a GUI.

