//! # Derangements
//!
//! `derangements` allows you to derange iterables: permutations where no element equals its index
//!
//! # Examples
//!
//! ```
//! use itertools::{assert_equal, Itertools};
//! use derangements::derangements;
//! assert_equal(derangements(vec![0, 1, 2].into_iter(), 3), [[1, 2, 0], [2, 0, 1]]);
//! ```

mod derangements;
mod restricted_permutations;

pub use derangements::derangements;
pub use derangements::derangements_box;
pub use derangements::derangements_range;
pub use derangements::derangements_range_fast;
pub use derangements::derangements_vec;
pub use derangements::distinct_derangements;

pub use restricted_permutations::restricted_permutations_by_self;
