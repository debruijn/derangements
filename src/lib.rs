//! # Derangements
//!
//! `derangements` allows you to derange iterables: permutations where no element equals its index
//!
//! # Examples
//!
//! ```
//! use std::collections::HashMap;
//! use itertools::{assert_equal, Itertools};
//! use derangements::{derangements, restricted_permutations_by_map_value};
//! assert_equal(derangements(vec![0usize, 1, 2].into_iter(), 3).sorted(), [[1, 2, 0], [2, 0, 1]]);
//!
//! // The module also contains other variants of restricted permutations
//! // Exclude from value 0 from indices 0 and 1, and value 1 from indices 1 and 2.
//! let restrict = HashMap::from([(0, vec![0, 1]), (1, vec![1, 2])]);
//! assert_equal(restricted_permutations_by_map_value(vec![0, 1, 2, 3].into_iter(), 3, restrict).sorted(),
//!    [[1, 2, 0], [1, 2, 3], [1, 3, 0], [1, 3, 2], [2, 3, 0], [3, 2, 0]]);
//! ```

mod derangements;
mod derangements_range;
mod fast_permutations;
mod restricted_permutations;

pub use derangements_range::derangements_range;
pub use derangements_range::derangements_range_fast;

pub use restricted_permutations::restricted_permutations;
pub use restricted_permutations::restricted_permutations_by_map_index;
pub use restricted_permutations::restricted_permutations_by_map_value;
pub use restricted_permutations::restricted_permutations_by_self;

pub use derangements::derangements;
pub use derangements::distinct_derangements;

pub use fast_permutations::distinct_permutations;
pub use fast_permutations::fast_permutations;
