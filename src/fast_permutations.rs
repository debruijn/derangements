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
    use itertools::Itertools;

    #[test]
    fn derangements_manual() {
        println!("{:?}", fast_permutations(0..4, 4).collect_vec());
        println!(
            "{:?}",
            fast_permutations([1, 1, 2, 3].into_iter(), 4).collect_vec()
        );
    }
}
