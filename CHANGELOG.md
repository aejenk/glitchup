# Changelog

All notable changes to this project are documented in this file.

## Overview

- [Changelog](#changelog)
  - [Overview](#overview)
  - [[0.4.2]](#042)
    - [Changes](#changes)
  - [[0.4.0] – _Working in Parallel_](#040--working-in-parallel)
    - [Changes](#changes-1)
  - [[0.3.0] – _The Alpha_](#030--the-alpha)

## [0.4.2]

_2020.02.03_

### Changes

- **Made the `Loop` and `Swap` mutations safer to use**, meaning that they won't panic if given incompatible settings, only display a warning.
- **Made the `Multiply` mutation more accurate:** Before, if only given a single option, it would generate an option between the range [0.5, 1.5]. This has been fixed to generate [0.5, 0.5 + 10e-10].
- **Changed the format for configuration from deserializing TOML, to using `CfgMap`:** This has had a drastic effect on code structure, removing a large amount of lines, while also making the code generally cleaner.
- **Single options no longer require `[]`:** If you want to specify a single option rather than a range, you should now do `chunksize=5000` directly.

## [0.4.0] – _Working in Parallel_

_2019.09.14_
This release contains a major upgrade in performance, due to having
implemented partial parallellism. Now, every list of mutations is run in
parallel, which leads to large gains in speed.

To note: the `mutations` option is a LIST of LISTS. Only the top-level
list has been parallellised; only *iterations* are run in parallel,
actual *mutations* are run in serial.

The improvement in speed has been around 36% (4 lists) to 61% (4 lists +
100 loops)

### Changes

- **added tutorial**
- **updated readme and q&a**
- **added seed to bender to avoid conflicts**

## [0.3.0] – _The Alpha_

_2019.09.09_
The first release of `glitchup`, the databender!

It comes with **9** mutation modes, extensive customization, and *(hopefully)* simple instructions.

This is **far** from the final program, but it's still useable, and even satisfactory! If there are more features that you would like, please open an issue, and I'll be happy to address any concerns/ideas.

The performance is ridiculously improved over `BEND++`. Benchmarks will be coming soon. However I would heavily suggest you use `glitchup` over `BEND++` *(unless you'd like to use any of the other* ***9*** *mutations that aren't included with this release)*

I hope you enjoy!

P.S. There are separate binaries for `windows` and `linux`. Make sure you install the right one. Also, `Options.toml` is compulsory. Otherwise you can't really configure it, eh?


<!-- [releases] -->

[unreleased]: #/compare/v0.4.0...HEAD
[0.4.0]: #/releases/tag/v0.4.0
[0.3.0]: #/releases/tag/v0.3.0
