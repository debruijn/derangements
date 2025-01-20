use crate::fast_permutations;
use crate::fast_permutations::FastPermutations;
use std::fmt::Debug;
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
pub struct Derangements<I: Iterator> {
    permutations: FastPermutations<I>,
}

impl<I> Clone for Derangements<I>
where
    I: Clone + Iterator,
    I::Item: Clone,
{
    clone_fields!(permutations);
}

impl<I> Debug for Derangements<I>
where
    I: Iterator + Debug,
    I::Item: Debug,
{
    debug_fmt_fields!(Derangements, permutations);
}

/// Derange k or all elements of an iterable.
///
/// # Arguments
///
/// * `iterable`: `Vec<T>` with type T having Clone and `usize::TryFrom<T>`
/// * `k`: `usize` integer that determines how many elements each derangement should have
///
/// returns: `Vec<Vec<T>>`
///
/// # Examples
///
/// ```
/// use itertools::{assert_equal, Itertools};
/// use derangements::derangements;
/// assert_equal(derangements(vec![0usize, 1, 2].into_iter(), 3), [[2, 0, 1], [1, 2, 0]]);
///
/// // There can be repeated values in the input, which will not be deduplicated
/// assert_equal(derangements(vec![0usize, 1, 1].into_iter(), 3), [[1, 0, 1], [1, 0, 1]]);
///
/// // The length of the derangements can be shorter than the input iterable
/// assert_equal(derangements(vec![0usize, 1, 2].into_iter(), 2), [[2, 0], [1, 0], [1, 2]]);
///
/// // There can be values that are outside the range of the indices
/// assert_equal(derangements(vec![0usize, 1, 7].into_iter(), 3), [[7, 0, 1], [1, 0, 7], [1, 7, 0]]);
/// ```
pub fn derangements<I>(iter: I, k: usize) -> Derangements<I>
where
    I: Iterator,
    I::Item: Clone + Ord,
    usize: From<I::Item>,
{
    Derangements {
        permutations: fast_permutations(iter, k),
    }
}

impl<I> Iterator for Derangements<I>
where
    I: Iterator,
    I::Item: Clone + TryInto<usize> + Ord + Copy + Debug,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.permutations.next() {
            None => return None,
            Some(x) => {
                if !x
                    .iter()
                    .enumerate()
                    .any(|x| x.0 == (*x.1).try_into().unwrap_or(usize::MAX))
                {
                    return Some(x);
                }
            }
        };
        self.next()
    }
}

impl<I> FusedIterator for Derangements<I>
where
    I: Iterator,
    I::Item: Clone + TryInto<usize> + Ord + Copy + Debug,
{
}

#[derive(Debug, Clone)]
pub struct DistinctDerangements<I: Iterator> {
    buffer: Vec<I::Item>,
    start: bool,
    index: usize,
}

/// Derange k or all elements of an iterable without repetitions.
///
/// # Arguments
///
/// * `iterable`: `Vec<T>` containing the iterable of items of type `U` that have Clone and `usize::TryFrom<U>`
/// * `k`: `usize` integer that determines how many elements each derangement should have
///
/// returns: `Vec<Vec<U>>`
///
/// # Examples
///
/// ```
/// use itertools::{assert_equal, Itertools};
/// use derangements::distinct_derangements;
/// assert_equal(distinct_derangements(vec![0usize, 1, 2].into_iter()), [[2, 0, 1], [1, 2, 0]]);
///
/// // There can be repeated values in the input, which will not be deduplicated
/// assert_equal(distinct_derangements(vec![0usize, 1, 1].into_iter()), [[1, 0, 1]]);
///
/// // There can be values that are outside the range of the indices
/// assert_equal(distinct_derangements(vec![0usize, 1, 7].into_iter()), [[7, 0, 1], [1, 7, 0], [1, 0, 7]]);
/// ```
pub fn distinct_derangements<I>(iter: I) -> DistinctDerangements<I>
where
    I: Iterator,
    I::Item: Ord + Clone,
    usize: From<I::Item>,
{
    let mut buffer = Vec::from_iter(iter);
    buffer.sort_unstable_by(|a, b| b.cmp(a));
    let length = buffer.len();
    DistinctDerangements {
        buffer,
        start: true,
        index: length.saturating_sub(2),
    }
}

impl<I> Iterator for DistinctDerangements<I>
where
    I: Iterator,
    I::Item: Ord + Copy + Clone,
    usize: From<I::Item>,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        // Start iteration with buffer itself
        if self.start {
            self.start = false;
            if !self
                .buffer
                .iter()
                .enumerate()
                .any(|x| x.0 == <I::Item as Into<usize>>::into(*x.1))
            {
                return Some(self.buffer.clone());
            }
        }

        // Exhausted iteration
        let has_two_next = self.index + 2 < self.buffer.len();
        if !has_two_next
            && (self.buffer.len() <= self.index + 1
                || self.buffer[0] <= self.buffer[self.index + 1])
        {
            return None;
        }

        // Determine shift index
        let shift_index = if has_two_next && self.buffer[self.index + 2] <= self.buffer[self.index]
        {
            self.index + 2
        } else {
            self.index + 1
        };

        // Prefix shift
        let shift_elem = self.buffer[shift_index];
        let mut swap_index = shift_index;
        while swap_index > 0 {
            self.buffer[swap_index] = self.buffer[swap_index - 1];
            swap_index -= 1;
        }
        self.buffer[0] = shift_elem;

        // Update index
        if self.buffer[0] < self.buffer[1] {
            self.index = 0;
        } else {
            self.index += 1;
        }
        if !self
            .buffer
            .iter()
            .enumerate()
            .any(|x| x.0 == <I::Item as Into<usize>>::into(*x.1))
        {
            Some(self.buffer.clone())
        } else {
            self.next()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::derangements_range;
    use itertools::{assert_equal, Itertools};

    #[test]
    fn test_nonrange_range() {
        for k in 0..8 {
            assert_equal(
                derangements_range(k).into_iter().sorted(),
                derangements(0..k, k).sorted(),
            );
        }
    }

    #[test]
    fn test_nonrange_manual() {
        assert_equal(derangements(vec![0usize, 2].into_iter(), 2), [[2, 0]]);
        assert_equal(
            derangements(vec![0u8, 1, 3].into_iter(), 3),
            [[3, 0, 1], [1, 0, 3], [1, 3, 0]],
        );
        assert_equal(
            derangements(vec![0usize, 1, 3].into_iter(), 2),
            [[3, 0], [1, 0], [1, 3]],
        );
        assert_equal(
            derangements(vec![0u16, 1, 1].into_iter(), 3),
            [[1, 0, 1], [1, 0, 1]],
        );
        assert_equal(
            derangements(vec![0usize, 1, 1].into_iter(), 2),
            [[1, 0], [1, 0]],
        );
    }

    #[test]
    fn test_nonrange_distinct() {
        assert_equal(
            distinct_derangements(vec![0u8, 1, 3].into_iter()),
            [[3, 0, 1], [1, 3, 0], [1, 0, 3]],
        );
        assert_equal(
            distinct_derangements(vec![0u16, 1, 1].into_iter()),
            [[1, 0, 1]],
        );
    }

    // Test to uncomment when I want to time performance - to convert into proper benchmark
    // #[test]
    // fn test_time() {
    //     use std::time::Instant;
    //     for k in 0..10 {
    //         let before = Instant::now();
    //         _ = derangements_range(k).len();
    //         let between = Instant::now();
    //         _ = derangements_range_fast(k).len();
    //         let between2 = Instant::now();
    //         _ = derangements_old(0..k, k).len();
    //         let after = Instant::now();
    //         _ = derangements(0..k, k).collect_vec().len();
    //         let after2 = Instant::now();
    //         _ = distinct_derangements(0..k).collect_vec().len();
    //         let after3 = Instant::now();
    //         println!(
    //             "{:?}, range old {:?}, range new {:?}, non-range {:?}, iter: {:?}, distinct_iter: {:?}",
    //             k,
    //             between - before,
    //             between2 - between,
    //             after - between2,
    //             after2 - after,
    //             after3 - after2
    //         )
    //     }
    // }
}
