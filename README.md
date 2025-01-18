# derangements
Generate `derangements` and variants in line with permutations, combinations etc in the `itertools` crate. This is still
very early and WIP, although the key functionality is there so it is usable.

## Get started
To get started:
- add `derangements = 0.1.0` to your `Cargo.toml`
- add `use derangements::derangements` or `use derangements::derangements_range` to your Rust file
  - `derangements(iterable: Vec<usize>, k: usize)` will apply all derangements of input `iterable` up to the first `k`
    elements
  - `derangements_range(n: usize)` will return all derangements of the range `0..n`
- output of either will be a Vec<Vec<usize>> containing all derangements

For more options, including more derangement variants and also other restricted permutations, see
https://docs.rs/derangements

## Future plans (striked through will be in next release)
Ideally the following would be added or explored:
- generalize inputs to allow for non-usize inputs (even non-integer) -> partially now done, can be negative
  - note: if this is needed for generating a k-length derangement, you can always map the non-integers to values
    outside 0..k and then map them back afterwards
- ~~return an iterator instead of a vec~~
- ~~add dinstict_derangements~~
- ~~add derangement_vec (second input indicates which element can't be placed on that spot - might need a new name)~~
- ~~add derangement_self (the above but with first input having both roles)~~
- ~~add derangement_map (the above but with multiple restrictions per index)~~
- ~~add derangement_map variant where you don't restrict the indices but you restrict the elements (same end result)~~
- add random_derangement, at least for the default derangement types
- add examples/use cases of how/when to use this
- ~~add docstrings~~
- ~~speed up derangements_range by caching earlier derangements results~~
- Fix tests:
  - move old Vec tests to iter based implementation
  - keep range tests for now
  - add more tests for restricted permutations
  - add more tests for fast permutations
  - update test in lib.rs for default usage
- Explore creating an iterable for a faster derangement_range as well (if that is faster) - or otherwise just remove
- Consider introducing `derangements_by_value` here as well to keep naming consistent with Python's `more_itertools`

If you have more ideas, let me know!

## License
Licensed under either of Apache License, Version 2.0 or MIT license at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as
defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
