use itertools::Itertools;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct FastPermutations<I: Iterator> {
    buffer: Vec<usize>,
    values: Vec<I::Item>,
    start: bool,
    index: usize,
    k: usize,
}

/// Permute k or all elements of an iterable.
///
/// # Arguments
///
/// * `iterable`: the iterable of items to permute
/// * `k`: `usize` integer that determines how many elements each permutation should have
///
/// returns: iterable with the permutations
///
/// # Examples
///
/// ```
/// use itertools::{assert_equal, Itertools};
/// use derangements::fast_permutations;
///
/// println!("{:?}", fast_permutations(vec![0usize, 1, 2].into_iter(), 3).collect_vec());
///
/// assert_equal(fast_permutations(vec![0usize, 1, 2].into_iter(), 3),
/// [[0, 1, 2], [2, 0, 1], [0, 2, 1], [1, 0, 2], [2, 1, 0], [1, 2, 0]]);
///
/// // There can be repeated values in the input, which will not be deduplicated
/// assert_equal(fast_permutations(vec![0usize, 1, 1].into_iter(), 3),
/// [[0, 1, 1], [1, 0, 1], [0, 1, 1], [1, 0, 1], [1, 1, 0], [1, 1, 0]]);
///
/// // The length of the derangements can be shorter than the input iterable
/// assert_equal(fast_permutations(vec![0usize, 1, 2].into_iter(), 2),
/// [[0, 1], [2, 0], [0, 2], [1, 0], [2, 1], [1, 2]]);
///
/// ```
pub fn fast_permutations<I>(iter: I, k: usize) -> FastPermutations<I>
where
    I: Iterator,
    I::Item: Ord + Clone,
{
    let mut values = Vec::from_iter(iter);
    values.sort_unstable();
    let length = values.len();
    let buffer = (0..length).rev().collect_vec();
    FastPermutations {
        buffer,
        values,
        start: true,
        index: length.saturating_sub(2),
        k,
    }
}

impl<I> FastPermutations<I>
where
    I: Iterator,
    I::Item: Clone,
{
    pub fn get_values(&self) -> Vec<I::Item> {
        self.values.clone()
    }
}

impl<I> Iterator for FastPermutations<I>
where
    I: Iterator,
    I::Item: Ord + Copy + Clone,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        // Start iteration with buffer itself
        if self.start {
            self.start = false;
            return Some(self.values[0..self.k].to_owned());
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
        let shift_val = self.values[shift_index];
        let mut swap_index = shift_index;
        while swap_index > 0 {
            self.buffer[swap_index] = self.buffer[swap_index - 1];
            self.values[swap_index] = self.values[swap_index - 1];
            swap_index -= 1;
        }
        self.buffer[0] = shift_elem;
        self.values[0] = shift_val;

        // Update index
        if self.buffer[0] < self.buffer[1] {
            self.index = 0;
        } else {
            self.index += 1;
        }

        Some(self.values[0..self.k].to_owned())
    }
}

#[derive(Debug, Clone)]
pub struct DistinctPermutations<I: Iterator> {
    buffer: Vec<I::Item>,
    start: bool,
    index: usize,
}

/// Permute k or all elements of an iterable without repetitions.
///
/// # Arguments
///
/// * `iterable`: the iterable of items to permute
///
/// returns: iterator over the permutations
///
/// # Examples
///
/// ```
/// use itertools::{assert_equal, Itertools};
/// use derangements::distinct_permutations;
///
/// assert_equal(distinct_permutations(vec![0usize, 1, 1].into_iter()),
/// [[1, 1, 0], [0, 1, 1], [1, 0, 1]]);
/// ```
pub fn distinct_permutations<I>(iter: I) -> DistinctPermutations<I>
where
    I: Iterator,
    I::Item: Ord + Clone,
{
    let mut buffer = Vec::from_iter(iter);
    buffer.sort_unstable_by(|a, b| b.cmp(a));
    let length = buffer.len();
    DistinctPermutations {
        buffer,
        start: true,
        index: length.saturating_sub(2),
    }
}

impl<I> Iterator for DistinctPermutations<I>
where
    I: Iterator,
    I::Item: Ord + Copy + Clone,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        // Start iteration with buffer itself
        if self.start {
            self.start = false;
            return Some(self.buffer.clone());
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

        Some(self.buffer.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::{assert_equal, Itertools};

    #[test]
    fn test_permutations_range() {
        for k in 0..8 {
            assert_equal(
                fast_permutations(0..k, k).sorted(),
                Itertools::permutations(0..k, k).sorted(),
            );
        }
    }

    #[test]
    fn test_permutations_manual() {
        assert_equal(
            fast_permutations(vec![0usize, 2].into_iter(), 2),
            [[0, 2], [2, 0]],
        );
        assert_equal(
            fast_permutations(vec![0u8, 1, 3].into_iter(), 3),
            [
                [0, 1, 3],
                [3, 0, 1],
                [0, 3, 1],
                [1, 0, 3],
                [3, 1, 0],
                [1, 3, 0],
            ],
        );
        assert_equal(
            fast_permutations(vec![0usize, 1, 3].into_iter(), 2),
            [[0, 1], [3, 0], [0, 3], [1, 0], [3, 1], [1, 3]],
        );
        assert_equal(
            fast_permutations(vec![0u16, 1, 1].into_iter(), 3),
            [
                [0, 1, 1],
                [1, 0, 1],
                [0, 1, 1],
                [1, 0, 1],
                [1, 1, 0],
                [1, 1, 0],
            ],
        );
        assert_equal(
            fast_permutations(vec![0usize, 1, 1].into_iter(), 2),
            [[0, 1], [1, 0], [0, 1], [1, 0], [1, 1], [1, 1]],
        );
    }

    #[test]
    fn test_distinct_permutations() {
        assert_equal(
            distinct_permutations(vec![0u8, 1, 3].into_iter()),
            [
                [3, 1, 0],
                [0, 3, 1],
                [3, 0, 1],
                [1, 3, 0],
                [0, 1, 3],
                [1, 0, 3],
            ],
        );
        assert_equal(
            distinct_permutations(vec![0u16, 1, 1].into_iter()),
            [[1, 1, 0], [0, 1, 1], [1, 0, 1]],
        );
    }
}
