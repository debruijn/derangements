//! # Derangements
//!
//! `derangements` allows you to derange iterables: permutations where no element equals its index
//!
//! # Examples
//!
//! ```
//! use itertools::{assert_equal, Itertools};
//! use derangements::derangements;
//! assert_equal(derangements(vec![0, 1, 2], 3), [[1, 2, 0], [2, 0, 1]]);
//! ```

mod derangements;

pub use derangements::derangements;
pub use derangements::derangements_range;
