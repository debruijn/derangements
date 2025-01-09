use itertools::Itertools;
use std::convert::TryFrom;

/// Permute k or all elements of an integer Vec while excluding any results where one of the
/// elements doesn't change.
///
/// # Arguments
///
/// * `iterable`: Vec<T> with T having Clone and PartialEq, and usize::TryFrom<T>
/// * `k`: usize integer that determines how many elements each permutation should have
///
/// returns: Vec<Vec<T>>
///
/// # Examples
///
/// ```
/// use itertools::{assert_equal, Itertools};
/// use derangements::{derangements_vec, restricted_permutations_by_self};
///
/// // Compare the differences between derangements and this function
/// assert_equal(derangements_vec(vec![1, 0, 2], 3), [[1, 2, 0], [2, 0, 1]]);
/// assert_equal(restricted_permutations_by_self(vec![1, 0, 2], 3), [[0, 2, 1], [2, 1, 0]]);
///
/// // In case the input is already range-like, the output is the same
/// assert_equal(derangements_vec(vec![0, 1, 2], 3), restricted_permutations_by_self(vec![0, 1, 2], 3));
/// ```
pub fn restricted_permutations_by_self<T>(iterable: Vec<T>, k: usize) -> Vec<Vec<T>>
where
    T: Clone + PartialEq,
    usize: TryFrom<T>,
{
    iterable
        .clone()
        .into_iter()
        .permutations(k)
        .filter(|i| !i.iter().enumerate().any(|x| iterable[x.0] == *x.1))
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
