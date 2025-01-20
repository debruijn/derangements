use itertools::Itertools;
use std::collections::HashMap;
use std::hash::Hash;

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
/// assert_equal(restricted_permutations_by_self(vec![1, 0, 2], 3), [[0, 2, 1], [2, 1, 0]]);
///
/// // In case the input is already range-like, the output is the same
/// assert_equal(derangements(vec![0usize, 1, 2].into_iter(), 3).sorted(),
///     restricted_permutations_by_self(vec![0, 1, 2], 3));
///
/// // This can also be applied to a non-integer type in the vector
/// assert_equal(restricted_permutations_by_self(vec!["I", "like", "permutations"], 3),
///     [["like", "permutations", "I"], ["permutations", "I", "like"]])
/// ```
pub fn restricted_permutations_by_self<T>(iterable: Vec<T>, k: usize) -> Vec<Vec<T>>
where
    T: Clone + PartialEq,
{
    iterable
        .clone()
        .into_iter()
        .permutations(k)
        .filter(|i| !i.iter().enumerate().any(|x| iterable[x.0] == *x.1))
        .collect_vec()
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
/// use derangements::{restricted_permutations_by_self, restricted_permutations_by_vec};
///
/// // If iterable is repeated as restrict input, result is same as "by_self"
/// assert_equal(restricted_permutations_by_vec(vec![1, 0, 2], 3, vec![1, 0, 2]),
///     restricted_permutations_by_self(vec![1, 0, 2], 3));
///
/// // Otherwise, the result will be different depending on the restrict input
/// assert_equal(restricted_permutations_by_vec(vec![1, 0, 2, 2], 3, vec![1, 0, 2]),
///     [[0, 2, 1], [0, 2, 1], [2, 1, 0], [2, 2, 1], [2, 2, 0], [2, 1, 0], [2, 2, 1], [2, 2, 0]]);
///
/// // This can also be applied to a non-integer type in the vector
/// assert_equal(restricted_permutations_by_vec(vec!["I", "like", "permutations"], 2, vec!["like", "I"]),
///     [["I", "like"], ["I", "permutations"], ["permutations", "like"]])
/// ```
pub fn restricted_permutations_by_vec<T>(
    iterable: Vec<T>,
    k: usize,
    restrict: Vec<T>,
) -> Vec<Vec<T>>
where
    T: Clone + PartialEq,
{
    iterable
        .into_iter()
        .permutations(k)
        .filter(|i| !i.iter().enumerate().any(|x| restrict[x.0] == *x.1))
        .collect_vec()
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
/// assert_equal(restricted_permutations_by_map_index(vec![0, 1, 2, 3], 3, restrict),
///     [[2, 0, 1], [2, 0, 3], [2, 3, 0], [2, 3, 1], [3, 0, 1], [3, 0, 2]]);
/// ```
///
/// The opposite can be done by using ``restricted_permutations_by_map_value``.
///
pub fn restricted_permutations_by_map_index<T>(
    iterable: Vec<T>,
    k: usize,
    restrict: HashMap<usize, Vec<T>>,
) -> Vec<Vec<T>>
where
    T: Clone + PartialEq,
{
    iterable
        .into_iter()
        .permutations(k)
        .filter(|i| {
            !i.iter().enumerate().any(|x| {
                if restrict.contains_key(&x.0) {
                    restrict[&x.0].contains(x.1)
                } else {
                    false
                }
            })
        })
        .collect_vec()
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
/// assert_equal(restricted_permutations_by_map_value(vec![0, 1, 2, 3], 3, restrict),
///    [[1, 2, 0], [1, 2, 3], [1, 3, 0], [1, 3, 2], [2, 3, 0], [3, 2, 0]]);
/// ```
///
/// The opposite can be done by using ``restricted_permutations_by_map_index``.
///
pub fn restricted_permutations_by_map_value<T>(
    iterable: Vec<T>,
    k: usize,
    restrict: HashMap<T, Vec<usize>>,
) -> Vec<Vec<T>>
where
    T: Clone + PartialEq + Hash + Eq,
{
    iterable
        .into_iter()
        .permutations(k)
        .filter(|i| {
            !i.iter().enumerate().any(|x| {
                if restrict.contains_key(x.1) {
                    restrict[x.1].contains(&x.0)
                } else {
                    false
                }
            })
        })
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::assert_equal;

    #[test]
    fn test_self_restricted_manual() {
        assert_equal(restricted_permutations_by_self(vec![1, 0], 2), [[0, 1]]);
        assert_equal(
            restricted_permutations_by_self(vec![1, 0, 2], 3),
            [[0, 2, 1], [2, 1, 0]],
        );
        assert_equal(
            restricted_permutations_by_self(vec![1, 0, 2], 2),
            [[0, 1], [0, 2], [2, 1]],
        );
    }
}
