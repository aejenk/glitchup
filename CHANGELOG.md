# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog], and this project adheres to
[Semantic Versioning]. The file is auto-generated using [Conventional Commits].

[keep a changelog]: https://keepachangelog.com/en/1.0.0/
[semantic versioning]: https://semver.org/spec/v2.0.0.html
[conventional commits]: https://www.conventionalcommits.org/en/v1.0.0-beta.4/

## Overview

- [unreleased](#unreleased)
- [`0.4.0`](#040) – _2019.09.14_
- [`0.3.0`](#030) – _2019.09.09_

## _[Unreleased]_

_nothing new to show for… yet!_

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


### Contributions

This release is made possible by the following people (in alphabetical order).
Thank you all for your contributions. Your work – no matter how significant – is
greatly appreciated by the community. 💖

- Calmynt (<mctech26@gmail.com>)

### Changes

#### Bug Fixes

- **added seed to bender to help conflicts** ([`8a05010`])

#### Documentation

- **updated readme and q&a** ([`4c2908d`])

## [0.3.0] – _The Alpha_

_2019.09.09_
The first release of `glitchup`, the databender!

It comes with **9** mutation modes, extensive customization, and *(hopefully)* simple instructions.

This is **far** from the final program, but it's still useable, and even satisfactory! If there are more features that you would like, please open an issue, and I'll be happy to address any concerns/ideas.

The performance is ridiculously improved over `BEND++`. Benchmarks will be coming soon. However I would heavily suggest you use `glitchup` over `BEND++` *(unless you'd like to use any of the other* ***9*** *mutations that aren't included with this release)*

I hope you enjoy!

P.S. There are separate binaries for `windows` and `linux`. Make sure you install the right one. Also, `Options.toml` is compulsory. Otherwise you can't really configure it, eh?


### Contributions

This release is made possible by the following people (in alphabetical order).
Thank you all for your contributions. Your work – no matter how significant – is
greatly appreciated by the community. 💖

- Andre Jenkins (<mctech26@gmail.com>)

### Changes

#### Miscellaneous Tasks

- **changed version back** ([`d821fce`])

<!-- [releases] -->

[unreleased]: #/compare/v0.4.0...HEAD
[0.4.0]: #/releases/tag/v0.4.0
[0.3.0]: #/releases/tag/v0.3.0

<!-- [commits] -->

[`8a05010`]: #/commit/8a05010262325c3e176744f5c1be427ebd0cd641
[`4c2908d`]: #/commit/4c2908ddf80b167b19ff7e4ec2f69322e893fed1
[`d821fce`]: #/commit/d821fcefd5074a6286d3e43a0ed7ed4075f2308d
