use crate::fast_permutations;
use crate::fast_permutations::FastPermutations;
use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::iter::FusedIterator;

macro_rules! clone_fields {  // Note: copied from Itertools - need to attribute before release - or just directly write the clone func
    ($($field:ident),*) => {
        #[inline]
        fn clone(&self) -> Self {
            Self {
                $($field: self.$field.clone(),)*
            }
        }
    }
}
macro_rules! debug_fmt_fields {  // Note: copied from Itertools - need to attribute before release - or just directly write the fmt func
    ($tyname:ident, $($($field:tt).+),*) => {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            f.debug_struct(stringify!($tyname))
                $(
              .field(stringify!($($field).+), &self.$($field).+)
              )*
              .finish()
        }
    }
}

#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct RestrictedPermutations<I: Iterator> {
    permutations: FastPermutations<I>,
    restrict: Vec<I::Item>,
}

impl<I> Clone for RestrictedPermutations<I>
where
    I: Clone + Iterator,
    I::Item: Clone,
{
    clone_fields!(permutations, restrict);
}

impl<I> Debug for RestrictedPermutations<I>
where
    I: Iterator + Debug,
    I::Item: Debug,
{
    debug_fmt_fields!(RestrictedPermutations, permutations, restrict);
}

/// Permute k or all elements of an integer Vec while excluding based on an input restriction
///
/// # Arguments
///
/// * `iterable`: `Vec<T>` with T having Clone and PartialEq
/// * `k`: `usize` integer that determines how many elements each permutation should have
/// * `restrict`: `Vec<T>` with the restricted values for each index
///
/// returns: `Vec<Vec<T>>`
///
/// # Examples
///
/// ```
/// use itertools::{assert_equal, Itertools};
/// use derangements::{restricted_permutations_by_self, restricted_permutations};
///
/// // If iterable is repeated as restrict input, result is same as "by_self"
/// assert_equal(restricted_permutations(vec![1, 0, 2].into_iter(), 3, vec![1, 0, 2].into_iter()),
///     restricted_permutations_by_self(vec![1, 0, 2].into_iter(), 3));
///
/// // Otherwise, the result will be different depending on the restrict input
/// assert_equal(restricted_permutations(vec![1, 0, 2, 2].into_iter(), 3, vec![1, 0, 2].into_iter()),
///     [[0, 2, 1], [2, 1, 0], [0, 2, 1], [2, 2, 0], [2, 2, 0], [2, 1, 0], [2, 2, 1], [2, 2, 1]]);
///
/// // This can also be applied to a non-integer type in the vector
/// assert_equal(restricted_permutations(vec!["I", "like", "permutations"].into_iter(), 2, vec!["like", "I"].into_iter()),
///     [["I", "like"], ["I", "permutations"], ["permutations", "like"]])
/// ```
pub fn restricted_permutations<I>(iter: I, k: usize, restrict: I) -> RestrictedPermutations<I>
where
    I: Iterator,
    I::Item: Clone + Ord,
{
    RestrictedPermutations {
        permutations: fast_permutations(iter, k),
        restrict: restrict.collect_vec(),
    }
}

/// Permute k or all elements of an integer Vec while excluding any results where one of the
/// elements doesn't change.
///
/// # Arguments
///
/// * `iterable`: `Vec<T>` with T having Clone and PartialEq
/// * `k`: `usize` integer that determines how many elements each permutation should have
///
/// returns: `Vec<Vec<T>>`
///
/// # Examples
///
/// ```
/// use itertools::{assert_equal, Itertools};
/// use derangements::{derangements, restricted_permutations_by_self};
///
/// // Compare the differences between derangements and this function
/// assert_equal(derangements(vec![1usize, 0, 2].into_iter(), 3).sorted(), [[1, 2, 0], [2, 0, 1]]);
/// assert_equal(restricted_permutations_by_self(vec![1, 0, 2].into_iter(), 3), [[0, 2, 1], [2, 1, 0]]);
///
/// // In case the input is already range-like, the output is the same
/// assert_equal(derangements(vec![0usize, 1, 2].into_iter(), 3).sorted(),
///     restricted_permutations_by_self(vec![0, 1, 2].into_iter(), 3).sorted());
///
/// // This can also be applied to a non-integer type in the vector
/// assert_equal(restricted_permutations_by_self(vec!["I", "like", "permutations"].into_iter(), 3).sorted(),
///     [["like", "permutations", "I"], ["permutations", "I", "like"]])
/// ```
pub fn restricted_permutations_by_self<I>(iter: I, k: usize) -> RestrictedPermutations<I>
where
    I: Iterator + Clone,
    I::Item: Clone + Ord,
{
    let permutations = fast_permutations(iter.clone(), k);
    let restrict = iter.collect_vec();
    RestrictedPermutations {
        permutations,
        restrict,
    }
}

impl<I> Iterator for RestrictedPermutations<I>
where
    I: Iterator,
    I::Item: Clone + Ord + Copy,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.permutations.next() {
            None => return None,
            Some(x) => {
                if !x.iter().enumerate().any(|x| self.restrict[x.0] == *x.1) {
                    return Some(x);
                }
            }
        };
        self.next()
    }
}

impl<I> FusedIterator for RestrictedPermutations<I>
where
    I: Iterator,
    I::Item: Clone + Ord + Copy + Debug,
{
}

#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct RestrictedPermutationsByMapIndex<I: Iterator> {
    permutations: FastPermutations<I>,
    restrict: HashMap<usize, Vec<I::Item>>,
}

impl<I> Clone for RestrictedPermutationsByMapIndex<I>
where
    I: Clone + Iterator,
    I::Item: Clone,
{
    clone_fields!(permutations, restrict);
}

impl<I> Debug for RestrictedPermutationsByMapIndex<I>
where
    I: Iterator + Debug,
    I::Item: Debug,
{
    debug_fmt_fields!(RestrictedPermutationsByMapIndex, permutations, restrict);
}

/// Permute k or all elements of an integer Vec while excluding based on an input restriction
///
/// # Arguments
///
/// * `iterable`: `Vec<T>` with T having Clone and PartialEq
/// * `k`: `usize` integer that determines how many elements each permutation should have
/// * `restrict`: `HashMap<usize, Vec<T>>`, indicating which T elements can not be at which index
///
/// returns: `Vec<Vec<T>>`
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
/// use itertools::{assert_equal, Itertools};
/// use derangements::restricted_permutations_by_map_index;
///
/// // Exclude from index 0 the values 0 and 1, and from index 1 the values 1 and 2.
/// let restrict = HashMap::from([(0, vec![0, 1]), (1, vec![1, 2])]);
/// assert_equal(restricted_permutations_by_map_index(vec![0, 1, 2, 3].into_iter(), 3, restrict).sorted(),
///     [[2, 0, 1], [2, 0, 3], [2, 3, 0], [2, 3, 1], [3, 0, 1], [3, 0, 2]]);
/// ```
///
/// The opposite can be done by using ``restricted_permutations_by_map_value``.
///
pub fn restricted_permutations_by_map_index<I>(
    iter: I,
    k: usize,
    restrict: HashMap<usize, Vec<I::Item>>,
) -> RestrictedPermutationsByMapIndex<I>
where
    I: Iterator,
    I::Item: Clone + Ord,
{
    RestrictedPermutationsByMapIndex {
        permutations: fast_permutations(iter, k),
        restrict,
    }
}

impl<I> Iterator for RestrictedPermutationsByMapIndex<I>
where
    I: Iterator,
    I::Item: Clone + Ord + Copy,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.permutations.next() {
            None => return None,
            Some(x) => {
                if !x.iter().enumerate().any(|x| {
                    if self.restrict.contains_key(&x.0) {
                        self.restrict[&x.0].contains(x.1)
                    } else {
                        false
                    }
                }) {
                    return Some(x);
                }
            }
        };
        self.next()
    }
}

impl<I> FusedIterator for RestrictedPermutationsByMapIndex<I>
where
    I: Iterator,
    I::Item: Clone + Ord + Copy + Debug,
{
}

#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct RestrictedPermutationsByMapValue<I: Iterator> {
    permutations: FastPermutations<I>,
    restrict: HashMap<I::Item, Vec<usize>>,
}

impl<I> Clone for RestrictedPermutationsByMapValue<I>
where
    I: Clone + Iterator,
    I::Item: Clone,
{
    clone_fields!(permutations, restrict);
}

impl<I> Debug for RestrictedPermutationsByMapValue<I>
where
    I: Iterator + Debug,
    I::Item: Debug,
{
    debug_fmt_fields!(RestrictedPermutationsByMapValue, permutations, restrict);
}

/// Permute k or all elements of an integer Vec while excluding based on an input restriction
///
/// # Arguments
///
/// * `iterable`: `Vec<T>` with T having Clone and PartialEq
/// * `k`: `usize` integer that determines how many elements each permutation should have
/// * `restrict`: `HashMap<T, Vec<usize>>`, indicating at which indices an element T can't be
///
/// returns: `Vec<Vec<T>>`
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
/// use itertools::{assert_equal, Itertools};
/// use derangements::restricted_permutations_by_map_value;
///
/// // Exclude from value 0 from indices 0 and 1, and value 1 from indices 1 and 2.
/// let restrict = HashMap::from([(0, vec![0, 1]), (1, vec![1, 2])]);
/// assert_equal(restricted_permutations_by_map_value(vec![0, 1, 2, 3].into_iter(), 3, restrict).sorted(),
///    [[1, 2, 0], [1, 2, 3], [1, 3, 0], [1, 3, 2], [2, 3, 0], [3, 2, 0]]);
/// ```
///
/// The opposite can be done by using ``restricted_permutations_by_map_index``.
///
pub fn restricted_permutations_by_map_value<I>(
    iter: I,
    k: usize,
    restrict: HashMap<I::Item, Vec<usize>>,
) -> RestrictedPermutationsByMapValue<I>
where
    I: Iterator,
    I::Item: Clone + Ord,
{
    RestrictedPermutationsByMapValue {
        permutations: fast_permutations(iter, k),
        restrict,
    }
}

impl<I> Iterator for RestrictedPermutationsByMapValue<I>
where
    I: Iterator,
    I::Item: Clone + Ord + Copy + Hash,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.permutations.next() {
            None => return None,
            Some(x) => {
                if !x.iter().enumerate().any(|x| {
                    if self.restrict.contains_key(x.1) {
                        self.restrict[x.1].contains(&x.0)
                    } else {
                        false
                    }
                }) {
                    return Some(x);
                }
            }
        };
        self.next()
    }
}

impl<I> FusedIterator for RestrictedPermutationsByMapValue<I>
where
    I: Iterator,
    I::Item: Clone + Ord + Copy + Debug + Hash,
{
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::assert_equal;

    #[test]
    fn test_self_restricted_manual() {
        assert_equal(
            restricted_permutations_by_self(vec![1, 0].into_iter(), 2),
            [[0, 1]],
        );
        assert_equal(
            restricted_permutations_by_self(vec![1, 0, 2].into_iter(), 3),
            [[0, 2, 1], [2, 1, 0]],
        );
        assert_equal(
            restricted_permutations_by_self(vec![1, 0, 2].into_iter(), 2),
            [[0, 1], [0, 2], [2, 1]],
        );
    }

    #[test]
    fn test_restricted() {
        assert_equal(
            restricted_permutations(vec![1usize, 0].into_iter(), 2, vec![1, 0].into_iter()),
            [[0, 1]],
        );
        assert_equal(
            restricted_permutations_by_self(vec![1, 0, 2].into_iter(), 3),
            [[0, 2, 1], [2, 1, 0]],
        );
        assert_equal(
            restricted_permutations_by_self(vec![1, 0, 2].into_iter(), 2),
            [[0, 1], [0, 2], [2, 1]],
        );
    }
}
