# Changelog!

## v0.2.0 - 28/07/2019
- Added `#[ignore]` attribute for `#[derive(MutConfig)]`.
  - Any fields tagged with this attribute will be ignored. Very useful to have options in the `Config` that aren't used by `Mutation`s, and may have more complicated types.