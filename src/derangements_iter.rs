use itertools::{Itertools, Permutations};
use std::fmt::Debug;
use std::iter::FusedIterator;

macro_rules! clone_fields {  // Note: copied from Itertools - need to attribute before release - or just directly write the clone func
    ($($field:ident),*) => {
        #[inline] // TODO is this sensible?
        fn clone(&self) -> Self {
            Self {
                $($field: self.$field.clone(),)*
            }
        }
    }
}
macro_rules! debug_fmt_fields {  // Note: copied from Itertools - need to attribute before release - or just directly write the fmt func
    ($tyname:ident, $($($field:tt/*TODO ideally we would accept ident or tuple element here*/).+),*) => {
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
    permutations: Permutations<I>,
    // restrictions: Vec<I>
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

pub fn derangements_iter<I>(iter: I, k: usize) -> Derangements<I>
where
    I: Iterator,
    I::Item: Clone,
{
    Derangements {
        permutations: Itertools::permutations(iter, k),
    }
}

impl<I> Iterator for Derangements<I>
where
    I: Iterator,
    I::Item: Clone + TryInto<usize>,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.permutations.next() {
            None => return None,
            Some(x) => {
                if !x
                    .iter()
                    .enumerate()
                    .any(|x| x.0 == x.1.clone().try_into().unwrap_or(usize::MAX))
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
    I::Item: Clone + TryInto<usize>,
{
}

#[derive(Debug, Clone)]
pub struct DistinctDerangements<I> {
    buffer: Vec<I>,
    start: bool,
    index: usize,
}

pub fn distinct_derangements<I>(iter: I) -> DistinctDerangements<I::Item>
where
    I: Iterator,
    I::Item: Ord,
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

impl<I: Copy> Iterator for DistinctDerangements<I>
where
    I: Ord,
    usize: From<I>,
{
    type Item = Vec<I>;

    fn next(&mut self) -> Option<Self::Item> {
        // Start iteration with buffer itself
        if self.start {
            self.start = false;
            if !self
                .buffer
                .iter()
                .enumerate()
                .any(|x| x.0 == <I as Into<usize>>::into(*x.1))
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
            .any(|x| x.0 == <I as Into<usize>>::into(*x.1))
        {
            Some(self.buffer.clone())
        } else {
            self.next()
        }
    }
}

#[derive(Debug, Clone)]
pub struct MultisetPermutations<I> {
    buffer: Vec<I>,
    start: bool,
    index: usize,
}

pub fn multiset_permutations<I>(iter: I) -> MultisetPermutations<I::Item>
where
    I: Iterator,
    I::Item: Ord,
{
    let mut buffer = Vec::from_iter(iter);
    buffer.sort_unstable_by(|a, b| b.cmp(a));
    let length = buffer.len();
    MultisetPermutations {
        buffer,
        start: true,
        index: length.saturating_sub(2),
    }
}

impl<I: Copy> Iterator for MultisetPermutations<I>
where
    I: Ord,
{
    type Item = Vec<I>;

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
    use crate::{derangements, derangements_range, derangements_range_fast};
    use itertools::assert_equal;

    #[test]
    fn derangements_manual() {
        assert_equal(derangements_iter(0..1, 1), Vec::<Vec<usize>>::new());
        assert_equal(derangements_iter(0..2, 2), vec![[1, 0]]);
        assert_equal(derangements_iter(0..3, 3), vec![[1, 2, 0], [2, 0, 1]]);
        assert_equal(derangements_iter(0..3, 2), vec![[1, 0], [1, 2], [2, 0]]);
        assert_equal(
            derangements_iter([0, 0, 1].into_iter(), 3),
            vec![[1, 0, 0], [1, 0, 0]],
        );
        for k in 8..10 {
            println!("{:?}", (0..k).permutations(k).collect_vec().len());
            println!("{:?}", multiset_permutations(0..k).collect_vec().len());
            println!("{:?}", distinct_derangements(0..k).collect_vec().len());
            println!("{:?}", derangements_iter(0..k, k).collect_vec().len());
        }
    }

    #[test]
    fn test_time() {
        use std::time::Instant;
        for k in 0..10 {
            let before = Instant::now();
            _ = derangements_range(k).len();
            let between = Instant::now();
            _ = derangements_range_fast(k).len();
            let between2 = Instant::now();
            _ = derangements(0..k, k).len();
            let after = Instant::now();
            _ = derangements_iter(0..k, k).collect_vec().len();
            let after2 = Instant::now();
            _ = distinct_derangements(0..k).collect_vec().len();
            let after3 = Instant::now();
            println!(
                "{:?}, range old {:?}, range new {:?}, non-range {:?}, iter: {:?}, distinct_iter: {:?}",
                k,
                between - before,
                between2 - between,
                after - between2,
                after2 - after,
                after3 - after2
            )
        }
    }
}
