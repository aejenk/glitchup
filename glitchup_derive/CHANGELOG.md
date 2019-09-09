# Changelog!

## v0.3.0 - 04/08/2019
- Added support for recursive generics.
  - Before, something like `Option<Vec<isize>>` was not possible. Only first-level generics were supported. This is now possible. The following functions were added to make this achievable:
    - `into_oval` : Turns a field into an OValue. `foo: isize` becomes `OInt(self.foo.clone())`.
    - `get_first_generic` : Retrieves the first generic of a type. For example, for `Option<isize>` it would retrieve `isize`.
    - `into_subval` : Used by `into_oval` to represent nested values. So for `Option<Vec<isize>>`, `into_subval` would be called by `into_oval` with `Vec<isize>` and a random string `a` to represent a closure argument. It would then return `OArray(a.iter().map(|aa| OInt(aa.clone())).collect())`.
      - _Note:_ For further nested generics, it simply recursively calls itself with different argument names `aa`, `aaa`, `aaaa...`.

## v0.2.0 - 28/07/2019
- Added `#[ignore]` attribute for `#[derive(MutConfig)]`.
  - Any fields tagged with this attribute will be ignored. Very useful to have options in the `Config` that aren't used by `Mutation`s, and may have more complicated types.